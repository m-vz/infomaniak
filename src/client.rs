use reqwest::{
    header::{self, HeaderMap},
    Response, StatusCode, Url,
};
use serde::de::DeserializeOwned;

use crate::error::Error;

pub struct Client {
    url: Url,
    client: reqwest::Client,
}

impl Client {
    pub fn new(token: &str) -> Client {
        Client {
            url: Url::parse("https://api.infomaniak.com").unwrap(),
            client: build_client(token),
        }
    }

    pub fn with_url(url: &str, token: &str) -> Result<Client, Error> {
        Ok(Client {
            url: Url::parse(url).map_err(|_| Error::InvalidUrl)?,
            client: build_client(token),
        })
    }

    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let response = self
            .client
            .get(self.url.join(path).map_err(|_| Error::InvalidUrl)?)
            .send()
            .await?;
        check_response_code(&response)?;
        Ok(response.json().await?)
    }
}

fn check_response_code(response: &Response) -> Result<(), Error> {
    match response.status() {
        StatusCode::OK | StatusCode::CREATED => Ok(()),
        StatusCode::UNAUTHORIZED => Err(Error::Unauthorised),
        StatusCode::FORBIDDEN => Err(Error::Forbidden),
        StatusCode::NOT_FOUND => Err(Error::NotFound),
        StatusCode::UNPROCESSABLE_ENTITY => Err(Error::UnprocessableEntity),
        StatusCode::INTERNAL_SERVER_ERROR => Err(Error::InternalServerError),
        _ => Err(Error::UnexpectedResponseCode),
    }
}

fn build_client(token: &str) -> reqwest::Client {
    reqwest::Client::builder()
        .default_headers(build_headers(token))
        .build()
        .expect("Could not create HTTP client")
}

fn build_headers(token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        format!("Bearer {token}").parse().unwrap(),
    );
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    headers
}
