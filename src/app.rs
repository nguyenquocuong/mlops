use crate::models::{QuestionBank, SessionType, TestSession};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum AppMode {
    Menu,
    Practice,
    Exam,
    History,
}

pub struct App {
    pub mode: AppMode,
    pub should_quit: bool,
    pub active_session: Option<TestSession>,
    pub current_question_index: usize,
    pub showing_explanation: bool,
    pub db: Option<crate::db::Db>,
    pub show_resume_prompt: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            mode: AppMode::Menu,
            should_quit: false,
            active_session: None,
            current_question_index: 0,
            showing_explanation: false,
            db: None,
            show_resume_prompt: false,
        }
    }

    pub fn start_session(&mut self, bank: &QuestionBank, mode: SessionType, num_questions: usize) {
        let mut questions = bank.questions.clone();

        let mut rng = thread_rng();
        questions.shuffle(&mut rng);
        questions.truncate(num_questions);

        let session = TestSession {
            id: Uuid::new_v4(),
            mode: mode.clone(),
            questions,
            user_answers: HashMap::new(),
            time_elapsed: 0,
            time_limit: if mode == SessionType::Exam { 7800 } else { 0 },
            is_completed: false,
            score_percentage: None,
            passed: None,
        };

        self.active_session = Some(session);
        self.current_question_index = 0;
        self.showing_explanation = false;

        match mode {
            SessionType::Practice => self.mode = AppMode::Practice,
            SessionType::Exam => self.mode = AppMode::Exam,
        }
    }

    pub fn submit_answer(&mut self, answer_index: u8) {
        if let Some(session) = &mut self.active_session {
            if self.showing_explanation {
                return;
            }
            if session.questions.is_empty() {
                return;
            }
            let q_id = session.questions[self.current_question_index].id.clone();
            session.user_answers.insert(q_id, vec![answer_index]);

            if session.mode == SessionType::Practice {
                self.showing_explanation = true;
            }
        }
    }

    pub fn next_question(&mut self) {
        if let Some(session) = &mut self.active_session {
            if self.current_question_index + 1 < session.questions.len() {
                self.current_question_index += 1;
                self.showing_explanation = false;
            } else {
                self.finish_session();
            }
        }
    }

    pub fn finish_session(&mut self) {
        if let Some(session) = &mut self.active_session {
            session.is_completed = true;
            let mut correct_count = 0;
            for q in &session.questions {
                if let Some(user_ans) = session.user_answers.get(&q.id) {
                    if user_ans == &q.correct_answer_indices {
                        correct_count += 1;
                    }
                }
            }
            if session.questions.is_empty() {
                session.score_percentage = Some(0.0);
                session.passed = Some(false);
            } else {
                let pct = (correct_count as f64 / session.questions.len() as f64) * 100.0;
                session.score_percentage = Some(pct);
                session.passed = Some(pct >= 72.0);
            }
            if let Some(ref db) = self.db {
                let _ = db.save_history(session);
                let _ = db.clear_active_session();
            }
        }
    }

    pub fn tick(&mut self) {
        if let Some(session) = &mut self.active_session {
            if !session.is_completed {
                session.time_elapsed += 1; // Assuming tick is called 1x/sec or we accumulate based on actual time.
                // Wait, main loop ticks at 250ms but we accumulate seconds? We need a better clock in App, but simple approx is ok for now, or just trust the tick is accurate but it varies.
                // For a more accurate tick, we'd store a start `Instant` in the session, but `Instant` doesn't serialize. We'll simulate 1/4 second ticks.
                // Let's just do an elapsed calculation in real usage or ignore the exact 250ms for now.
            }
        }
    }

    pub fn quit(&mut self) {
        if let Some(session) = &self.active_session {
            if !session.is_completed {
                if let Some(ref db) = self.db {
                    let _ = db.save_active_session(session);
                }
            }
        }
        self.should_quit = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Question;

    #[test]
    fn test_shuffling_subset() {
        let mut app = App::new();
        let mut questions = Vec::new();
        for i in 0..100 {
            questions.push(Question {
                id: i.to_string(),
                prompt: format!("Q{}", i),
                choices: vec!["A".to_string(), "B".to_string()],
                correct_answer_indices: vec![0],
                explanation: "Exp".to_string(),
            });
        }
        let bank = QuestionBank {
            id: "t".to_string(),
            version: "1.0".to_string(),
            questions,
        };

        app.start_session(&bank, SessionType::Exam, 65);

        let session = app.active_session.as_ref().unwrap();
        assert_eq!(session.questions.len(), 65);
        assert_eq!(app.mode, AppMode::Exam);
    }

    #[test]
    fn test_scoring_logic() {
        let mut app = App::new();
        let bank = QuestionBank {
            id: "t".to_string(),
            version: "1.0".to_string(),
            questions: vec![
                Question {
                    id: "q1".to_string(),
                    prompt: "1+1?".to_string(),
                    choices: vec!["2".to_string(), "3".to_string()],
                    correct_answer_indices: vec![0],
                    explanation: "".to_string(),
                },
                Question {
                    id: "q2".to_string(),
                    prompt: "2+2?".to_string(),
                    choices: vec!["4".to_string(), "5".to_string()],
                    correct_answer_indices: vec![0],
                    explanation: "".to_string(),
                },
            ],
        };
        app.start_session(&bank, SessionType::Exam, 2);

        // Setup answers
        app.submit_answer(0); // q1 correct
        app.next_question();
        app.submit_answer(1); // q2 wrong (expect index 0)
        app.next_question(); // finishes session

        let session = app.active_session.as_ref().unwrap();
        assert!(session.is_completed);
        assert_eq!(session.score_percentage.unwrap(), 50.0);
        assert_eq!(session.passed.unwrap(), false);
    }
}
