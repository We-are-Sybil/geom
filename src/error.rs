use thiserror::Error;
use std::num::ParseFloatError;
use csv::Error as CsvError;
use reqwest::Error as ReqwestError;
use ndarray::ShapeError;
use chrono::ParseError as ChronoParseError;

#[derive(Error, Debug)]
pub enum GeomError {
    #[error("CSV error: {0}")]
    CsvError(#[from] CsvError),

    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] ReqwestError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Parse float error: {0}")]
    ParseFloatError(#[from] ParseFloatError),

    #[error("Shape error: {0}")]
    ShapeError(#[from] ShapeError),

    #[error("Column not found")]
    ColumnNotFound,

    #[error("Chrono parse error: {0}")]
    ChronoParseError(#[from] ChronoParseError),
}
