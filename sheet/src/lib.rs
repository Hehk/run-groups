extern crate google_sheets4 as sheets4;

use serde::Deserialize;
use serde_json::Value;
use sheets4::Sheets;
use sheets4::{oauth2, oauth2::ServiceAccountAuthenticator};

#[derive(Debug, Deserialize, Clone)]
struct Meetup {
    #[serde(rename = "Running Group")]
    group: String,
    #[serde(rename = "Day of the Week")]
    day: String,
    #[serde(rename = "Time")]
    time: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Location")]
    location: String,
}

#[derive(Debug)]
struct Meetups {
    monday: Vec<Meetup>,
    tuesday: Vec<Meetup>,
    wednesday: Vec<Meetup>,
    thursday: Vec<Meetup>,
    friday: Vec<Meetup>,
    saturday: Vec<Meetup>,
    sunday: Vec<Meetup>,
}

pub async fn create_sheets(
    service_account_path: String,
) -> Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
    let creds = oauth2::read_service_account_key(service_account_path)
        .await
        .expect("Can't read credential, an error occurred");

    let sa = ServiceAccountAuthenticator::builder(creds)
        .build()
        .await
        .expect("There was an error, trying to build connection with authenticator");

    Sheets::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .enable_http2()
                .build(),
        ),
        sa,
    )
}

pub async fn get_sheet(
    hub: Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    spreadsheet_id: &String,
    sheet_name: &String,
) -> Result<Vec<Vec<Value>>, String> {
    let resp = hub
        .spreadsheets()
        .values_get(spreadsheet_id, sheet_name)
        .doit()
        .await;

    match resp {
        Ok((_, response)) => {
            let values = response.values;
            match values {
                Some(values) => {
                    return Ok(values);
                }
                None => {
                    return Err("No Values".to_string());
                }
            }
        }
        Err(e) => {
            let error = format!("There was an error: {:?}", e);
            return Err(error);
        }
    }
}

async fn get_meetups(
    hub: Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    spreadsheet_id: &String,
) -> Option<Meetups> {
    let values = get_sheet(hub, spreadsheet_id, &"Meetups".to_string()).await;
    if let Err(e) = values {
        println!("Error: {}", e);
        return None;
    }
    let values = values.unwrap();

    let csv = values
        .iter()
        .map(|row| {
            let row = row
                .iter()
                .map(|cell| match cell {
                    Value::String(s) => format!("\"{}\"", s),
                    _ => "".to_string(),
                })
                .collect::<Vec<String>>()
                .join(",");
            row
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("CSV: {}", csv);
    let mut csv_reader = csv::Reader::from_reader(csv.as_bytes());
    let mut meetups: Meetups = Meetups {
        monday: Vec::new(),
        tuesday: Vec::new(),
        wednesday: Vec::new(),
        thursday: Vec::new(),
        friday: Vec::new(),
        saturday: Vec::new(),
        sunday: Vec::new(),
    };
    for result in csv_reader.deserialize() {
        let meetup: Meetup = result.unwrap();

        match meetup.day.clone() {
            day if day.contains("Monday") => meetups.monday.push(meetup),
            day if day.contains("Tuesday") => meetups.tuesday.push(meetup),
            day if day.contains("Wednesday") => meetups.wednesday.push(meetup),
            day if day.contains("Thursday") => meetups.thursday.push(meetup),
            day if day.contains("Friday") => meetups.friday.push(meetup),
            day if day.contains("Saturday") => meetups.saturday.push(meetup),
            day if day.contains("Sunday") => meetups.sunday.push(meetup),
            _ => (),
        }
    }
    return Some(meetups);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get() {
        dotenv::dotenv().ok();
        let service_account_path = dotenv::var("SERVICE_ACCOUNT_PATH").unwrap();
        let spreadsheet_id = dotenv::var("SPREADSHEET_ID").unwrap();
        let sheet = "Events".to_string();

        let hub = create_sheets(service_account_path).await;
        let values = get_sheet(hub, &spreadsheet_id, &sheet).await;

        match values {
            Ok(values) => {
                println!("Values: {:?}", values);
                assert!(values.len() > 0);
            }
            Err(e) => {
                println!("Error: {}", e);
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_meetups() {
        dotenv::dotenv().ok();
        let service_account_path = dotenv::var("SERVICE_ACCOUNT_PATH").unwrap();
        let spreadsheet_id = dotenv::var("SPREADSHEET_ID").unwrap();

        let hub = create_sheets(service_account_path).await;
        let meetups = get_meetups(hub, &spreadsheet_id).await;

        print!("Meetups: {:?}", meetups);
    }

    // #[tokio::test]
    // async fn test_create_client() {
    //     let access_token = get_token(String::from(
    //         "/Users/kyle/Projects/run-groups/sheet/src/client_secret.json",
    //     ))
    //     .await;

    //     if let Some(ref token) = access_token.token() {
    //         assert!(token.len() > 0);
    //     } else {
    //         assert!(false);
    //     }
    // }
}
