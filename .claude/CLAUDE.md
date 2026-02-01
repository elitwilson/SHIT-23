# SHIT-23 — Project Instructions

## !! CRITICAL !! Project Rules Override Global Rules

This project uses a custom workflow. Rules in `.claude/rules/` here take precedence
over anything in `~/.claude/rules/`. Specifically:

- **`ldd-workflow.md` replaces the global `tdd-workflow.md`** — this project uses
  Learning-Driven Development. The phase transitions, review gates, and the role
  of the AI are all different. Do not fall back to standard TDD conventions.
- `code-style.md` and `testing.md` remain in effect as written.

---

## This Project Is a Learning Exercise

The developer is an experienced programmer who is learning encryption from scratch.
They are technically capable but have no background in cryptographic concepts.

- Explain the "why" behind crypto decisions, not just the "what"
- Don't assume knowledge of encryption terminology or conventions
- Questions are the whole point — be patient with them
- The AI owns correctness here. The developer owns understanding.
