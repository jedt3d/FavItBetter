---
id: feature.mvp-desktop-chromium-cleaner
title: MVP Desktop Chromium Bookmark Cleaner
status: in_progress
owner: product owner
business_priority: critical
ai_priority: medium
automation_level: execute_after_approval
human_approval: required
audit_required: true
source_of_truth:
  - docs/01-lifecycle/features/feature.mvp-desktop-chromium-cleaner.md
related_issue: https://github.com/jedt3d/FavItBetter/issues/1
related:
  - docs/00-foundation/project-brief.md
  - docs/01-lifecycle/features/feature.bookmark-core.md
  - docs/03-tracks/ux-architecture-track.md
---

# MVP Desktop Chromium Bookmark Cleaner

## Current Feature Cycle

Current Phase: Development

Status: Vertical Slice 0.2 local clean engine implemented locally

Objective: Add network dead-link checking with bounded concurrency after local duplicate and tracking-query cleanup.

## Summary

Build a desktop-only MVP for one personal user on macOS and Windows. The app imports bookmark files saved locally by Google Chrome and Microsoft Edge, merges them into one pool, cleans dead links, duplicate links, and known tracking query parameters, then shows the result in a single-screen interface with a table on the left, resizable web preview on the right, and a plain text report at the bottom.

The MVP does not sync, does not support mobile, does not write back into browsers, and does not require cleanup options. The main workflow is import from local browser bookmark files, then press **Clean** to run the configured cleanup process automatically.

## Raw Interview Capture

- The MVP is for the product owner to use alone.
- First platforms are macOS and Windows only.
- First browsers are Chromium-based browsers only, specifically Google Chrome and Microsoft Edge.
- Import reads bookmark files saved on the local machine by those browsers.
- Cleanup should handle dead links, duplicate links, and tracking query parameters.
- Deleted/removed entries should be archive-only behavior, not browser deletion.
- No browser writeback in the MVP.
- No Shared Drive integration in the MVP.
- No mobile implementation in the MVP.
- Network link checking is allowed.
- Link checking should retry two to three times, five seconds apart.
- Link check timeout is 80 seconds per attempt, with up to three attempts per link and five seconds between attempts.
- AI is future scope for tags and categories, not MVP cleanup.
- Imported and cleaned data should persist in a local SQLite project database.
- Preferred MVP stack is Tauri 2.0, SvelteKit running on Svelte 5, and local SQLite.
- Link checking should try `HEAD` first, tag uncertain results as `needs_get_fallback`, and run `GET` fallback for those tagged cases.
- MVP demo success is importing from Google Chrome and reporting how many dead links were removed.
- The UI can be one screen.
- Toolbar includes import and clean actions.
- The table supports sorting and search.
- The right panel previews the selected link in a WebView or embedded browser.
- The divider between table and preview is resizable.
- The bottom report is a plain text area showing what was cleaned.

## Facts

- Browser target is constrained to Chromium bookmark file formats for the MVP.
- The MVP is single-user and local desktop only.
- The MVP can use network access for link checks.
- The user wants automatic cleanup rather than a configurable review flow in the MVP.
- The user prefers Tauri 2.0 with a SvelteKit frontend running on Svelte 5 and local SQLite storage.
- High-concurrency link checking is a material requirement for the MVP architecture.

## Decisions

- MVP platforms: macOS and Windows.
- MVP browser sources: Google Chrome and Microsoft Edge.
- MVP import source: local browser bookmark files, not browser extension APIs and not manual exported HTML.
- MVP cleanup actions: dead link detection, duplicate detection, and URL query cleanup.
- MVP query cleanup removes tracking parameters only, not every query string.
- MVP removal behavior: archive-only inside the app pool; no deletion from source browser files.
- MVP UI: one window with toolbar, searchable/sortable table, web preview, resizable split, and text report.
- MVP button language: use **Clean**, not **Process**.
- MVP persistence: store imported bookmarks, cleaned URLs, archive state, check results, and reports in a local SQLite project database.
- MVP implementation stack: Tauri 2.0 shell, SvelteKit frontend running on Svelte 5, Rust backend commands, and SQLite local storage.
- MVP link checking: use `HEAD` first, classify inconclusive results as `needs_get_fallback`, then run `GET` fallback as a second pass inside the same **Clean** action.
- MVP timeout policy: 80 seconds per attempt, up to three attempts, with five seconds between attempts.
- MVP concurrency model: use bounded asynchronous checking in the Rust backend and keep the UI free of concurrency controls.
- MVP excludes Shared Drive sync.
- MVP excludes mobile.
- MVP excludes AI except as future tagging/categorization.
- MVP excludes browser writeback.

