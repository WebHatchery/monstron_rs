# Windows preview rollback and first-patch procedure

Audit date: 27 August 2026

This procedure applies to internal/restricted Windows ZIP candidates. It does not authorize a
public upload and does not replace itch.io authentication, restricted-download, or clean-machine
checks.

## Before deploying a patch

1. Complete the clean publish → preview-package → release-smoke → deploy-only sequence.
2. Run `scripts/preserve_windows_preview.ps1`.
3. Keep the printed commit and SHA-256 in the release record. The script stores the exact ZIP,
   external manifest, and checksum under `dist/history/<full-commit>`.
4. Build, package, smoke, and deploy the patch candidate from a new clean commit.
5. Run `scripts/rehearse_windows_rollback.ps1 -PreservedDir <preserved-directory>` before asking a
   human to upload either build.

Preservation is non-destructive and idempotent. It refuses dirty candidates, mismatched hashes,
invalid commits, and replacement of different historical evidence.

## Restricted-channel rollback

1. Pause new tester invitations and record the reason for rollback.
2. Select the preserved directory by its full commit, not by a mutable filename or “latest” label.
3. Verify the preserved ZIP against both its external manifest and `.sha256` sidecar.
4. Upload that exact ZIP to the existing restricted Windows channel without rebuilding it.
5. Download the processed file, verify its SHA-256, launch it on a clean machine, and record the
   result. These external account/device steps remain human gates.
6. Keep the rejected patch and its evidence; do not overwrite the forensic record.

## First-patch restoration

After the patch defect is resolved, repeat the full clean candidate sequence. Rehearse switching
from the patch to the preserved build and back to the patch, then upload the exact verified patch
ZIP. The local rehearsal checks both external hashes and embedded build identities at every switch
and leaves `target/release-rollback-rehearsal.json` as machine-readable evidence.

The rehearsal uses an isolated temporary directory under `target` and removes it after success or
failure. It does not mutate the deployed local preview or any remote channel.

## First completed local rehearsal

On 27 August 2026, the isolated sequence passed with:

- patch `0.1.0+g597b73d06168`, commit `597b73d06168f5d0cdb7053a5cebca003555f69b`,
  SHA-256 `5ffbab3741297cef7f5ce016c8232491d5cc40ef984d6cc4ac35f94e0127f0aa`;
- rollback `0.1.0+g6a7cacd701e7`, commit `6a7cacd701e7fa7d07d3d3923ecb8b9eb293632f`,
  SHA-256 `179e474cb72ae04cfa5d8fd6312aa238551fdb22ef86788faf973b2ffd242567`;
  and
- final state restored to the patch hash after the rollback hash and both embedded identities passed.

Both candidates remain preserved under commit-addressed ignored `dist/history` directories on this
workstation. This proves the local artifact-switching procedure, not remote itch processing or a
clean-machine launch.
