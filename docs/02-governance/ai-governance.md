---
id: governance.ai
title: AI Governance
status: draft
owner: product owner
ai_priority: high
last_updated: 2026-07-09
---

# AI Governance

AI may be useful for bookmark categorization, title cleanup, summaries, topic clustering, and keep/archive suggestions. It is not required for the first MVP.

## Principles

- AI suggestions must remain suggestions unless the user delegates automation.
- AI behavior must be documented as configuration.
- Bookmarks can reveal private interests, accounts, projects, and research. External AI processing requires explicit user approval.
- Costs, providers, prompts, and retention assumptions must be documented before implementation.

## Current Position

AI enrichment is optional and unaccepted. The core product should work without it.

## Open Questions

- Should FavItBetter support local-only AI models?
- Should AI run on all bookmarks or only selected review batches?
- Should AI be allowed to fetch page content, or only use URL/title/folder metadata?
