use crate::state::{Task};


pub enum ModelContent {
    Text(String),
    ToolCall {
        id: String,
        name: String,
        input: String,
    }
}

pub struct ModelOutput {
    pub id: String,
    pub content: Vec<ModelContent>,
    stop_reason: StopReason
}

pub trait Model {
    fn invoke(&self, task: &Task) -> Result<ModelOutput, String>;
}

pub enum StopReason {
    ToolCall,
}

// Starting with OpenAI. Plan is to use Codex
pub struct OpenAIModel {
    pub endpoint: String,
    pub model: String,
}

impl Model for OpenAIModel {
    fn invoke(&self, task: &Task) -> Result<ModelOutput, String> {
        // a dummy response for now
        Ok(ModelOutput {
            id: "resp_123".to_string(),

            content: vec![
                ModelContent::ToolCall {
                    id: "call_456".to_string(),
                    name: "shell".to_string(),
                    input: r#"{"command":"ls"}"#.to_string(),
                }
            ],
            stop_reason: StopReason::ToolCall,
        })

    }
}