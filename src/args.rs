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

    /// Column name for Action
    #[arg(short = 'a', long, default_value = "Accion")]
    pub action_column: String,

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
}
