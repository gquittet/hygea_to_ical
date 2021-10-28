use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostalCodeResult {
    pub value: String,
    pub alone: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CalendarResult {
    pub title: String,
    pub class_name: String,
    pub start: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreetResult {
    pub label: String,
    pub value: u16,
}

pub struct HygeaApiClient {
    client: reqwest::Client,
    host: &'static str,
}

impl HygeaApiClient {
    pub fn new() -> HygeaApiClient {
        HygeaApiClient {
            client: reqwest::Client::new(),
            host: "https://www.hygea.be",
        }
    }

    pub async fn get_calendar(
        &self,
        postal_code: u16,
        start: i64,
        end: i64,
    ) -> Result<Vec<CalendarResult>, reqwest::Error> {
        let uri = match postal_code > 1000 {
            true => format!("{host}/displaycalws.html", host = self.host),
            false => format!("{host}/displaycal.html", host = self.host),
        };

        self.client
            .get(uri)
            .query(&[
                ("street", postal_code.to_string()),
                ("start", start.to_string()),
                ("end", end.to_string()),
            ])
            .send()
            .await?
            .json::<Vec<CalendarResult>>()
            .await
    }

    pub async fn check_postal_code(
        &self,
        postal_code: u16,
    ) -> Result<Vec<PostalCodeResult>, reqwest::Error> {
        let uri = format!("{host}/callcp.html", host = self.host);

        self.client
            .get(&uri)
            .query(&[("term", postal_code.to_string())])
            .send()
            .await?
            .json::<Vec<PostalCodeResult>>()
            .await
    }

    pub async fn check_street(
        &self,
        postal_code: u16,
        street: String,
    ) -> Result<Vec<StreetResult>, reqwest::Error> {
        let uri = format!("{host}/callstreet.html", host = self.host);

        self.client
            .get(uri)
            .query(&[
                ("cp", postal_code.to_string()),
                ("term", street.to_string()),
            ])
            .send()
            .await?
            .json::<Vec<StreetResult>>()
            .await
    }
}
