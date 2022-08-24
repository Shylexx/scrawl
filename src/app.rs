use std::{io::{self, Stdout}, thread, time::Duration};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture}};
use tui::{backend::CrosstermBackend,  Terminal, widgets::{Block, Borders}};

use crate::input::{InputEvent, key::Key};
use crate::input::events::Events;
use crate::app::state::AppState;
use crate::todo::Todo;

pub mod actions;
pub mod state;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AppReturn {
    Exit,
    Continue,
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub events: Events,
    state: AppState,
    todo: Todo,
}


impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Result<(App<'a>,Terminal<CrosstermBackend<Stdout>>), io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        let tick_rate = Duration::from_millis(200);
        let events = Events::new(tick_rate);
        let state = AppState::initialize();
        let todo = Todo::load("db.json".to_string());
        Ok((App {
            title,
            should_quit: false,
            events,
            state,
            todo,
        }, terminal))
    }
    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), io::Error> {

        loop {
            // Handle Inputs
            let result = match self.events.next().unwrap_or_else(|_| {
                panic!("Could not Get Event!");
            }) {
                InputEvent::Input(key) => self.do_action(key),
                // Tick if no input
                InputEvent::Tick => self.update_on_tick(),
            };
            if result == AppReturn::Exit {
                break;
            }

        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Block")
                .borders(Borders::ALL);
            f.render_widget(block, size);
        })?;
        
        }


        Ok(())
    }
    pub fn cleanup(&self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), io::Error> {
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        Ok(())
    }
    fn do_action(&mut self, key: Key) -> AppReturn {
        let action_result = match key {
            Key::Char('q') => AppReturn::Exit,
            _ => AppReturn::Continue,
        };
        action_result
    }
    fn update_on_tick(&mut self) -> AppReturn {
        self.state.incr_tick();
        AppReturn::Continue
    }
}
