use async_graphql::{Context, Object, Schema};
use sqlx::Row;

use crate::db::PgPool;
use crate::redis_cache::RedisCache;
use crate::schema::{DiningHall, MenuItem, PreferenceInput};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub cache: RedisCache,
}

pub type AppSchema = Schema<QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

pub struct QueryRoot;

fn clamp01(x: f64) -> f64 {
    if x < 0.0 { 0.0 } else if x > 1.0 { 1.0 } else { x }
}

/// Deterministic scoring:
/// score = w_pop * popularity + w_diet * dietary_match + w_cal * calorie_score
fn compute_score(
    popularity: f64,
    dietary_match: f64,
    calorie_score: f64,
    w_pop: f64,
    w_diet: f64,
    w_cal: f64,
) -> f64 {
    w_pop * popularity + w_diet * dietary_match + w_cal * calorie_score
}

#[Object]
impl QueryRoot {
    async fn dining_halls(&self, ctx: &Context<'_>, query: Option<String>) -> async_graphql::Result<Vec<DiningHall>> {
        let state = ctx.data::<AppState>()?;
        let q = query.unwrap_or_default();

        let rows = if q.trim().is_empty() {
            sqlx::query("SELECT id,name,lat,lon,cuisine,opening_hours FROM dining_halls ORDER BY name LIMIT 50")
                .fetch_all(&state.db).await?
        } else {
            sqlx::query("SELECT id,name,lat,lon,cuisine,opening_hours FROM dining_halls WHERE name ILIKE $1 ORDER BY name LIMIT 50")
                .bind(format!("%{}%", q))
                .fetch_all(&state.db).await?
        };

        Ok(rows.into_iter().map(|r| DiningHall {
            id: r.get("id"),
            name: r.get("name"),
            lat: r.get("lat"),
            lon: r.get("lon"),
            cuisine: r.get("cuisine"),
            opening_hours: r.get("opening_hours"),
        }).collect())
    }

    async fn recommend(
        &self,
        ctx: &Context<'_>,
        hall_id: i64,
        prefs: PreferenceInput,
        limit: Option<i32>,
    ) -> async_graphql::Result<Vec<MenuItem>> {
        let state = ctx.data::<AppState>()?;
        let limit = limit.unwrap_or(15).clamp(1, 50);

        let vegan_only = prefs.vegan_only.unwrap_or(false);
        let vegetarian_only = prefs.vegetarian_only.unwrap_or(false);
        let max_calories = prefs.max_calories;
        let text_q = prefs.query.unwrap_or_default();

        // weights (default balanced, clamp to avoid weird values)
        let w_pop = clamp01(prefs.popularity_weight.unwrap_or(0.45));
        let w_diet = clamp01(prefs.dietary_weight.unwrap_or(0.35));
        let w_cal = clamp01(prefs.calorie_weight.unwrap_or(0.20));

        // Pull candidates with SQL filtering
        // Keep this simple for MVP; rank in Rust for deterministic scoring.
        let mut sql = String::from(
            "SELECT id,hall_id,name,calories,vegan,vegetarian,popularity_score
             FROM menu_items
             WHERE hall_id = $1"
        );

        if vegan_only {
            sql.push_str(" AND vegan = true");
        }
        if vegetarian_only {
            sql.push_str(" AND vegetarian = true");
        }
        if max_calories.is_some() {
            sql.push_str(" AND (calories IS NULL OR calories <= $2)");
        }
        if !text_q.trim().is_empty() {
            sql.push_str(" AND name ILIKE $3");
        }
        sql.push_str(" LIMIT 200");

        let rows = match (max_calories, text_q.trim().is_empty()) {
            (Some(maxc), false) => {
                sqlx::query(&sql)
                    .bind(hall_id)
                    .bind(maxc)
                    .bind(format!("%{}%", text_q))
                    .fetch_all(&state.db).await?
            }
            (Some(maxc), true) => {
                // still need $2 binding; no $3
                let sql2 = sql.replace(" AND name ILIKE $3", "");
                sqlx::query(&sql2)
                    .bind(hall_id)
                    .bind(maxc)
                    .fetch_all(&state.db).await?
            }
            (None, false) => {
                let sql2 = sql.replace(" AND (calories IS NULL OR calories <= $2)", "");
                sqlx::query(&sql2)
                    .bind(hall_id)
                    .bind(format!("%{}%", text_q))
                    .fetch_all(&state.db).await?
            }
            (None, true) => {
                let mut sql2 = sql.clone();
                sql2 = sql2.replace(" AND (calories IS NULL OR calories <= $2)", "");
                sql2 = sql2.replace(" AND name ILIKE $3", "");
                sqlx::query(&sql2)
                    .bind(hall_id)
                    .fetch_all(&state.db).await?
            }
        };

        // Rank deterministically in Rust
        let mut items: Vec<MenuItem> = rows.into_iter().map(|r| {
            let calories: Option<i32> = r.get("calories");
            let vegan: Option<bool> = r.get("vegan");
            let vegetarian: Option<bool> = r.get("vegetarian");
            let pop: f64 = r.get("popularity_score");

            // dietary match: 1 if meets requested diet, else 0; if no diet requested, 0.5 neutral
            let dietary_match = if vegan_only {
                if vegan.unwrap_or(false) { 1.0 } else { 0.0 }
            } else if vegetarian_only {
                if vegetarian.unwrap_or(false) { 1.0 } else { 0.0 }
            } else {
                0.5
            };

            // calorie score: prefer lower calories if max_calories set, else neutral
            let calorie_score = match (max_calories, calories) {
                (Some(maxc), Some(c)) if maxc > 0 => {
                    let ratio = (maxc - c).max(0) as f64 / maxc as f64; // 0..1
                    clamp01(ratio)
                }
                (Some(_), None) => 0.6,
                _ => 0.5
            };

            let score = compute_score(pop, dietary_match, calorie_score, w_pop, w_diet, w_cal);

            MenuItem {
                id: r.get("id"),
                hall_id: r.get("hall_id"),
                name: r.get("name"),
                calories,
                vegan,
                vegetarian,
                popularity_score: pop,
                score,
            }
        }).collect();

        items.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        items.truncate(limit as usize);

        Ok(items)
    }
}
