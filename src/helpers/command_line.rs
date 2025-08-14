use crossterm::{style::{Color,ResetColor, SetForegroundColor},
ExecutableCommand,
};

use std::io::{stdin, stdout};

use crate::{apis::call_request::call_gpt, helpers::general::extend_ai_function, models::general::llm::Message};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos:&str , agent_statement: &str){
        let mut stdout = stdout();
        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Yellow
        };

        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}:",agent_pos);
        stdout.execute(SetForegroundColor(statement_color)).unwrap();

        println!("{}", agent_statement);


        stdout.execute(ResetColor).unwrap();


    }
}

pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();


    // Print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);
   
   stdout.execute(ResetColor).unwrap();

   let mut user_response: String = String::new();
   stdin().read_line(&mut user_response)
   .expect("Failed to read response");

   return user_response.trim().to_string();
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn tests_prints_agent_msg() {
        PrintCommand::AICall.print_agent_message("Managing Agent", "Testing testing, processing something");
    }
}

pub async fn ai_task_request(message_context: String, agent_position: &str, agent_operation: &str, function_pass: for<'a> fn(&str) -> &'static str) -> String{
    let func_msg: Message = extend_ai_function(function_pass, &message_context);

    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);
    let fmsg = func_msg.clone();

    let llm_response_text = call_gpt(vec!(func_msg));


    // Handle success
    let llm_response: String = match llm_response_text.await {
        Ok(llm_res) => llm_res,
        Err(_) => call_gpt(vec!(fmsg)).await.expect("Failed to call OpenAI")
    };
    String::from("some string")

}