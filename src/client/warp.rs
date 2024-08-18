use serde_json::json;
use serde::{Deserialize, Serialize};

use super::cipher;
use super::date;

const END_POINT: &str = "https://api.cloudflareclient.com/v0a2077/reg";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACCOUNT {
    id: String,
    license: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Internal {
    id: String,
    key: String,
    token: String,
    account: ACCOUNT
}

#[derive(Debug, Clone)]
pub struct WARP {
    pub client: reqwest::Client,
    pub intern: Internal,
}

impl WARP {
    pub fn new(client: reqwest::Client, intern: Internal) -> Self {
        Self { client, intern }
    }

    pub async fn build(geoip: String) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-Forwarded-For", geoip.parse().unwrap());

        let client = reqwest::Client::builder()
            .http2_keep_alive_timeout(tokio::time::Duration::from_secs(45))
            .default_headers(headers)
            .build()
            .unwrap();

        let payload = json!({
            "tos": date::get_tos(),
            "key": cipher::get_key()
        });

        let response = client.post(END_POINT)
            .json(&payload)
            .send()
            .await?;

        let intern = response.json::<Internal>().await?;
        Ok(Self::new(client, intern))
    }

    pub async fn update_license(self, license: String) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let end_point = format!("{}/{}/account", END_POINT, self.intern.id);

        let payload = json!({
            "license": license,
        });

        let response = self.client.put(end_point.clone())
            .bearer_auth(self.intern.token.clone())
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("PUT HTTP error: {}", response.text().await?).into());
        }

        Ok(self.clone())
    }

    pub async fn get_license(self, seed: String) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let end_point = format!("{}/{}/account", END_POINT, self.intern.id);

        let payload = json!({
            "license": seed,
        });

        let response = self.client.put(end_point.clone())
            .bearer_auth(self.intern.token.clone())
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("PUT HTTP error: {}", response.text().await?).into());
        }

        let payload = json!({
            "license": self.intern.account.license,
        });

        let response = self.client.put(end_point.clone())
            .bearer_auth(self.intern.token.clone())
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("PUT HTTP error: {}", response.text().await?).into());
        }

        let response = self.client.delete(end_point)
            .bearer_auth(self.intern.token.clone())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("DELETE HTTP error: {}", response.text().await?).into());
        }

        Ok(self.clone())
    }

    pub async fn get_quota(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let end_point = format!("{}/{}/account", END_POINT, self.intern.id);
        let response = self.client.get(end_point)
            .bearer_auth(&self.intern.token)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("GET HTTP error: {}", response.text().await?).into());
        }

        // let json = response.json().await?;
        let json: serde_json::Value = response.json().await?;
        let info = json["quota"].to_string();
        // let caption = json["candidates"][0]["content"]["parts"][0]["text"]
        //     .as_str()
        //     .ok_or_else(|| format!("Failed to parse response: {}", response_text))?;
        Ok(info)
    }

    pub async fn delete(self) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let end_point = format!("{}/{}/account", END_POINT, self.intern.id);
        let response = self.client.delete(end_point)
            .bearer_auth(self.intern.token.clone())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("DELETE HTTP error: {}", response.text().await?).into());
        }
        Ok(self.clone())
    }

    pub fn license(&self) -> String{
        self.intern.account.license.clone()
    }
}