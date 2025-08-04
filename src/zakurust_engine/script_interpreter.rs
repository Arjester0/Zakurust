use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ScriptCommand {
    CharacterLine {
        name: String,
        expression: String,
        line: String,
    },
    Choice {
        prompt: String,
        option1: String,
        option2: String,
    },
}

pub fn load_script_from_file(path: &str) -> Result<Vec<ScriptCommand>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let parsed: Vec<ScriptCommand> = serde_yaml::from_str(&contents)?;
    Ok(parsed)
}

pub fn load_script_from_str(yaml: &str) -> Result<Vec<ScriptCommand>, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}
