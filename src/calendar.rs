use std::collections::HashMap;

use crate::{config::Config, timeframe::Timeframe};

use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEntry {
    pub title: String,
    pub class_name: String,
    pub start: String,
}

pub struct CalendarHTTPClient {
    client: reqwest::Client,
    uri: String,
}

impl CalendarHTTPClient {
    pub fn new() -> CalendarHTTPClient {
        CalendarHTTPClient {
            client: reqwest::Client::new(),
            uri: "https://www.hygea.be/displaycalws.html".to_string(),
        }
    }

    pub async fn get(
        &self,
        config: Config,
        timeframe: Timeframe,
    ) -> Result<Vec<CalendarEntry>, reqwest::Error> {
        let data: Vec<CalendarEntry> = self
            .client
            .get(&self.uri)
            .query(&[
                ("street", config.postal_code.to_string()),
                ("start", timeframe.start.to_string()),
                ("end", timeframe.end.to_string()),
            ])
            .send()
            .await?
            .json::<Vec<CalendarEntry>>()
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

    pub fn to_ical(
        data: Vec<CalendarEntry>,
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
