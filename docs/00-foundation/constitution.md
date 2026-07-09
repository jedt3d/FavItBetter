---
id: foundation.constitution
title: FavItBetter Constitution
status: draft
owner: product owner
ai_priority: medium
last_updated: 2026-07-09
---

# FavItBetter Constitution

## Purpose

FavItBetter should help a user turn scattered browser bookmarks into a compact, clean, useful personal library.

The product should be useful even without a hosted backend. The user's data should remain under user control, with synchronization based on files stored in a personal Shared Drive folder.

## Product Principles

1. Local-first by default.
2. User approval before destructive cleanup or browser writeback.
3. Compact over exhaustive: surface the necessary bookmarks, not every historical artifact.
4. Extensible where sources, rules, exports, metadata, and sync providers may vary.
5. Cross-device behavior must be understandable and recoverable.
6. Privacy-sensitive operations should be explicit, especially network link checks and AI metadata enrichment.

## Human-Owned Decisions

- Which browsers are officially supported first.
- Whether browser bookmarks can be modified automatically.
- Which Shared Drive provider is the first-class sync target.
- Whether AI metadata enrichment is in scope.
- Release approval and production risk acceptance.

## Agent-Owned Work

- Maintain project knowledge before code changes.
- Draft feature specs, workflows, acceptance criteria, and validation plans.
- Flag drift between docs, code, tests, and release notes.
- Recommend architecture and implementation paths.
