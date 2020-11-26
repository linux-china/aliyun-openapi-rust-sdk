<p align="center">
<a href=" https://www.alibabacloud.com"><img src="https://aliyunsdk-pages.alicdn.com/icons/AlibabaCloud.svg"></a>
</p>

<h1 align="center">Alibaba Cloud SDK for Rust</h1>

<p align="center">
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

# Aliyun

* Region endpoints: https://api.aliyun.com/#/region
* Auth Signature：  https://help.aliyun.com/document_detail/31951.html  https://www.alibabacloud.com/help/zh/doc-detail/25492.htm
* oss-rust-sdk: https://github.com/NoXF/oss-rust-sdk

# References

* reqwest: https://github.com/seanmonstar/reqwest
* Serde: https://serde.rs/
