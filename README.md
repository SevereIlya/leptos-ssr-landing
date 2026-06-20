# SSR Landing Page

A production-ready, Server-Side Rendered (SSR) web application built entirely in Rust. This project serves as a commercial landing page and lead generation system, demonstrating advanced architectural patterns, strict type safety, and high performance.

## Architecture & Design Patterns

This project strictly adheres to **Clean Architecture** and **Domain-Driven Design (DDD)** principles, ensuring a decoupled, maintainable, and scalable codebase.

* **Layered Architecture:** Strict separation of concerns between Domain, Application (Use Cases), and Infrastructure/Adapters.
* **Domain-Driven Design:** Business logic is isolated in the Domain layer. Uses **Value Objects** with compile-time and runtime validation (powered by `nutype`, `phonenumber`, and `email_address`) to ensure invalid states are unrepresentable.
* **Event-Driven Integration:** Implements a Domain Event model. The `Lead` aggregate generates events that are dispatched via a **Composite Event Publisher**.
* **Fire-and-Forget Notifications:** External integrations (Telegram API, SMTP) are executed asynchronously in the background using `tokio::spawn`. This decouples the database transaction from network calls, reducing API latency to mere milliseconds.
* **Progressive Enhancement:** Built with Leptos `ActionForm` and Server Functions. The lead capture system functions perfectly using standard HTML POST requests even if JavaScript/WASM fails to load or hydrate on the client.

## Tech Stack

* **Frontend:** Leptos (v0.8) for reactive UI and SSR, Tailwind CSS (v4) for styling.
* **Backend:** Axum, Tokio.
* **Database:** PostgreSQL with SQLx.
* **Integrations:** `teloxide` (Telegram Bot API), `lettre` (SMTP with `rustls` and `ring` crypto providers).
* **WASM FFI:** `wasm-bindgen` for seamless integration with external JavaScript analytics tools.

## Key Features

* **Zero-Allocation State UI:** Optimized conditional rendering using CSS classes for loading states (`pending`) to prevent unnecessary DOM mutations and allocations.
* **Protection against Race Conditions:** UI-level locks and reactive signals prevent double-submissions during network latency.
* **Advanced Error Handling:** Custom Domain Errors mapped to user-friendly UI feedback without exposing internal infrastructure stack traces.
* **Cross-Compilation:** The codebase compiles to both a native server binary (Axum) and a WebAssembly module for browser hydration.

## Getting Started

### Prerequisites

* Rust toolchain (configured via `rust-toolchain.toml`)
* `cargo-leptos` CLI tool
* PostgreSQL database

### Configuration

1. Copy the example configuration file:
   ```bash
   cp config.example.toml config.toml
   cp .env.example .env
   ```
2. Update `config.toml` with your actual database credentials, Telegram bot token, and SMTP settings. (Ensure `config.toml` and `.env` remains in `.gitignore`).

### Database Setup

Install SQLx CLI and run migrations:
```bash
cargo install sqlx-cli
sqlx database create
sqlx migrate run
```

### Running the Project

To start the development server with hot-reloading (compiles both SSR and WASM targets):
```bash
cargo leptos watch
```

For a production build:
```bash
cargo leptos build --release
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

*Note: The MIT license applies only to the source code. The copy, branding, design system, and assets are proprietary and may not be reused without permission.*