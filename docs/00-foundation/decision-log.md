---
id: foundation.decision_log
title: Decision Log
status: draft
owner: product owner
ai_priority: high
last_updated: 2026-07-09
---

# Decision Log

## 2026-07-09 - Use ProDOS As Project Operating Framework

Status: Superseded

Owner: product owner

Superseded by: 2026-07-09 - Publish ProDOS Documentation To GitHub Pages With Jekyll

## Context

The user explicitly requested the ProDOS framework skill to start brainstorming and shaping the project.

## Decision

FavItBetter will use ProDOS for knowledge capture, product planning, lifecycle governance, and documentation drift checks.

## Rationale

The project is early, multi-platform, and likely to involve product, UX, data, sync, browser integration, and extension design decisions. ProDOS keeps those decisions visible before implementation starts.

## Consequences

- New important knowledge must be classified as Fact, Decision, Recommendation, Assumption, or Open Question.
- Feature work should be tied to specs, docs, validation, and release notes.
- Important decisions are recorded here.

## Related Artifacts

- Docs: `AGENTS.md`
- Docs: `docs/README.md`

## 2026-07-09 - Target Desktop And Mobile With Personal Shared Drive Sync

Status: Accepted

Owner: product owner

## Context

The user specified desktop and mobile targets and synchronization through a personal Shared Drive.

## Decision

FavItBetter will be planned as a desktop and mobile product with user-owned file synchronization.

## Rationale

This matches the user's stated target and supports a private, local-first personal knowledge workflow.

## Consequences

- Browser access capabilities must be evaluated separately for desktop and mobile.
- The sync design must tolerate cloud drive conflict behavior.
- The app should not depend on a hosted backend for core use.

## Related Artifacts

- Docs: `docs/00-foundation/project-brief.md`
- Docs: `docs/01-lifecycle/features/feature.bookmark-core.md`

## 2026-07-09 - Prefer Sync Package Over Live Database Sync

Status: Proposed

Owner: product owner

## Context

The product needs to sync user data by files through a personal Shared Drive folder.

## Decision

Use a portable sync package with manifest, append-only events, snapshots, checksums, and conflict resolution instead of placing a live database file directly in a cloud drive folder.

## Rationale

Cloud drive tools can sync partial files, duplicate conflicted copies, or reorder writes. A sync package is easier to validate, repair, and merge.

## Consequences

- Each device can keep its own local working database.
- The Shared Drive folder becomes an interchange format, not the live runtime database.
- The sync engine needs schema versioning, device IDs, event IDs, and conflict handling.

## Related Artifacts

- Docs: `docs/01-lifecycle/features/feature.bookmark-core.md`

## 2026-07-09 - Publish ProDOS Documentation To GitHub Wiki Instead Of GitHub Pages

Status: Accepted

Owner: product owner

## Context

The user requested a project-specific change to normal ProDOS behavior: documentation that would usually live under `docs/` and publish to GitHub Pages should instead use GitHub Wiki while preserving the same structure.

GitHub Wiki is a git-backed documentation area attached to a repository. GitHub supports editing wiki pages locally and pushing them through Git. GitHub also supports custom `_Sidebar` and `_Footer` wiki files.

## Decision

FavItBetter will keep `docs/` as the canonical repository source for ProDOS audit, review, and drift detection, but publish or mirror reader-facing documentation to the GitHub Wiki for `git@github.com:jedt3d/FavItBetter.git`.

## Rationale

This preserves ProDOS knowledge governance while honoring the user's preference for GitHub Wiki as the documentation surface.

## Consequences

- GitHub Pages and MkDocs are not the default publication path for this project.
- A future wiki sync workflow may publish from `docs/` to `git@github.com:jedt3d/FavItBetter.wiki.git`.
- The wiki cannot be treated as a perfect MkDocs replacement: navigation, build-time validation, theming, and exact path hierarchy are more limited.
- Documentation drift checks must include the repository `docs/` source and the published wiki copy when the wiki exists.

## Alternatives Considered

