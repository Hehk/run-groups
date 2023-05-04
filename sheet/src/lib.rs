extern crate google_sheets4 as sheets4;

use serde_json::Value;
use sheets4::Sheets;
use sheets4::{oauth2, oauth2::ServiceAccountAuthenticator};
use std::env;

pub mod model;

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

pub async fn get_meetups(
    hub: Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    spreadsheet_id: &String,
) -> Result<Vec<model::Meetup>, String> {
    let values = match get_sheet(hub, spreadsheet_id, &"Meetups".to_string()).await {
        Ok(it) => it,
        Err(_) => return Err("Failed to get sheet".to_string()),
    };

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

    model::read_meetups(csv)
}
