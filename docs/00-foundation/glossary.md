---
id: foundation.glossary
title: Glossary
status: draft
owner: product owner
ai_priority: medium
last_updated: 2026-07-09
---

# Glossary

## Bookmark

A saved URL with optional title, folder path, tags, source browser metadata, timestamps, and user notes.

## Browser Connector

A module that imports from or exports to a browser-specific bookmark format or storage location.

## Cleanup Rule

A configurable rule that produces a suggestion such as duplicate, dead link, archive candidate, rename candidate, merge candidate, or noisy folder.

## Clean Library

The app-owned curated bookmark set after review, merge, archive, tagging, and deletion decisions.

## Review Queue

The user-facing queue of suggested cleanup actions. Destructive or browser-writing actions require explicit approval.

## Shared Drive

A user-owned folder synchronized by a cloud or local file sync provider. The current project treats this as a generic file system location until a provider is selected.

## Sync Package

The portable folder structure stored in Shared Drive for exchanging bookmark library state between devices.

## Source Snapshot

A point-in-time import from a browser or file source. It should be preserved enough to audit where an app bookmark came from.

## Tombstone

A sync record showing that an item was deleted or archived, used to prevent deleted data from reappearing during sync.

## Dead Link

A bookmark URL that fails network validation after the configured retry and timeout policy.

## Duplicate Link

Two or more imported bookmarks that resolve to the same canonical URL after normalization.

## Canonical URL

The normalized URL used for duplicate detection and cleanup comparison. The original URL should remain available for audit and preview.

## Query Cleanup

Removal or normalization of known tracking query parameters for canonicalization and reporting while preserving query parameters that may identify the destination content.

## Tracking Parameter

A URL query parameter used primarily for attribution, analytics, campaigns, or cross-site tracking rather than identifying the actual destination content. MVP cleanup removes known tracking parameters and preserves other query parameters.

## HEAD Check

A link validation request that asks a server for response headers without downloading the response body. The MVP uses `HEAD` first because it is usually cheaper for large bookmark batches.

## GET Fallback

A later or secondary validation request that loads the URL with `GET` when `HEAD` is inconclusive. The MVP tags these cases as `needs_get_fallback`, runs fallback, and only then reports remaining unresolved links as uncertain.

## Uncertain Link

A bookmark URL whose network result is not reliable enough to classify as alive or dead in the current pass.

## Project Database

The local SQLite database used by the desktop MVP to store imported bookmarks, canonical URLs, archive state, link check results, and cleanup reports.

## Adaptive Concurrency

A bounded background scheduling strategy that changes the number of simultaneous link checks based on latency, timeout rate, connection errors, and rate-limit responses.

## Archive-Only Cleanup

Cleanup behavior that removes a bookmark from the active pool without deleting it from the browser source or permanently erasing it from the app data.
