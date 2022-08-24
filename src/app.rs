use std::{io::{self, Stdout}, thread, time::Duration};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture}};
use tui::{backend::CrosstermBackend,  Terminal, widgets::{Block, Borders}};

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
}


impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Result<(App<'a>,Terminal<CrosstermBackend<Stdout>>), io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok((App {
            title,
            should_quit: false,
        }, terminal))
    }
    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), io::Error> {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Block")
                .borders(Borders::ALL);
            f.render_widget(block, size);
        })?;
        

        thread::sleep(Duration::from_millis(5000));

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
}
