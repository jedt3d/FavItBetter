---
id: foundation.project_brief
title: FavItBetter Project Brief
status: draft
owner: product owner
ai_priority: high
last_updated: 2026-07-09
source_of_truth:
  - docs/01-lifecycle/features/feature.bookmark-core.md
related:
  - docs/03-tracks/ux-architecture-track.md
---

# FavItBetter Project Brief

## Raw Narrative Intake

The user wants to start a ProDOS-governed brainstorm for software that manages bookmarks across multiple browsers. The software should make bookmarks compact, clean, and limited to what is necessary. It should also support adding more capabilities later. The project target includes desktop and mobile versions, with file synchronization through a personal Shared Drive.

## Facts

- The repository was empty at project setup time except for Git metadata.
- The user requested ProDOS as the project operating framework.
- The product is about bookmark management across multiple browsers.
- The product should have desktop and mobile versions.
- The product should sync files through a personal Shared Drive.

## Decisions

- FavItBetter will begin with knowledge capture and product planning before implementation.
- Desktop and mobile are first-class targets for the product direction.
- The product must support future extension points rather than only a fixed browser list.

## Recommendations

- Use a local-first architecture with a user-selected sync folder.
- Do not sync a live SQLite database directly through cloud drive storage. Prefer a portable sync package with manifest, event log, snapshots, checksums, and conflict handling.
- Treat browser modification as approval-gated. The first version should import, analyze, review, and export before it writes back to browsers.
- Model extensibility explicitly through connectors and rule providers:
  - browser connectors
  - cleanup rules
  - export targets
  - metadata providers
  - sync providers
- Make mobile useful even if direct browser bookmark access is limited. Mobile should support quick save, search, review, and synced library access through platform-allowed mechanisms.

## Assumptions

- "Shared Drive" means a user-owned folder synchronized by a service such as Google Drive, iCloud Drive, Dropbox, OneDrive, Synology Drive, or another file sync tool.
- Initial use is personal or single-owner, not team bookmark management.
- The first release can prioritize curation and review over full automatic browser rewrite.
- Browser extension support can be added after the core app and data model are stable.

## Open Questions

- Which Shared Drive provider should be treated as the first-class target first?
- Which browsers should be supported in the first desktop import pass: Safari, Chrome, Edge, Brave, Firefox, or another browser?
- Should the app ever write changes back into browser bookmark stores automatically?
- Should cleanup include network link checking, and if yes, how aggressive can it be?
- Should AI be used for categorization, summaries, or "keep/delete" suggestions?
- Should the mobile app rely only on the app library and share sheet, or does direct mobile browser integration matter?
- Does "เหลือเฉพาะอันที่จำเป็น" mean user-reviewed essentials only, automatic scoring, or configurable rule sets?

## Product Shape

FavItBetter should be a local-first curation tool:

1. Import bookmarks from browser sources.
2. Normalize URLs and metadata.
3. Detect duplicates, redirects, dead links, noisy folders, and rarely used items when evidence exists.
4. Present a review queue with suggested actions.
5. Keep, merge, tag, archive, delete from the app library, or export changes.
6. Sync the app-owned library through files in a personal Shared Drive.
7. Support new connectors and cleanup rules without redesigning the core.

## MVP Boundary

The first MVP should prove that a user can consolidate bookmarks from at least two desktop browser sources into one clean library, review cleanup suggestions, and sync that library to another device through a file sync folder.

## Non-Goals For The First MVP

- Hosted multi-user backend.
- Enterprise policy management.
- Automatic destructive browser cleanup.
- Full browser extension ecosystem.
- Public social bookmark sharing.
- Paid marketplace for rule packs or connectors.
