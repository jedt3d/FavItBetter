---
id: docs.home
title: FavItBetter Documentation
status: draft
owner: product owner
ai_priority: high
layout: doc
nav_order: 0
last_updated: 2026-07-09
hero: true
hero_title: FavItBetter Knowledge Base
hero_subtitle: ProDOS project memory for a local-first cross-browser bookmark manager.
body_class: show_hero_search
---

# FavItBetter Documentation

FavItBetter is a local-first bookmark manager in early MVP development. The current documentation captures ProDOS project knowledge, the Tauri/SvelteKit app foundation, and the local Clean engine.

## Start Here

<div class="listing-grid">
  <a class="listing-card" href="{{ "/hello-world.html" | relative_url }}">
    <strong>Hello World</strong>
    <span>Small Jekyll publishing smoke test.</span>
  </a>
  <a class="listing-card" href="{{ "/00-foundation/project-brief.html" | relative_url }}">
    <strong>Project Brief</strong>
    <span>Initial product narrative and classified knowledge.</span>
  </a>
  <a class="listing-card" href="{{ "/01-lifecycle/features/feature.bookmark-core.html" | relative_url }}">
    <strong>Core Feature Spec</strong>
    <span>Cross-browser bookmark curation and sync plan.</span>
  </a>
  <a class="listing-card" href="{{ "/01-lifecycle/features/feature.mvp-desktop-chromium-cleaner.html" | relative_url }}">
    <strong>MVP Desktop Cleaner</strong>
    <span>Mac/Windows Chromium bookmark import, clean, preview, and report.</span>
  </a>
  <a class="listing-card" href="{{ "/00-foundation/decision-log.html" | relative_url }}">
    <strong>Decision Log</strong>
    <span>Accepted and superseded project decisions.</span>
  </a>
</div>

## Current Feature Cycle

Feature: MVP Desktop Chromium Cleaner

Current Phase: Development

Status: Vertical Slice 0.2.2 report copy/export implemented locally

Objective: Add network dead-link checking with bounded concurrency after large-list import, cleanup, review, and report export are stable.
