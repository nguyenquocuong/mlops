pub mod exam;
pub mod history;
pub mod menu;
pub mod practice;

use crate::app::{App, AppMode};
use ratatui::Frame;

pub fn draw(f: &mut Frame, app: &mut App) {
    match app.mode {
        AppMode::Menu => menu::draw(f, app),
        AppMode::Practice => practice::draw(f, app),
        AppMode::Exam => exam::draw(f, app),
        AppMode::History => history::draw(f, app),
    }
}
