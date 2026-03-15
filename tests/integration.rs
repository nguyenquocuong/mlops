use mla_c01::app::{App, AppMode};
use mla_c01::models::{QuestionBank, SessionType};

#[test]
fn test_practice_mode_init_and_answer() {
    let json = r#"{
        "id": "builtin-c01",
        "version": "1.0",
        "questions": [
            {
                "id": "q1",
                "domain": 1,
                "prompt": "Test?",
                "choices": ["A", "B"],
                "correct_answer_indices": [0],
                "explanation": "Because"
            }
        ]
    }"#;

    let bank = QuestionBank::from_json(json).expect("valid json expected");

    let mut app = App::new();
    app.start_session(&bank, SessionType::Practice, 1, None);

    assert_eq!(app.mode, AppMode::Practice);
    let session = app.active_session.as_mut().unwrap();
    assert_eq!(session.questions.len(), 1);

    app.submit_answer(0);
    assert!(app.showing_explanation);
}

#[test]
fn test_exam_mode_completion_flow() {
    let json = r#"{
        "id": "builtin",
        "version": "1.0",
        "questions": [
            {
                "id": "q1",
                "domain": 1,
                "prompt": "Test?",
                "choices": ["A", "B"],
                "correct_answer_indices": [0],
                "explanation": "Because"
            }
        ]
    }"#;

    let bank = QuestionBank::from_json(json).expect("valid json expected");

    let mut app = App::new();
    app.start_session(&bank, SessionType::Exam, 1, None);

    assert_eq!(app.mode, AppMode::Exam);
    let _session = app.active_session.as_mut().unwrap();

    app.submit_answer(0);
    app.next_question();

    let session = app.active_session.as_ref().unwrap();
    assert!(session.is_completed);
    assert_eq!(session.score_percentage.unwrap(), 100.0);
    assert_eq!(session.passed.unwrap(), true);
}

#[test]
fn test_cli_custom_bank_error() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_mla-c01"))
        .arg("--bank")
        .arg("nonexistent_file.json")
        .output()
        .expect("Failed to execute cleanly");

    // The app should not succeed since file does not exist, so status should be non-zero
    assert!(!output.status.success());
}

#[test]
fn test_end_to_end_domain_stats_db() {
    let json = r#"{
        "id": "builtin",
        "version": "1.0",
        "questions": [
            {
                "id": "q1",
                "domain": 1,
                "prompt": "Test?",
                "choices": ["A", "B"],
                "correct_answer_indices": [0],
                "explanation": "Because"
            }
        ]
    }"#;
    let bank = QuestionBank::from_json(json).expect("valid json expected");

    // Temp db
    let db_path = std::path::PathBuf::from("test_db.sqlite3");
    let _ = std::fs::remove_file(&db_path);
    let db = mla_c01::db::Db::new(db_path.clone()).unwrap();

    let mut app = App::new();
    app.db = Some(db);
    app.start_session(&bank, SessionType::Exam, 1, None);

    app.submit_answer(0);
    app.next_question();

    let session = app.active_session.as_ref().unwrap();
    assert!(session.is_completed);

    let history = app.db.as_ref().unwrap().get_history().unwrap();
    assert_eq!(history.len(), 1);
    assert!(history[0].contains("D1: 100%"));

    let _ = std::fs::remove_file(&db_path);
}

#[test]
fn test_malformed_bank_missing_domain() {
    use std::io::Write;
    let json = r#"{
        "id": "builtin",
        "version": "1.0",
        "questions": [
            {
                "id": "q1",
                "prompt": "Test?",
                "choices": ["A", "B"],
                "correct_answer_indices": [0],
                "explanation": "Because"
            }
        ]
    }"#;

    let path = "missing_domain.json";
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_mla-c01"))
        .arg("--bank")
        .arg(path)
        .output()
        .expect("Failed to execute");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Failed to parse question bank JSON"));
    assert!(stderr.contains("missing field `domain`"));

    let _ = std::fs::remove_file(path);
}

#[test]
fn test_performance_domain_allocation() {
    let mut questions = Vec::new();
    for i in 0..1000 {
        let domain = (i % 4) as u8 + 1;
        questions.push(mla_c01::models::Question {
            id: i.to_string(),
            domain,
            prompt: format!("Q{}", i),
            choices: vec!["A".to_string(), "B".to_string()],
            correct_answer_indices: vec![0],
            explanation: "Exp".to_string(),
        });
    }
    let bank = QuestionBank {
        id: "perf".to_string(),
        version: "1.0".to_string(),
        questions,
    };

    let mut app = App::new();
    let start = std::time::Instant::now();
    app.start_session(&bank, SessionType::Exam, 65, None);
    let elapsed = start.elapsed();

    // Constraint is 500ms
    assert!(
        elapsed.as_millis() < 500,
        "Allocation took too long: {}ms",
        elapsed.as_millis()
    );
}
