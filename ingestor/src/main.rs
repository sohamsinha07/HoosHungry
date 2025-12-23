mod overpass;
mod off;

use rand::Rng;
use sqlx::Row;
use tracing_subscriber::EnvFilter;

const DEFAULT_ITEMS: &[&str] = &[
    "pizza", "salad", "burger", "sushi", "sandwich", "pasta", "taco", "burrito", "coffee", "yogurt"
];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let db_url = std::env::var("DATABASE_URL")?;
    let bbox = std::env::var("BBOX").unwrap_or_else(|_| "38.022,-78.520,38.050,-78.460".to_string());

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    tracing::info!("Fetching dining locations from Overpass with BBOX={}", bbox);
    let halls = overpass::fetch_halls(&bbox).await?;
    tracing::info!("Found {} locations", halls.len());

    // upsert halls
    for (osm_id, name, lat, lon, cuisine, opening_hours) in &halls {
        sqlx::query(
            r#"
            INSERT INTO dining_halls (osm_id, name, lat, lon, cuisine, opening_hours)
            VALUES ($1,$2,$3,$4,$5,$6)
            ON CONFLICT (osm_id) DO UPDATE SET
              name = EXCLUDED.name,
              lat = EXCLUDED.lat,
              lon = EXCLUDED.lon,
              cuisine = EXCLUDED.cuisine,
              opening_hours = EXCLUDED.opening_hours
            "#
        )
        .bind(osm_id)
        .bind(name)
        .bind(lat)
        .bind(lon)
        .bind(cuisine)
        .bind(opening_hours)
        .execute(&pool)
        .await?;
    }

    // create some menu items per hall (MVP)
    // choose 5 items per hall; enrich via OpenFoodFacts when possible
    let hall_rows = sqlx::query("SELECT id, name FROM dining_halls ORDER BY id LIMIT 25")
        .fetch_all(&pool)
        .await?;

    tracing::info!("Generating menu items for {} halls (MVP)", hall_rows.len());

    for r in hall_rows {
        let hall_id: i64 = r.get("id");
        let hall_name: String = r.get("name");

        // wipe existing items for repeatability (MVP)
        sqlx::query("DELETE FROM menu_items WHERE hall_id = $1")
            .bind(hall_id)
            .execute(&pool)
            .await?;

        let mut rng = rand::thread_rng();
        for _ in 0..5 {
            let term = DEFAULT_ITEMS[rng.gen_range(0..DEFAULT_ITEMS.len())];

            let (name, brand, calories, allergens, vegan, vegetarian) =
                match off::search_product(term).await {
                    Ok(Some(tuple)) => tuple,
                    _ => (term.to_string(), None, None, None, None, None),
                };

            // popularity_score in [0,1] for MVP (stable-ish but random per run)
            let popularity_score: f64 = rng.gen_range(0.05..0.95);

            sqlx::query(
                r#"
                INSERT INTO menu_items (hall_id, name, brand, calories, allergens, vegan, vegetarian, popularity_score)
                VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
                "#
            )
            .bind(hall_id)
            .bind(format!("{} ({})", name, hall_name))
            .bind(brand)
            .bind(calories)
            .bind(allergens)
            .bind(vegan)
            .bind(vegetarian)
            .bind(popularity_score)
            .execute(&pool)
            .await?;
        }
    }

    tracing::info!("Ingestion complete.");
    Ok(())
}
