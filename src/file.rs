use std::collections::HashMap;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, Error};
use uuid::Uuid;

pub async fn ical_to_file(
    name: &'static str,
    ical_entries: HashMap<String, &'static str>,
    postal_code: u16,
    street_index: Option<u16>,
) -> Result<(), Error> {
    let mut buffer = File::create(name).await?;

    buffer
        .write(
            r###"BEGIN:VCALENDAR
VERSION:2.0
PRODID:www.example.com
X-PUBLISHED-TTL:P1W
"###
            .as_bytes(),
        )
        .await?;

    for (date, description) in ical_entries {
        buffer
            .write(
                format!(
                    r###"BEGIN:VEVENT
UID:{uuid}
DTSTART;TZID=Europe/Brussels;VALUE=DATE:{date}
SEQUENCE:0
TRANSP:OPAQUE
DTEND;TZID=Europe/Brussels;VALUE=DATE:{date}
URL:https://www.hygea.be/votre-calendrier-de-collecte.html?cp={postal_code}&streetIndex={street_index}
SUMMARY:Collecte des déchets
DESCRIPTION:{description}
X-MICROSOFT-CDO-ALLDAYEVENT:TRUE
END:VEVENT
"###,
                    uuid = Uuid::new_v4(),
                    date = date,
                    description = description.replace("\n", "\\n"),
                    postal_code = postal_code,
                    street_index = match street_index {
                        Some(index) => index.to_string(),
                        None => "".to_string(),
                    }
                )
                .as_bytes(),
            )
            .await?;
    }

    buffer.write("END:VCALENDAR".as_bytes()).await?;

    Ok(())
}
