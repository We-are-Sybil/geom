use colored::Colorize;
use csv::StringRecord;
use sha2::{Sha256, Digest};

pub fn find_columns(headers: &StringRecord, column_names: &[String]) -> Vec<Result<String, String>> {
    column_names.iter().map(|col_name| {
        if headers.iter().any(|h| h == col_name) {
            Ok(col_name.clone())
        } else {
            Err(format!("{} Column '{}' not found in the csv", "[✗]".red().bold(), format!("{}", col_name).bold().underline().on_red()))
        }
    }).collect()
}

pub fn print_column_results(results: &[Result<String, String>]) {
    for result in results {
        match result {
            Ok(col_name) => println!("{} Column '{}' found in the csv", "[+] Process:".blue().bold(), format!("{}", col_name).bold().underline().on_blue()),
            Err(msg) => println!("{}", msg),
        }
    }
}

pub fn hash_with_salt(action: &str, salt: &str) -> String {
    let combined = format!("{}.{}", action, salt);
    let mut hasher = Sha256::new();
    hasher.update(combined);
    format!("{:x}", hasher.finalize())
}
