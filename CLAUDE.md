# LinkShort — Backend Rust Learning Project

## Context

This is a **learning project** for a developer preparing for a Rust backend interview in 2 weeks. The learner has:
- Basic Rust knowledge (built a CHIP8 emulator)
- Strong backend experience (Go, Python, distributed systems)
- 4 hours/day to dedicate

**Goal:** Build a production-quality URL shortener while learning backend Rust patterns.

---

## Teaching Approach

When helping with this project:

1. **Ask before showing.** When the learner encounters an error or asks "how do I do X", first ask what they think might be happening or how they'd approach it. Guide them to the answer.

2. **Connect to prior knowledge.** Reference Go/Python equivalents when explaining Rust concepts (e.g., "This is like Go's `sync.Mutex`, but...")

3. **Explain the why.** Don't just fix code — explain why Rust requires things a certain way.

4. **Challenge with follow-ups.** After solving something, ask: "What would happen if...?" or "Can you explain why this works?"

5. **Flag interview-relevant concepts.** When covering something commonly asked in interviews, note it explicitly.

---

## Project Structure

```
linkshort/
├── Cargo.toml
├── src/
│   ├── main.rs           # Entry point, server setup
│   ├── lib.rs            # Re-exports for testing
│   ├── config.rs         # Configuration loading
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── short_code.rs # ShortCode value object
│   │   ├── url.rs        # URL validation
│   │   └── error.rs      # Domain errors
│   ├── storage/
│   │   ├── mod.rs
│   │   ├── traits.rs     # UrlStore trait
│   │   ├── memory.rs     # In-memory implementation
│   │   └── postgres.rs   # SQLx implementation
│   ├── api/
│   │   ├── mod.rs
│   │   ├── routes.rs     # Axum routes
│   │   ├── handlers.rs   # Request handlers
│   │   ├── extractors.rs # Custom extractors
│   │   └── error.rs      # API error responses
│   └── observability/
│       ├── mod.rs
│       └── tracing.rs    # Logging setup
├── migrations/           # SQLx migrations
└── tests/
    └── integration/      # API integration tests
```

---

## Learning Phases

### Phase 1: Rust Core (Days 1-2)
**Focus:** Ownership edge cases, error handling, trait design

Build:
- [ ] `ShortCode` type with validation (no allocation on invalid input)
- [ ] Custom error enum using `thiserror`
- [ ] `UrlStore` trait with in-memory implementation

Key concepts to cover:
- When to use `&str` vs `String` vs `Cow<str>`
- `From`/`Into` trait implementations
- Making errors informative and composable

**Checkpoint questions:**
- Why does `&String` auto-deref to `&str`?
- When would you use `thiserror` vs `anyhow`?
- What makes a good domain error type?

---

### Phase 2: Async & Tokio (Days 3-4)
**Focus:** Async mental model, shared state, cancellation

Build:
- [ ] Async version of `UrlStore` trait
- [ ] Shared state with `Arc<Mutex<_>>` or `Arc<RwLock<_>>`
- [ ] Experiment with holding locks across `.await`

Key concepts to cover:
- Futures as state machines (poll-based, not callback-based)
- `Send`/`Sync` bounds in async — why they matter
- `tokio::spawn` vs `spawn_blocking`
- Cancellation safety

**Checkpoint questions:**
- What happens if a task is dropped mid-execution?
- Why can't you hold a `MutexGuard` across an `.await` with std::sync::Mutex?
- When would you use `tokio::sync::Mutex` vs `std::sync::Mutex`?

---

### Phase 3: HTTP with Axum (Days 5-7)
**Focus:** Web framework patterns, extractors, middleware

Build:
- [ ] `POST /shorten` — create short URL
- [ ] `GET /:code` — redirect to original
- [ ] `GET /stats/:code` — click statistics
- [ ] Request logging middleware
- [ ] Proper error → HTTP status mapping

Key concepts to cover:
- How extractors work (the `FromRequest` trait)
- State management with `State<T>`
- Tower middleware and layers
- Graceful error handling patterns

**Checkpoint questions:**
- Why does `State<T>` require `T: Clone`?
- How would you implement a custom extractor?
- What's the difference between `Extension` and `State`?

---

### Phase 4: Database with SQLx (Days 8-9)
**Focus:** Async database access, transactions, migrations

Build:
- [ ] Postgres schema and migrations
- [ ] `UrlStore` implementation with SQLx
- [ ] Connection pooling
- [ ] Handle race conditions (concurrent creates)

Key concepts to cover:
- Compile-time vs runtime query checking
- `query!` vs `query_as!` vs `QueryBuilder`
- Transaction handling patterns
- Connection pool sizing

**Checkpoint questions:**
- How does SQLx verify queries at compile time?
- What's the trade-off of `query!` requiring a live database?
- How would you handle "insert if not exists" atomically?

---

### Phase 5: Reliability & Observability (Days 10-11)
**Focus:** Production readiness

Build:
- [ ] Structured logging with `tracing`
- [ ] Request tracing with span context
- [ ] Graceful shutdown (SIGTERM handling)
- [ ] Health check endpoint
- [ ] Request timeouts

Key concepts to cover:
- `tracing` spans and structured fields
- Graceful shutdown patterns in Tokio
- Idempotency and retry safety

**Checkpoint questions:**
- How do you propagate trace context across async tasks?
- What's the difference between `tracing::info!` and `log::info!`?
- How would you make URL creation idempotent?

---

### Phase 6: Testing (Days 12-13)
**Focus:** Testing strategies for async Rust

Build:
- [ ] Unit tests for domain logic
- [ ] Integration tests with test database
- [ ] Property-based test with `proptest`

Key concepts to cover:
- Testing async code
- `axum::test` helpers
- Test database strategies (transactions, migrations)
- Property-based testing basics

---

## Common Commands

```bash
# Run the server
cargo run

# Run tests
cargo test

# Run with logging
RUST_LOG=linkshort=debug cargo run

# Database migrations
sqlx migrate run

# Check without running
cargo check

# Clippy lints
cargo clippy -- -W clippy::all

# Format
cargo fmt
```

---

## Key Crates

| Crate | Purpose | Docs |
|-------|---------|------|
| `tokio` | Async runtime | https://docs.rs/tokio |
| `axum` | HTTP framework | https://docs.rs/axum |
| `sqlx` | Async database | https://docs.rs/sqlx |
| `thiserror` | Error derive macros | https://docs.rs/thiserror |
| `tracing` | Structured logging | https://docs.rs/tracing |
| `serde` | Serialization | https://docs.rs/serde |
| `proptest` | Property testing | https://docs.rs/proptest |

---

## Interview Prep Notes

As concepts come up, note them here for review:

### Ownership & Borrowing
- (Add notes as you learn)

### Async/Await
- (Add notes as you learn)

### Error Handling
- (Add notes as you learn)

### Concurrency
- (Add notes as you learn)

### Testing
- (Add notes as you learn)

---

## Current Progress

- [ ] Phase 1: Rust Core
- [ ] Phase 2: Async & Tokio
- [ ] Phase 3: HTTP with Axum
- [ ] Phase 4: Database with SQLx
- [ ] Phase 5: Reliability & Observability
- [ ] Phase 6: Testing