- Original ProDOS plan: publish `docs/` through GitHub Pages.
- Wiki-only canonical documentation: rejected for now because baseline ProDOS audit and PR review work better with docs in the main repository.

## Related Artifacts

- Docs: `docs/02-governance/wiki-publication.md`
- Docs: `docs/02-governance/github-workflow.md`

## 2026-07-09 - Publish ProDOS Documentation To GitHub Pages With Jekyll

Status: Accepted

Owner: product owner

## Context

The user reversed the earlier GitHub Wiki preference and asked to return to the normal GitHub Pages direction. The user also suggested a Read the Docs style/template or Jekyll with CloudCannon's `base-jekyll-template`.

GitHub documents Jekyll as a static site generator with built-in support for GitHub Pages and recommends GitHub Actions for deploying and automating Pages sites. CloudCannon's `base-jekyll-template` is a Jekyll knowledge-base template under the MIT license.

## Decision

FavItBetter will publish ProDOS documentation through GitHub Pages using Jekyll. `docs/` remains the canonical source for project knowledge and site publishing. CloudCannon's `base-jekyll-template` is the preferred knowledge-base design reference, adapted locally rather than copied as an unreviewed full starter.

## Rationale

This returns to the ProDOS default publication model and keeps documentation, review, and publishing in the main repository. Jekyll is directly aligned with GitHub Pages and the CloudCannon template suggested by the user.

## Consequences

- The GitHub Wiki publication decision is superseded.
- GitHub Pages should be configured to deploy through GitHub Actions.
- Jekyll configuration lives in `docs/_config.yml`.
- A Pages deployment workflow lives in `.github/workflows/pages.yml`.
- Future visual customization should adapt CloudCannon Base intentionally and preserve its MIT license attribution if source files are copied.

## Alternatives Considered

- GitHub Wiki: superseded by user request.
- Read the Docs style through Sphinx or MkDocs: viable later, but less direct than Jekyll for the selected CloudCannon template.

## Related Artifacts

- Docs: `docs/02-governance/pages-publication.md`
- Docs: `.github/workflows/pages.yml`

## 2026-07-09 - Use CloudCannon Base As Adapted Jekyll Documentation Theme

Status: Accepted

Owner: product owner

## Context

The user asked to use Jekyll RB and the previously discussed CloudCannon Base Jekyll template, then publish a small Hello World page as a smoke test.

CloudCannon Base is a Jekyll knowledge-base template under the MIT license. It is a full starter site rather than a drop-in gem theme.

## Decision

FavItBetter will use a local Jekyll theme adapted from CloudCannon Base patterns: blue header, hero search area, data-driven navigation, and knowledge-base listing cards. The full starter site and sample content will not be copied into the repository.

## Rationale

This satisfies the requested theme direction while keeping the repository small, current, and aligned with ProDOS documentation structure.

## Consequences

- Theme attribution is recorded in `THIRD_PARTY_NOTICES.md`.
- Jekyll layout lives in `docs/_layouts/doc.html`.
- Navigation lives in `docs/_data/navigation.yml`.
- Styling lives in `docs/assets/css/site.css`.
- `docs/hello-world.md` serves as the first publishing smoke test.

## Related Artifacts

- Docs: `THIRD_PARTY_NOTICES.md`
- Docs: `docs/hello-world.md`
- Docs: `docs/_layouts/doc.html`

## 2026-07-09 - Scope First MVP To Desktop Chromium Bookmark Cleaner

Status: Accepted

Owner: product owner

## Context

The product owner answered the first MVP interview. The broader product vision includes desktop, mobile, sync, extensibility, and future AI support, but the first buildable MVP needs a narrower target.

## Decision

The first MVP will be a single-user desktop app for macOS and Windows. It will import local Google Chrome and Microsoft Edge bookmark files, merge them into one pool, automatically clean confirmed dead links, duplicate links, and known tracking query parameters, then show the result in one screen with a toolbar, searchable/sortable table, right-side WebView preview, resizable split, and bottom plain text report.

