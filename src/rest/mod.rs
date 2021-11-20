mod sled;
mod safebrowsing;

use url::{Url, Position};
use anyhow::Result;

pub(crate) mod endpoint {
    pub static SAFEBROWSING_ENDPOINT: &'static str = "https://transparencyreport.google.com/transparencyreport/api/v3/safebrowsing/status?site=";
}

pub struct SafeData {
    pub(crate) safe: bool,
    pub(crate) time: i64,
}

pub struct Safe {
    pub(crate) safebrowsing: safebrowsing::Safebrowsing,
    pub(crate) sled: sled::Sled,
}

impl SafeData {
    pub fn new() -> Self {
        SafeData {
            safe: true,
            time: -1,
        }
    }

    pub fn is_safe(&self) -> bool {
        self.safe
    }

    pub fn get_time(&self) -> i64 {
        self.time
    }
}

impl Safe {
    pub async fn new(database: &str) -> Result<Self> {
        Ok(Self {
            safebrowsing: safebrowsing::Safebrowsing::new(),
            sled: sled::Sled::new(database).await?,
        })
    }

    pub fn deny(&mut self, url: &str) -> Result<()> {
        let url = self.form_url(url)?;
        self.sled.deny(&url)
    }

    pub fn allow(&mut self, url: &str) -> Result<()> {
        let url = self.form_url(url)?;
        self.sled.allow(&url)
    }

    pub fn is_denied(&mut self, url: &str) -> Result<bool> {
        let url = self.form_url(url)?;
        self.sled.is_denied(&url)
    }

    fn form_url(&self, url: &str) -> Result<String> {
        let url = Url::parse(url).expect("url");
        let input = &url[..Position::BeforePath];

        Ok(input.to_string().to_ascii_lowercase())
    }

    pub async fn is_safe(&mut self, input: &str) -> Result<SafeData> {

        let input = self.form_url(input)?;

        let mut safe_state = SafeData {
            safe: true,
            time: -1,
        };

        let sled = self.is_sled_safe(&input)?;
        let safebrowsing = self.is_safebrowsing_safe(&input).await?;

        if !sled.is_safe() {
            safe_state.safe = false;
            safe_state.time = sled.get_time();
        }

        if !safebrowsing.is_safe() {
            safe_state.safe = false;
            safe_state.time = safebrowsing.get_time();
        }

        Ok(safe_state)
    }

    fn is_sled_safe(&mut self, input: &str) -> Result<SafeData> {
        let (sled_safe, sled_time) = self.sled.is_safe(input)?;
        if !sled_safe {
            Ok(SafeData {
                safe: false,
                time: sled_time,
            })
        } else {
            Ok(SafeData::new())
        }
    }

    async fn is_safebrowsing_safe(&mut self, input: &str) -> Result<SafeData> {
        let safebrowsing_time = self.safebrowsing.is_safe(input).await?;
        if safebrowsing_time != -1 {
            Ok(SafeData {
                safe: false,
                time: safebrowsing_time,
            })
        } else {
            Ok(SafeData::new())
        }
    }
}