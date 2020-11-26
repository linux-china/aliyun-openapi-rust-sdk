<p align="center">
   <a href="https://cn.aliyun.com/">
      <img src="https://docs.alibabagroup.com/assets2/images/en/news/library_logos_aliyun_large.png">
   </a>
</p>

<h1 align="center">Alibaba Cloud OpenAPI SDK for Rust</h1>

<p align="center">
  <a href="https://github.com/linux-china/aliyun-openapi-rust-sdk/actions">
    <img alt="Github Actions" src="https://img.shields.io/github/workflow/status/linux-china/aliyun-openapi-rust-sdk/Rust">
  </a>
  <a href="https://crates.io/crates/aliyun-openapi">
    <img alt="Crate" src="https://img.shields.io/crates/v/aliyun-openapi">
  </a>

</p>

The Alibaba Cloud SDK for Rust allows you to access Alibaba Cloud services such as Elastic Compute Service (ECS), Server Load Balancer (SLB), and CloudMonitor.  
You can access Alibaba Cloud services without the need to handle API related tasks, such as signing and constructing your requests.

# How to start?
Please install aliyun command line from https://github.com/aliyun/aliyun-cli, then config the credential info.

```
$ aliyun configure
```

# OSS example

```rust
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
```

# References

* Alibaba Cloud: https://us.alibabacloud.com/
* Aliyun OpenAPI Explorer: https://api.aliyun.com/
