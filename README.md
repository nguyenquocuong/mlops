# MLA-C01 Practice Tool

A Terminal User Interface (TUI) practice testing tool for the AWS MLA-C01 certification. Built with Rust.

## Features:

- **Practice Mode**: Get immediate feedback on every question with clear explanations.
- **Exam Mode**: Take a timed, simulated test to assess readiness.
- **Progress Tracking**: Your history and active uncompleted sessions are automatically tracked and saved.
- **Custom Question Banks**: Use `--bank path_to_file.json` to override the built-in bank.

## Quickstart

```sh
# Run the application
cargo run

# Create an optimized standalone binary
cargo build --release

# Run with a custom question bank
cargo run -- --bank ./my_questions.json
```
