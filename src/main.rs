use std::process;

use chrono::{Datelike, Utc};
use clap::Parser;

use hygea_to_ical::{
    calendar::Calendar, config::Config, file::ical_to_file, timeframe::generate_timeframe,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = Config::parse();

    let current_year = Utc::now().year();

    let timeframe = generate_timeframe(current_year);

    let calendar_client = Calendar::new();

    let postal_code_result = calendar_client.check_postal_code(&config).await?;
    let postal_code = u16::from_str_radix(&postal_code_result.value, 10).unwrap();

    let (calendar, street_index) = match postal_code_result.alone {
        0 => {
            let street_name = match config.street {
                Some(ref name) => name.to_owned(),
                None => {
                    println!("Please provide your street name to get your planning.");
                    process::exit(1);
                }
            };
            let street_index = calendar_client
                .check_street(postal_code, street_name)
                .await?;
            (
                calendar_client.get(street_index, timeframe).await?,
                Some(street_index),
            )
        }
        1 => (calendar_client.get(postal_code, timeframe).await?, None),
        _ => panic!("'alone' is always between 0 and 1. This should never happened."),
    };

    let ical_entries = Calendar::to_ical(calendar)?;

    ical_to_file("hygea.ics", ical_entries, postal_code, street_index).await?;

    Ok(())
}
