use std::env;

use chrono::{Datelike, Utc};

use hygea_to_ical_rust::{
    calendar::CalendarHTTPClient, config::parse_config, file::ical_to_file,
    timeframe::generate_timeframe,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    let current_year = Utc::now().year();

    let timeframe = generate_timeframe(current_year);

    let calendar_client = CalendarHTTPClient::new();

    let calendar = calendar_client.get(config, timeframe).await?;

    let ical_entries = CalendarHTTPClient::to_ical(calendar)?;

    ical_to_file("hygea.ics", ical_entries, config).await?;

    Ok(())
}
