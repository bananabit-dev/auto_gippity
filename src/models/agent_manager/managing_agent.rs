use crate::models::agents::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agents::agent_traits::{FactSheet, SpecialFunctions};

#[derive(Debug)]
struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: FactSheet,
    agents: Vec<BasicAgent>
}


impl ManagingAgent {
    pub async fn new(usr_req: String) -> Result<Self, Box<dyn std::error::Error>> {
        let attributes: BasicAgent = BasicAgent {
            objective: "Manage agents who are building an excellent website for the user".to_string(),
            position: position.clone(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        let ai_response: String = ai_task_request{
            msg_context: usr_req,
            agent_position: &position,
            agent_operation: get_function_string!(convert_user_input_to_goal),
            function_pass: convert_user_input_to_goal,
        }.await;

        let agents: Vec<Box<dyn SpecialFunctions>> = vec![];

        let factsheet: FactSheet = FactSheet {
            project_description,
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };

        Ok(Self {attributes,factsheet,agents})
    }

    fn add_agent(&mut self, agent: Box<dyn SpecialFunctions>) {
        self.agents.push(agent);
    }

    fn create_agents(&mut self) {
        self.add_agent(Box::new(AgentSolutionArchitect::new()));
    }

    pub async fn execute_project(&mut self) {
        self.create_agents();

        for agent in &mut self.agents {
            let agent_res: Result<(), Box<dyn std::error::Error>> = agent.execute(&mut self.factsheet).await;

            let agent_info = agent.get_attributes_from_agent();
            dbg!(agent_info);
        }
    }

}

#[cfg(test)]
mod tests{
    use super::*;

    #[tokio::test]
    async fn test_managing_agent(){
        let usr_requests: str = "Need a full stack app that fetches and tracks my fitness progress. needs to include timezone";

        let mut managing_agent: ManagingAgent = agent.get_attributes_from_agent();
        dbg!(managing_agent.factsheet);
    }
}