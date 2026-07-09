---
id: governance.wiki_publication
title: Superseded GitHub Wiki Documentation Publication Evaluation
status: superseded
owner: product owner
ai_priority: high
last_updated: 2026-07-09
source_of_truth:
  - docs/02-governance/wiki-publication.md
related:
  - docs/02-governance/pages-publication.md
  - docs/02-governance/github-workflow.md
  - docs/00-foundation/decision-log.md
---

# Superseded GitHub Wiki Documentation Publication Evaluation

## Status

This evaluation is superseded. FavItBetter now uses GitHub Pages with Jekyll for documentation publishing.

Canonical active publishing rule: `docs/02-governance/pages-publication.md`.

## Evaluation

Using GitHub Wiki instead of GitHub Pages was evaluated and found possible for FavItBetter, but it is no longer the selected project direction.

GitHub Wiki supports repository documentation, local editing through Git, Markdown rendering, and custom sidebar/footer files. This is enough for FavItBetter's early ProDOS documentation needs.

## Facts

- The primary repository is `git@github.com:jedt3d/FavItBetter.git`.
- The expected wiki repository is `git@github.com:jedt3d/FavItBetter.wiki.git`.
- GitHub Wiki pages can be edited locally and pushed through Git after the wiki has been initialized.
- GitHub supports `_Sidebar.<extension>` and `_Footer.<extension>` files for wiki navigation and footer content.
- GitHub documents a soft limit of 5,000 wiki files.

## Decision

FavItBetter will not redirect reader-facing ProDOS documentation from GitHub Pages to GitHub Wiki unless the Pages decision is superseded later.

## Historical Recommendations

The superseded Wiki path would have kept `docs/` as the canonical source in the main repository and published to the wiki through a controlled manual or automated mirror step.

The proposed wiki page names were:

  - `Home.md`
  - `00-Foundation---Project-Brief.md`
  - `00-Foundation---Decision-Log.md`
  - `01-Lifecycle---Feature-Lifecycle.md`
  - `01-Lifecycle---Feature-bookmark-core.md`
  - `02-Governance---GitHub-Workflow.md`
  - `02-Governance---Wiki-Publication.md`
  - `03-Tracks---UX-Architecture-Track.md`

## Historical Assumptions

- The wiki is enabled for `jedt3d/FavItBetter`.
- The wiki is initialized with at least one page before local clone/sync automation is attempted.
- Exact MkDocs-style paths and navigation are not required as long as the logical ProDOS structure is preserved.

## Historical Open Questions

- Should the wiki mirror preserve YAML frontmatter, or should the published pages be cleaner reader-facing Markdown?
- Should wiki sync be manual at first or automated through GitHub Actions?
- Should `docs/` remain the only canonical source, or should the wiki become editable by humans with back-sync into `docs/`?

## Limitations

- GitHub Wiki is not a complete replacement for MkDocs Material.
- Wiki navigation is simpler and should be maintained through `_Sidebar.md`.
- GitHub Wiki does not provide the same static-site build validation as GitHub Pages.
- Public wiki search engine indexing can be limited compared with GitHub Pages.
- Wiki file naming rules mean the published structure should be logical rather than a strict copy of every repository path.

## Historical Drift Check

Before a docs-related feature is complete:

- Confirm the source file in `docs/` is updated.
- Confirm the corresponding wiki page exists or the missing page is documented.
- Confirm `_Sidebar.md` links to any new important wiki page.
- Record any manual wiki edits back into the repository source.
