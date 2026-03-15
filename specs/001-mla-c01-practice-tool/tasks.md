---
description: "Task list for MLA C01 Practice Tool implementation"
---

# Tasks: MLA C01 Practice Tool

**Input**: Design documents from `/specs/001-mla-c01-practice-tool/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/cli.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Initialize Rust application project (`cargo init`)
- [x] T002 [P] Configure dependencies (`ratatui`, `crossterm`, `serde`, `serde_json`, `rusqlite`, `include_dir`, `clap`) in `Cargo.toml`
- [x] T003 [P] Setup project directory structure (`src/models.rs`, `src/db.rs`, `src/app.rs`, `src/ui/mod.rs`)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 Create foundational entity structures (`QuestionBank`, `Question`, `SessionType`, `TestSession`) in `src/models.rs`
- [x] T005 [P] Setup base application state management loop (`App`) in `src/app.rs`
- [x] T006 Implement base terminal event handling and drawing loop in `src/main.rs`
- [x] T007 Initialize SQLite database and connection pool, creating schema for `active_sessions` and `session_history` in `src/db.rs`
- [x] T007a Implement Main Menu layout in `src/ui/menu.rs` and mode switching transition in `src/app.rs`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Practice Mode Experience (Priority: P1) 🎯 MVP

**Goal**: Present questions in a learning-focused environment, showing immediate feedback and explanations.

**Independent Test**: Can be tested by selecting "Practice Mode", answering a question, and confirming that the correct answer and a detailed explanation are immediately displayed before the next question is shown.

### Tests for User Story 1 (Constitution Mandated) ⚠️

- [x] T008 [P] [US1] Unit tests for JSON parsing and question shuffling logic (`src/models.rs`, `src/app.rs`)
- [x] T008a [P] [US1] Integration test for parsing questions and updating practice state in `tests/integration.rs`

### Implementation for User Story 1

- [x] T009 [P] [US1] Create a sample `default_bank.json` placeholder in a new `src/data/` directory
- [x] T010 [US1] Implement JSON parsing logic to load `QuestionBank` in `src/models.rs`
- [x] T011 [US1] Implement question bank shuffling logic (subset of 65) for session initialization in `src/app.rs`
- [x] T012 [P] [US1] Scaffold Practice mode UI layout in `src/ui/practice.rs`
- [x] T013 [US1] Implement user navigation rendering and answer submission logic in `src/ui/practice.rs`
- [x] T014 [US1] Implement explanation rendering state when a user answers incorrectly or correctly in `src/ui/practice.rs`

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Exam Mode Simulation (Priority: P1)

**Goal**: Simulate a real exam with a timer and no immediate feedback.

**Independent Test**: Can be tested by selecting "Exam Mode", answering questions, and seeing no feedback until completion, where a final score is calculated.

### Tests for User Story 2 (Constitution Mandated) ⚠️

- [x] T014a [P] [US2] Unit tests for final test scoring logic and pass/fail thresholds in `src/app.rs`
- [x] T014b [P] [US2] Integration test for Exam mode completion flow in `tests/integration.rs`

### Implementation for User Story 2

- [x] T015 [P] [US2] Scaffold Exam mode UI layout in `src/ui/exam.rs`
- [x] T016 [US2] Implement timer countdown state management in `src/app.rs`
- [x] T017 [US2] Implement elapsed time rendering and suppression of immediate answer feedback in `src/ui/exam.rs`
- [x] T018 [US2] Implement final test scoring logic and pass/fail thresholds in `src/app.rs`
- [x] T019 [US2] Create summary end-screen rendering for Exam mode in `src/ui/exam.rs`

**Checkpoint**: Exam mode and passing calculations function independently.

---

## Phase 5: User Story 3 - Standalone Binary Distribution (Priority: P2)

**Goal**: Embed the default question bank into the executable and allow CLI overrides.

**Independent Test**: Run executable out-of-directory and verify embedded JSON loads; run `--bank` and verify custom loading.

### Tests for User Story 3 (Constitution Mandated) ⚠️

- [x] T019a [P] [US3] Unit tests for `clap` arg parsing and override conditional logic in `src/main.rs`
- [x] T019b [P] [US3] Integration test ensuring custom bank loading behavior in `tests/integration.rs`

### Implementation for User Story 3

- [x] T020 [P] [US3] Embed `src/data/default_bank.json` into binary via `include_str!` or `include_dir!` in `src/models.rs`
- [x] T021 [US3] Implement `clap` arguments parsing (`--bank`, `--help`, `--version`) in `src/main.rs`
- [x] T022 [US3] Override embedded bank loading logic conditionally if `--bank` is supplied in `src/main.rs`

---

## Phase 6: User Story 4 - Progress Tracking (Priority: P2)

**Goal**: Allow users to see their past session history and suspend/resume active tests.

**Independent Test**: Complete an exam, find it saved in the history menu. Kill the app mid-exam, restart it, and see a prompt to resume.

### Implementation for User Story 4

- [x] T023 [P] [US4] Implement SQLite inserts for session completion to `session_history` table in `src/db.rs`
- [x] T024 [P] [US4] Scaffold History menu view in `src/ui/history.rs`
- [x] T025 [US4] Implement history retrieval from SQLite and rendering inside `src/ui/history.rs` 
- [x] T026 [US4] Implement history reset/deletion operation in `src/db.rs` and bound to UI in `src/ui/history.rs`
- [x] T027 [US4] Implement state serialization (JSON) and save on exit flow to `active_sessions` in `src/db.rs` and `src/main.rs`
- [x] T028 [US4] Implement startup check for `active_sessions` and prompt the user to resume in `src/main.rs`, ensuring to catch JSON deserialization/tampering errors and gracefully discord invalid saves

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T029 [P] Refactor terminal styling (colors, borders) across all modules in `src/ui/` to ensure UX Consistency
- [x] T030 [P] Implement graceful text-wrapping for question prompts larger than the terminal window in `src/ui/` modules
- [x] T031 Provide descriptive error pages for malformed JSON loading
- [x] T032 Add README usage guidance

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Can start immediately
- **Foundational (Phase 2)**: Depends on Setup
- **User Stories (Phase 3-6)**: ALL depend on Phase 2. Can be run in parallel or sequentially.
- **Polish (Phase 7)**: Depends on completion of all required user stories

### Parallel Opportunities

- T002, T003 can be parallelized with initial repo config
- T005, T007 (Database, State, Base Models) can be written independently before wiring together in T006
- T008, T009, T012 can be written independently of T011 and T010
- US1 UI tasks can be built out while US4 DB persistence logic is being written
