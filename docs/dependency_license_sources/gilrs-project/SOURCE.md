# GilRs supplemental license source record

Audit date: 27 August 2026

The crates.io packages for `gilrs 0.10.10` and `gilrs-core 0.5.15` declare
`Apache-2.0/MIT` but omit their workspace-root license files. The two tracked files here were copied
verbatim from both matching tags in the repository declared by their Cargo metadata:

- `https://gitlab.com/gilrs-project/gilrs/-/raw/v0.10.10/LICENSE-APACHE`
- `https://gitlab.com/gilrs-project/gilrs/-/raw/v0.10.10/LICENSE-MIT`
- `https://gitlab.com/gilrs-project/gilrs/-/raw/gilrs-core-v0.5.15/LICENSE-APACHE`
- `https://gitlab.com/gilrs-project/gilrs/-/raw/gilrs-core-v0.5.15/LICENSE-MIT`

The two tags supplied byte-identical files:

| File | SHA-256 |
| --- | --- |
| `LICENSE-APACHE` | `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2` |
| `LICENSE-MIT` | `74497fa3c93ebb5ec32d85de117469cb3fab3276f7c02426f14c0f19f31f4424` |

The packager verifies these hashes before copying the texts into both exact dependency directories.
This source record resolves missing-file collection only; it is not legal approval or a decision
about which dual-license option the publisher should exercise.
