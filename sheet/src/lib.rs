extern crate google_sheets4 as sheets4;

use sheets4::Sheets;
use sheets4::{oauth2, oauth2::ServiceAccountAuthenticator};

pub async fn get_sheet(service_account_path: String, spreadsheet_id: String) {
    let creds = oauth2::read_service_account_key(service_account_path)
        .await
        .expect("Can't read credential, an error occurred");

    let sa = ServiceAccountAuthenticator::builder(creds)
        .build()
        .await
        .expect("There was an error, trying to build connection with authenticator");

    let hub = Sheets::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .enable_http2()
                .build(),
        ),
        sa,
    );

    let result = hub.spreadsheets().get(&spreadsheet_id).doit().await;

    match result {
        Ok((_, response)) => {
            let sheets = response.sheets.unwrap();

            for sheet in sheets {
                let properties = sheet.properties.unwrap();
                let title = properties.title.unwrap();

                // The lib does not like the / char
                if title == "Races/Events" {
                    continue;
                }

                let values = hub
                    .spreadsheets()
                    .values_get(&spreadsheet_id, &title)
                    .doit()
                    .await;

                match values {
                    Ok((_, response)) => {
                        println!("\nThe values: {:?}", response);
                    }
                    Err(e) => {
                        println!("There was an error: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("There was an error: {:?}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get() {
        dotenv::dotenv().ok();
        let service_account_path = dotenv::var("SERVICE_ACCOUNT_PATH").unwrap();
        let spreadsheet_id = dotenv::var("SPREADSHEET_ID").unwrap();

        get_sheet(service_account_path.to_string(), spreadsheet_id.to_string()).await;
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
