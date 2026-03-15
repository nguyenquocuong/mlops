# Command Line Interface Contract

The MLA-C01 Practice Tool is primarily an interactive TUI, but exposes a small CLI contract for configuration and automation.

## Usage
```bash
mla-practice [OPTIONS]
```

## Options

| Option | Short | Type | Description |
|--------|-------|------|-------------|
| `--bank` | `-b` | Path | Overrides the embedded question bank with an external JSON file. |
| `--help` | `-h` | Flag | Prints help information and exits. |
| `--version` | `-V` | Flag | Prints version information and exits. |

## Exit Codes

*   `0`: Process completed successfully (normal exit from TUI).
*   `1`: General error (e.g., TUI drawing failed, unhandled panic).
*   `2`: File I/O error (e.g., provided `--bank` file path does not exist or cannot be read).
*   `3`: Parsing error (e.g., the provided `--bank` file is malformed JSON or lacks required fields).
*   `4`: Database error (e.g., unable to open or write to the SQLite history file).