## Recommendations

- Preserve the original URL for audit and preview even when a cleaned/canonical URL is generated.
- Use a `canonical_url` for duplicate detection and cleanup reporting.
- Strip known tracking parameters such as `utm_*`, `fbclid`, `gclid`, `msclkid`, and similar marketing identifiers. Preserve non-tracking query parameters because some websites require them to identify the actual page.
- Treat "dead link removed" in reports as "archived from the active pool" until the product owner explicitly approves permanent deletion.
- Detect Chrome and Edge profile bookmark files automatically, but allow manual file selection as a fallback.
- Use Rust async work in the Tauri backend for network checks, not browser-side JavaScript, so link checking can run with bounded high concurrency and avoid frontend lifecycle limits.
- Stream progress from the backend to the SvelteKit UI so the report and table can update during long clean runs.
- Use a SQLite write queue or batched transactions. Link check workers should produce results concurrently, but database writes should be serialized or carefully bounded to avoid lock contention.
- Start with adaptive internal concurrency, for example 16 concurrent link checks, a floor of 4, and a cap of 64. Increase cautiously when latency and errors are low; reduce when timeouts, connection errors, or `429` responses rise.

## Assumptions

- "Archive only" means cleaned-out bookmarks stay in the app's archived state and can be inspected later.
- "Remove dead links" means remove from active table/pool, not delete from Chrome or Edge files.
- Query string cleanup is first applied to canonicalization and duplicate detection; the original URL remains available.
- Chrome and Edge bookmark files are expected to be Chromium `Bookmarks` JSON files under user profile directories.
- SvelteKit is built as a static Tauri frontend for the MVP.
- The local SQLite database is the runtime project database, not the future Shared Drive sync artifact.

## Open Questions

- Should archived bookmarks remain searchable in the MVP?
- What is the exact first tracking parameter list, and should it be configurable by editing a local config file after MVP?

## Goals

- Import Google Chrome bookmarks from local files.
- Import Microsoft Edge bookmarks from local files.
- Merge imports into one bookmark pool.
- Clean dead links using network checks with retry and timeout behavior.
- Clean duplicates using canonical URL comparison.
- Clean known tracking query parameters.
- Present active bookmarks in a sortable and searchable table.
- Show selected bookmark in a right-side WebView preview.
- Allow resizing between table and preview.
- Show a plain text cleanup report at the bottom.
- Avoid changing the original browser bookmark stores.

## Non-Goals

- Mobile app.
- Shared Drive sync.
- Browser extension.
- Writing changes back to Chrome or Edge.
- Configurable cleanup options in the UI.
- AI-based cleanup.
- Multi-user support.
- Non-Chromium browser support.
- Full review queue workflow.

## Users And Roles

- Product owner / personal user: imports bookmarks, cleans them, previews active bookmarks, and reads the cleanup report.

## MVP Workflow

1. User opens the desktop app.
2. User clicks **Import** in the toolbar.
3. App finds local Google Chrome and Microsoft Edge bookmark files, or lets the user choose files if automatic discovery fails.
4. App parses Chromium bookmark JSON into a single bookmark pool.
5. App shows imported bookmarks in a left-side table.
6. User clicks **Clean** in the toolbar.
7. App canonicalizes URLs, strips configured tracking query parameters, finds duplicate canonical URLs, and checks links over the network with `HEAD`.
8. App tags inconclusive `HEAD` results as `needs_get_fallback` and runs `GET` fallback for those links.
9. App archives confirmed dead/duplicate entries from the active pool and leaves unresolved uncertain links active with a report note.
10. App updates the table to show the active cleaned pool.
11. App writes a plain text report into the bottom textarea.
12. User clicks any row in the table.
13. App loads that row's original or display URL in the right-side WebView preview.
14. User can search and sort the table and resize table/preview width.

## Data Contract

### ImportedBookmark

- `id`
- `source_browser`: chrome, edge
- `source_profile`
- `source_path`
- `folder_path`
- `title`
- `original_url`
- `canonical_url`
- `cleaned_url`
- `date_added`
- `status`: active, archived
- `archive_reason`: dead_link, duplicate, query_cleaned_duplicate, manual_future
- `link_check_state`: unchecked, alive, dead, uncertain, needs_get_fallback
- `last_checked_at`
- `http_status`
- `check_attempts`
- `check_error`

### CleanReport

