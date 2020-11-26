use aliyun_openapi::prelude::*;
use std::str::from_utf8;
use bytes::{Bytes, Buf};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = "oss-cn-hangzhou.aliyuncs.com";
    let ref http_client = reqwest::Client::new();
    let oss = OSS { endpoint, http_client };
    let bytes: Bytes = oss.get_object("eren-assets", "hello.txt").await?;
    print!("object: {}", from_utf8(bytes.bytes()).unwrap());
    Ok(())
}
