use chrono::{Datelike, NaiveDate};
use ndarray::Array2;

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
        xyz[[i, 2]] = alt;
    }
    xyz
}

pub fn process_points(points: &Array2<f64>) -> Array2<f64> {
    if points.is_empty() {
        return Array2::zeros((0, 3));
    }

    let xyz_points = lat_long_alt_to_xyz(points);

    let min_point = xyz_points.rows().into_iter()
        .min_by(|a, b| {
            let mag_a = a.iter().map(|&x| x * x).sum::<f64>();
            let mag_b = b.iter().map(|&x| x * x).sum::<f64>();
            mag_a.partial_cmp(&mag_b).unwrap()
        })
        .unwrap();

    &xyz_points - &min_point
}
