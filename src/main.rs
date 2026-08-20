use crate::agent::Agent;
use crate::model::OpenAIModel;
use crate::state::{Message, Task, TaskStatus};
use crate::tool::{ShellTool, Tool};

mod state;
mod model;
mod tool;
mod agent;

fn main() {

    let model = OpenAIModel{
        endpoint: "".to_string(),
        model: "".to_string(),
    };

    let tools = vec![
        Box::new(ShellTool) as Box<dyn Tool>
    ];

    let agent = Agent::new(model, tools);

    let mut task = Task {
        id: "task-1".to_string(),
        status: TaskStatus::Running,
        messages: vec![
            Message {
                role: "user".to_string(),
                content: "List all files in the current directory".to_string(),
            }
        ]
    };
    match agent.run(&mut task) {
        Ok(()) => {
            println!("{} Task completed", task.id);
        }
        Err(error) => {
            eprintln!("Task Failed: {}", error);
        }
    }
    println!("------------------------");
    println!("Final task state:");
    println!("{:#?}", task);

}