- `started_at`
- `finished_at`
- `imported_count`
- `active_count_before`
- `active_count_after`
- `dead_link_count`
- `duplicate_count`
- `query_cleaned_count`
- `archived_count`
- `errors`
- `plain_text`

### ProjectDatabase

- `path`
- `schema_version`
- `created_at`
- `updated_at`
- `last_imported_at`
- `last_cleaned_at`

### LinkCheckResult

- `bookmark_id`
- `method_used`: HEAD, GET
- `classification`: alive, dead, uncertain, needs_get_fallback
- `attempts`
- `per_attempt_timeout_seconds`
- `delay_between_attempts_seconds`
- `total_elapsed_ms`
- `final_status_code`
- `redirect_url`
- `error_kind`
- `checked_at`

### LinkCheckPolicy

- `max_attempts`: 3
- `delay_between_attempts_seconds`: 5
- `timeout_seconds_per_attempt`: 80
- `primary_method`: HEAD
- `fallback_method`: GET
- `fallback_execution`: automatic_second_pass_inside_clean
- `uncertain_tag`: needs_get_fallback
- `network_allowed`: true

### LinkCheckClassification

- `alive`: `HEAD` returns a successful or redirect status that can be followed within limits.
- `dead`: repeated attempts produce conclusive dead responses such as `404` or `410`, or repeated timeouts/errors after the full retry policy.
- `needs_get_fallback`: `HEAD` returns `405`, `403`, `401`, `429`, inconsistent redirects, unsupported method behavior, TLS/server behavior that is likely method-specific, or another result where `GET` may succeed.
- `uncertain`: the checker cannot confidently classify the link after the current pass and should not archive it as dead unless the accepted policy says full retry exhaustion is enough.

### QueryCleanPolicy

- `mode`: tracking_params_first
- `preserve_original_url`: true
- `tracking_params`: `utm_*`, `fbclid`, `gclid`, `msclkid`, `mc_cid`, `mc_eid`, `igshid`, `ref`, `spm`

### ConcurrencyPolicy

- `mode`: adaptive_bounded
- `initial_concurrency`: 16
- `min_concurrency`: 4
- `max_concurrency`: 64
- `reduce_on`: timeout_rate, connection_error_rate, rate_limit_rate
- `increase_on`: low_latency, low_error_rate
- `database_write_mode`: queued_or_batched

## Permissions

- Read local Chrome and Edge bookmark files.
- Network access for link checking and WebView preview.
- Local SQLite app storage for imported pool, archive state, link check results, and reports.
- No write permission to browser bookmark stores in MVP.

## Configuration

The MVP should avoid user-facing cleanup options. Internal constants can exist for:

- Chrome and Edge default bookmark paths by OS.
- Link check retry count, retry delay, and timeout.
- Query parameter cleanup list.
- Internal link check concurrency policy.
- WebView preview behavior.

## UI Contract

- Single main window.
- Top toolbar with:
  - **Import**
  - **Clean**
- Main split area:
  - Left: searchable/sortable bookmark table.
  - Right: WebView/WebBrowser preview for selected row.
  - Splitter is resizable.
- Bottom:
  - Plain text report textarea.
- During **Clean**, progress can be streamed into the report area and table state without adding cleanup options.

## Technical Evaluation

Tauri 2.0, SvelteKit running on Svelte 5, and SQLite are feasible for this MVP.

- Tauri backend commands can run asynchronous Rust work and stream progress to the frontend through channels or events.
- SvelteKit can be used as a static frontend for Tauri with SSR disabled.
- SQLite is appropriate as the local project database when writes are batched or queued.
- High-concurrency link checking should live in Rust, using bounded async workers and an adaptive scheduler. SvelteKit should render state and send commands, not perform the network check loop itself.

References:

