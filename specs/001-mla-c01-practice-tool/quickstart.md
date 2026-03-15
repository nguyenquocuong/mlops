# Quickstart: MLA C01 Practice Tool

## Prerequisites
*   Rust toolchain (1.76 or newer)
*   SQLite development headers (`libsqlite3-dev` on Linux)

## Development Setup

1. **Build the Application**
   ```bash
   cargo build
   ```

2. **Run the Application**
   Launch with the embedded default question bank:
   ```bash
   cargo run
   ```

3. **Run with a Custom Question Bank**
   Pass the path to a custom JSON question list:
   ```bash
   cargo run -- --bank /path/to/custom_bank.json
   ```

## Testing

Run unit tests and UI rendering test stubs:
```bash
cargo test
```
