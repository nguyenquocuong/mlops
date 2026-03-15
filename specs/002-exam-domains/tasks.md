---
description: "Task list for Exam Domains and Weightings implementation"
---

# Tasks: Exam Domains and Weightings

**Input**: Design documents from `/specs/002-exam-domains/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure. Since this is extending an existing feature, setup is minimal.

- [x] T001 Verify project compiles cleanly on new feature branch.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core model updates that MUST be complete before ANY user story can be implemented.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T002 Update `Question` struct in `src/models.rs` to include `domain: u8` field.
- [x] T003 Update JSON loading logic in `src/models.rs` to strictly require the `domain` element and throw an explicit error if missing.
- [x] T004 [P] Update `src/data/default_bank.json` embedding valid `domain` fields (1-4) in the placeholder questions.
- [x] T005 Update `TestSession` and add `DomainStat` tracking structs in `src/models.rs`.
- [x] T006 Update `init_schema` in `src/db.rs` to execute `ALTER TABLE session_history ADD COLUMN domain_stats TEXT DEFAULT '{}';` for existing databases (using SQLite PRAGMA checks or graceful IF NOT EXISTS logic).

**Checkpoint**: Foundation ready - user story implementation can now begin.

---

## Phase 3: User Story 1 - Officially Weighted Exam Generation (Priority: P1) 🎯 MVP

**Goal**: As a user taking a simulated exam, I want the generated exam to proportionally match the official AWS MLA-C01 domain weightings.

**Independent Test**: Start an exam mode test and internally assert the proportion of domain allocations.

### Tests for User Story 1

- [x] T007 [P] [US1] Create unit tests for Largest Remainder Method and fallback allocation in `src/app.rs`.
- [x] T008 [P] [US1] Add integration test for verifying exam generation domain allocations in `tests/integration.rs`.

### Implementation for User Story 1

- [x] T009 [US1] Implement Largest Remainder Method in `src/app.rs` inside `start_session` for precise domain allocation (Domain 1: 28%, 2: 26%, 3: 22%, 4: 24%).
- [x] T010 [US1] Implement fallback logic within `start_session` in `src/app.rs` to greedily select questions from alternative domains if a specific domain quota falls short.

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently.

---

## Phase 4: User Story 2 - Domain-Specific Performance Tracking (Priority: P2)

**Goal**: As a user reviewing my exam results, I want to see a breakdown of my score by exam domain.

**Independent Test**: Complete an exam and view the breakdown on the summary screen and history menu.

### Tests for User Story 2 (Constitution Mandated) ⚠️

- [x] T010a [P] [US2] Create unit tests for domain performance stat aggregation in `src/app.rs`.
- [x] T010b [P] [US2] Add integration test for end-to-end `domain_stats` tracking and SQLite persistence in `tests/integration.rs`.

### Implementation for User Story 2

- [x] T011 [US2] Implement domain statistics calculation (correct/total per domain) during `finish_session` in `src/app.rs`.
- [x] T012 [US2] Update `save_history` and `get_history` queries and deserialization in `src/db.rs` to write/read the `domain_stats` JSON data.
- [x] T013 [P] [US2] Update summary end-screen rendering in `src/ui/exam.rs` to iterate over and display the domain-specific correct percentages cleanly.
- [x] T014 [P] [US2] Update history detail rendering in `src/ui/history.rs` to deserialize and show past domain performances when browsing.

**Checkpoint**: Exam summaries and history screens accurately portray domain breakdowns.

---

## Phase 5: User Story 3 - Domain-Targeted Practice (Priority: P3)

**Goal**: As a user in practice mode, I want the ability to focus exclusively on a single chosen domain.

**Independent Test**: Select Practice mode, pick Domain 1, ensure all given questions are exclusively Domain 1.

### Tests for User Story 3 (Constitution Mandated) ⚠️

- [x] T014a [P] [US3] Create unit tests to verify single-domain practice filtering logic in `src/app.rs`.

### Implementation for User Story 3

- [x] T015 [US3] Update Menu UI in `src/ui/menu.rs` to prompt the user to choose 'All Domains' or a specific domain (1-4) upon pressing enter on Practice Mode.
- [x] T016 [US3] Modify the application state event propagation to pass the selected domain target through to `start_session` in `src/app.rs`.
- [x] T017 [US3] Update question subset shuffling logic in `start_session` in `src/app.rs` to pre-filter to only the requested domain if one was selected.

**Checkpoint**: Practice drilling targeting is complete.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T018 Refactor and confirm all integration tests and unit tests pass stably (`cargo test`).
- [x] T019 Test missing-domain validation error handling against a malformed JSON file via custom CLI `--bank` import to ensure it fails predictably and gracefully in `src/main.rs`.
- [x] T020 Run performance benchmarks spanning the domain shuffling algorithm to assert that allocations compute within the defined 500ms constraint.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Immediate
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - US1 directly provides the math foundations for US2's scoring
  - Thus execute sequentially: P1 -> P2 -> P3.

### Parallel Opportunities

- Tests within US1 can be scaffolded natively while the method stubs are written.
- UI modifications in US2 (T013, T014) can run parallel to SQLite storage wiring (T012).
