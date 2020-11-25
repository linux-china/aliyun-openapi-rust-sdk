use reqwest::header::{HeaderMap, DATE};
use chrono::prelude::*;
use crate::credential::oss_sign;
use bytes::Bytes;

pub struct OSS<'a> {
    pub http_client: &'a reqwest::Client,
}

impl<'a> OSS<'a> {
    pub async fn get_object(&self, bucket: &str, object: &str) -> reqwest::Result<Bytes> {
        let endpoint = "oss-cn-hangzhou.aliyuncs.com";
        let mut headers = HeaderMap::new();
        // date http header
        let now = Utc::now().format("%a, %d %b %Y %T GMT").to_string();
        headers.insert(DATE, now.parse().unwrap());
        // Authorization http header
        let authorization = oss_sign("GET", bucket, object, &headers);
        headers.insert("Authorization", authorization.parse().unwrap());
        // request
        let url = format!("https://{}.{}/{}", bucket, endpoint, object);
        let resp = self.http_client.get(&url).headers(headers).send().await?;
        resp.bytes().await
    }
}

#[cfg(test)]
mod tests {
    use crate::oss::OSS;
    use std::str::from_utf8;
    use bytes::Buf;

    #[tokio::test]
    async fn test_get_object() -> Result<(), Box<dyn std::error::Error>> {
        let ref http_client = reqwest::Client::new();
        let oss = OSS { http_client };
        let bytes = oss.get_object("eren-assets", "health.txt").await?;
        print!("object: {}", from_utf8(bytes.bytes()).unwrap());
        Ok(())
    }
}
