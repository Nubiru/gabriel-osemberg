# IDENTITY — Active Task

**Task**: L2-01 — PDF CV Generation (ATS-friendly)
**Status**: IN PROGRESS
**Started**: 2026-03-20

## Approach

Server-side PDF generation using `typst-as-lib` (Rust-native Typst compiler).
ATS-friendly version first: single-column, standard fonts, no graphics.

## Subtasks

- [ ] Add typst-as-lib to Cargo.toml (SSR-only)
- [ ] Create ATS Typst template
- [ ] Create server function: generate_cv_pdf
- [ ] Create Axum route for PDF download
- [ ] Create download button component on about page
- [ ] cargo check --features ssr
- [ ] Commit + push
