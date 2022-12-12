use jsonpath;
use scraper::{selector, Html};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "http://jsonplaceholder.typicode.com/comments";
    let resp = reqwest::get(url).await?;
    let json_body: serde_json::Value = resp.json().await?;
    
    Ok(())
}
