use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn new(path: PathBuf) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS session_history (
                id TEXT PRIMARY KEY,
                session_date DATETIME DEFAULT CURRENT_TIMESTAMP,
                mode TEXT NOT NULL,
                score_percentage REAL,
                passed BOOLEAN,
                total_questions INTEGER,
                correct_answers INTEGER
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS active_sessions (
                id TEXT PRIMARY KEY,
                last_saved DATETIME DEFAULT CURRENT_TIMESTAMP,
                mode TEXT NOT NULL,
                session_data TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    pub fn save_history(&self, session: &crate::models::TestSession) -> Result<()> {
        let pct = session.score_percentage.unwrap_or(0.0);
        let passed = session.passed.unwrap_or(false);

        // Count correct
        let mut correct = 0;
        for q in &session.questions {
            if let Some(user_ans) = session.user_answers.get(&q.id) {
                if user_ans == &q.correct_answer_indices {
                    correct += 1;
                }
            }
        }

        let mode_str = match session.mode {
            crate::models::SessionType::Practice => "Practice",
            crate::models::SessionType::Exam => "Exam",
        };

        self.conn.execute(
            "INSERT INTO session_history (id, mode, score_percentage, passed, total_questions, correct_answers)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                session.id.to_string(),
                mode_str,
                pct,
                passed,
                session.questions.len() as i32,
                correct as i32
            ],
        )?;
        Ok(())
    }

    pub fn get_history(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT session_date, mode, score_percentage, passed, total_questions, correct_answers FROM session_history ORDER BY session_date DESC")?;

        let rows = stmt.query_map([], |row| {
            let date: String = row.get(0)?;
            let mode: String = row.get(1)?;
            let score: f64 = row.get(2)?;
            let passed: bool = row.get(3)?;
            let total: i32 = row.get(4)?;
            let correct: i32 = row.get(5)?;

            let passed_str = if passed { "PASS" } else { "FAIL" };
            Ok(format!(
                "{} - Mode: {} - Score: {:.1}% ({}/{}) - [{}]",
                date, mode, score, correct, total, passed_str
            ))
        })?;

        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }

        Ok(out)
    }

    pub fn clear_history(&self) -> Result<()> {
        self.conn.execute("DELETE FROM session_history", [])?;
        Ok(())
    }

    pub fn save_active_session(&self, session: &crate::models::TestSession) -> Result<()> {
        let mode_str = match session.mode {
            crate::models::SessionType::Practice => "Practice",
            crate::models::SessionType::Exam => "Exam",
        };

        let json = serde_json::to_string(session).unwrap();

        self.conn.execute(
            "REPLACE INTO active_sessions (id, mode, session_data) VALUES (?1, ?2, ?3)",
            rusqlite::params!["CURRENT", mode_str, json],
        )?;
        Ok(())
    }

    pub fn load_active_session(&self) -> Result<Option<crate::models::TestSession>> {
        let mut stmt = self
            .conn
            .prepare("SELECT session_data FROM active_sessions WHERE id = 'CURRENT'")?;
        let mut rows = stmt.query([])?;
        if let Some(row) = rows.next()? {
            let data: String = row.get(0)?;
            let parse_res: std::result::Result<crate::models::TestSession, _> =
                serde_json::from_str(&data);
            if let Ok(session) = parse_res {
                return Ok(Some(session));
            } else {
                // Return gracefully instead of crashing on tampering
                let _ = self.clear_active_session();
            }
        }
        Ok(None)
    }

    pub fn clear_active_session(&self) -> Result<()> {
        self.conn
            .execute("DELETE FROM active_sessions WHERE id = 'CURRENT'", [])?;
        Ok(())
    }
}
