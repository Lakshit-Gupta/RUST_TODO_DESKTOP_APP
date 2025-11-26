# Rust Desktop App

A Dioxus-based desktop application built with Rust.

## Project Structure

```
RUST_DESKTOP_APP/
├─ todo-app/
│  ├─ assets/          # Static assets (CSS, images, etc.)
│  │  └─ main.css
│  ├─ src/
│  │  └─ main.rs       # Application entry point and components
│  ├─ Cargo.toml       # Dependencies and feature flags
│  ├─ Dioxus.toml      # Dioxus configuration
│  └─ target/          # Build artifacts (gitignored)
├─ AGENTS.md           # Dioxus 0.7 documentation
├─ APP_INFO.md
└─ README.md
```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- Dioxus CLI

Install Dioxus CLI:
```bash
curl -sSL http://dioxus.dev/install.sh | sh
```

### Development

Navigate to the project directory and run:

```bash
cd todo-app
dx serve
```

For a specific platform:
```bash
dx serve --platform desktop
```

### Building

To build for production:
```bash
dx build --release
```


