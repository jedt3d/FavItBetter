# FavItBetter

FavItBetter is a planned local-first bookmark manager for people who use multiple browsers and want a cleaner, smaller, more useful bookmark library.

The project is currently in ProDOS knowledge capture and product planning. There is no application code yet.

## Current Direction

- Target desktop and mobile versions.
- Import and organize bookmarks from multiple browsers.
- Help remove duplicates, stale links, noise, and unused entries.
- Keep the product library synced through a user-owned Shared Drive folder.
- Leave extension points for browser connectors, cleanup rules, export targets, metadata providers, and sync providers.

## First MVP

The first MVP is narrower than the full product vision: a single-user Tauri 2.0 desktop app for macOS and Windows that imports local Google Chrome and Microsoft Edge bookmark files, persists the pool in SQLite, archives confirmed dead and duplicate links, removes known tracking query parameters, previews selected links in a WebView, and reports the cleanup in plain text.

## Documentation

Start with `docs/README.md`.

Reader-facing documentation should be published through GitHub Pages using the Jekyll site in `docs/`. The repository `docs/` tree remains the canonical source used for ProDOS review, publishing, and drift detection.
