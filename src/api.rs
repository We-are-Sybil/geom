use serde::Deserialize;
use crate::error::GeomError;
use crate::args::Args;
use colored::Colorize;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    results: Vec<ApiLocation>,
}

#[derive(Debug, Deserialize)]
struct ApiLocation {
    elevation: Option<f64>,
}

pub async fn fetch_elevation(latitude: f64, longitude: f64, host: &str, args: &Args) -> Result<f64, GeomError> {
    let url = format!(
        "http://{}/api/v1/lookup?locations={},{}",
        host, latitude, longitude
    );

    if args.debug_verbose {
        println!("{} Request URL: {}", "[?] Debug:".yellow().bold(), url);
    }

    let response = reqwest::get(&url).await?;
    
    if args.debug_verbose {
        println!("{} Response status: {}", "[?] Debug:".yellow().bold(), response.status());
        println!("{} Response headers: {:#?}", "[?] Debug:".yellow().bold(), response.headers());
    }

    let body = response.text().await?;
    
    if args.debug_verbose {
        println!("{} Response body: {}", "[?] Debug:".yellow().bold(), body);
    }

    let api_response: Result<ApiResponse, _> = serde_json::from_str(&body);

    match api_response {
        Ok(response) => {
            Ok(response.results.first().and_then(|loc| loc.elevation).unwrap_or(0.0))
        },
        Err(e) => {
            if args.debug_verbose || args.debug_on_error {
                println!("{} Parse error: {}", "[!] Error:".red().bold(), e);
                println!("{} Request URL: {}", "[?] Debug:".yellow().bold(), url);
                println!("{} Response body: {}", "[?] Debug:".yellow().bold(), body);
            }
            Err(GeomError::ElevationFetchError(format!("Failed to parse JSON: {}", e)))
        }
    }
}
