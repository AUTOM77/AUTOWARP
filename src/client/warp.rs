use serde_json::json;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

use super::cipher;
use super::date;

const END_POINT: &str = "https://api.cloudflareclient.com/v0a2077/reg";
const RETRY_DELAY: Duration = Duration::from_secs(60);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACCOUNT {
    pub id: String,
    pub license: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WARP {
    pub id: String,
    pub key: String,
    pub token: String,
    pub account: ACCOUNT
}

impl WARP {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>>{
        let payload = json!({
            "tos": date::get_tos(),
            "key": cipher::get_key()
        });

        let client = reqwest::Client::builder()
            .http2_keep_alive_timeout(std::time::Duration::from_secs(15))
            .build()?;

        let response = client.post(END_POINT)
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.text().await?).into());
        }

        let warp = response
            .json::<WARP>()
            .await?;
        Ok(warp)
    }

    pub async fn build() -> Result<Self, Box<dyn std::error::Error>>{
        let warp = loop {
            match Self::new().await {
                Ok(warp) => break warp,
                Err(_) => {
                    eprintln!("Retrying in {} seconds...", RETRY_DELAY.as_secs());
                    sleep(RETRY_DELAY).await;
                },
            }
        };

        Ok(warp)
    }

    pub async fn update(&mut self, license: String) -> Result<String, Box<dyn std::error::Error>>{
        let end_point = format!("{}/{}/account", END_POINT, self.id);
        let payload = json!({
            "license": license,
        });

        let client = reqwest::Client::builder()
            .http2_keep_alive_timeout(std::time::Duration::from_secs(15))
            .build()?;

        let response = client.put(end_point.clone())
            .bearer_auth(&self.token)
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("PUT HTTP error: {}", response.text().await?).into());
        }

        let response = client.get(end_point)
            .bearer_auth(&self.token)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("GET HTTP error: {}", response.text().await?).into());
        }

        let last_license = self.account.license.clone();
        self.account = response.json::<ACCOUNT>().await?;
        Ok(last_license)
    }

    pub async fn delete(&self) -> Result<(), Box<dyn std::error::Error>>{
        let end_point = format!("{}/{}", END_POINT, self.id);

        let client = reqwest::Client::builder()
            .http2_keep_alive_timeout(std::time::Duration::from_secs(45))
            .build()?;

        let response = client.delete(end_point)
            .bearer_auth(&self.token)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("DELETE HTTP error: {}", response.text().await?).into());
        }
        Ok(())
    }

    pub async fn get_license(&mut self, seed: String) -> Result<String, Box<dyn std::error::Error>>{
        let license = self.update(seed).await.unwrap();
        let seed = self.update(license.clone()).await.unwrap();
        let _ = self.delete().await.unwrap();

        Ok(license)
    }

}