The first MVP will not include mobile, Shared Drive sync, browser writeback, non-Chromium browsers, or AI cleanup.

## Rationale

This scope proves the core value quickly: import real browser bookmarks, clean obvious bookmark waste, preview remaining links, and report what happened.

## Consequences

- MVP implementation should focus on Chromium bookmark JSON parsing.
- Cleanup is automatic after the user clicks **Clean** and has no options in the MVP.
- "Removed" bookmarks are archive-only inside the app and are not deleted from Chrome or Edge.
- Network link checking is allowed with retries and timeout behavior.
- Query cleanup needs careful canonicalization to avoid breaking links that require query parameters.

## Alternatives Considered

- Mobile MVP: rejected for first implementation.
- Shared Drive sync MVP: rejected for first implementation.
- Browser writeback: rejected for first implementation.
- AI tagging/categorization: deferred as future scope.

## Related Artifacts

- Docs: `docs/01-lifecycle/features/feature.mvp-desktop-chromium-cleaner.md`
- Docs: `docs/00-foundation/project-brief.md`
- Docs: `docs/03-tracks/ux-architecture-track.md`

## 2026-07-09 - Use Tauri 2 SvelteKit 5 And SQLite For Desktop MVP

Status: Accepted

Owner: product owner

## Context

The product owner confirmed a preference for Tauri 2.0 with a SvelteKit 5 frontend and local SQLite storage. The MVP must run on macOS and Windows and needs high-concurrency URL checking.

## Decision

The desktop MVP will use Tauri 2.0 as the application shell, SvelteKit 5 as the frontend, Rust backend commands for local system work and network checks, and SQLite as the local project database.

High-concurrency link checking will run in the Rust backend with bounded/adaptive async workers. SQLite writes should be batched or queued rather than performed independently by every checker task.

## Rationale

This stack matches the user's preference, supports a desktop WebView-style UI on macOS and Windows, and lets the network checker use Rust concurrency without putting long-running cleanup work in the frontend.

## Consequences

- The SvelteKit frontend should be built as a static Tauri frontend.
- The local SQLite database is the runtime project database for the MVP.
- Future Shared Drive sync should not directly sync the live SQLite runtime database.
- The UI should expose **Import** and **Clean**, but not expose concurrency tuning in the MVP.

## Related Artifacts

- Docs: `docs/01-lifecycle/features/feature.mvp-desktop-chromium-cleaner.md`
- Docs: `docs/00-foundation/project-brief.md`

## 2026-07-09 - Define MVP URL Cleanup And Link Check Policy

Status: Accepted

Owner: product owner

## Context

The product owner clarified that query cleanup should remove tracking parameters only, not every query string. The owner also clarified that the 80-second timeout is per attempt and asked for criteria to identify uncertain `HEAD` results that should later fall back to `GET`.

## Decision

MVP query cleanup will strip known tracking parameters only and preserve non-tracking query parameters. Link checking will try `HEAD` first, allow up to three attempts per link, use 80 seconds per attempt, wait five seconds between attempts, tag inconclusive results as `needs_get_fallback`, and run `GET` fallback for those tagged links before final classification.

Inconclusive `HEAD` examples include `405`, `403`, `401`, `429`, inconsistent redirects, likely method-specific server/TLS behavior, and other cases where a `GET` request may succeed even though `HEAD` was not reliable.

## Rationale

Tracking-only cleanup reduces bookmark noise without breaking links that depend on query parameters. `HEAD` is efficient for large batches, while explicit fallback tagging avoids false dead-link decisions for servers that do not handle `HEAD` correctly.

## Consequences

- The report must separate confirmed dead links from unresolved uncertain links.
- The app should not archive every inconclusive `HEAD` response as dead.
- Tests need fixture coverage for tracking-parameter cleanup, non-tracking parameter preservation, and `HEAD` classification.

## Related Artifacts

- Docs: `docs/01-lifecycle/features/feature.mvp-desktop-chromium-cleaner.md`
- Docs: `docs/00-foundation/glossary.md`
