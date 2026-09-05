# Hatchspire toolkit migration — 5 September 2026

The remaining generic text layout now uses the shared toolkit. Help paths use
measured literal wrapping that preserves repeated spaces and Unicode bytes.
Tower status, event choices, context descriptions, field-guide entries,
anomalies, journals, town logs, tutorials and finale text use shared measured
wrapping, fitting or truncation against their actual panel widths. Game-owned
line caps, presentation and story content remain local.

The existing TowerMapRng already delegates to the toolkit LegacyLcg64 with the
original increment. Its seed-zero policy, invalid ranges, chance semantics and
established map sequences are unchanged and covered by the existing fixture.
Content loading, persistence, audio and other previously adopted toolkit paths
remain in use; no additional generic infrastructure replacement was needed.

Validation: 182 game tests passed with none ignored; 405 all-feature toolkit
library tests passed. Formatting, all-target/all-feature Clippy with warnings
denied and the 800-line Rust source limit passed. The default publish.ps1 built
Windows and WebGL, deployed to Preview and updated Project Roost successfully.

Release captures cover long literal paths, capped tower status, the field guide,
finale and tower tutorial in docs/verification. Visual review corrected the help
panel's bottom padding and made capture scenes reset the field-guide overlay.
The literal-wrapper fixtures verify variable glyph widths, exact reconstruction,
empty strings and glyphs wider than their available line. Font glyph coverage
remains a property of the selected game font; preserving Unicode does not add
missing glyphs to that font. Obsolete character-budget tests were removed.
