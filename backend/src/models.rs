use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiningHallRow {
    pub id: i64,
    pub osm_id: String,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub cuisine: Option<String>,
    pub opening_hours: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItemRow {
    pub id: i64,
    pub hall_id: i64,
    pub name: String,
    pub brand: Option<String>,
    pub calories: Option<i32>,
    pub allergens: Option<Vec<String>>,
    pub vegan: Option<bool>,
    pub vegetarian: Option<bool>,
    pub popularity_score: f64,
}
