use crate::models::general::llm::{ChatCompletion, Message, APIResponse};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    // Extract API key information;
    let api_key: String =
        env::var("OPENROUTER_API_KEY").expect("OPENROUTER_API_KEY not found in env variable.");

    let url: &str = "https://openrouter.ai/api/v1/chat/completions";

    let content_type: &str = "application/json";

    let mut headers = HeaderMap::new();

    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    headers.insert(
        "content-type",
        HeaderValue::from_str(&format!("{}", content_type))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    let client = Client::builder().default_headers(headers).build().map_err(|e| ->  Box<dyn std::error::Error + Send> { Box::new(e) }  )?;

    // Create chat completion

    let chat_completion: ChatCompletion = ChatCompletion {
        model: "z-ai/glm-4-32b".to_string(),
        messages,
        temperature: 0.1,
    };

    // Troubleshooting
    /*    let res_json: serde_json::Value = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await? // send request
        .json()
        .await?; // parse JSON

    Ok(res_json)*/

    // Get API Response
    let res: APIResponse = client .post(url)
        .json(&chat_completion)
        .send()
        .await.map_err(|e| ->  Box<dyn std::error::Error + Send> { Box::new(e) }  )?.json().await.map_err(|e| ->  Box<dyn std::error::Error + Send> { Box::new(e) }  )?;

    // Send response
    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test, give me a short response.".to_string(),
        };

        let messages = vec![message];

        let res = call_gpt(messages).await;
        match res {
            Ok(res_str) => {
                dbg!(res_str);
                assert!(true);
            },
            Err(e) => {
                dbg!(e);
                assert!(false);
            }
        }
    }
}
