mod args;
mod api;
mod csv_utils;
mod geometry;
mod error;
mod save_strategy;

use args::Args;
use api::fetch_elevation;
use csv_utils::{find_columns, print_column_results, hash_with_salt};
use geometry::{date_to_angle, process_points};
use error::GeomError;
use save_strategy::{Record, DataSaver};

use clap::Parser;
use colored::Colorize;
use colored::control::set_override;
use csv::Reader;
use ndarray::{Array2, Axis};
use chrono::NaiveDate;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), GeomError> {
    set_override(true);
    let args = Args::parse();

    println!("{} {:?}", "[+] Processing file:".bold().underline(), args.input);

    let mut reader = Reader::from_path(&args.input)?;
    let headers = reader.headers()?.clone();

    let columns_to_find = vec![
        args.date_column.clone(),
        args.action_column.clone(),
        args.longitude_column.clone(),
        args.latitude_column.clone(),
    ];

    let results = find_columns(&headers, &columns_to_find);

    print_column_results(&results);
    if results.iter().any(|result| result.is_err()) {
        return Err(GeomError::ColumnNotFound);
    }

    let mut all_points = Array2::zeros((0, 3));
    let mut records = reader.records();
    let batch_size = 15;

    let data_saver = DataSaver::new(args.output_format.clone(), args.output.clone());

    println!("{} {}", "[+] Info:".bold().underline(), "Altitude is being fetched and data is being processed.");
    
    let mut all_processed_records = Vec::new();
    let mut batch_number = 0;

    while let Some(batch_result) = records.by_ref().take(batch_size).collect::<Result<Vec<_>, _>>().ok() {
        if batch_result.is_empty() {
            break;
        }

        batch_number += 1;
        let batch_id = Uuid::new_v4().to_string();
        
        println!("{} Starting to process batch {} with ID: {}", 
            "[+] Process:".yellow().bold(), 
            batch_number.to_string().magenta(), 
            batch_id.green());

        let mut batch_points = Vec::new();
        let mut batch_data = Vec::new();

        for record in &batch_result {
            let date_str = record.get(headers.iter().position(|h| h == &args.date_column).unwrap()).unwrap();
            let date = NaiveDate::parse_from_str(date_str, &args.date_format)?;
            let angle = date_to_angle(date);
            
            let action = record.get(headers.iter().position(|h| h == &args.action_column).unwrap()).unwrap().to_string();
            let latitude: f64 = record.get(headers.iter().position(|h| h == &args.latitude_column).unwrap()).unwrap().parse()?;
            let longitude: f64 = record.get(headers.iter().position(|h| h == &args.longitude_column).unwrap()).unwrap().parse()?;

            let elevation = fetch_elevation(latitude, longitude, &args.host, &args).await?;

            batch_points.push([longitude, latitude, elevation]);
            batch_data.push((angle, action));
        }

        if !batch_points.is_empty() {
            let batch_array = Array2::from_shape_vec((batch_points.len(), 3), batch_points.into_iter().flatten().collect())?;
            
            println!("{} Discretizing and processing points for batch {}", 
                "[+] Process:".yellow().bold(), 
                batch_number.to_string().magenta());
            
            let centered_points = process_points(&batch_array, args.discretization_points, &args.host, &args).await?;

            // Calculate angles for discretized points
            let mut all_angles = Vec::new();
            for i in 0..batch_data.len() - 1 {
                let start_angle = batch_data[i].0;
                let end_angle = batch_data[i + 1].0;
                let angle_step = (end_angle - start_angle) / (args.discretization_points as f64 + 1.0);
                
                all_angles.push(start_angle);
                for j in 1..=args.discretization_points {
                    all_angles.push(start_angle + angle_step * j as f64);
                }
            }
            all_angles.push(batch_data.last().unwrap().0);

            // Process records
            for (i, row) in centered_points.rows().into_iter().enumerate() {
                let is_original = i % (args.discretization_points + 1) == 0;
                let action = if is_original {
                    hash_with_salt(&batch_data[i / (args.discretization_points + 1)].1, &args.salt)
                } else {
                    String::new()
                };

                all_processed_records.push(Record {
                    batch_id: batch_id.clone(),
                    angle: all_angles[i],
                    action,
                    longitude: row[0],
                    latitude: row[1],
                    elevation: row[2],
                    is_original,
                });
            }

            all_points.append(Axis(0), centered_points.view())?;
        }

        println!("{} Completed processing batch {} with ID: {}", 
            "[✓] Success:".green().bold(), 
            batch_number.to_string().yellow(), 
            batch_id.green());
    }

    // Save processed records using the DataSaver
    data_saver.save(&all_processed_records).map_err(|e| GeomError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    println!("{} {}", "[✓] Success:".green().bold(), format!("Data processing complete. Output saved to '{}'.", args.output.display()));

    Ok(())
}
