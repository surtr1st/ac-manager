use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, List, ListItem},
    Frame,
};

pub fn block_part<B: Backend>(f: &mut Frame<B>) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size())
}

pub fn render_list<'l>(block: Block<'l>, items: Vec<ListItem<'l>>) -> List<'l> {
    List::new(items)
        .block(block)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ")
}

pub fn render_table() {}