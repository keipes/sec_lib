use reqwest::header::{USER_AGENT, ACCEPT_ENCODING, HOST, HeaderMap};

pub mod tickers;
pub mod company_names;
pub mod resources;

pub fn sec_headers() -> HeaderMap {
    let mut map = HeaderMap::new();
    map.append(USER_AGENT, "Cash and Cash Equivalents admin@cashandcashequivalents.com".parse().unwrap());
    map.append(ACCEPT_ENCODING, "gzip, deflate".parse().unwrap());
    map.append(HOST, "www.sec.gov".parse().unwrap());
    map
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
