#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {{
        stringify!($func)
    }
        
    };
}

mod ai_functions;
mod apis;
mod helpers;
mod models;
use crate::models::agent_manager::managing_agent::ManagingAgent;

use helpers::command_line::get_user_response;

#[tokio::main]
async fn main() {
    let usr_req: String = get_user_response("What webserver are we building today?");

    let mut manage_agent: ManagingAgent = ManagingAgent::new(usr_req).await.expect("Error creating agent!");

    manage_agent.execute_project().await;

    dbg!(manage_agent);
}
