use std::collections::HashMap;

use crate::{config::Config, timeframe::Timeframe};

use chrono::DateTime;
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
            .json()
            .await?;

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
                    "Collecte des sacs blancs, des sacs bleus (PMC) et papiers cartons",
                );

                continue;
            }
        }

        Ok(calendar_entries)
    }
}
