use aliyun_openapi::prelude::*;
use anyhow::Result;
use std::str::from_utf8;
use bytes::Buf;

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint = "oss-cn-hangzhou.aliyuncs.com";
    let ref http_client = reqwest::Client::new();
    let oss = OSS { endpoint, http_client };
    let bytes = oss.get_object("eren-assets", "hello.txt").await?;
    print!("object: {}", from_utf8(bytes.bytes()).unwrap());
    Ok(())
}
