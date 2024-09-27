use std::path::PathBuf;
use std::str::FromStr;
use csv::Writer;
use clap::ValueEnum;

#[derive(Clone, ValueEnum)]
pub enum SavingType {
    Csv,
}

impl FromStr for SavingType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(SavingType::Csv),
            _ => Err(format!("Unknown saving type: {}", s)),
        }
    }
}

// Struct to hold the data for a single record
pub struct Record {
    pub batch_id: String,
    pub angle: f64,
    pub action: String,
    pub longitude: f64,
    pub latitude: f64,
    pub elevation: f64,
    pub is_original: bool,
}

// Trait for different saving strategies
pub trait SaveStrategy {
    fn save(&self, records: &[Record], output_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>>;
}

// CSV saving strategy
pub struct CsvSaveStrategy;

impl SaveStrategy for CsvSaveStrategy {
    fn save(&self, records: &[Record], output_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = Writer::from_path(output_path)?;
        
        // Write header
        writer.write_record(&["BatchID", "Angle", "Action", "Processed_Longitude", "Processed_Latitude", "Processed_Elevation", "Is_Original"])?;
        
        // Write records
        for record in records {
            writer.write_record(&[
                &record.batch_id,
                &record.angle.to_string(),
                &record.action,
                &record.longitude.to_string(),
                &record.latitude.to_string(),
                &record.elevation.to_string(),
                &record.is_original.to_string(),
            ])?;
        }
        
        writer.flush()?;
        Ok(())
    }
}

// Factory to create the appropriate save strategy
pub fn create_save_strategy(saving_type: SavingType) -> Box<dyn SaveStrategy> {
    match saving_type {
        SavingType::Csv => Box::new(CsvSaveStrategy),
        // Add more cases here as we implement more saving types
    }
}

// Main struct to handle saving
pub struct DataSaver {
    strategy: Box<dyn SaveStrategy>,
    output_path: PathBuf,
}

impl DataSaver {
    pub fn new(saving_type: SavingType, output_path: PathBuf) -> Self {
        let strategy = create_save_strategy(saving_type);
        Self { strategy, output_path }
    }
    
    pub fn save(&self, records: &[Record]) -> Result<(), Box<dyn std::error::Error>> {
        self.strategy.save(records, &self.output_path)
    }
}
