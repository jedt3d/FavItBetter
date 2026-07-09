---
id: release.unreleased
title: Unreleased
status: draft
owner: product owner
ai_priority: medium
last_updated: 2026-07-09
---

# Unreleased

## Status

No application release exists yet.

## Knowledge Changes

- Initialized ProDOS project knowledge for FavItBetter.
- Captured the initial bookmark manager product direction.
- Drafted the first Agent-First Specification for core bookmark curation and sync.
- Selected GitHub Pages with Jekyll as the documentation publishing path.
- Superseded the earlier GitHub Wiki publishing evaluation.
- Added a Jekyll Hello World page as the first publishing smoke test.
- Adapted the documentation site toward CloudCannon Base knowledge-base theme patterns.
- Captured the first MVP scope as a desktop-only Chromium bookmark cleaner.
- Confirmed the desktop MVP architecture: Tauri 2.0, SvelteKit running on Svelte 5, Rust backend link checking, and local SQLite project storage.
- Clarified MVP cleanup policy: remove tracking query parameters only, use 80 seconds per link-check attempt, and run `GET` fallback for inconclusive `HEAD` checks.
- Upgraded the GitHub Pages workflow actions to Node 24-compatible major versions.
- Added the first MVP app foundation: Tauri desktop shell, SvelteKit UI, SQLite-backed Chromium bookmark import, sortable/searchable table, preview pane, and import report.
- Added the local MVP Clean engine: tracking-parameter cleanup, duplicate archiving by cleaned canonical URL, clean reports in SQLite, and a wired Clean button in the desktop UI.

## Release Impact

Not applicable until implementation begins.
