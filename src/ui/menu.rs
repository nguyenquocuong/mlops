use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.area());

    let block = Block::default()
        .title(" MLA-C01 Practice Tool ")
        .borders(Borders::ALL);

    let mut text =
        "Main Menu\n\n1. Practice Mode\n2. Exam Mode\n3. History\n\nPress 'q' to quit.".to_string();

    if app.show_resume_prompt {
        text = "Found an active uncompleted session!\n\nPress 'r' to Resume\nPress 'c' to Clear and start over\nPress 'q' to quit.".to_string();
    } else if app.show_domain_prompt {
        text = "Practice Mode - Select Domain\n\n0. All Domains\n1. Domain 1: Data Preparation\n2. Domain 2: ML Model Development\n3. Domain 3: Deployment and Orchestration\n4. Domain 4: Monitoring, Maintenance, Security\n\nPress 'b' to go back.".to_string();
    }

    let paragraph = Paragraph::new(text).block(block);

    f.render_widget(paragraph, chunks[0]);
}
