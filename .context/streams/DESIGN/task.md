# DESIGN — Active Task

**Task**: L1.1 — Responsive Navigation (mobile hamburger menu + drawer)
**Started**: 2026-03-20
**Status**: IN PROGRESS

## Scope

1. Mobile hamburger button (visible below `md` breakpoint)
2. Slide-out drawer/overlay menu for mobile
3. Active route highlighting
4. Keyboard navigation (Tab, Enter, Escape to close)
5. Skip-to-content link (accessibility)
6. ARIA attributes on nav elements
7. Smooth open/close animation

## Files to Create/Modify

- CREATE: `src/components/mobile_menu.rs`
- MODIFY: `src/components/mod.rs` (add mobile_menu module)
- MODIFY: `src/components/layout.rs` (integrate mobile menu, add skip-to-content)

## Verification

- `cargo build` both targets
- `cargo clippy -- -D warnings` both targets
- `cargo fmt --check`
- Mobile menu opens/closes
- Keyboard accessible
