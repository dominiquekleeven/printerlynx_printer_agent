use crate::adapters::agent_adapter::AgentAdapter;

pub struct PrinterAgent {
    pub adapter: Box<dyn AgentAdapter>,
}
