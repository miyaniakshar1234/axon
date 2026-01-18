mod client;
mod heartbeat;
mod process_manager;
mod tui;

use client::AxonClient;
use client::axon::life_support_server::LifeSupportServer;
use crossterm::event::{self, Event, KeyCode};
use heartbeat::HeartbeatService;
use process_manager::AgentProcess;
use std::error::Error;
use std::time::Duration;
use tokio::time;
use tonic::transport::Server;
use tui::Tui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. Start Heartbeat Server (Hub listens on 50052)
    let addr = "127.0.0.1:50052".parse()?;
    let heartbeat_service = HeartbeatService::default();

    tokio::spawn(async move {
        Server::builder()
            .add_service(LifeSupportServer::new(heartbeat_service))
            .serve(addr)
            .await
            .unwrap();
    });

    // 2. Spawn the Agent
    let mut agent = AgentProcess::spawn()?;
    time::sleep(Duration::from_millis(500)).await;

    // 3. Connect gRPC Client
    let mut client = match AxonClient::connect("http://127.0.0.1:50051".to_string()).await {
        Ok(c) => c,
        Err(e) => {
            agent.kill()?;
            return Err(e.into());
        }
    };

    // 4. Initialize TUI
    let mut tui = Tui::new()?;
    let agent_status = "Connected".to_string();

    // 5. Main Loop
    loop {
        let stats = match client.get_stats().await {
            Ok(s) => format!(
                "CPU: {:.1}% | RAM: {:.1}MB | Uptime: {:.0}s",
                s.cpu_usage, s.ram_usage, s.uptime
            ),
            Err(_) => "Connection Lost".to_string(),
        };

        tui.draw(&format!("{} - {}", agent_status, stats))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }
    }

    tui.shutdown()?;
    agent.kill()?;
    Ok(())
}
