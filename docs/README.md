# FavItBetter Documentation

This is the canonical knowledge base for FavItBetter.

## Current Feature Cycle

Feature: MVP Desktop Chromium Cleaner

Current Phase: Development

Status: Vertical Slice 0.2.2 report copy/export implemented locally

Objective: Add network dead-link checking with bounded concurrency after large-list import, cleanup, review, and report export are stable.

## Primary Docs

- `00-foundation/project-brief.md`: raw narrative, classified knowledge, and project outline.
- `01-lifecycle/features/feature.bookmark-core.md`: initial Agent-First Specification for the core product.
- `01-lifecycle/features/feature.mvp-desktop-chromium-cleaner.md`: first MVP scope for the desktop Chromium bookmark cleaner.
- `00-foundation/decision-log.md`: accepted and proposed decisions.
- `03-tracks/ux-architecture-track.md`: mental model, information architecture, and workflow direction.
- `02-governance/pages-publication.md`: FavItBetter-specific documentation publishing rule.

## Working Rule

When a new idea changes product intent, browser support, sync behavior, permissions, extension points, or cleanup rules, update the relevant documentation before changing code.

## Publication Rule

FavItBetter uses GitHub Pages as the reader-facing documentation destination. `docs/` remains the source tree for ProDOS audit, pull request review, documentation publishing, and documentation drift checks.
