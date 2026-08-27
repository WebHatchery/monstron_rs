# Third-party notices — incomplete release draft

Hatchspire uses Rust libraries including macroquad and macroquad-toolkit. The Windows preview
packager generates `docs/THIRD_PARTY_COMPONENTS.txt` from the exact Cargo dependency graph and
records each crate version and Cargo license expression. It also bundles every top-level license,
notice, copying, or unlicense file present in each registry crate under `docs/licenses/`.

That generated component list and license bundle are audit aids, not legal approval. The current
`gilrs 0.10.10` and `gilrs-core 0.5.15` packages omit their workspace-root license files from the
cached registry packages. Hatchspire therefore tracks hash-verified copies from each exact upstream
tag and the packager adds their identical Apache-2.0 and MIT texts to both dependency directories.
`quad-rand 0.2.3` declares MIT but neither its cached crate nor its declared repository contains a
license file. Hatchspire and the internal macroquad-toolkit also do not currently declare license
metadata in their Cargo manifests. Required notice selection and copyright attribution have not
received human review. These are release blockers, not implied permissions.

The publisher must review the exact locked dependency graph, preserve all required notices, resolve
missing project/toolkit licensing, and obtain qualified advice for any ambiguity before public
distribution. This document is not legal advice.
