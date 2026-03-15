use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.area());

    let mut text = vec![];
    text.push(Line::from("Session History"));
    text.push(Line::from(""));

    if let Some(ref db) = app.db {
        if let Ok(history) = db.get_history() {
            if history.is_empty() {
                text.push(Line::from("No history yet."));
            } else {
                for rec in history {
                    text.push(Line::from(rec));
                }
            }
        } else {
            text.push(Line::from("Error loading history."));
        }
    }

    text.push(Line::from(""));
    text.push(Line::from(
        "Press 'c' to clear history. Press 'q' or 'm' to go back.",
    ));

    let block = Block::default().title(" History ").borders(Borders::ALL);

    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[0]);
}
