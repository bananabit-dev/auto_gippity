use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize, Serialize, Clone, PartialEq)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize,Serialize, Clone)]
pub struct ChatCompletion {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
}


#[derive(Debug,Deserialize, Serialize, Clone)]
pub struct APIMessage {
    pub content: String,
}

#[derive(Debug,Deserialize, Serialize, Clone)]
pub struct APIChoice {
    pub message: APIMessage,
}


#[derive(Debug, Deserialize,Serialize, Clone)]
pub struct APIResponse {
    pub choices: Vec<APIChoice>,
}