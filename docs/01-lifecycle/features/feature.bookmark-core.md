---
id: feature.bookmark-core
title: Core Cross-Browser Bookmark Curation And Sync
status: proposed
owner: product owner
business_priority: high
ai_priority: medium
automation_level: suggest
human_approval: required
audit_required: true
source_of_truth:
  - docs/01-lifecycle/features/feature.bookmark-core.md
related_issue:
related:
  - docs/00-foundation/project-brief.md
  - docs/03-tracks/ux-architecture-track.md
---

# Core Cross-Browser Bookmark Curation And Sync

## Current Feature Cycle

Current Phase: Product Planning

Status: Draft

Objective: Define the core bookmark management product before implementation.

## Summary

Build the core FavItBetter experience: import bookmarks from multiple browsers, normalize and clean them into a compact personal library, review suggested actions, and sync the app-owned library across desktop and mobile devices through a user-selected Shared Drive folder.

## Facts

- The user wants bookmark management across multiple browsers.
- The product target includes desktop and mobile.
- Sync should use files in a personal Shared Drive.
- The product should stay clean, compact, and focused on necessary bookmarks.
- The product should allow more capabilities to be added later.

## Decisions

- The first feature is product and architecture planning, not application code.
- Desktop and mobile are first-class product targets.
- Extension points are part of the core architecture direction.

## Recommendations

- Use local device databases for runtime performance, then publish a durable sync package into Shared Drive.
- Avoid automatic destructive actions in the first MVP.
- Start with import, review, app-library cleanup, and export. Treat browser writeback as a later approval-gated capability.
- Design connectors as replaceable modules:
  - `BrowserConnector`
  - `CleanupRuleProvider`
  - `ExportTarget`
  - `MetadataProvider`
  - `SyncProvider`
- Use stable item IDs based on normalized URL plus source-independent identity, with separate source references for browser origin.

## Assumptions

- Personal single-owner workflow is enough for the first release.
- Mobile cannot reliably read every browser's bookmark store directly, so mobile should focus on the app library, search, quick save, review, and sync status.
- The first implementation can use manual folder selection for Shared Drive rather than direct provider APIs.
- Link checking and AI enrichment are optional modules because they may have privacy, cost, and network implications.

## Open Questions

- Which two browsers should define the first desktop import proof: Safari and Chrome, Chrome and Firefox, or another pair?
- Should cleanup suggestions be conservative by default, or aggressively shrink the library?
- Is "necessary" determined by explicit user review, rules, usage history, AI scoring, or a mix?
- Should archived bookmarks remain searchable?
- Which export formats are required first: browser HTML, JSON, Safari plist, Chromium JSON, Firefox JSON, or app-only sync?
- Which platforms are first: macOS, Windows, Linux, iOS, Android?
- Which Shared Drive provider is the target test environment?

## Goals

- Consolidate bookmarks from multiple browser sources into one clean app library.
- Reduce duplicates and low-value entries through reviewable suggestions.
- Preserve enough source metadata to audit where data came from.
- Sync across user devices without a hosted backend.
- Keep destructive actions reversible or approval-gated.
- Provide extension points for future browser support, cleanup rules, metadata enrichment, and sync providers.

## Non-Goals

- Hosted team workspace.
- Enterprise browser policy management.
- Automatic browser cleanup without confirmation.
- Public social bookmarking.
- Paid extension marketplace.
- Full browser extension implementation in the first planning cycle.

## Personas

- Personal power user: uses several browsers and wants one trusted library.
- Cross-device user: wants desktop curation and mobile access.
- Future connector author: adds support for a new browser, export format, metadata source, or cleanup rule.

## Core Workflow

1. Select browser sources or import files.
2. Create source snapshots.
3. Normalize URLs, titles, folder paths, timestamps, and tags.
4. Detect duplicates, equivalent URLs, dead links, weak titles, empty folders, and archive candidates.
5. Present suggestions in a review queue.
6. User approves keep, merge, rename, tag, archive, delete, or export actions.
7. App updates the clean library.
8. Sync engine writes a sync package to Shared Drive.
9. Other devices import sync package changes and resolve conflicts.
10. Optional export writes a browser-compatible file or approval-gated browser update.

## Data Contract

### BookmarkItem

- `id`
- `canonical_url`
- `display_url`
- `title`
- `description`
- `tags`
- `collections`
- `status`: active, archived, deleted
- `created_at`
- `updated_at`
- `last_checked_at`
- `source_refs`

### SourceSnapshot

- `id`
- `source_type`
- `source_name`
- `device_id`
- `captured_at`
- `raw_format`
- `checksum`
- `item_count`

### CleanupSuggestion

- `id`
- `type`: duplicate, dead_link, redirect, weak_title, empty_folder, archive_candidate, merge_candidate
- `confidence`
- `affected_item_ids`
- `reason`
- `proposed_action`
- `status`: pending, accepted, rejected, deferred

### SyncPackage

- `schema_version`
- `library_id`
- `device_id`
- `manifest`
- `events`
- `snapshots`
- `checksums`
- `conflicts`

### ExtensionManifest

- `id`
- `name`
- `type`: browser_connector, cleanup_rule, export_target, metadata_provider, sync_provider
- `version`
- `capabilities`
- `permissions`
- `configuration_schema`

## Permissions

- Read permission for selected browser bookmark files or exported bookmark files.
- Write permission for app local storage.
- Read/write permission for the selected Shared Drive sync folder.
- Explicit approval for browser writeback or destructive cleanup.
- Optional network permission for link checking, metadata fetching, or AI enrichment.

## Configuration

- Enabled browser connectors.
- Shared Drive sync folder path.
- Cleanup rule strictness.
- Archive retention.
- Conflict resolution policy.
- Link check network limits.
- AI enrichment on/off and provider settings if AI is used.
- Export target preferences.

## Acceptance Criteria

- [ ] Product can define at least two browser source connectors for the first desktop MVP.
- [ ] App-owned bookmark library model supports source snapshots, normalized items, tags, collections, status, and tombstones.
- [ ] Cleanup suggestions are reviewable before destructive action.
- [ ] Sync package format is documented with schema versioning and checksums.
- [ ] Mobile workflow is defined without assuming direct browser-store access.
- [ ] Extension points are documented with permissions and configuration.
- [ ] Documentation drift check confirms project brief, glossary, feature spec, and UX track agree.

## Tests And Validation

Planned validation:

- Import sample bookmarks from two browser formats.
- Verify duplicate detection with normalized URL variants.
- Verify review queue actions are reversible or approval-gated.
- Simulate two devices writing sync events and resolving a conflict.
- Verify mobile can read the synced library package.
- Verify docs and release notes mention user-visible behavior.

## Documentation Impact

- Project brief must stay aligned with this feature spec.
- Glossary must define new domain objects.
- UX architecture track must reflect the current workflow.
- Decision log must capture accepted choices about sync, browser support, and writeback behavior.

## Release Impact

No release yet. First release notes live in `docs/09-release-notes/unreleased.md`.

