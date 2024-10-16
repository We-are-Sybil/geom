use clap::Parser;
use std::path::PathBuf;
use crate::save_strategy::SavingType;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input CSV file path
    #[arg(short, long)]
    pub input: PathBuf,

    /// Column name for Date
    #[arg(short = 'd', long, default_value = "Fecha")]
    pub date_column: String,

    /// Date format string (e.g., "%d/%m/%Y" for DD/MM/YYYY) 
    #[arg(long, default_value = "%d/%m/%Y")]
    pub date_format: String,

    /// Column name for Battalion
    #[arg(short = 'b', long, default_value = "Batallon")]
    pub battalion_column: String,

    /// Column name for Platoon
    #[arg(short = 'p', long, default_value = "Peloton")]
    pub platoon_column: String,

    /// Column name for Company
    #[arg(short = 'c', long, default_value = "Compañia")]
    pub company_column: String,

    /// Column name for Latitude
    #[arg(short = 'x', long, default_value = "Latitud")]
    pub latitude_column: String,

    /// Column name for Longitude
    #[arg(short = 'y', long, default_value = "Longitud")]
    pub longitude_column: String,

    /// Host name for API
    #[arg(short = 'H', long, default_value = "api.open-elevation.com")]
    pub host: String,

    /// Output file path
    #[arg(short = 'o', long, default_value = "output.csv")]
    pub output: PathBuf,

    /// Output format
    #[arg(short = 'f', long, value_enum, default_value_t = SavingType::Csv)]
    pub output_format: SavingType,

    /// Hashing salt
    #[arg(short = 's', long)]
    pub salt: String,

    /// Number of points to add when discretizing between each pair of original points
    #[arg(short = 'n', long = "num-discretize", default_value_t = 20)]
    pub discretization_points: usize,

    /// Enable verbose debugging for all requests
    #[arg(long, short = 'D')]
    pub debug_verbose: bool,

    /// Enable debugging only for errors
    #[arg(long, short = 'e')]
    pub debug_on_error: bool,

    /// Mapping file path for original to hashed values
    #[arg(short = 'm', long, default_value = "mapping.csv")]
    pub mapping_output: PathBuf,
}
