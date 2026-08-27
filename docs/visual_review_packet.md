# Hatchspire exact-candidate visual review packet

Audit date: 27 August 2026

The release smoke renders every registered scene at 1280×720, 1366×768, 1920×1080 fullscreen,
and 960×540. After every technical gate passes, it writes
`target/release-smoke/capture_summary.json` with the exact package identity, scene matrix, PNG sizes,
dimensions, and SHA-256 hashes. A failed or interrupted smoke run cannot leave a new passing summary.

Generate the human handoff from that evidence with:

```powershell
.\scripts\create_visual_review_packet.ps1
```

The generator re-verifies the sealed archive and manifest, matches the packaged executable, rehashes
all 76 source PNGs, and copies them into
`target/visual-review-packet/<full-commit>/`. The ignored packet contains:

- `CAPTURE_MANIFEST.json`, initially marked `awaiting_human_visual_review`;
- `REVIEW_GALLERY.html`, a four-resolution gallery with native-image links;
- `VISUAL_REVIEW_RECORD.md`, a blank scene matrix and cross-scene review record;
- `captures/`, containing the exact hash-verified PNGs reviewed.

The reviewer must open each native-size image, record every scene/resolution result, name issue IDs,
and sign the review. Blank cells, technical PNG validation, and agent judgment do not pass the visual
gate. Keep personal reviewer information outside Git unless its inclusion is explicitly consented.
The generator refuses to overwrite an existing packet for the same commit so it cannot silently erase
review notes; use a different output location for a deliberately separate review copy.

This packet covers deterministic release scenes, not the complete interactive demo, animation,
transitions, audio, input feel, or storefront image selection. Those retain their separate gates.
