use serde::{Deserialize, Serialize};
use std::fs;
use std::env;
use lazy_static::lazy_static;
use reqwest::header::{CONTENT_TYPE, DATE};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use base64::encode as encode_base64;


lazy_static! {
    pub static ref PROFILE: Profile = {
        let profile = load_default_profile();
        profile
    };
}

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

pub fn oss_sign(verb: &str, bucket: &str, object: &str, headers: &HeaderMap) -> String {
    let date = headers
        .get(DATE)
        .and_then(|d| Some(d.to_str().unwrap_or_default()))
        .unwrap_or_default();
    let content_type = headers
        .get(CONTENT_TYPE)
        .and_then(|c| Some(c.to_str().unwrap_or_default()))
        .unwrap_or_default();
    let content_md5 = headers
        .get("Content-MD5")
        .and_then(|md5| Some(encode_base64(md5.to_str().unwrap_or_default())))
        .unwrap_or_default();
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
    let sign_str = format!(
        "{}\n{}\n{}\n{}\n{}{}",
        verb, content_md5, content_type, date, oss_headers_str, oss_resource
    );
    let mut hasher = Hmac::new(Sha1::new(), PROFILE.access_key_secret.as_bytes());
    hasher.input(sign_str.as_bytes());
    let sign_str_base64 = encode_base64(hasher.result().code());
    format!("OSS {}:{}", PROFILE.access_key_id, sign_str_base64)
}

#[cfg(test)]
mod tests {
    use crate::credential::{load_default_profile, oss_sign};
    use crate::credential::PROFILE;
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
    fn test_sign() {
        let headers = HeaderMap::new();
        let signature = oss_sign("GET", "bucket-1", "", &headers);
        println!("{}", signature);
    }
}

