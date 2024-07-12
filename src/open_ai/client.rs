use futures::StreamExt;
use futures::{future::BoxFuture, stream::BoxStream};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::OpenAiClientError;
use super::Request;
use super::ResponseStreamEvent;

pub async fn stream_completion(
    client: &Client,
    api_url: &str,
    api_key: &str,
    request: Request,
    low_speed_timeout: Option<Duration>,
) -> Result<BoxStream<'static, Result<ResponseStreamEvent, OpenAiClientError>>, OpenAiClientError> {
    let uri = format!("{}/chat/completions", api_url);

    let request = client
        .post(&uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request);

    let request = if let Some(low_speed_timeout) = low_speed_timeout {
        request.timeout(low_speed_timeout)
    } else {
        request
    };

    let response = request.send().await?;

    if response.status().is_success() {
        let stream = response.bytes_stream().filter_map(|item| async {
            match item {
                Ok(bytes) => {
                    let line = std::str::from_utf8(&bytes).ok()?;
                    let line = line.strip_prefix("data: ")?;
                    if line == "[DONE]" {
                        None
                    } else {
                        match serde_json::from_str(line) {
                            Ok(parsed) => Some(Ok(parsed)),
                            Err(error) => Some(Err(OpenAiClientError::ParseResponseFailed(error))),
                        }
                    }
                }
                Err(error) => Some(Err(OpenAiClientError::SendRequestFailed(error))),
            }
        });

        Ok(stream.boxed())
    } else {
        let status = response.status();
        let body = response.text().await?;

        #[derive(Deserialize)]
        struct OpenAiResponse {
            error: OpenAiError,
        }

        #[derive(Deserialize)]
        struct OpenAiError {
            message: String,
        }

        match serde_json::from_str::<OpenAiResponse>(&body) {
            Ok(response) if !response.error.message.is_empty() => {
                Err(OpenAiClientError::Custom(response.error.message))
            }
            _ => Err(OpenAiClientError::Custom(format!(
                "Failed to connect to OpenAI API: {} {}",
                status, body
            ))),
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum OpenAiEmbeddingModel {
    #[serde(rename = "text-embedding-3-small")]
    TextEmbedding3Small,
    #[serde(rename = "text-embedding-3-large")]
    TextEmbedding3Large,
}

#[derive(Serialize, Debug)]
struct OpenAiEmbeddingRequest<'a> {
    model: OpenAiEmbeddingModel,
    input: Vec<&'a str>,
}

#[derive(Deserialize, Debug)]
pub struct OpenAiEmbeddingResponse {
    pub data: Vec<OpenAiEmbedding>,
}

#[derive(Deserialize, Debug)]
pub struct OpenAiEmbedding {
    pub embedding: Vec<f32>,
}

pub fn embed<'a>(
    client: &'a Client,
    api_url: &'a str,
    api_key: &'a str,
    model: OpenAiEmbeddingModel,
    texts: impl IntoIterator<Item = &'a str> + 'a,
) -> BoxFuture<'a, Result<OpenAiEmbeddingResponse, OpenAiClientError>> {
    let uri = format!("{}/embeddings", api_url);

    let request = OpenAiEmbeddingRequest {
        model,
        input: texts.into_iter().collect(),
    };

    let body = serde_json::to_string(&request).unwrap();

    Box::pin(async move {
        let response = client
            .post(&uri)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .body(body)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if status.is_success() {
            let response: OpenAiEmbeddingResponse =
                serde_json::from_str(&body).map_err(OpenAiClientError::ParseResponseFailed)?;
            Ok(response)
        } else {
            Err(OpenAiClientError::Custom(format!(
                "error during embedding, status: {:?}, body: {:?}",
                status, body
            )))
        }
    })
}
