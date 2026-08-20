pub trait Tool {
    fn name(&self) -> &str;
    fn execute(&self, input: &str) -> Result<String, String>;
}

pub struct ShellTool;

impl Tool for ShellTool {

    fn name(&self) -> &str {
        "shell"
    }

    fn execute(&self, input: &str) -> Result<String, String> {
        // todo!(
        //     "Execute this {input} on shell"
        // )
        Ok(input.to_string())
    }
}