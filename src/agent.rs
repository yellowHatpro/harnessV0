use crate::model::{Model, ModelContent};
use crate::state::Task;
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
            for content in response.content {
                return match content {
                    ModelContent::Text(text) => {
                        //handling model text
                        println!("Handling Model content text: {}", text);
                        Ok(())
                    }
                    ModelContent::ToolCall { id, name, input } => {
                        // execute tool
                        println!("Handling Model content tool call for {}: {}", id, name);
                        Ok(())
                    }
                }
            }
        }
    }
}