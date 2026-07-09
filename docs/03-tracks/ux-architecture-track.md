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

### Desktop

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

### Mobile

- Library
- Quick Save
- Search
- Review
- Bookmark Detail
- Sync Status
- Settings

## Core Workflows

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

## Component Inventory

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

- Should the primary desktop layout be sidebar plus detail pane, or a command/search centered workspace?
- Should archive be the default for cleanup instead of delete?
- Should review queue use simple decisions or expose confidence, rule source, and evidence by default?

