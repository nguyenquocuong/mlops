use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use clap::Parser;
use mla_c01::app::{App, AppMode};
use mla_c01::models::QuestionBank;
use mla_c01::ui;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[arg(short, long, help = "Path to custom question bank JSON file")]
    pub bank: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_bank_arg() {
        let args = vec!["mla-c01", "--bank", "custom.json"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.bank, Some("custom.json".to_string()));

        let args2 = vec!["mla-c01"];
        let cli2 = Cli::try_parse_from(args2).unwrap();
        assert_eq!(cli2.bank, None);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let bank_json = if let Some(path) = &cli.bank {
        fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("Error: Cannot read custom bank file '{}'.", path);
            eprintln!("Reason: {}", e);
            std::process::exit(1);
        })
    } else {
        include_str!("data/default_bank.json").to_string()
    };

    let bank = match QuestionBank::from_json(&bank_json) {
        Ok(b) => b,
        Err(e) => {
            eprintln!(
                "Error: Failed to parse question bank JSON. It may be malformed or missing fields."
            );
            eprintln!("Reason: {}", e);
            std::process::exit(1);
        }
    };

    let mut db_path = home::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    db_path.push(".config/mla-c01-practice");
    std::fs::create_dir_all(&db_path)?;
    db_path.push("history.db");

    let db = mla_c01::db::Db::new(db_path)?;

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();

    if let Ok(Some(saved)) = db.load_active_session() {
        app.active_session = Some(saved);
        app.show_resume_prompt = true;
    }

    app.db = Some(db);

    let res = run_app(&mut terminal, &mut app, &bank);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    bank: &QuestionBank,
) -> io::Result<()>
where
    io::Error: From<<B as Backend>::Error>,
{
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            ui::draw(f, app);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match app.mode {
                    AppMode::Menu => {
                        if app.show_resume_prompt {
                            if key.code == KeyCode::Char('r') {
                                app.show_resume_prompt = false;
                                if let Some(session) = &app.active_session {
                                    app.mode = match session.mode {
                                        mla_c01::models::SessionType::Practice => AppMode::Practice,
                                        mla_c01::models::SessionType::Exam => AppMode::Exam,
                                    };
                                }
                            } else if key.code == KeyCode::Char('c') {
                                app.show_resume_prompt = false;
                                app.active_session = None;
                                if let Some(ref db) = app.db {
                                    let _ = db.clear_active_session();
                                }
                            } else if key.code == KeyCode::Char('q') {
                                app.quit();
                            }
                        } else if app.show_domain_prompt {
                            if key.code == KeyCode::Char('0') {
                                app.start_session(
                                    bank,
                                    mla_c01::models::SessionType::Practice,
                                    65,
                                    None,
                                );
                                app.show_domain_prompt = false;
                            } else if key.code == KeyCode::Char('1') {
                                app.start_session(
                                    bank,
                                    mla_c01::models::SessionType::Practice,
                                    65,
                                    Some(1),
                                );
                                app.show_domain_prompt = false;
                            } else if key.code == KeyCode::Char('2') {
                                app.start_session(
                                    bank,
                                    mla_c01::models::SessionType::Practice,
                                    65,
                                    Some(2),
                                );
                                app.show_domain_prompt = false;
                            } else if key.code == KeyCode::Char('3') {
                                app.start_session(
                                    bank,
                                    mla_c01::models::SessionType::Practice,
                                    65,
                                    Some(3),
                                );
                                app.show_domain_prompt = false;
                            } else if key.code == KeyCode::Char('4') {
                                app.start_session(
                                    bank,
                                    mla_c01::models::SessionType::Practice,
                                    65,
                                    Some(4),
                                );
                                app.show_domain_prompt = false;
                            } else if key.code == KeyCode::Char('b')
                                || key.code == KeyCode::Char('q')
                            {
                                app.show_domain_prompt = false;
                            }
                        } else {
                            if key.code == KeyCode::Char('q') {
                                app.quit();
                            } else if key.code == KeyCode::Char('1') {
                                app.show_domain_prompt = true;
                            } else if key.code == KeyCode::Char('2') {
                                app.start_session(
                                    bank,
                                    mla_c01::models::SessionType::Exam,
                                    65,
                                    None,
                                );
                            } else if key.code == KeyCode::Char('3') {
                                app.mode = AppMode::History;
                            }
                        }
                    }
                    AppMode::Practice => {
                        if key.code == KeyCode::Char('q') {
                            app.mode = AppMode::Menu;
                        } else if app.showing_explanation {
                            if key.code == KeyCode::Char('n') {
                                app.next_question();
                            }
                        } else {
                            if key.code == KeyCode::Char('1') {
                                app.submit_answer(0);
                            } else if key.code == KeyCode::Char('2') {
                                app.submit_answer(1);
                            } else if key.code == KeyCode::Char('3') {
                                app.submit_answer(2);
                            } else if key.code == KeyCode::Char('4') {
                                app.submit_answer(3);
                            }
                        }
                    }
                    AppMode::Exam => {
                        if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('m') {
                            app.mode = AppMode::Menu;
                        } else {
                            if let Some(session) = &app.active_session {
                                if !session.is_completed {
                                    if key.code == KeyCode::Char('n') {
                                        app.next_question();
                                    } else if key.code == KeyCode::Char('1') {
                                        app.submit_answer(0);
                                    } else if key.code == KeyCode::Char('2') {
                                        app.submit_answer(1);
                                    } else if key.code == KeyCode::Char('3') {
                                        app.submit_answer(2);
                                    } else if key.code == KeyCode::Char('4') {
                                        app.submit_answer(3);
                                    }
                                }
                            }
                        }
                    }
                    AppMode::History => {
                        if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('m') {
                            app.mode = AppMode::Menu;
                        } else if key.code == KeyCode::Char('c') {
                            if let Some(ref db) = app.db {
                                let _ = db.clear_history();
                            }
                        }
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
