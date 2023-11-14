use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: Option<String>,
    pub scope: String,
}

pub struct DiscordClient {
    http: HttpClient,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

impl DiscordClient {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            http: HttpClient::new(),
            client_id,
            client_secret,
            redirect_uri,
        }
    }

    pub async fn exchange_code(&self, code: String) -> anyhow::Result<AccessToken> {
        let response = self
            .http
            .post("https://discord.com/api/v10/oauth2/token")
            .query(&[
                ("grant_type", "authorization_code"),
                ("code", &code),
                ("redirect_uri", &self.redirect_uri),
            ])
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await?
            .json::<AccessToken>()
            .await?;
        Ok(response)
    }
}
