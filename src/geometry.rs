use chrono::{Datelike, NaiveDate};
use ndarray::Array2;
use crate::api::fetch_elevation;
use crate::error::GeomError;
use crate::args::Args;

pub fn date_to_angle(date: NaiveDate) -> f64 {
    let days_in_year = if date.year() % 4 == 0 { 366 } else { 365 };
    let day_of_year = date.ordinal() as f64;
    (day_of_year / days_in_year as f64) * 360.0
}

fn lat_long_alt_to_xyz(points: &Array2<f64>) -> Array2<f64> {
    let r: f64 = 6_371_000.0;
    
    let mut xyz = Array2::zeros(points.raw_dim());
    for (i, row) in points.rows().into_iter().enumerate() {
        let lat = row[0].to_radians();
        let long = row[1].to_radians();
        let alt = row[2];
        let r_alt = r + alt;
        xyz[[i, 0]] = r_alt * lat.cos() * long.cos();
        xyz[[i, 1]] = r_alt * lat.cos() * long.sin();
        xyz[[i, 2]] = r_alt * lat.sin();
    }
    xyz
}

async fn discretize_points(points: &Array2<f64>, discretization_points: usize, host: &str, args: &Args) -> Result<Array2<f64>, GeomError> {
    let mut discretized_points = Vec::new();
    
    for window in points.windows((2, 3)) {
        let start = window.row(0);
        let end = window.row(1);
        
        discretized_points.push(start.to_vec());
        
        for i in 1..=discretization_points {
            let t = i as f64 / (discretization_points as f64 + 1.0);
            let lat = start[0] * (1.0 - t) + end[0] * t;
            let lon = start[1] * (1.0 - t) + end[1] * t;
            let elevation = fetch_elevation(lat, lon, host, args).await
                .map_err(|e| GeomError::ElevationFetchError(e.to_string()))?;
            discretized_points.push(vec![lat, lon, elevation]);
        }
    }
    
    discretized_points.push(points.row(points.nrows() - 1).to_vec());

    Array2::from_shape_vec((discretized_points.len(), 3), discretized_points.into_iter().flatten().collect())
        .map_err(|e| GeomError::DiscretizationError(e.to_string()))
}

pub async fn process_points(points: &Array2<f64>, discretization_points: usize, host: &str, args: &Args) -> Result<Array2<f64>, GeomError> {
    if points.is_empty() {
        return Ok(Array2::zeros((0, 3)));
    }

    let discretized_array = discretize_points(points, discretization_points, host, args).await?;
    let xyz_points = lat_long_alt_to_xyz(&discretized_array);

    let min_point = xyz_points.rows().into_iter()
        .min_by(|a, b| {
            let mag_a = a.iter().map(|&x| x * x).sum::<f64>();
            let mag_b = b.iter().map(|&x| x * x).sum::<f64>();
            mag_a.partial_cmp(&mag_b).unwrap()
        })
        .unwrap();

    Ok(&xyz_points - &min_point)
}
