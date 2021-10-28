use std::{collections::HashMap, process};

use crate::{
    config::Config,
    hygea::{CalendarResult, HygeaApiClient, PostalCodeResult},
    timeframe::Timeframe,
};

use chrono::{DateTime, NaiveDateTime};

pub struct Calendar {
    hygea_api_client: HygeaApiClient,
}

impl Calendar {
    pub fn new() -> Calendar {
        Calendar {
            hygea_api_client: HygeaApiClient::new(),
        }
    }

    pub async fn get(
        &self,
        postal_code: u16,
        timeframe: Timeframe,
    ) -> Result<Vec<CalendarResult>, reqwest::Error> {
        let data: Vec<CalendarResult> = self
            .hygea_api_client
            .get_calendar(postal_code, timeframe.start, timeframe.end)
            .await?
            .into_iter()
            .filter(|entry| {
                // Keep only the entry that are in the timeframe.
                // This is because of the API that return all the data even if the start and end
                // query params are specified.
                let date_parsed =
                    NaiveDateTime::parse_from_str(&entry.start, "%Y-%m-%dT%H:%M:%S%z");
                let start_timestamp = match date_parsed {
                    Ok(date) => date.timestamp(),
                    Err(error) => panic!("Unable to parse date from API: {:?}", error),
                };
                return start_timestamp >= timeframe.start && start_timestamp <= timeframe.end;
            }) // Format: 2019-01-03T00:00:00+01:00
            .collect();

        Ok(data)
    }

    pub async fn check_postal_code(
        &self,
        config: &Config,
    ) -> Result<PostalCodeResult, reqwest::Error> {
        let postal_codes = self
            .hygea_api_client
            .check_postal_code(config.postal_code)
            .await?;

        match postal_codes.len() {
            0 => {
                println!("No postal code found with the given one.");
                process::exit(1);
            }
            1 => Ok(postal_codes[0].clone()),
            _ => {
                println!("Write your postal code correctly!");
                let mut postal_codes_found = postal_codes
                    .into_iter()
                    .map(|pc| pc.value)
                    .collect::<Vec<String>>();
                postal_codes_found.sort();
                println!("We found many results corresponding to the given one:");
                println!("- {}", postal_codes_found.join("\n- "));
                process::exit(1);
            }
        }
    }

    pub async fn check_street(
        &self,
        postal_code: u16,
        street: String,
    ) -> Result<u16, reqwest::Error> {
        let streets = self
            .hygea_api_client
            .check_street(postal_code, street)
            .await?;

        match streets.len() {
            0 => {
                println!("No street found with the given one.");
                process::exit(1);
            }
            1 => Ok(streets[0].value),
            _ => {
                println!("Write street name correctly!");
                let mut streets_found = streets
                    .into_iter()
                    .map(|street| street.label)
                    .collect::<Vec<String>>();
                streets_found.sort();
                println!("we found many results corresponding to the given one:");
                println!("- {}", streets_found.join("\n- "));
                process::exit(1);
            }
        }
    }

    pub fn to_ical(
        data: Vec<CalendarResult>,
    ) -> Result<HashMap<String, &'static str>, chrono::ParseError> {
        let mut calendar_entries: HashMap<String, &'static str> = HashMap::new();

        for entry in data.iter() {
            if entry.class_name.contains(" om ") {
                calendar_entries.insert(
                    DateTime::parse_from_rfc3339(&entry.start)?
                        .format("%Y%m%d")
                        .to_string(),
                    "Collecte des sacs blancs",
                );

                continue;
            }

            if entry.class_name.contains(" pmc ") {
                calendar_entries.insert(
                    DateTime::parse_from_rfc3339(&entry.start)?
                        .format("%Y%m%d")
                        .to_string(),
                    r###"Collecte des :
    - Sacs blancs
    - Sacs PMC
    - Cartons
    "###,
                );

                continue;
            }
        }

        Ok(calendar_entries)
    }
}
