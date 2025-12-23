use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OverpassResponse {
    pub elements: Vec<Element>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Element {
    #[serde(rename = "node")]
    Node { id: i64, lat: f64, lon: f64, tags: Option<Tags> },

    #[serde(rename = "way")]
    Way { id: i64, center: Option<Center>, tags: Option<Tags> },

    #[serde(rename = "relation")]
    Relation { id: i64, center: Option<Center>, tags: Option<Tags> },
}

#[derive(Debug, Deserialize)]
pub struct Center {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Tags {
    pub name: Option<String>,
    pub amenity: Option<String>,
    pub cuisine: Option<String>,
    pub opening_hours: Option<String>,
}

pub async fn fetch_halls(bbox: &str) -> anyhow::Result<Vec<(String, String, f64, f64, Option<String>, Option<String>)>> {
    // bbox: "south,west,north,east"
    let query = format!(r#"
[out:json][timeout:25];
(
  node["amenity"~"restaurant|fast_food|cafe"]({bbox});
  way["amenity"~"restaurant|fast_food|cafe"]({bbox});
  relation["amenity"~"restaurant|fast_food|cafe"]({bbox});
);
out center tags;
"#, bbox=bbox);

    let client = reqwest::Client::new();
    let resp = client
        .post("https://overpass-api.de/api/interpreter")
        .body(query)
        .send()
        .await?
        .error_for_status()?
        .json::<OverpassResponse>()
        .await?;

    let mut halls = Vec::new();

    for el in resp.elements {
        match el {
            Element::Node { id, lat, lon, tags } => {
                let name = tags.as_ref().and_then(|t| t.name.clone()).unwrap_or_else(|| format!("OSM Place {}", id));
                let cuisine = tags.as_ref().and_then(|t| t.cuisine.clone());
                let oh = tags.as_ref().and_then(|t| t.opening_hours.clone());
                halls.push((format!("node:{}", id), name, lat, lon, cuisine, oh));
            }
            Element::Way { id, center, tags } |
            Element::Relation { id, center, tags } => {
                if let Some(c) = center {
                    let name = tags.as_ref().and_then(|t| t.name.clone()).unwrap_or_else(|| format!("OSM Place {}", id));
                    let cuisine = tags.as_ref().and_then(|t| t.cuisine.clone());
                    let oh = tags.as_ref().and_then(|t| t.opening_hours.clone());
                    halls.push((format!("obj:{}", id), name, c.lat, c.lon, cuisine, oh));
                }
            }
        }
    }

    // De-dupe by osm_id
    halls.sort_by(|a,b| a.0.cmp(&b.0));
    halls.dedup_by(|a,b| a.0 == b.0);

    Ok(halls)
}
