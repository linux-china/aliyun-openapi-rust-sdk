use reqwest::header::{HeaderMap, DATE, CONTENT_TYPE};
use chrono::prelude::*;
use crate::auth::oss_sign_header;
use bytes::Bytes;
use reqwest::{Url, Response};

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
        let authorization = oss_sign_header("GET", bucket, object, &headers);
        headers.insert("Authorization", authorization.parse().unwrap());
        // GET request
        let url = format!("https://{}.{}/{}", bucket, endpoint, object);
        let resp = self.http_client.get(&url).headers(headers).send().await?;
        resp.bytes().await
    }

    pub async fn put_object(&self, bucket: &str, object: &str,
                            content_type: &str, bytes: &[u8]) -> reqwest::Result<bool> {
        let endpoint = "oss-cn-hangzhou.aliyuncs.com";
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, content_type.parse().unwrap());
        // date http header
        let now = Utc::now().format("%a, %d %b %Y %T GMT").to_string();
        headers.insert(DATE, now.parse().unwrap());
        // Authorization http header
        let authorization = oss_sign_header("PUT", bucket, object, &headers);
        headers.insert("Authorization", authorization.parse().unwrap());
        // PUT object
        let url = format!("https://{}.{}/{}", bucket, endpoint, object);
        let resp = self.http_client.put(&url).headers(headers).body(bytes.to_owned()).send().await?;
        return resp.error_for_status().map(|response| response.status().is_success());
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
        let bytes = oss.get_object("eren-assets", "hello.txt").await?;
        print!("object: {}", from_utf8(bytes.bytes()).unwrap());
        Ok(())
    }

    #[tokio::test]
    async fn test_put_object() -> Result<(), Box<dyn std::error::Error>> {
        let ref http_client = reqwest::Client::new();
        let oss = OSS { http_client };
        let result = oss.put_object("eren-assets", "hello.txt", "text/plain", "hello".as_bytes()).await?;
        print!("result: {}", result);
        Ok(())
    }
}
