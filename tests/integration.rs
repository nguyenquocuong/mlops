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
                "prompt": "Test?",
                "choices": ["A", "B"],
                "correct_answer_indices": [0],
                "explanation": "Because"
            }
        ]
    }"#;

    let bank = QuestionBank::from_json(json).expect("valid json expected");

    let mut app = App::new();
    app.start_session(&bank, SessionType::Practice, 1);

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
                "prompt": "Test?",
                "choices": ["A", "B"],
                "correct_answer_indices": [0],
                "explanation": "Because"
            }
        ]
    }"#;

    let bank = QuestionBank::from_json(json).expect("valid json expected");

    let mut app = App::new();
    app.start_session(&bank, SessionType::Exam, 1);

    assert_eq!(app.mode, AppMode::Exam);
    let session = app.active_session.as_mut().unwrap();

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
