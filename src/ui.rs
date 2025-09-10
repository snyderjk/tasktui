use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem},
};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(frame.size());

    let items: Vec<ListItem> = app
        .tasks
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let mut line = Line::from(format!("{} {}", status_emoji(&t.status), t.title));
            if i == app.selected {
                line = line.patch_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(ratatui::style::Color::Yellow),
                );
            }
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Tasks"));
    frame.render_widget(list, chunks[0]);

    let right = Block::default()
        .borders(Borders::ALL)
        .title("Details & Timer");
    frame.render_widget(right, chunks[1]);
}

fn status_emoji(s: &crate::model::Status) -> &'static str {
    match s {
        crate::model::Status::Todo => "□",
        crate::model::Status::Doing => "▶",
        crate::model::Status::Done => "✓",
        crate::model::Status::Archived => "⎌",
    }
}
