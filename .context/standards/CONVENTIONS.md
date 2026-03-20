# Coding Conventions — gabriel-osemberg

**Version**: 1.0

---

## Naming

| Element | Convention | Example |
|---------|-----------|---------|
| Functions / Methods | `snake_case` | `render_project_card()` |
| Variables | `snake_case` | `project_count` |
| Constants | `SCREAMING_SNAKE_CASE` | `MAX_PROJECTS_PER_PAGE` |
| Types / Structs | `PascalCase` | `ProjectCard` |
| Enums | `PascalCase` type, `PascalCase` variants | `ProjectStatus::Active` |
| Traits | `PascalCase` (adjective or -able) | `Renderable`, `Queryable` |
| Modules | `snake_case` | `project_showcase` |
| File names | `snake_case.rs` | `project_card.rs` |
| Leptos components | `PascalCase` function | `fn ProjectCard()` |
| CSS classes | Tailwind utility classes | `bg-surface-primary` |
| Database tables | `snake_case` (plural) | `projects` |
| Database columns | `snake_case` | `tech_stack` |

---

## File Organization

```
gabriel-osemberg/
  src/
    main.rs             Server entry point
    app.rs              Root Leptos app component + router
    components/
      layout/           Header, Footer, Navigation, ThemeToggle
      projects/         ProjectCard, ProjectDetail, MetricsChart
      cv/               Timeline, SkillsRadar, AboutSection
      animations/       ScrollReveal, PageTransition, HeroAnimation
      common/           Button, Badge, Card, LoadingState
    pages/
      home.rs           Landing/hero page
      projects.rs       Projects index
      project_detail.rs Individual project case study
      about.rs          About/CV page
      contact.rs        Contact page
    models/             Database models (SQLx structs)
    server/             Server functions (data access, GitHub API, PDF)
    pdf/                PDF generation logic
  tests/
    unit/               Unit tests (mirror src structure)
    integration/        Integration tests (DB, server functions)
  migrations/           SQLx migrations
  style/                Tailwind CSS source
  public/               Static assets (images, videos, fonts)
    assets/
      projects/         Per-project screenshots and media
  Cargo.toml
  Cargo.lock
  tailwind.config.js
```

### Module Boundaries

- One concept per file (a struct, a component, a model)
- Keep files under 400 lines; split if larger
- Group related files in directories by domain

---

## Import / Dependency Ordering

Imports ordered in groups, separated by a blank line:

1. **Standard library** (`std::`)
2. **Third-party crates** (`leptos`, `sqlx`, `serde`, etc.)
3. **Project / local** (`crate::`, `super::`)

Within each group, sort alphabetically.

```rust
use std::collections::HashMap;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::project::Project;
use crate::server::github::fetch_repo_stats;
```

---

## Commenting Policy

- Comment **WHY**, not **WHAT** (code should be self-documenting)
- Every file starts with a `//!` module doc comment explaining purpose
- No comment noise (do not comment obvious code)
- Use `// TODO(ROADMAP-P{N})` format for deferred work — never naked TODOs
- Inline comments for non-obvious Rust patterns, lifetime decisions, unsafe justifications

---

## Error Handling

### Pattern: Result Types (Rust)

- Return `Result<T, E>` — never panic in application code
- Use `?` operator to propagate errors cleanly
- Define domain-specific error enums with `thiserror`
- Leptos server function errors use `ServerFnError`
- Component errors use `ErrorBoundary` components

### Universal Rules

- **Fail fast**: Detect errors as early as possible
- **Fail loudly**: Errors should be visible, not hidden
- **Log context**: Include what was attempted, what failed, and why
- **Never ignore errors**: Every error path must be handled explicitly
- **No `.unwrap()`** in application code — always handle the error case

---

## Formatting & Style

| Tool | Config File |
|------|-------------|
| `rustfmt` | `rustfmt.toml` |
| `clippy` | Cargo.toml `[lints]` section |
| Tailwind CSS | `tailwind.config.js` |

**Rule**: Run `cargo fmt` and `cargo clippy` before every commit.

---

## Git Commit Format

```
type(scope): subject line (imperative, <72 chars)

- What changed and why
- What changed and why
```

**Types**: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`, `perf`, `style`
**Scopes**: `layout`, `projects`, `cv`, `data`, `pdf`, `server`, `animation`, `infra`, `design`

---

## Design System & Constants Policy

**Rule**: All visual constants (sizes, colors, timing, opacity, spacing) MUST use Tailwind design tokens. No magic numbers in component code.

**Exception — Domain Data**: Values that encode real-world data (project metrics, dates, counts) are data, not design choices.

**Real violations to fix**: Hardcoded `px` values, hex colors, opacity numbers, and timing durations that should use the Tailwind config token system.

---

## Leptos-Specific Conventions

### Component Structure

```rust
/// Brief description of what this component renders.
#[component]
pub fn ComponentName(
    /// Description of prop
    #[prop(into)] prop_name: String,
    /// Optional prop with default
    #[prop(optional)] optional_prop: bool,
) -> impl IntoView {
    // Signals and derived signals first
    let (signal, set_signal) = signal(initial_value);
    let derived = move || signal.get() * 2;

    // Effects (if any)
    // ...

    // View
    view! {
        <div class="component-wrapper">
            // ...
        </div>
    }
}
```

### Server Functions

```rust
#[server(GetProjects)]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    // Database access here
    // Always use SQLx compile-time checked queries
    Ok(projects)
}
```

### Reactivity Rules

- Signals are the source of truth — never bypass them
- Derived signals (`move || ...`) over `create_effect` when possible
- Resources for async data loading (`create_resource`)
- Don't fight the borrow checker in closures — restructure ownership
