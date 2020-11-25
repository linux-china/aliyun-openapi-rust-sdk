use reqwest::header::{HeaderMap, DATE, CONTENT_TYPE};
use chrono::prelude::*;
use crate::auth::oss_sign_header;
use bytes::Bytes;

/// OSS
pub struct OSS<'a> {
    /// endpoint, such as `oss-cn-hangzhou.aliyuncs.com`
    pub endpoint: &'a str,
    /// global reqwest::Client
    pub http_client: &'a reqwest::Client,
}

impl<'a> OSS<'a> {
    /// get object from bucket
    /// # Arguments
    /// * `bucket` - bucket name
    /// * `object` - object name
    pub async fn get_object(&self, bucket: &str, object: &str) -> reqwest::Result<Bytes> {
        let mut headers = HeaderMap::new();
        self.add_date_and_sign(&mut headers, bucket, object);
        let resp = self.http_client
            .get(&self.request_url(bucket, object))
            .headers(headers)
            .send().await?;
        resp.bytes().await
    }

    /// put object into bucket
    /// # Arguments
    /// * `bucket` - bucket name
    /// * `object` - object name
    /// * `content_type` - content type
    /// * `content` - binary content
    pub async fn put_object(&self, bucket: &str, object: &str,
                            content_type: &str, content: &[u8]) -> reqwest::Result<bool> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, content_type.parse().unwrap());
        self.add_date_and_sign(&mut headers, bucket, object);
        let resp = self.http_client
            .put(&self.request_url(bucket, object))
            .headers(headers)
            .body(content.to_owned())
            .send().await?;
        return resp.error_for_status().map(|response| response.status().is_success());
    }

    /// delete object from bucket
    /// # Arguments
    /// * `bucket` - bucket name
    /// * `object` - object name
    pub async fn delete_object(&self, bucket: &str, object: &str) -> reqwest::Result<bool> {
        let mut headers = HeaderMap::new();
        self.add_date_and_sign(&mut headers, bucket, object);
        let resp = self.http_client
            .delete(&self.request_url(bucket, object))
            .headers(headers)
            .send().await?;
        return resp.error_for_status().map(|response| response.status().is_success());
    }

    /// request URL for object's operation
    fn request_url(&self, bucket: &str, object: &str) -> String {
        format!("https://{}.{}/{}", bucket, self.endpoint, object)
    }

    /// add DATE HTTP HEADER and sign with Authorization HTTP header
    fn add_date_and_sign(&self, headers: &mut HeaderMap, bucket: &str, object: &str) {
        // date http header
        let now = Utc::now().format("%a, %d %b %Y %T GMT").to_string();
        headers.insert(DATE, now.parse().unwrap());
        // Authorization http header
        let authorization = oss_sign_header("GET", bucket, object, &headers);
        headers.insert("Authorization", authorization.parse().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use crate::oss::OSS;
    use std::str::from_utf8;
    use bytes::Buf;

    #[tokio::test]
    async fn test_get_object() -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = "oss-cn-hangzhou.aliyuncs.com";
        let ref http_client = reqwest::Client::new();
        let oss = OSS { endpoint, http_client };
        let bytes = oss.get_object("eren-assets", "hello.txt").await?;
        print!("object: {}", from_utf8(bytes.bytes()).unwrap());
        Ok(())
    }

    #[tokio::test]
    async fn test_put_object() -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = "oss-cn-hangzhou.aliyuncs.com";
        let ref http_client = reqwest::Client::new();
        let oss = OSS { endpoint, http_client };
        let result = oss.put_object("eren-assets", "hello.txt", "text/plain", "hello".as_bytes()).await?;
        print!("result: {}", result);
        Ok(())
    }
}
