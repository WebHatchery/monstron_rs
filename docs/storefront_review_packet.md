# Hatchspire storefront review packet

Audit date: 27 August 2026

The tracked itch page, FAQ, release notes, private invitation, public launch post, and patch-post
drafts contain technical tokens and conspicuous human-decision placeholders. Generate a copy bound
to the current clean sealed candidate with:

```powershell
.\scripts\create_storefront_review_packet.ps1
```

The generator verifies the exact ZIP and manifest, fills only candidate facts, and writes an ignored
packet under `target/storefront-review-packet/<full-commit>/`. It refuses dirty packages and refuses
to overwrite an existing packet that may contain human edits.

The packet's initial status is `awaiting_human_storefront_approval`. Technical substitution does not
approve claims, platform requirements, tags, price, access mode, support promises, disclosures,
screenshots, release notes, or publication. Resolve every `[[HUMAN APPROVAL REQUIRED: ...]]` marker,
review the resulting live page as a player, and record the final decision separately.

Official platform guidance used by the drafts:

- [Your first itch.io page](https://itch.io/docs/creators/getting-started)
- [Controlling access](https://itch.io/docs/creators/access-control)
- [Content creator quality guidelines](https://itch.io/docs/creators/quality-guidelines)
- [Pushing builds with Butler](https://itch.io/docs/butler/pushing.html)

The human owner must create/control the account and page, choose the slug and commercial settings,
authenticate uploads, approve page truth and media, and authorize any external action.
