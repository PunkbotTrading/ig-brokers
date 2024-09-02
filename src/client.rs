use crate::config::Config;
use crate::models::{LoginReq, LoginRes};
use reqwest::RequestBuilder;
use reqwest::header::HeaderMap;
use serde::Serialize;
use serde::de::DeserializeOwned;
use reqwest::Error;

#[derive(Clone)]
pub struct Client {
	account_id: String,
	api_key: String,
	username: String,
	password: String,
	client: reqwest::Client,
	config: Config
}

fn get_url(config: &Config, endpoint: &String) -> String {
	format!("https://{}{}", config.base_url, endpoint)
}

impl Client {
	pub fn new(account_id: String, api_key: String, username: String, password: String, config: Config) -> Client {
		Client {
			account_id,
			api_key,
			username,
			password,
			config,
			client: reqwest::Client::new()
		}
	}

	pub async fn get_signed<T: DeserializeOwned, U: Serialize>(&self, endpoint: &String, version: u8, query: Option<U>) -> Result<T, Error> {
		let url = get_url(&self.config, endpoint);
		let mut req = self.set_headers(self.client.get(url), version).await?;

		if let Some(query) = query {
			req = req.query(&query);
		}

		let res = req.send().await?;
		Ok(res.json::<T>().await?)
	}
	
	pub async fn post_signed<T: DeserializeOwned, U: Serialize>(&self, endpoint: &String, version: u8, data: Option<U>) -> Result<T, Error> {
		let url = get_url(&self.config, endpoint);
		let mut req = self.set_headers(self.client.post(url), version).await?;

		if let Some(data) = data {
			req = req.json(&data);
		}

		let res = req.send().await?;
		Ok(res.json::<T>().await?)
	}

	pub async fn put_signed<T: DeserializeOwned, U: Serialize>(&self, endpoint: &String, version: u8, data: Option<U>) -> Result<T, Error> {
		let url = get_url(&self.config, endpoint);
		let mut req = self.set_headers(self.client.put(url), version).await?;

		if let Some(data) = data {
			req = req.json(&data);
		}

		let res = req.send().await?;
		Ok(res.json::<T>().await?)
	}

	pub async fn delete_signed<T: DeserializeOwned, U: Serialize>(&self, endpoint: &String, version: u8, data: Option<U>) -> Result<T, Error> {
		let url = get_url(&self.config, endpoint);
		let mut req = self.set_headers(self.client.post(url), version).await?;

		let mut headers = HeaderMap::new();
		headers.insert("_method", "DELETE".to_string().parse().unwrap());
		req = req.headers(headers);

		if let Some(data) = data {
			req = req.json(&data);
		}

		let res = req.send().await?;
		Ok(res.json::<T>().await?)
	}

	async fn get_token(&self) -> Result<LoginRes, Error> {
		let login = LoginReq {
			identifier: self.username.clone(),
			password: self.password.clone()
		};

		let mut headers = HeaderMap::new();
		headers.insert("X-IG-API-KEY", self.api_key.parse().unwrap());
		headers.insert("IG-ACCOUNT-ID", self.account_id.parse().unwrap());
		headers.insert("VERSION", "3".parse().unwrap());

		let url = get_url(&self.config, &"/session".into());
		let res = self.client.post(&url).headers(headers).json(&login).send().await?;
		res.json::<LoginRes>().await
	}

	async fn set_headers(&self, req: RequestBuilder, version: u8) -> Result<RequestBuilder, Error> {
		let token = self.get_token().await?;
		let authorization = format!("Bearer {}", token.oauth_token.access_token);

		let mut headers = HeaderMap::new();
		headers.insert("IG-ACCOUNT-ID", self.account_id.parse().unwrap());
		headers.insert("X-IG-API-KEY", self.api_key.parse().unwrap());
		headers.insert("Authorization", authorization.parse().unwrap());
		headers.insert("VERSION", version.to_string().parse().unwrap());
		Ok(req.headers(headers))
	}
}
