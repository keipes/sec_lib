use std::error::Error;
use reqwest::{Client, ClientBuilder};
use reqwest::header::{USER_AGENT, ACCEPT_ENCODING, HOST, HeaderMap};

pub mod tickers;
pub mod company_names;
pub mod resources;

fn http_client() -> Result<Client, Box<dyn Error>>{
    let client: Client = ClientBuilder::new()
        .gzip(true)
        .default_headers(sec_headers())
        .build()?;
    Ok(client)
}

pub fn sec_headers() -> HeaderMap {
    let mut map = HeaderMap::new();
    map.append(USER_AGENT, "Cash and Cash Equivalents admin@cashandcashequivalents.com".parse().unwrap());
    map.append(ACCEPT_ENCODING, "gzip, deflate".parse().unwrap());
    map.append(HOST, "www.sec.gov".parse().unwrap());
    map
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::future::Future;
    use log::LevelFilter;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn company_names() {
        simple_logger::SimpleLogger::new().with_utc_timestamps()
            .with_level(LevelFilter::Info).init().unwrap();
        log::info!("Start.");
        let response = match crate::company_names::request_lookup_data().await {
            Ok(str) => str,
            Err(e) => panic!("Failed to retrieve document: {:?}", e)
        };
        log::info!("Got response.");
        let map = crate::company_names::consume_response(response).await;
        crate::resources::put_company_names(map).await;
        log::info!("Finish.");
    }
}
