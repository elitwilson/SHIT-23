# Learning-Driven Development (LDD) Workflow

## !! CRITICAL !! This Supersedes the Global TDD Workflow

This project uses LDD, not TDD. All phase transitions, review gates, and the AI's role
are different from the global `tdd-workflow.md`. Do not fall back to standard TDD.

---

## Core Principles

**AI owns correctness. Developer owns understanding.**

The AI is responsible for writing correct tests and implementation. The developer is
responsible for being able to follow and explain the code. Neither moves on until
both conditions are met.

**Explanations are built into every phase — they are not optional.**

Before each phase transition, the AI explains what was written and why. "Why" comes
before "what." The developer understands the problem before seeing the solution.

**Understanding gates replace correctness gates.**

The developer's approval at each phase isn't "does this look right?" — it's
"can I follow what's happening here?" Questions at any point. Always.

---

## The LDD Cycle

### Planning Phase

**Goal:** Create a concrete feature plan

**Process:**
- Collaborative discussion
- AI leads on domain decisions and explains the reasoning behind them
- Developer asks questions, decides scope
- Result: feature plan in `notes/`

**Gate:** Developer understands what we're building and why

---

### RED Phase: Write Failing Tests

**Goal:** Write failing tests that define the contract — with enough explanation to understand what we're testing and why

**Process:**
1. AI writes real failing tests directly (no scaffolding phase)
2. Each test includes a "Why" comment explaining:
   - What the test is checking
   - Why this behavior matters
   - The context or edge case being covered
3. Comments explain the logic behind assertions — not just what, but why
4. Developer reads through, asks questions freely
5. Tests run and fail for the right reasons

**Why document tests?** Tests define the system's behavior. The "why" helps future readers (including the developer) understand the contract, not just the mechanics.

**Gate:** Developer can follow each assertion and explain what it's verifying

**Git commit:** `test: add failing tests for [feature]`

---

### GREEN Phase: Implement + Walkthrough

**Goal:** Make tests pass — then make sure the developer understands how

**Process:**
1. AI implements the code to make tests pass
2. AI walks through the implementation, explaining the logic
3. Developer asks questions until they can follow the code
4. All tests pass

**Gate:** Developer can follow the implementation logic

**Git commit:** `feat: implement [feature]`

---

### REFACTOR Phase (Optional)

Only if there's obvious duplication or complexity to simplify. Same as standard TDD.

**Git commit:** `refactor: [description]`

---

## Phase Transitions

AI explains what was done and why. Developer confirms understanding. Then we move on.
Never skip phases. Never move on without understanding.

---

## Git Conventions

- `test: add failing tests for [feature]`
- `feat: implement [feature]`
- `refactor: [description]`
- `fix:` / `docs:` / `chore:` as needed

Commit at end of each phase. Never mid-phase.
