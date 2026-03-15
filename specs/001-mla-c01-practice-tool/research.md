# Phase 0: Outline & Research

## Decision: Technology Stack
*   **Decision**: Rust with `ratatui` (TUI), `crossterm` (backend), and `rusqlite` (Storage).
*   **Rationale**: Rust provides the required memory safety, high performance (sub-500ms launch overhead), and makes compiling into an isolated, standalone binary extremely straightforward. The `ratatui` UI library is the current standard for creating highly responsive and accessible terminal user interfaces in Rust, satisfying the UX consistency requirement. `rusqlite` satisfies the user's explicit request for SQLite storage and offers robust, single-file ACID persistence for tracking user history.
*   **Alternatives considered**: 
    *   Python (`curses` / `Textual`): Rejected because it requires Python environment setup, making standalone binary distribution complex (e.g., PyInstaller edge cases).
    *   Go (`charmbracelet/bubbletea`): Valid alternative for single-binary generation, but Rust was explicitly requested.

## Decision: Data Storage Strategy
*   **Decision**: Question Bank is embedded into the binary at compile time via `include_str!` or `include_dir!`, parsed from JSON. The local session history will be stored in an SQLite database file at an OS-specific config location (e.g., `~/.config/mla-c01-practice/history.db`).
*   **Rationale**: Distributing the default questions inside the binary provides the zero-setup experience requested by User Story 3, while SQLite covers User Story 4 (progress tracking) with cross-session reliability.
*   **Alternatives considered**: JSON file for user history. Rejected as SQLite scales better for progressive logging of stats and is explicitly required by the user context.
