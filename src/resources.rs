use std::collections::HashMap;
use std::error::Error;
use aws_sdk_s3::Client;
use aws_sdk_s3::types::ByteStream;
use aws_config::meta::region::RegionProviderChain;


pub async fn client() -> Result<Client, Box<dyn Error>> {
    let region_provider = RegionProviderChain::default_provider()
        .or_else("us-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    Ok(client)
}

pub async fn put_tickers(s3_client: Client, tickers: HashMap<String, u64>) -> Result<(), Box<dyn Error>> {
    let body: Vec<u8> = serde_json::to_vec(&tickers)?;

    s3_client.put_object()
        .bucket("cce-resources")
        .body(ByteStream::from(body))
        .key("tickers.json")
        .send().await?;
    Ok(())
}
