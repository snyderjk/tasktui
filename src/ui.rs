use crate::app::{AddField, App, Mode};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
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

    let description = Line::from(app.tasks[app.selected].notes.clone());

    let right = Paragraph::new(description).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Details & Timer"),
    );
    frame.render_widget(right, chunks[1]);

    if let Mode::Adding(form) = &app.mode {
        let area = centered_rect(60, 50, frame.size());
        frame.render_widget(Clear, area);

        let block = Block::default().borders(Borders::ALL).title("Add Task");
        let inner = block.inner(area);

        frame.render_widget(block, area);

        let inner_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(1),
            ])
            .split(inner);

        let title_para = Paragraph::new(form.title.as_str())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Title")
                    .border_style(match form.field {
                        AddField::Title => Style::default().fg(ratatui::style::Color::Yellow),
                        _ => Style::default(),
                    }),
            )
            .wrap(Wrap { trim: false });

        let notes_para = Paragraph::new(form.notes.as_str())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Notes")
                    .border_style(match form.field {
                        AddField::Notes => Style::default().fg(ratatui::style::Color::Yellow),
                        _ => Style::default(),
                    }),
            )
            .wrap(Wrap { trim: false });

        let help = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw("Esc cancel   "),
            Span::raw("Enter save   "),
            Span::raw("Tab/Shift+Tab switch   "),
        ])]))
        .alignment(Alignment::Center);

        frame.render_widget(title_para, inner_chunks[0]);
        frame.render_widget(notes_para, inner_chunks[1]);
        frame.render_widget(help, inner_chunks[2]);

        match form.field {
            AddField::Title => {
                let x = inner_chunks[0].x + 1 + form.title.len() as u16;
                let y = inner_chunks[0].y + 1;
                frame.set_cursor(x, y);
            }
            AddField::Notes => {
                let lines = form.notes.split('\n').collect::<Vec<_>>();
                let last = lines.last().unwrap_or(&"");
                let x = inner_chunks[1].x + 1 + last.len() as u16;
                let y = inner_chunks[1].y + 1 + (lines.len().saturating_sub(1)) as u16;
                frame.set_cursor(x, y);
            }
        }
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
