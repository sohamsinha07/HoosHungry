use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OffSearchResponse {
    pub products: Vec<OffProduct>,
}

#[derive(Debug, Deserialize)]
pub struct OffProduct {
    #[serde(default)]
    pub product_name: Option<String>,
    #[serde(default)]
    pub brands: Option<String>,
    #[serde(default)]
    pub nutriments: Option<Nutriments>,
    #[serde(default)]
    pub allergens: Option<String>,
    #[serde(default)]
    pub ingredients_analysis_tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Nutriments {
    #[serde(default)]
    pub energy_kcal_100g: Option<f64>,
}

pub async fn search_product(term: &str) -> anyhow::Result<Option<(String, Option<String>, Option<i32>, Option<Vec<String>>, Option<bool>, Option<bool>)>> {
    let url = format!(
        "https://world.openfoodfacts.org/cgi/search.pl?search_terms={}&search_simple=1&action=process&json=1&page_size=1",
        urlencoding::encode(term)
    );

    let resp = reqwest::get(url).await?.error_for_status()?.json::<OffSearchResponse>().await?;
    let p = resp.products.into_iter().next()?;

    let name = p.product_name.unwrap_or_else(|| term.to_string());
    let brand = p.brands;

    let calories = p.nutriments
        .and_then(|n| n.energy_kcal_100g)
        .map(|kcal| kcal.round() as i32);

    let allergens = p.allergens.map(|a| {
        a.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect::<Vec<_>>()
    });

    let tags = p.ingredients_analysis_tags.unwrap_or_default();
    let vegan = Some(tags.iter().any(|t| t.contains("vegan")));
    let vegetarian = Some(tags.iter().any(|t| t.contains("vegetarian")));

    Ok(Some((name, brand, calories, allergens, vegan, vegetarian)))
}
