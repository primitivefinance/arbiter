#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Used to generate price paths for a simulation.
//! Managers will be able to read from this data to change prices of for infinitely liquid pools.

use std::{error::Error, fs::File};

use csv::ReaderBuilder;

/// Import CSV file of price data
/// # Arguments
/// * `file_path` - path to csv file of price data (String)
/// # Returns
/// * `price_data` - Vector of prices. (Vec<f64>)
pub fn import_price_from_csv(file_path: &str) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut price_data: Vec<f64> = Vec::new();
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().from_reader(file);

    for result in reader.deserialize() {
        let num: f64 = result?;
        price_data.push(num);
    }

    Ok(price_data)
}
