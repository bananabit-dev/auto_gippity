use reqwest::Client;
use serde::de::DeserializeOwned;
use tokio::fs;

use crate::{helpers::command_line::ai_task_request, models::general::llm::Message};

const CODE_TEMPLATE_PATH: &str = "C:\\Users\\marvi\\workspace\\rust-web-server-template\\src\\code_template.rs";
const EXEC_MAIN_PATH: &str = "C:\\Users\\marvi\\workspace\\rust-web-server-template\\src\\main.rs";
const API_SCHEMA_PATH: &str = "C:\\Users\\marvi\\workspace\\auto_gippity\\schema\\";

pub async fn read_code_template_contents() -> String {
    let path: String = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).await.expect("msg")
}

pub async fn save_backend_code(contents: &String){
        let path: String = String::from(EXEC_MAIN_PATH);
        fs::write(path, contents).await.expect("Failed to write a file")
}

// Extends ai function to encourage specific output
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message{
    let ai_function_str = ai_func(func_input);
    let msg: String = format!("FUNCTION {} INSTRUCTION: You are a function printer. You ONLY print the results of functions. Nothing else. No commentary. Here is the Input of the function:{} Print out what the function will return.", ai_function_str, func_input);
    dbg!(&msg);
    Message {
        role:"system".to_string(),
        content: msg
    }
}

pub async fn ai_task_request_decoded<T: DeserializeOwned>(msg_context: String,agent_position: &str, agent_operation: &str,function_pass: for<'a> fn(&'a str) -> &'static str) -> T{
    let llm_response: String = ai_task_request(msg_context,agent_position,agent_operation,function_pass).await;

    let decoded_response: T = serde_json::from_str(llm_response.as_str()).expect("Failed to decode ai response from serde_json");

    decoded_response
}

pub async fn check_status_code(client: &Client,url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())    
}

// Get Code Template

// Save new backend code

// save json api endpoint schema

pub async fn save_api_endpoint(api_endpoints: &String) {
            let path: String = String::from(API_SCHEMA_PATH);
        fs::write(path, api_endpoints).await.expect("Failed to write a file")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ai_functions::aifunc_managing::convert_user_input_to_goal, helpers::command_line::ai_task_request};

    #[test]
    fn tests_extending_ai_function() {
        let extended: Message = extend_ai_function(convert_user_input_to_goal, "dummy variable");
        assert_eq!(extended.role, "system".to_string())
    }

    #[tokio::test]
    async fn test_ai_task_request() {
        let ai_func_param: &str = "Build me a webserver for making stock price api requests.";
        
        let res = ai_task_request(ai_func_param.to_string(), "Managing Agent", "Defining user requirements", convert_user_input_to_goal).await;
        dbg!(&res);
        assert!(res.len() > 20)
    }
}