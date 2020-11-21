use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpbinIp {
    origin: String,
}

#[tokio::test]
async fn test_http_get() -> Result<(), Box<dyn std::error::Error>> {
    let resp: HttpbinIp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json()
        .await?;
    println!("IP: {}", resp.origin);
    Ok(())
}

