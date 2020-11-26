use serde::{Deserialize, Serialize};
use std::fs;
use std::env;
use lazy_static::lazy_static;
use reqwest::header::{CONTENT_TYPE, DATE};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use chrono::prelude::*;
use crate::utils::url_encode;


lazy_static! {
     static ref PROFILE: Profile = {
        let profile = load_default_profile();
        profile
    };
    static ref CONTENT_MD5: HeaderName = "Content-MD5".parse().unwrap();
}

const ALIYUN_OPENAPI_DEFAULT_PARAMS: &[(&str, &str)] = &[
    ("Format", "JSON"),
    ("SignatureMethod", "HMAC-SHA1"),
    ("SignatureVersion", "1.0"),
];

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    current: String,
    profiles: Vec<Profile>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    name: String,
    access_key_id: String,
    access_key_secret: String,
    region_id: String,
}

pub fn load_default_profile() -> Profile {
    let home = env::var("HOME").unwrap();
    let config_json_path = format!("{}/.aliyun/config.json", home);
    let config_json = fs::read_to_string(config_json_path).expect("Failed to read ~/.aliyun/config.json");
    let config: Config = serde_json::from_str(&config_json).unwrap();
    let x = config.profiles.get(0).expect("No profile found for default!");
    return (*x).clone();
}


fn get_header_value<'a>(headers: &'a HeaderMap, name: &HeaderName) -> &'a str {
    return headers
        .get(name)
        .and_then(|header_value| Some(header_value.to_str().unwrap_or_default()))
        .unwrap_or_default();
}

/// Aliyun OSS security signature by https://help.aliyun.com/document_detail/31951.html
pub fn oss_sign_header(verb: &str, bucket: &str, object: &str, headers: &HeaderMap) -> String {
    let date = get_header_value(headers, &DATE);
    let content_type = get_header_value(headers, &CONTENT_TYPE);
    let content_md5 = get_header_value(headers, &CONTENT_MD5);
    let mut oss_headers: Vec<(&HeaderName, &HeaderValue)> = headers
        .iter()
        .filter(|(k, _)| k.as_str().contains("x-oss-"))
        .collect();
    oss_headers.sort_by(|a, b| a.0.to_string().cmp(&b.0.to_string()));
    let mut oss_headers_str = String::new();
    for (k, v) in oss_headers {
        oss_headers_str += &format!(
            "{}:{}\n",
            k.to_owned().as_str(),
            v.to_owned().to_str().unwrap_or("")
        );
    }
    let oss_resource = format!("/{}/{}", bucket, object);
    let text = format!(
        "{}\n{}\n{}\n{}\n{}{}",
        verb, content_md5, content_type, date, oss_headers_str, oss_resource
    );
    let signature = sign_base64(&PROFILE.access_key_secret, &text);
    format!("OSS {}:{}", PROFILE.access_key_id, signature)
}

/// sign url
pub fn sign_url(http_method: &str, endpoint: &str, version: &str,
                action: &str, queries: &[(&str, &str)]) -> String {
    let nonce = Local::now().timestamp_subsec_nanos().to_string();
    let ts = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    // build request params.
    let mut params = Vec::from(ALIYUN_OPENAPI_DEFAULT_PARAMS);
    params.push(("Action", &action));
    params.push(("AccessKeyId", &PROFILE.access_key_id));
    params.push(("SignatureNonce", &nonce));
    params.push(("Timestamp", &ts));
    params.push(("Version", version));
    params.extend_from_slice(&queries);
    params.sort_by_key(|item| item.0);
    // encode request params to pairs
    let pairs: Vec<String> = params
        .into_iter()
        .map(|(k, v)| format!("{}={}", url_encode(k), url_encode(v)))
        .collect();
    let sorted_query = pairs.join("&");
    let string_to_sign = format!(
        "{}&{}&{}",
        http_method,
        url_encode("/"),
        url_encode(&sorted_query)
    );
    // sign params, get final request url.
    let sign = sign_base64(&format!("{}&", &PROFILE.access_key_secret), &string_to_sign);
    let signature = url_encode(&sign);
    format!(
        "https://{}?Signature={}&{}",
        endpoint, signature, sorted_query
    )
}

/// signed and encoded with base64
fn sign_base64(key_secret: &str, body: &str) -> String {
    let mut mac = Hmac::new(Sha1::new(), key_secret.as_bytes());
    mac.input(body.as_bytes());
    base64::encode(mac.result().code())
}

#[cfg(test)]
mod tests {
    use crate::auth::{PROFILE, load_default_profile, oss_sign_header, sign_url};
    use reqwest::header::{HeaderMap};

    #[test]
    fn test_load_profile() {
        let profile = load_default_profile();
        println!("{:#?}", profile)
    }

    #[test]
    fn test_static_profile() {
        println!("{:#?}", *PROFILE)
    }

    #[test]
    fn test_oss_sign() {
        let headers = HeaderMap::new();
        let signature = oss_sign_header("GET", "bucket-1", "", &headers);
        println!("{}", signature);
    }

    #[test]
    fn test_sign_url() {
        let params: Vec<(&str, &str)> = Vec::new();
        let url = sign_url("POST", "dm.aliyuncs.com", "2015-11-23",
                           "SingleSendMail", params.as_ref());
        println!("{}", url);
    }
}

