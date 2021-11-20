use std::collections::HashMap;

use anyhow::Result;
use crate::rest::endpoint::SAFEBROWSING_ENDPOINT;

pub type Time = i64;

pub struct Safebrowsing {
    deny: HashMap<String, Time>,
}

enum SafeState {
    NoAvailableData = 6,
    SomePagesUnsafe = 3,
    HarmfulPage = 2
}

impl Safebrowsing {

    pub fn new() -> Self {
        Self { deny: HashMap::new() }
    }

    pub async fn is_safe(&mut self, input: &str) -> Result<Time> {

        if self.deny.contains_key(input) {
            return Ok(self.deny.get(input).unwrap().clone());
        }

        match self.check_url(input).await {
            Ok(time) => {
                self.deny.insert(input.to_string(), time);
                return Ok(time);
            },
            Err(err) => {
                return Err(err);
            }
        }

    }

    async fn check_url(&mut self, url: &str) -> Result<Time> {

        match reqwest::get(&format!("{}{}", SAFEBROWSING_ENDPOINT, url)).await {
            Ok(response) => {
                let text = response.text().await?;
                let root = serde_json::from_str::<serde_json::Value>(&text[7..text.len()-1])?;
                let safe_state = root.pointer("/1")
                    .expect("no /1 found").as_u64().expect("no value at /1 found");

                if safe_state == SafeState::NoAvailableData as u64
                        || safe_state == SafeState::SomePagesUnsafe as u64
                        || safe_state == SafeState::HarmfulPage as u64 {
                    let time = root.pointer("/7").expect("no /7 found").as_i64().expect("no value at /7 found");
                    self.deny.insert(url.to_owned(), time);
                    return Ok(time);                    
                }

                Ok(-1)
            },
            Err(err) => anyhow::private::Err(anyhow::Error::msg(err))
        }
    }

}