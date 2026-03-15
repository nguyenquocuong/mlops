# Implementation Plan: Exam Domains and Weightings

**Branch**: `002-exam-domains` | **Date**: 2026-03-15 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-exam-domains/spec.md`

## Summary

This feature extends the MLA-C01 Practice Tool to categorize questions into the four official AWS MLOps exam domains, implement exam generation with precise domain weightings (28%, 26%, 22%, 24%), provide domain performance breakdowns in exam results/history, and allow domain-targeted practice mode filters.

## Technical Context

**Language/Version**: Rust 1.76+
**Primary Dependencies**: ratatui, crossterm, serde, serde_json, rusqlite, rand
**Storage**: SQLite (Local database file `~/.config/mla-c01-practice/history.db`)
**Testing**: cargo test
**Target Platform**: Linux, macOS, Windows (Terminal)
**Project Type**: CLI / TUI Application
**Performance Goals**: Launch < 500ms, Instant navigation between questions
**Constraints**: Single binary distribution, zero external system dependencies
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
specs/002-exam-domains/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── tasks.md
```

### Source Code (repository root)

```text
src/
├── app.rs            # Application state (domain calculation logic)
├── db.rs             # SQLite interactions (rusqlite - domain stats storage updates)
├── main.rs           # Entry point
├── models.rs         # Data structures (Questions with `domain` integer, domain stats tracking)
└── ui/
    ├── exam.rs       # Exam mode view (results screen showing domain breakdown)
    ├── history.rs    # History view (details showing domain stats)
    ├── menu.rs       # Main menu view (updated for domain prompt in Practice mode)
    └── practice.rs   # Practice mode view

tests/
├── integration.rs
```

**Structure Decision**: Extending the existing Single Rust workspace using standard binary crate template. Add domain calculation subroutines in `app.rs` and augment `models.rs` and `db.rs` schema accordingly.
