use crate::app::{AddField, App, Mode};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem},
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

    if let Mode::Adding(form) = &app.mode {
        let area = centered_rect(60, 50, frame.size());
        frame.render_widget(Clear, area);

        let block = Block::default().borders(Borders::ALL).title("Add Task");
        let inner = block.inner(area);

        frame.render_widget(block, area);
    }
}

fn status_emoji(s: &crate::model::Status) -> &'static str {
    match s {
        crate::model::Status::Todo => "□",
        crate::model::Status::Doing => "▶",
        crate::model::Status::Done => "✓",
        crate::model::Status::Archived => "⎌",
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1]);

    horizontal[1]
}
