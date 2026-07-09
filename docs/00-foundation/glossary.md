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
