use anyhow::Result;
use log::{info, warn};
use std::collections::HashMap;
use std::process::Command;

#[derive(Clone)] 
pub struct CommandMatcher {
    commands: HashMap<String, String>,
}

impl CommandMatcher {
    pub fn new(commands: HashMap<String, String>) -> Self {
        Self { commands }
    }

    pub async fn match_and_execute(&self, text: &str) -> Result<()> {
        let clean_input = text.to_lowercase()
            .chars()
            .filter(|c| !c.is_ascii_punctuation())
            .collect::<String>();
        
        let clean_input = clean_input.trim();

        if clean_input.is_empty() {
            return Ok(());
        }

        for (trigger, shell_cmd) in &self.commands {
            let clean_trigger = trigger.to_lowercase()
                .chars()
                .filter(|c| !c.is_ascii_punctuation())
                .collect::<String>();
            
            let clean_trigger = clean_trigger.trim();

            if clean_input.contains(clean_trigger) {
                info!("Matched command: '{}' -> '{}'", trigger, shell_cmd);
                self.execute_command(shell_cmd).await?;
                return Ok(());
            }
        }

        warn!("No command matched for input: '{}'", text);
        Ok(())
    }

    async fn execute_command(&self, cmd: &str) -> Result<()> {
        info!("Executing: {}", cmd);
        
        let child = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .spawn(); 

        match child {
            Ok(_) => {
                info!("Process spawned successfully: {}", cmd);
            }
            Err(e) => {
                warn!("Failed to spawn command '{}': {}", cmd, e);
            }
        }

        Ok(())
    }
}
