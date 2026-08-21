use std::process::Command;
use serde::Deserialize;

pub trait Tool {
    fn name(&self) -> &str;
    fn execute(&self, input: &str) -> Result<String, String>;
}

pub struct ShellTool;

#[derive(Deserialize)]
struct ShellInput {
    command: String,
}

impl Tool for ShellTool {

    fn name(&self) -> &str {
        "shell"
    }

    fn execute(&self, input: &str) -> Result<String, String> {
        let input: ShellInput = serde_json::from_str(input).map_err(|e| e.to_string())?;
        let output = Command::new("sh")
            .arg("-lc")
            .arg(&input.command)
            .output()
            .map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}