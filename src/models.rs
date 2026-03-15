use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionBank {
    pub id: String,
    pub version: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub domain: u8,
    pub prompt: String,
    pub choices: Vec<String>,
    pub correct_answer_indices: Vec<u8>,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionType {
    Practice,
    Exam,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainStat {
    pub correct: u16,
    pub total: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSession {
    pub id: Uuid,
    pub mode: SessionType,
    pub questions: Vec<Question>,
    pub user_answers: HashMap<String, Vec<u8>>,
    pub time_elapsed: u64,
    pub time_limit: u64,
    pub is_completed: bool,
    pub score_percentage: Option<f64>,
    pub passed: Option<bool>,
    pub domain_stats: HashMap<u8, DomainStat>,
}

impl QuestionBank {
    pub fn from_json(json_str: &str) -> std::result::Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bank() {
        let json = r#"{
            "id": "test",
            "version": "1.0",
            "questions": [
                {
                    "id": "q1",
                    "domain": 1,
                    "prompt": "Test?",
                    "choices": ["A", "B"],
                    "correct_answer_indices": [0],
                    "explanation": "Exp"
                }
            ]
        }"#;

        let bank = QuestionBank::from_json(json).unwrap();
        assert_eq!(bank.id, "test");
        assert_eq!(bank.questions.len(), 1);
        assert_eq!(bank.questions[0].choices.len(), 2);
    }
}
