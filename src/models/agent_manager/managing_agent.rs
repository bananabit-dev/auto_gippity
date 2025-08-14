use crate::models::agents::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agents::agent_traits::{FactSheet, SpecialFunctions};

#[derive(Debug)]
struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: FactSheet,
    agents: Vec<BasicAgent>
}

pub trait SpecialFunctions: Debug {
    // used to that manager can get attributes from agents
    fn get_attributes_from_agent(&self) -> &BasicAgent;

    async fn execute(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>>;
}