---
id: governance.documentation_drift
title: Documentation Drift
status: draft
owner: product owner
ai_priority: high
last_updated: 2026-07-09
---

# Documentation Drift

Documentation drift means two project artifacts disagree about product intent, user-visible behavior, architecture, tests, release notes, or governance.

## Drift Checks For FavItBetter

Before completing a feature, check that:

- `docs/00-foundation/project-brief.md` and the active feature spec describe the same product intent.
- `docs/00-foundation/glossary.md` defines any new domain terms.
- Decision log entries exist for sync, writeback, AI, privacy, or architecture direction changes.
- UX architecture docs match the actual screens and workflows.
- Release notes mention user-visible behavior if a release exists.
- Tests validate the behavior described by the feature spec.
- GitHub Pages output matches the canonical repository `docs/` source when documentation publishing exists.

## Response To Drift

1. Identify conflicting artifacts.
2. Select or ask for the canonical source.
3. Update docs, code, tests, or release notes.
4. Record material decisions.
5. Leave open questions visible if unresolved.
