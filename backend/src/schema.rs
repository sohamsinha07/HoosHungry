use async_graphql::{InputObject, SimpleObject};

#[derive(SimpleObject, Clone)]
pub struct DiningHall {
    pub id: i64,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub cuisine: Option<String>,
    pub opening_hours: Option<String>,
}

#[derive(SimpleObject, Clone)]
pub struct MenuItem {
    pub id: i64,
    pub hall_id: i64,
    pub name: String,
    pub calories: Option<i32>,
    pub vegan: Option<bool>,
    pub vegetarian: Option<bool>,
    pub popularity_score: f64,
    pub score: f64,
}

#[derive(InputObject, Clone)]
pub struct PreferenceInput {
    pub vegan_only: Option<bool>,
    pub vegetarian_only: Option<bool>,
    pub max_calories: Option<i32>,
    /// Optional text query (e.g., "pizza")
    pub query: Option<String>,
    /// Weight for popularity in ranking (0..=1)
    pub popularity_weight: Option<f64>,
    /// Weight for dietary match in ranking (0..=1)
    pub dietary_weight: Option<f64>,
    /// Weight for calorie preference in ranking (0..=1)
    pub calorie_weight: Option<f64>,
}
