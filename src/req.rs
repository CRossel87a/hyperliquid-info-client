use anyhow::{anyhow, Context, Result};
use reqwest::{Client, Response};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ErrorData {
    data: String,
    code: u16,
    msg: String,
}

pub struct HttpClient {
    pub client: Client,
    pub base_url: String,
}

async fn parse_response(response: Response) -> Result<String> {
    let status_code = response.status().as_u16();
    let headers = response.headers().clone();
    let text = response
        .text()
        .await
        .context("failed to read response body")?;

    if status_code < 400 {
        return Ok(text);
    }
    let error_data = serde_json::from_str::<ErrorData>(&text);
    if (400..500).contains(&status_code) {
        return match error_data {
            Ok(error_data) => Err(anyhow!(
                "Client error: status code: {status_code}, error code: {}, error message: {}, headers: {headers:?}, error data: {}",
                error_data.code,
                error_data.msg,
                error_data.data,
            )),
            Err(err) => Err(anyhow!(
                "Client error: status code: {status_code}, error message: {text}, headers: {headers:?}, error data: {err}",
            )),
        };
    }

    Err(anyhow!(
        "Server error: status code: {status_code}, error message: {text}",
    ))
}

impl HttpClient {
    pub async fn post(&self, url_path: &'static str, data: String) -> Result<String> {
        let full_url = format!("{}{url_path}", self.base_url);
        let request = self
            .client
            .post(full_url)
            .header("Content-Type", "application/json")
            .body(data)
            .build()
            .context("failed to build request")?;
        let result = self
            .client
            .execute(request)
            .await
            .context("failed to execute request")?;
        parse_response(result).await
    }
}
