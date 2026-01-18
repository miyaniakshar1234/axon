use std::path::Path;
use std::process::{Child, Command};

pub struct AgentProcess {
    child: Child,
}

impl AgentProcess {
    pub fn spawn() -> Result<Self, std::io::Error> {
        // Assume axon-agent.exe is in the same directory or a known relative path
        // For dev: ../axon-agent/axon-agent.exe (if built there)
        // For release: ./axon-agent.exe

        let agent_path = if Path::new("axon-agent.exe").exists() {
            "axon-agent.exe"
        } else if Path::new("../axon-agent/axon-agent.exe").exists() {
            "../axon-agent/axon-agent.exe"
        } else {
            "axon-agent" // Hope it's in PATH
        };

        let child = Command::new(agent_path)
            // .arg("--port=50051") // Future flag
            .spawn()?;

        Ok(Self { child })
    }

    pub fn kill(&mut self) -> Result<(), std::io::Error> {
        self.child.kill()
    }
}
