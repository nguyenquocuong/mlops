# Implementation Plan: MLA C01 Practice Tool

**Branch**: `001-mla-c01-practice-tool` | **Date**: 2026-03-15 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-mla-c01-practice-tool/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

A single-binary terminal user interface (TUI) practice testing tool for the AWS MLA-C01 certification. Built with Rust and ratatui, featuring an embedded question bank with support for external JSON question banks. It utilizes local SQLite to persist completed session histories and save in-progress active sessions for resume functionality.

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: Rust 1.76+
**Primary Dependencies**: ratatui, crossterm, serde, serde_json, rusqlite, include_dir
**Storage**: SQLite (Local database file `~/.config/mla-c01-practice/history.db`)
**Testing**: cargo test
**Target Platform**: Linux, macOS, Windows (Terminal)
**Project Type**: CLI / TUI Application
**Performance Goals**: Launch < 500ms, Instant navigation between questions
**Constraints**: Single binary distribution, zero external system dependencies (except SQLite dynamic/static lib handled via rust-rusqlite)
**Scale/Scope**: Handles question banks up to ~1000 questions gracefully. SQLite stores lightweight session histories.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Code Quality**: Architecture promotes modularity and maintainability.
- [x] **Testing Standards**: Testing strategy is defined and adequate.
- [x] **UX Consistency**: Design aligns with the established design system.
- [x] **Performance**: Performance bottlenecks are anticipated and mitigated.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   └── cli.md           # CLI interface and configuration contracts
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
```text
src/
├── app.rs            # Application state
├── db.rs             # SQLite interactions (rusqlite)
├── main.rs           # Entry point
├── models.rs         # Data structures (Questions, History)
└── ui/
    ├── exam.rs       # Exam mode view
    ├── history.rs    # History view
    ├── menu.rs       # Main menu view
    └── practice.rs   # Practice mode view

tests/
├── contract.rs
└── integration.rs
```

**Structure Decision**: Single Rust workspace using standard binary crate template. UI rendering is separated from underlying state management (models/app) and persistence (db).

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
