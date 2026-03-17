# HN-TUI

A terminal-based [Hacker News](https://news.ycombinator.com/) reader built with Rust.


## Features

- Browse **Top 30** Hacker News stories
- Read **top comments** on any story
- Open story links directly in your **default browser**
- **Concurrent** story fetching for fast load times
- Clean, scrollable TUI powered by [Ratatui](https://github.com/ratatui/ratatui)

## Keybindings

| Key       | Action                        |
|-----------|-------------------------------|
| `↑` / `↓` | Navigate stories              |
| `Enter`   | View comments for a story     |
| `o`       | Open story URL in browser     |
| `Esc`     | Back to story list            |
| `q`       | Quit                          |

## Tech Stack

- **[Ratatui](https://crates.io/crates/ratatui)** + **[Crossterm](https://crates.io/crates/crossterm)** — TUI rendering & terminal control
- **[Tokio](https://crates.io/crates/tokio)** — Async runtime
- **[Reqwest](https://crates.io/crates/reqwest)** — HTTP client for the [HN API](https://github.com/HackerNews/API)
- **[Serde](https://crates.io/crates/serde)** — JSON deserialization

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)

### Run

```bash
git clone https://github.com/Skip06/HN-TUI.git
cd HN-TUI
cargo run
```

## License

MIT
