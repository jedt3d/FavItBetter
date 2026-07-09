---
id: governance.github_workflow
title: GitHub Workflow
status: draft
owner: product owner
ai_priority: medium
last_updated: 2026-07-09
---

# GitHub Workflow

## Purpose

Define how FavItBetter links product knowledge, issues, pull requests, validation, and release notes.

## Feature Rule

Every meaningful feature should have:

- an Agent-First Specification
- a GitHub issue
- an implementation branch
- a pull request
- validation evidence
- documentation updates
- release notes or an explicit "not applicable" entry

## Branch Naming

Use `codex/<short-feature-name>` unless the human asks for another branch name.

## Issue Requirements

Feature issues should link:

- feature ID
- source docs
- acceptance criteria
- decisions needed
- open questions
- affected areas
- documentation checklist
- validation evidence

## Pull Request Requirements

Pull requests should include:

- feature and issue links
- summary of changes
- validation evidence
- documentation drift check
- release note impact
- human approval needs

## Current Status

No GitHub issue or PR has been created yet for `feature.bookmark-core`.

## Remote Repository

Primary repository:

```text
git@github.com:jedt3d/FavItBetter.git
```

## Documentation Publishing

FavItBetter uses the normal ProDOS direction of publishing documentation to GitHub Pages.

Project rule:

- `docs/` remains the canonical source inside the main repository.
- Reader-facing documentation is published to GitHub Pages.
- Jekyll is the initial static site generator.
- GitHub Actions is the deployment path.
- CloudCannon's `base-jekyll-template` is the preferred knowledge-base design reference.
