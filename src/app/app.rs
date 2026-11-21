use std::os;
use std::time::{Duration, Instant};
use super::render::draw;
use super::event::handle_events;
use sysinfo::System;

const TICK_RATE: Duration = Duration::from_millis(500);

pub struct App {
    pub running: bool,
    pub sys: System,
    pub cpu_usage: f32,
    pub ram_used: u64,
    pub system_name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub host_name: String,
    pub core_count: usize,
    pub cpu_model: String,
    pub cpu_architecture: String,
    pub total_ram: u64,
    pub username: String
}

impl App {
    pub fn new() -> Self {
        let mut sys = System::new_all().expect("couldn't start sysinfo c crate");
        sys.refresh_all();
        let system_name = System::name().expect("couldn't find system name");
        let kernel_version = System::kernel_version().expect("couldn't find kernel version");
        let os_version = System::long_os_version().expect("couldn't find os version");
        let host_name = System::host_name().expect("couldn't find host name");
        let core_count = System::physical_core_count().expect("couldn't find core count");
        let cpu_model = System::cpus(&sys)[0].brand().to_string();
        let cpu_architecture = System::cpu_arch();
        let total_ram = System::total_memory(&sys).to_be();
        let username = std::env::var("USER").expect("couldn't find username");
        Self {
            running: true,
            sys,
            cpu_usage: 0.0,
            ram_used: 0,
            system_name,
            kernel_version,
            os_version,
            host_name,
            core_count,
            cpu_model,
            cpu_architecture,
            total_ram,
            username
        }
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