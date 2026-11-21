use std::time::{Duration, Instant};
use super::render::draw;
use super::event::handle_events;


const TICK_RATE: Duration = Duration::from_millis(500);

pub struct App {
    running: bool
}

impl App {
    pub fn new() -> Self {
        Self { running: (true) }
    }

    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
        let mut last_tick = Instant::now();

        while self.running {
            terminal.draw(|frame| draw(frame, self))?;

            let timeout: Duration = TICK_RATE.saturating_sub(last_tick.elapsed());
            let _ = handle_events(self, timeout, & mut last_tick, TICK_RATE);


        }
        Ok(())
    }
    pub fn exit(&mut self){
        self.running = false;
    }
}