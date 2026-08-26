# Third-party notices — incomplete release draft

Hatchspire uses Rust libraries including macroquad and macroquad-toolkit. The Windows preview
packager generates `docs/THIRD_PARTY_COMPONENTS.txt` from the exact Cargo dependency graph and
records each crate version and Cargo license expression.

That generated component list is an audit aid, not a complete notice bundle. Required copyright
notices and full license texts have not yet been assembled or reviewed. Hatchspire and the internal
macroquad-toolkit also do not currently declare license metadata in their Cargo manifests. These are
release blockers, not implied permissions.

The publisher must review the exact locked dependency graph, preserve all required notices, resolve
missing project/toolkit licensing, and obtain qualified advice for any ambiguity before public
distribution. This document is not legal advice.
