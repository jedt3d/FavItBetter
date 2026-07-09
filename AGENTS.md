# AGENTS.md

Last updated: 2026-07-09

## ProDOS Operating Contract

This repository follows ProDOS: Project Development Operating System.

Tagline: Think with AI. Build with Knowledge.

Core principle: Code is a temporary implementation. Knowledge is the permanent asset.

## Current Feature Cycle Header

Every agent response while operating under ProDOS must begin with:

```text
Current Feature Cycle
Feature: <feature name>
Current Phase: <phase>
Status: <short status>
Objective: <current objective>
```

Use an Active Feature Board when several features are in motion.

## Feature Implementation Cycle

0. Narrative Brainstorm Intake
1. Knowledge Capture
2. Product Planning
3. Design Review
4. Development
5. User Validation
6. Merge and Release
7. Documentation Synchronization
8. Continuous Improvement

## Rules

1. Preserve knowledge before changing code.
2. Classify important statements as Fact, Decision, Recommendation, Assumption, or Open Question.
3. Ask when business rules are unclear.
4. Never silently overwrite important knowledge.
5. Record important decisions in `docs/00-foundation/decision-log.md`.
6. Keep one canonical source for each concept.
7. Prefer configuration over hardcoding where rules may vary.
8. Link features to issue, branch, PR, code, tests, docs, and release notes.
9. Detect and resolve documentation drift before completion.
10. Human decisions remain human-owned unless explicitly delegated.

## Project-Specific Direction

FavItBetter is being shaped as a local-first bookmark management application for desktop and mobile. The product should help the user collect bookmarks from multiple browsers, clean them down to the useful set, and synchronize the project-owned bookmark library through a personal Shared Drive folder.

The first MVP is intentionally narrower: macOS and Windows only, Tauri 2.0 with SvelteKit 5, Google Chrome and Microsoft Edge local bookmark files only, local SQLite project storage, tracking-parameter-only URL cleanup, `HEAD`-first link checking with `GET` fallback for inconclusive results, no mobile, no Shared Drive sync, no browser writeback, and no AI cleanup.

Current source of truth:

- Project brief: `docs/00-foundation/project-brief.md`
- Initial feature spec: `docs/01-lifecycle/features/feature.bookmark-core.md`
- MVP feature spec: `docs/01-lifecycle/features/feature.mvp-desktop-chromium-cleaner.md`
- Decisions: `docs/00-foundation/decision-log.md`

## Documentation Publication Rule

FavItBetter publishes ProDOS documentation through GitHub Pages using Jekyll.

Rules:

- Keep `docs/` as the canonical repository source for review, audit, drift detection, and site publishing.
- Use GitHub Actions to build the Jekyll site and deploy it to GitHub Pages.
- Use CloudCannon's `base-jekyll-template` as the preferred knowledge-base design reference, adapted locally rather than copied blindly.
- Do not use GitHub Wiki as the default publication surface unless this decision is superseded.
