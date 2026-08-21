use crate::model::{Model, ModelContent};
use crate::state::{Message, Task, TaskStatus};
use crate::tool::Tool;

pub struct Agent<M: Model>{
    model: M,
    tools: Vec<Box<dyn Tool>>,
}

impl<M: Model> Agent<M> {
    pub fn new(model: M, tools: Vec<Box<dyn Tool>>) -> Self {
        Self {model, tools}
    }

    pub fn run(&self, task: &mut Task) -> Result<(), String> {
        loop {
            let response = self.model.invoke(task)?;
            let mut tool_called = false;
            let mut text_recieved = false;
            for content in response.content {
                match content {
                    ModelContent::Text(text) => {
                        println!("Model: {}", text);
                        task.messages.push(Message {
                            role: "assistant".to_string(),
                            content: text
                        });

                        text_recieved = true;
                    }
                    ModelContent::ToolCall { id, name, input } => {
                        println!("Tool call: {} ({})", name, id);

                        let tool = self
                            .tools
                            .iter()
                            .find(|tool| tool.name() == name.as_str())
                            .ok_or_else(|| format!("Tool {} not found", name))?;
                        let res = tool.execute(&input)?;
                        println!("Tool result:\n {}", res);

                        task.messages.push(Message {
                            role: "tool".to_string(),
                            content: format!(
                                "tool_call_id: {}\n{}",
                                id, res
                            )
                        });
                        tool_called = true;
                    }
                }
            }
            if tool_called {
                continue;
            }
            if text_recieved {
                task.status = TaskStatus::Completed;
                return Ok(());
            }
            return Err("Model returned no actionable content".to_string());
        }
    }
}