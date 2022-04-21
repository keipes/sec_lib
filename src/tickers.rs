use std::collections::HashMap;
use std::str::from_utf8;
use futures_util::StreamExt;
use reqwest::{Client as HttpClient, Response};
use crate::sec_headers;
use log::{info, warn, error};

const _SEC_TICKER_URL: &str = "https://www.sec.gov/include/ticker.txt";

pub async fn get_tickers(client: HttpClient) -> Result<Response, Box<dyn std::error::Error>> {
    Ok(client.get(_SEC_TICKER_URL)
        .headers(sec_headers())
        .send().await?)
}

pub async fn parse_tickers(response: Response) -> Result<HashMap<String, u64>, Box<dyn std::error::Error>> {
    let mut ciks: HashMap<String, u64> = HashMap::new();
    let mut to_parse: String = String::new();
    let mut stream = response.bytes_stream();
    while let Some(item) = stream.next().await {
        to_parse.push_str(from_utf8(&item.unwrap()).unwrap());
        while let Some(line_end) = to_parse.find('\n') {
            let line: String = to_parse.drain(..line_end+1).collect();
            let ct_maybe = parse_line(line.as_str());
            if ct_maybe.is_err() {
                warn!("Error: {:?}", ct_maybe.err().unwrap())
            } else {
                let ct = ct_maybe.unwrap();
                let existing_maybe = ciks.insert(ct.ticker, ct.cik);
                if existing_maybe.is_some() {
                    error!("duplicate entry: {:?}", existing_maybe.ok_or(0));
                    panic!("duplicate entry");
                }
            }
        }
    }
    if to_parse.len() > 0 {
        let ct = parse_line(to_parse.as_str()).unwrap();
        ciks.insert(ct.ticker, ct.cik);
    }
    info!("found {:?} tickers", ciks.len());
    Ok(ciks)
}

struct CikTicker {
    ticker: String,
    cik: u64
}

impl CikTicker {
    fn new(t: &str, c: u64) -> CikTicker {
        CikTicker { ticker: t.to_string(), cik: c}
    }
}

fn parse_line(line: &str) -> Result<CikTicker, &'static str> {
    let mut split = line.split_whitespace();
    let ticker_maybe = split.next();
    let cik_maybe = split.next();
    if ticker_maybe.is_none() || cik_maybe.is_none() {
        error!("parse error: {:?}", line);
        Err("parse error")
    } else {
        let ticker = ticker_maybe.unwrap();
        let cik = cik_maybe.unwrap().parse::<u64>().unwrap();
        Ok(CikTicker::new(ticker, cik))
    }
}
