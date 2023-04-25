use crate::{stateful_list::StatefulList, term_event::TerminalEvent};
use crossterm::{
    event::{self, KeyCode},
    terminal::disable_raw_mode,
};
use std::sync::mpsc;
use tui::{backend::Backend, widgets::ListItem, Terminal};

pub fn use_vim_navigation<B: Backend>(
    terminal: &mut Terminal<B>,
    rx: &mpsc::Receiver<TerminalEvent<event::KeyEvent>>,
    stateful_list: &mut StatefulList<ListItem>,
) -> Result<(), Box<dyn std::error::Error>> {
    match rx.recv()? {
        TerminalEvent::Input(key) => match key.code {
            KeyCode::Char('q') => {
                // restore terminal
                disable_raw_mode()?;
                terminal.show_cursor()?;
                return Ok(());
            }
            KeyCode::Char('j') => {
                stateful_list.next();
            }
            KeyCode::Char('k') => {
                stateful_list.previous();
            }
            _ => {}
        },
        TerminalEvent::Tick => {}
    }
    Ok(())
}