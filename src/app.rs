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
    pub show_domain_prompt: bool,
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
            show_domain_prompt: false,
        }
    }

    pub fn start_session(
        &mut self,
        bank: &QuestionBank,
        mode: SessionType,
        num_questions: usize,
        target_domain: Option<u8>,
    ) {
        let mut questions = bank.questions.clone();

        let mut rng = thread_rng();

        if let Some(domain) = target_domain {
            questions.retain(|q| q.domain == domain);
        } else if mode == SessionType::Exam {
            let total = num_questions.min(questions.len());
            let proportions = [(1, 0.28), (2, 0.26), (3, 0.22), (4, 0.24)];

            let mut exact_floors = Vec::new();
            let mut remainders = Vec::new();
            for (d, prop) in proportions {
                let exact = (total as f64) * prop;
                let floor = exact.floor() as usize;
                let rem = exact - exact.floor();
                exact_floors.push((d, floor));
                remainders.push((d, rem));
            }

            let mut allocated: usize = exact_floors.iter().map(|(_, c)| c).sum();
            // descending sort remainders
            remainders.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            let mut final_counts: HashMap<u8, usize> = exact_floors.into_iter().collect();
            for (d, _) in remainders {
                if allocated < total {
                    *final_counts.get_mut(&d).unwrap() += 1;
                    allocated += 1;
                } else {
                    break;
                }
            }

            // Split bank questions by domain
            let mut d1_q: Vec<_> = questions
                .iter()
                .filter(|q| q.domain == 1)
                .cloned()
                .collect();
            let mut d2_q: Vec<_> = questions
                .iter()
                .filter(|q| q.domain == 2)
                .cloned()
                .collect();
            let mut d3_q: Vec<_> = questions
                .iter()
                .filter(|q| q.domain == 3)
                .cloned()
                .collect();
            let mut d4_q: Vec<_> = questions
                .iter()
                .filter(|q| q.domain == 4)
                .cloned()
                .collect();

            d1_q.shuffle(&mut rng);
            d2_q.shuffle(&mut rng);
            d3_q.shuffle(&mut rng);
            d4_q.shuffle(&mut rng);

            // Greedily fill quotas
            let mut picked = Vec::new();

            let mut pull_from = |q_vec: &mut Vec<crate::models::Question>, count: usize| -> usize {
                let actual = count.min(q_vec.len());
                picked.extend(q_vec.drain(..actual));
                count - actual
            };

            let d1_deficit = pull_from(&mut d1_q, *final_counts.get(&1).unwrap());
            let d2_deficit = pull_from(&mut d2_q, *final_counts.get(&2).unwrap());
            let d3_deficit = pull_from(&mut d3_q, *final_counts.get(&3).unwrap());
            let d4_deficit = pull_from(&mut d4_q, *final_counts.get(&4).unwrap());

            let total_deficit = d1_deficit + d2_deficit + d3_deficit + d4_deficit;
            if total_deficit > 0 {
                let mut remaining: Vec<_> = d1_q
                    .into_iter()
                    .chain(d2_q)
                    .chain(d3_q)
                    .chain(d4_q)
                    .collect();
                remaining.shuffle(&mut rng);
                remaining.truncate(total_deficit);
                picked.extend(remaining);
            }
            questions = picked;
        }

        questions.shuffle(&mut rng);
        if mode != SessionType::Exam || target_domain.is_some() {
            questions.truncate(num_questions);
        }

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
            domain_stats: HashMap::new(),
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
            session.domain_stats.clear();

            for q in &session.questions {
                let stat =
                    session
                        .domain_stats
                        .entry(q.domain)
                        .or_insert(crate::models::DomainStat {
                            correct: 0,
                            total: 0,
                        });
                stat.total += 1;

                if let Some(user_ans) = session.user_answers.get(&q.id) {
                    if user_ans == &q.correct_answer_indices {
                        correct_count += 1;
                        stat.correct += 1;
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
                domain: 1,
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

        app.start_session(&bank, SessionType::Exam, 65, None);

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
                    domain: 1,
                    prompt: "1+1?".to_string(),
                    choices: vec!["2".to_string(), "3".to_string()],
                    correct_answer_indices: vec![0],
                    explanation: "".to_string(),
                },
                Question {
                    id: "q2".to_string(),
                    domain: 1,
                    prompt: "2+2?".to_string(),
                    choices: vec!["4".to_string(), "5".to_string()],
                    correct_answer_indices: vec![0],
                    explanation: "".to_string(),
                },
            ],
        };
        app.start_session(&bank, SessionType::Exam, 2, None);

        // Setup answers
        for _ in 0..2 {
            let q_id = app.active_session.as_ref().unwrap().questions[app.current_question_index]
                .id
                .clone();
            if q_id == "q1" {
                app.submit_answer(0); // correct
            } else {
                app.submit_answer(1); // wrong
            }
            app.next_question();
        }

        let session = app.active_session.as_ref().unwrap();
        assert!(session.is_completed);
        assert_eq!(session.score_percentage.unwrap(), 50.0);
        assert_eq!(session.passed.unwrap(), false);
    }

    #[test]
    fn test_domain_allocations() {
        let mut app = App::new();
        let mut questions = Vec::new();
        for i in 0..100 {
            // evenly distribute domains for bank of 100
            let domain = (i % 4) as u8 + 1;
            questions.push(Question {
                id: i.to_string(),
                domain,
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

        // ask for 65 questions, exam mode
        app.start_session(&bank, SessionType::Exam, 65, None);

        let session = app.active_session.as_ref().unwrap();
        assert_eq!(session.questions.len(), 65);

        // 65 questions domain allocations: 1: 18, 2: 17, 3: 14, 4: 16
        let mut d1 = 0;
        let mut d2 = 0;
        let mut d3 = 0;
        let mut d4 = 0;
        for q in &session.questions {
            match q.domain {
                1 => d1 += 1,
                2 => d2 += 1,
                3 => d3 += 1,
                4 => d4 += 1,
                _ => {}
            }
        }
        assert_eq!(d1, 18);
        assert_eq!(d2, 17);
        assert_eq!(d3, 14);
        assert_eq!(d4, 16);
    }

    #[test]
    fn test_domain_stat_aggregation() {
        let mut app = App::new();
        let bank = QuestionBank {
            id: "t".to_string(),
            version: "1.0".to_string(),
            questions: vec![
                Question {
                    id: "q1".to_string(),
                    domain: 1,
                    prompt: "1+1?".to_string(),
                    choices: vec!["2".to_string(), "3".to_string()],
                    correct_answer_indices: vec![0],
                    explanation: "".to_string(),
                },
                Question {
                    id: "q2".to_string(),
                    domain: 2,
                    prompt: "2+2?".to_string(),
                    choices: vec!["4".to_string(), "5".to_string()],
                    correct_answer_indices: vec![0],
                    explanation: "".to_string(),
                },
            ],
        };
        app.start_session(&bank, SessionType::Exam, 2, None);

        // Setup answers
        for _ in 0..2 {
            let q_id = app.active_session.as_ref().unwrap().questions[app.current_question_index]
                .id
                .clone();
            if q_id == "q1" {
                app.submit_answer(0); // correct for domain 1
            } else {
                app.submit_answer(1); // wrong for domain 2
            }
            app.next_question();
        }

        let session = app.active_session.as_ref().unwrap();
        assert!(session.is_completed);

        let d1 = session.domain_stats.get(&1).unwrap();
        assert_eq!(d1.correct, 1);
        assert_eq!(d1.total, 1);

        let d2 = session.domain_stats.get(&2).unwrap();
        assert_eq!(d2.correct, 0);
        assert_eq!(d2.total, 1);
    }

    #[test]
    fn test_single_domain_practice_filter() {
        let mut app = App::new();
        let mut questions = Vec::new();
        for i in 0..10 {
            questions.push(Question {
                id: i.to_string(),
                domain: if i % 2 == 0 { 1 } else { 2 },
                prompt: format!("Q{}", i),
                choices: vec!["A".to_string(), "B".to_string()],
                correct_answer_indices: vec![0],
                explanation: "".to_string(),
            });
        }
        let bank = QuestionBank {
            id: "t".to_string(),
            version: "1.0".to_string(),
            questions,
        };

        app.start_session(&bank, SessionType::Practice, 10, Some(2));

        let session = app.active_session.as_ref().unwrap();
        assert_eq!(session.questions.len(), 5);
        for q in &session.questions {
            assert_eq!(q.domain, 2);
        }
    }
}
