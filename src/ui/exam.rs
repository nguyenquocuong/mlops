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
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(f.area());

    if let Some(session) = &app.active_session {
        if session.is_completed {
            let passed = session.passed.unwrap_or(false);
            let color = if passed { Color::Green } else { Color::Red };
            let result_str = if passed { "PASS" } else { "FAIL" };

            let mut text = vec![];
            text.push(Line::from("Exam Completed!"));
            text.push(Line::from(""));
            text.push(Line::from(vec![
                Span::raw("Score: "),
                Span::styled(
                    format!("{:.1}%", session.score_percentage.unwrap_or(0.0)),
                    Style::default().fg(color),
                ),
            ]));
            text.push(Line::from(vec![
                Span::raw("Result: "),
                Span::styled(result_str, Style::default().fg(color)),
            ]));
            text.push(Line::from(""));
            text.push(Line::from("Domain Breakdown:"));
            for i in 1..=4 {
                if let Some(stat) = session.domain_stats.get(&i) {
                    if stat.total > 0 {
                        let dom_pct = (stat.correct as f64 / stat.total as f64) * 100.0;
                        text.push(Line::from(format!(
                            " - Domain {}: {:.1}% ({}/{})",
                            i, dom_pct, stat.correct, stat.total
                        )));
                    }
                }
            }
            text.push(Line::from(""));
            text.push(Line::from("Press 'q' or 'm' to return to Main Menu."));

            let block = Block::default()
                .title(" Exam Results ")
                .borders(Borders::ALL);
            let para = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
            f.render_widget(para, chunks[0]);
            return;
        }

        if session.questions.is_empty() {
            return;
        }

        let question = &session.questions[app.current_question_index];
        let q_id = &question.id;
        let user_ans = session.user_answers.get(q_id);

        // Convert time_elapsed (ticks, ~0.25s) into MM:SS
        let total_secs_limit = session.time_limit;
        let ticks_passed = session.time_elapsed;
        let secs_passed = ticks_passed / 4;
        let secs_remaining = total_secs_limit.saturating_sub(secs_passed);
        let mins = secs_remaining / 60;
        let secs = secs_remaining % 60;

        let mut text = vec![];
        text.push(Line::from(format!(
            "Question {} of {} | Time Remaining: {:02}:{:02}",
            app.current_question_index + 1,
            session.questions.len(),
            mins,
            secs
        )));
        text.push(Line::from(""));
        text.push(Line::from(Span::styled(&question.prompt, Style::default())));
        text.push(Line::from(""));

        for (i, choice) in question.choices.iter().enumerate() {
            let prefix = if user_ans.map_or(false, |a| a.contains(&(i as u8))) {
                "[x]"
            } else {
                "[ ]"
            };
            text.push(Line::from(format!("{} {}. {}", prefix, i + 1, choice)));
        }

        let q_block = Block::default().title(" Exam Mode ").borders(Borders::ALL);

        let paragraph = Paragraph::new(text)
            .block(q_block)
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunks[0]);

        let mut hint_text = vec![];
        hint_text.push(Line::from("Press 1-4 to select answer."));
        hint_text.push(Line::from("Press 'n' for Next question."));
        hint_text.push(Line::from("Press 'q' or 'm' to Quit Exam/Return to Menu."));

        let hint_block = Block::default().title(" Controls ").borders(Borders::ALL);
        let hint_para = Paragraph::new(hint_text)
            .block(hint_block)
            .wrap(Wrap { trim: true });
        f.render_widget(hint_para, chunks[1]);
    }
}
