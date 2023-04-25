pub mod app_layout;
pub mod manager;
pub mod query;
pub mod reader;
pub mod term_event;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CrosstermEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use term_event::TerminalEvent;
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, ListItem, ListState},
    Terminal,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    enable_raw_mode()?;
    let rx = mpsc_setup();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let items = ["Item 1", "Item 2", "Item 3", "Item 4", "Item 5", "Item 6"]
        .iter()
        .map(|i| ListItem::new(i.to_string()))
        .collect::<Vec<ListItem>>();

    let mut list_state = ListState::default();
    list_state.select(Some(0));

    loop {
        terminal.draw(|f| {
            let chunks = app_layout::block_part(f);
            let select_block = Block::default().title("Container").borders(Borders::ALL);
            let list = app_layout::render_list(select_block, items.clone());
            f.render_stateful_widget(list, chunks[0], &mut list_state);
        })?;

        let index = list_state.selected().unwrap_or(0);
        match rx.recv()? {
            TerminalEvent::Input(key) => match key.code {
                KeyCode::Char('q') => {
                    // restore terminal
                    disable_raw_mode()?;
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('j') => {
                    if index < items.len() - 1 {
                        list_state.select(Some(index + 1));
                    }
                }
                KeyCode::Char('k') => {
                    if index > 0 {
                        list_state.select(Some(index - 1));
                    }
                }
                KeyCode::Enter => {
                    // restore terminal
                    disable_raw_mode()?;
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    terminal.show_cursor()?;
                    break;
                }
                _ => {}
            },
            TerminalEvent::Tick => {}
        }
    }

    Ok(())
}

fn mpsc_setup() -> mpsc::Receiver<TerminalEvent<event::KeyEvent>> {
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CrosstermEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(TerminalEvent::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(TerminalEvent::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });
    rx
}
