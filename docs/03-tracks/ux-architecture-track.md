---
id: track.ux_architecture
title: UX Architecture Track
status: draft
owner: product owner
business_priority: high
ai_priority: medium
last_updated: 2026-07-09
source_of_truth:
  - docs/01-lifecycle/features/feature.bookmark-core.md
  - docs/01-lifecycle/features/feature.mvp-desktop-chromium-cleaner.md
---

# UX Architecture Track

## Purpose

Define FavItBetter's mental model, information architecture, workflows, and screen map before visual design.

## Mental Model

FavItBetter is not a browser bookmark folder editor first. It is a curation workspace:

- Sources: where bookmarks came from.
- Inbox: newly imported or newly saved bookmarks.
- Review Queue: suggested cleanup decisions.
- Clean Library: the app-owned useful bookmark set.
- Sync: the shared file package that moves the clean library across devices.
- Extensions: connectors and rules that expand capabilities.

## Information Architecture

### MVP Desktop

- Single main window
- Top toolbar
- Bookmark pool table
- WebView preview
- Plain text report
- Local SQLite project state

### Future Desktop

- Dashboard
- Sources
- Import History
- Review Queue
- Clean Library
- Bookmark Detail
- Tags And Collections
- Cleanup Rules
- Sync Status
- Extensions
- Settings

### Future Mobile

- Library
- Quick Save
- Search
- Review
- Bookmark Detail
- Sync Status
- Settings

## Core Workflows

### MVP Import And Clean

1. User clicks **Import**.
2. App finds or accepts local Google Chrome and Microsoft Edge bookmark files.
3. App merges bookmarks into one pool.
4. App shows the pool in a left-side searchable/sortable table.
5. User clicks **Clean**.
6. App archives duplicate links and confirmed dead links from the active pool.
7. App canonicalizes URLs and removes known tracking query parameters for duplicate detection/reporting.
8. App tags inconclusive `HEAD` link checks as `needs_get_fallback` and runs `GET` fallback.
9. App reports confirmed dead links and unresolved uncertain links separately.
10. App writes a plain text report to the bottom textarea.
11. User selects a table row.
12. App loads the selected link in a right-side WebView preview.
13. User can resize the table/preview split.

### Import And Review

1. User chooses a browser source or bookmark export file.
2. App creates a source snapshot.
3. App shows import summary and cleanup suggestions.
4. User reviews suggestions in batches.
5. Accepted actions update the clean library.

### Quick Save On Mobile

1. User shares a URL into FavItBetter.
2. App captures URL, title if available, note, and tags.
3. Item enters Inbox or Clean Library depending on settings.
4. Sync package publishes the change when the Shared Drive folder is available.

### Sync Health

1. App shows last sync time, device status, conflicts, and package version.
2. User can inspect conflicts.
3. User can choose keep local, keep remote, merge, or defer.

### Extension Management

1. App lists installed connectors and rule providers.
2. User enables a connector or rule.
3. App shows required permissions and configuration.
4. User approves before the extension can access sources, network, or write targets.

## Interaction Pattern Decisions

- Suggestions should be batchable but individually inspectable.
- Destructive actions should support undo or archive-first behavior.
- Sync conflicts should be visible and explainable, not silent.
- Mobile screens should prioritize search, quick save, and light review rather than full source management.
- Desktop should handle heavier import, cleanup, and export workflows.
- MVP cleanup is automatic after pressing **Clean** and does not expose cleanup options.
- MVP uses archive-only behavior instead of destructive deletion.
- MVP uses a left table and right WebView preview with a resizable divider.
- MVP report distinguishes confirmed dead links from unresolved uncertain links after fallback.
- MVP does not expose link-check concurrency controls in the UI.

## Component Inventory

- MVP toolbar
- MVP import button
- MVP clean button
- MVP searchable/sortable bookmark table
- MVP resizable split view
- MVP WebView preview
- MVP plain text report textarea
- MVP clean progress/report status
- Source connector list
- Import summary
- Review queue item
- Duplicate comparison panel
- Bookmark detail panel
- Tag editor
- Collection picker
- Sync status indicator
- Conflict resolver
- Extension permission dialog
- Cleanup rule settings

## Open Questions

- Should archived bookmarks remain searchable in the MVP?
- Should the report textarea be read-only?
- Should WebView preview use original URL or cleaned URL when query parameters were removed?
