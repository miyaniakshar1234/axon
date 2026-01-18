use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};
use std::io;

pub struct Tui {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl Tui {
    pub fn new() -> Result<Self, io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    pub fn draw(&mut self, agent_status: &str) -> Result<(), io::Error> {
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(f.area()); // Updated: f.size() is deprecated, use f.area()

            let title = Paragraph::new("AXON SYSTEM CONTROLLER")
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            let status = Paragraph::new(format!("Agent Status: {}", agent_status))
                .block(Block::default().title("Status").borders(Borders::ALL));
            f.render_widget(status, chunks[1]);

            let footer =
                Paragraph::new("Press 'q' to quit").block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, chunks[2]);
        })?;
        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<(), io::Error> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
