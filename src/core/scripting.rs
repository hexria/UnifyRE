use crate::errors::Result;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct UreScript {
    pub name: String,
    pub tasks: Vec<Task>,
}

#[derive(Deserialize)]
pub enum Task {
    Analyze { output: String, html: bool },
    Scan { pattern: String },
    Disasm { entry: bool },
}

pub struct ScriptEngine;

impl ScriptEngine {
    pub fn run<P: AsRef<Path>>(script_path: P, _binary_path: &str) -> Result<()> {
        let content =
            fs::read_to_string(script_path).map_err(|e| crate::errors::UnifyError::Io(e))?;
        let script: UreScript = serde_json::from_str(&content).map_err(|e| {
            crate::errors::UnifyError::Internal(format!("Failed to parse script: {}", e))
        })?;

        println!("🚀 Running script: {}", script.name);

        for task in script.tasks {
            match task {
                Task::Analyze { output, html } => {
                    println!("  - Task: Analyze (output: {}, html: {})", output, html);
                    // Implementation logic will be called here
                }
                Task::Scan { pattern } => {
                    println!("  - Task: Scan (pattern: {})", pattern);
                }
                Task::Disasm { entry } => {
                    println!("  - Task: Disasm (entry: {})", entry);
                }
            }
        }

        Ok(())
    }
}
