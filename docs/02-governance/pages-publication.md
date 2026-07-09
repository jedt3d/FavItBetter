---
id: governance.pages_publication
title: GitHub Pages Documentation Publication
status: draft
owner: product owner
ai_priority: high
last_updated: 2026-07-09
source_of_truth:
  - docs/02-governance/pages-publication.md
related:
  - docs/02-governance/github-workflow.md
  - docs/00-foundation/decision-log.md
---

# GitHub Pages Documentation Publication

## Evaluation

Using GitHub Pages with Jekyll is possible and is now the selected publishing approach for FavItBetter.

GitHub documents Jekyll as a static site generator with built-in support for GitHub Pages. GitHub also recommends GitHub Actions for deploying and automating Pages sites.

## Decision

FavItBetter will publish reader-facing ProDOS documentation through GitHub Pages using a Jekyll site rooted at `docs/`.

## Template Direction

Use CloudCannon's `base-jekyll-template` as the preferred knowledge-base design reference.

The template is suitable as a reference because it is a Jekyll knowledge-base template, has an MIT license, and includes patterns for tutorials, FAQ content, sticky sidebars, SEO tags, and navigation. It should be adapted carefully because it was originally built with Jekyll 3.4.3 and is a full starter site rather than a drop-in gem theme.

Current local adaptation:

- data-driven top navigation in `docs/_data/navigation.yml`
- Base-style blue header and hero search area
- knowledge-base listing cards on the docs home page
- local attribution in `THIRD_PARTY_NOTICES.md`

## Recommendation

Use Jekyll directly for the first docs site rather than Read the Docs.

Read the Docs style can be achieved later through Sphinx, MkDocs, or a Jekyll theme that resembles that documentation pattern. For this repository, Jekyll is the cleaner first step because GitHub Pages supports it directly and the selected CloudCannon template is Jekyll-based.

## Current Implementation Plan

- Keep all ProDOS documentation in `docs/`.
- Add `docs/_config.yml` for Jekyll configuration.
- Add a local layout adapted from CloudCannon Base knowledge-base patterns.
- Deploy through `.github/workflows/pages.yml`.
- Keep GitHub Pages configured to use GitHub Actions.
- Adapt CloudCannon Base styling and information architecture incrementally if the first generated site needs richer docs navigation.

## Assumptions

- GitHub Pages will be enabled for `jedt3d/FavItBetter`.
- The repository Pages source will be configured as GitHub Actions.
- The public URL will use the standard project site path unless a custom domain is configured later.

## Open Questions

- Should the final documentation site copy more of CloudCannon Base's visual structure, or stay lightweight?
- Should docs search be added in the first implementation?
- Should the site include a public product-facing section separate from internal ProDOS governance docs?

## Drift Check

Before completing documentation work:

- Confirm `docs/` is still canonical.
- Confirm the Jekyll build includes new important docs.
- Confirm the GitHub Pages workflow still builds from `docs/`.
- Confirm release notes mention user-visible documentation publishing changes.
