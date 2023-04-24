use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem},
    Frame,
    style::{Style, Color, Modifier}
};

pub fn ui<B: Backend>(f: &mut Frame<B>) {
   let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(100),
                Constraint::Percentage(50)
            ].as_ref()
        )
        .split(f.size());
    let select_block = Block::default()
         .title("Container")
         .borders(Borders::ALL);
    let list = render_list(select_block);
    f.render_widget(list, chunks[0]);
}

pub fn render_list<'l>(block: Block<'l>) -> List<'l> {
    let items = [ListItem::new("Item 1"), ListItem::new("Item 2"), ListItem::new("Item 3")];
    let list = List::new(items)
        .block(block)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");
    list
}

pub fn render_table() {

}