- [Tauri command and channel guide](https://v2.tauri.app/develop/calling-rust/)
- [Tauri SvelteKit guide](https://v2.tauri.app/start/frontend/sveltekit/)
- [Tauri SQL plugin guide](https://v2.tauri.app/plugin/sql/)

## Acceptance Criteria

- [x] App is implemented as a Tauri 2.0 desktop app with a SvelteKit frontend running on Svelte 5.
- [x] App stores imported project state in local SQLite.
- [x] App builds on macOS.
- [ ] App runs on Windows.
- [ ] App imports local Google Chrome bookmark files.
- [ ] App imports local Microsoft Edge bookmark files.
- [ ] App merges Chrome and Edge bookmarks into one pool.
- [x] Table displays imported bookmarks with search and sorting.
- [x] Selecting a row loads the bookmark in a right-side preview pane.
- [x] Table and preview panes are resizable.
- [x] Clean button archives duplicates from the active pool.
- [ ] Clean button archives confirmed dead links from the active pool after three attempts with 80 seconds per attempt and five-second retry gaps.
- [ ] Clean button tags inconclusive `HEAD` results as `needs_get_fallback` and runs `GET` fallback before final classification.
- [x] Clean button applies tracking-parameter cleanup to canonical URLs and reports the effect.
- [ ] Link checking runs with bounded/adaptive concurrency in the backend.
- [ ] App does not modify Chrome or Edge bookmark stores.
- [x] Bottom textarea reports imported count, dead links, duplicates, query-cleaned links, archived count, and errors.
- [ ] Bottom textarea reports `needs_get_fallback` or uncertain link counts separately from confirmed dead links.
- [ ] MVP demo can show Chrome import and dead link cleanup report.

## Implementation Slices

### Vertical Slice 0.1: App Foundation And Import Preview

Status: implemented locally

Related issue: [#1](https://github.com/jedt3d/FavItBetter/issues/1)

Implemented:

- Tauri 2 desktop shell.
- SvelteKit frontend running on Svelte 5 with static adapter and SSR disabled.
- Rust backend commands for `import_bookmarks_json` and `list_bookmarks`.
- Local SQLite database at the app data directory.
- Chromium `Bookmarks` JSON parser for nested folders and URL rows.
- Single-screen UI with toolbar, browser source selector, import button, searchable/sortable table, resizable split, preview pane, and plain text report.
- Sample fixture at `tests/fixtures/chromium-bookmarks.sample.json`.

Not included in this slice:

- Clean button behavior.
- Duplicate cleanup.
- Tracking parameter cleanup.
- Dead-link checking.
- Automatic browser profile discovery.
- Windows validation.

### Vertical Slice 0.2: Local Clean Engine

Status: implemented locally

Related issue: [#1](https://github.com/jedt3d/FavItBetter/issues/1)

Implemented:

- Rust URL cleanup that removes `utm_*`, `fbclid`, `gclid`, `msclkid`, `mc_cid`, `mc_eid`, `igshid`, `ref`, and `spm`.
- Fragment removal and tracking-parameter cleanup while preserving non-tracking query parameters.
- Clean button wired to a Tauri `clean_bookmarks` command.
- SQLite-backed clean reports in `clean_reports`.
- Duplicate detection by cleaned canonical URL across the active pool.
- Archive-only duplicate removal from the active pool with `duplicate` or `query_cleaned_duplicate` reasons.
- Preview and external open actions prefer `cleaned_url` after cleanup while preserving `original_url` in the database.
- Plain text Clean report showing active counts, query-cleaned links, removed tracking parameters, duplicate archives, dead-link count placeholder, and errors.

Not included in this slice:

- Network dead-link checking.
- `HEAD`/`GET` fallback classification.
- Bounded/adaptive link-check concurrency.
- Automatic browser profile discovery.
- Windows validation.

## Tests And Validation

- Parse fixture Chromium bookmark JSON from Chrome.
- Parse fixture Chromium bookmark JSON from Edge.
- Merge fixture imports into one pool.
- Verify duplicate detection across Chrome and Edge for same canonical URL.
- Verify query cleanup for tracking parameters.
- Verify query cleanup preserves non-tracking query parameters.
- Verify original URL is preserved after query cleanup.
- Verify dead link retry policy with simulated timeout and HTTP failure responses.
- Verify `HEAD` responses are classified into alive, dead, and needs_get_fallback cases.
- Verify SQLite persistence can reopen an imported and cleaned project.
- Verify link check scheduler respects max concurrency and reduces concurrency under simulated timeout or rate-limit pressure.
- Verify active vs archived counts in report.
- Verify UI search and sorting behavior.
- Verify WebView loads selected active bookmark.
- Vertical Slice 0.1 validation: `npm run check`, `npm run build`, `cargo fmt --check`, `cargo test`, `cargo check`, and `npm run tauri -- build`.
- Vertical Slice 0.2 validation: `cargo test`, `npm run check`, `cargo check`, `npm run build`, `npm run tauri -- build`, ProDOS audit, Jekyll docs build, and `git diff --check`.

## Documentation Impact

- Project brief must describe MVP narrowing from broad desktop/mobile/sync vision to desktop-only Chromium cleaner.
- Core feature spec should link to this MVP spec as the first concrete implementation slice.
- UX architecture track must reflect the single-screen MVP.
- Decision log must record MVP scope decisions.
- Release notes must mention MVP planning updates.

## Release Impact

No application release yet. This MVP spec defines the first implementation target.
