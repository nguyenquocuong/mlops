use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.area());

    if let Some(session) = &app.active_session {
        if session.questions.is_empty() {
            return;
        }

        let question = &session.questions[app.current_question_index];
        let q_id = &question.id;
        let mut text = vec![];

        text.push(Line::from(format!(
            "Question {} of {}",
            app.current_question_index + 1,
            session.questions.len()
        )));
        text.push(Line::from(""));
        text.push(Line::from(Span::styled(&question.prompt, Style::default())));
        text.push(Line::from(""));

        let user_ans = session.user_answers.get(q_id);

        for (i, choice) in question.choices.iter().enumerate() {
            let prefix = if user_ans.map_or(false, |a| a.contains(&(i as u8))) {
                "[x]"
            } else {
                "[ ]"
            };
            text.push(Line::from(format!("{} {}. {}", prefix, i + 1, choice)));
        }

        let q_block = Block::default()
            .title(" Practice Mode ")
            .borders(Borders::ALL);

        let paragraph = Paragraph::new(text)
            .block(q_block)
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunks[0]);

        if app.showing_explanation {
            let mut exp_text = vec![];
            let is_correct = user_ans.map_or(false, |a| a == &question.correct_answer_indices);

            if is_correct {
                exp_text.push(Line::from(Span::styled(
                    "Correct!",
                    Style::default().fg(Color::Green),
                )));
            } else {
                exp_text.push(Line::from(Span::styled(
                    "Incorrect.",
                    Style::default().fg(Color::Red),
                )));
            }

            exp_text.push(Line::from(""));
            exp_text.push(Line::from(&*question.explanation));
            exp_text.push(Line::from(""));
            exp_text.push(Line::from("Press 'n' for next question."));

            let exp_block = Block::default()
                .title(" Explanation ")
                .borders(Borders::ALL);

            let exp_para = Paragraph::new(exp_text)
                .block(exp_block)
                .wrap(Wrap { trim: true });
            f.render_widget(exp_para, chunks[1]);
        } else {
            let hint_block = Block::default().title(" Help ").borders(Borders::ALL);

            let hint_para = Paragraph::new("Press 1-4 to select an answer.\nPress 'q' to quit.")
                .block(hint_block)
                .wrap(Wrap { trim: true });
            f.render_widget(hint_para, chunks[1]);
        }
    }
}
