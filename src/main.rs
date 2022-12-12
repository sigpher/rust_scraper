use std::{error, fs};

use http::HeaderMap;
use jsonpath;
use serde_json;
#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let url = "https://spa1.scrape.center/api/movie/?limit=10&offset=0";
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9".parse().unwrap());
    headers.insert(
        "Accept-Language",
        "zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6"
            .parse()
            .unwrap(),
    );
    headers.insert("Cache-Control", "max-age=0".parse().unwrap());
    headers.insert("Connection", "keep-alive".parse().unwrap());
    headers.insert("Sec-Fetch-Dest", "document".parse().unwrap());
    headers.insert("Sec-Fetch-Mode", "navigate".parse().unwrap());
    headers.insert("Sec-Fetch-Site", "cross-site".parse().unwrap());
    headers.insert("Sec-Fetch-User", "?1".parse().unwrap());
    headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36 Edg/105.0.1343.50".parse().unwrap());
    headers.insert(
        "sec-ch-ua",
        "\"Microsoft Edge\";v=\"105\", \" Not;A Brand\";v=\"99\", \"Chromium\";v=\"105\""
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
    let json_body: serde_json::Value = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;
    let selector = jsonpath::Selector::new("$.results[?(@.alias)]")?;

    for el in selector.find(&json_body) {
        println!("{}",el);
    }
    Ok(())
}
