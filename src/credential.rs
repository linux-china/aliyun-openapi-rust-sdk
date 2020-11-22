use serde::{Deserialize, Serialize};
use std::fs;
use std::env;
use lazy_static::lazy_static;

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

#[cfg(test)]
mod tests {
    use crate::credential::load_default_profile;
    use crate::credential::PROFILE;

    #[test]
    fn test_load_profile() {
        let profile = load_default_profile();
        println!("{:#?}", profile)
    }

    #[test]
    fn test_static_profile() {
        println!("{:#?}", *PROFILE)
    }
}

