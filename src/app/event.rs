use std::time::{Duration, Instant};
use crossterm::event::{self, Event::Key};
use ratatui::{
    crossterm::event::{KeyEvent, KeyCode, KeyEventKind, KeyEventState, Event}
};


use super::app::{
    App
};

pub fn handle_key_events(_app: &mut App, key: KeyEvent){
    if key.kind != KeyEventKind::Press {
        return;
    }
    match key.code {
        KeyCode::Char('q') => _app.exit(),
        _=>{}
    }
}

pub fn on_tick(_app: &mut App){
    
}

pub fn handle_events(_app: &mut App, timeout: Duration, last_tick: &mut Instant, tickrate: Duration)->std::io::Result<()>{
    if event::poll(timeout)? {
        if let Event::Key(key) = event::read()?{
            handle_key_events(_app, key);
        }
            
    }
    if last_tick.elapsed() >= tickrate{
        on_tick(_app);
        *last_tick = Instant::now()
    }
    Ok(())
}