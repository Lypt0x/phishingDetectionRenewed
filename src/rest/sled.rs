use anyhow::Result;

pub struct Sled {
    db: sled::Db,
}

pub type Time = i64;

impl Sled {
    pub async fn new(path: &str) -> Result<Self> {
        Ok(Self {
            db: sled::open(path)?
        })
    }

    pub fn deny(&mut self, url: &str) -> Result<()> {
        let time = chrono::Utc::now().timestamp() * 1000;
        self.db.insert(url, &time.to_be_bytes())?;
        Ok(())
    }

    pub fn allow(&mut self, url: &str) -> Result<()> {
        self.db.remove(url)?;
        Ok(())
    }

    pub fn is_denied(&mut self, url: &str) -> Result<bool> {
        let time = self.db.get(url)?;
        Ok(time.is_some())
    }

    pub fn is_safe(&mut self, url: &str) -> Result<(bool, Time)> {
        let time = self.db.get(url)?;
        if time.is_some() { Ok((false, self.get_time(url)?)) } else { Ok((true, -1)) }
    }

    pub fn get_time(&mut self, url: &str) -> Result<Time> {
        let time = self.db.get(url)?;
        if let Some(time) = time { Ok(Self::as_i64_be(&time)) } else { Ok(-1) }
    }

    fn as_i64_be(bytes: &[u8]) -> i64 {
        ((bytes[0] as i64) << 56) +
        ((bytes[1] as i64) << 48) +
        ((bytes[2] as i64) << 40) +
        ((bytes[3] as i64) << 32) +
        ((bytes[4] as i64) << 24) +
        ((bytes[5] as i64) << 16) +
        ((bytes[6] as i64) << 8) +
        (bytes[7] as i64)
    }

}