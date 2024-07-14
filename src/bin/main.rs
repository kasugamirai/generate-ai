use chatgpt_generative_ai::Model;
use chatgpt_generative_ai::{embed, stream_completion, OpenAiEmbeddingModel};
use chatgpt_generative_ai::{Request, RequestMessage};
use futures::stream::StreamExt;
use reqwest::Client;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let client = Client::new();

    let api_url = chatgpt_generative_ai::OPEN_AI_API_URL;
    let api_key = "sk-";

    let request = Request {
        model: Model::FourOmni,
        messages: vec![RequestMessage::User {
            content: "Tell me a joke.".to_string(),
        }],
        stream: true,
        stop: vec![],
        temperature: 0.7,
        tool_choice: None,
        tools: vec![],
    };

    match stream_completion(
        &client,
        api_url,
        api_key,
        request,
        Some(Duration::from_secs(10)),
    )
    .await
    {
        Ok(mut stream) => {
            while let Some(result) = stream.next().await {
                match result {
                    Ok(event) => println!("{:?}", event),
                    Err(error) => eprintln!("Error: {:?}", error),
                }
            }
        }
        Err(error) => eprintln!("Failed to initiate streaming completion: {:?}", error),
    }

    let texts = vec!["Hello world!", "How are you?"];

    match embed(
        &client,
        api_url,
        api_key,
        OpenAiEmbeddingModel::TextEmbedding3Small,
        texts,
    )
    .await
    {
        Ok(response) => println!("{:?}", response),
        Err(error) => eprintln!("Failed to get embeddings: {:?}", error),
    }
}
