use reqwest::Client;
use serde_json::{json, Value};

use crate::settings::{CHAR_DEFINITION, CONTEXT, DELIM, MEMORYLN};
pub static mut HISTORY: Vec<Value> = Vec::new();

pub async fn gemini_prompt(api_key: &str, prompt: &str) -> Result<String, String> {
    unsafe {
        let base_desc = vec![
            json!({
                "role": "user",
                "parts": [
                    { "text": format!(
                        "{} , Context:{}",
                        format!("{} and will always end the sentence with '{}'(NO TYPOS!)",CHAR_DEFINITION,DELIM) , CONTEXT) }
                ]
            }),
            json!({
                "role": "model",
                "parts": [
                    { "text": "Ok I will" }
                ]
            }),
        ];

        if HISTORY.is_empty() {
            HISTORY.extend(base_desc);
        }

        HISTORY.push(json!({
            "role": "user",
            "parts": [{ "text": prompt }]
        }));

        if HISTORY.len() >= (MEMORYLN * 2) as usize {
            HISTORY.remove(3);
            HISTORY.remove(2);
        }

        let request_body = json!({
            "contents": HISTORY,
            "generationConfig": {
                "temperature": 1.0,
                "topK": 40,
                "topP": 0.95,
                "maxOutputTokens": 256,
                "responseMimeType": "text/plain"
            },

            "safetySettings": [
                {
                    "category": "HARM_CATEGORY_HATE_SPEECH",
                    "threshold": "BLOCK_NONE"
                },
                {
                    "category": "HARM_CATEGORY_DANGEROUS_CONTENT",
                    "threshold": "BLOCK_NONE"
                },
                {
                    "category": "HARM_CATEGORY_HARASSMENT",
                    "threshold": "BLOCK_NONE"
                },
                {
                    "category": "HARM_CATEGORY_SEXUALLY_EXPLICIT",
                    "threshold": "BLOCK_NONE"
                }
            ]
        });

        // Log the request body
        let request_body_str = serde_json::to_string(&request_body).unwrap();
        println!("Request Body: {}", request_body_str);

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-8b:generateContent?key={}",
            api_key
        );

        let client = Client::new();
        let mut response = client.post(&url).json(&request_body).send().await;

        match response {
            Ok(mut res) => {  // `mut res` allows us to borrow it multiple times
                // First, check if the status is success before calling json()
                if res.status().is_success() {
                    let json_response: Value = res.json().await.unwrap_or(Value::Null);
                    println!("Response: {:?}", json_response); // Log the response body for debugging

                    if let Some(candidates) = json_response.get("candidates") {
                        if let Some(candidate) = candidates.as_array().and_then(|c| c.get(0)) {
                            if let Some(content) = candidate.get("content") {
                                if let Some(parts) = content.get("parts") {
                                    if let Some(part) = parts.as_array().and_then(|p| p.get(0)) {
                                        if let Some(text) = part.get("text") {
                                            HISTORY.push(json!({
                                                "role": "model",
                                                "parts": [{ "text": text.as_str().unwrap_or_default() }]
                                            }));
                                            println!("{:?}", HISTORY);
                                            return Ok(text
                                            .as_str()
                                            .unwrap_or_default()
                                            .to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    return Err(format!(
                        "Request failed with status: {}",
                        res.status()
                    ));
                }
            }
            Err(e) => return Err(format!("Error: {}", e)),
        }

        Err("No response generated".to_string())
    }
}


