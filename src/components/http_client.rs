use reqwest::Client;
use tokio;

#[tokio::main]
pub async fn make_request(method: reqwest::Method, url: &str) -> Result<String, reqwest::Error>{
    let client = Client::new();

    let response = client
        .request(method, url)
        .header("curl","8.5.0")
        .send()
        .await?
        .text()
        .await?;

    print!("Response: {}", response);
    Ok(response)
}