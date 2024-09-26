use serde::Deserialize;
use crate::error::GeomError;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    results: Vec<ApiLocation>,
}

#[derive(Debug, Deserialize)]
struct ApiLocation {
    elevation: Option<f64>,
}

pub async fn fetch_elevation(latitude: f64, longitude: f64, host: &str) -> Result<f64, GeomError> {
    let url = format!(
        "https://{}/api/v1/lookup?locations={},{}",
        host, latitude, longitude
    );

    let response: ApiResponse = reqwest::get(&url).await?.json().await?;

    Ok(response.results.first().and_then(|loc| loc.elevation).unwrap_or(0.0))
}
