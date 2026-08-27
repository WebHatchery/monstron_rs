# Hatchspire physical Windows test record

Audit date: 27 August 2026

Physical-machine evidence must identify the exact sealed ZIP. A successful developer-host smoke run,
render capture, Defender scan, or automated window probe does not count as a clean-PC result.

After packaging a clean candidate, generate a blank, candidate-stamped evidence packet with:

```powershell
.\scripts\create_physical_test_packet.ps1
```

The script verifies the external manifest and checksum sidecar, rehashes the executable inside the
ZIP, records its Authenticode state, and writes these ignored files under
`target/physical-windows-test-packet/<full-commit>/`:

- `CANDIDATE.json` — machine-readable package identity and signature state;
- `PHYSICAL_WINDOWS_TEST_RECORD.md` — blank human test record containing the expected checksum.

Copy the Markdown record somewhere appropriate for the test programme and fill it in there. Do not
commit tester names, hardware identifiers, recordings, contact details, or other personal data.

## Evidence acceptance rules

- Test the build downloaded from the restricted/unlisted distribution channel, not a developer copy.
- Record the downloaded ZIP's observed SHA-256 and require it to equal the prefilled expected hash.
- Use at least two clean Windows machines owned by different people; include a modest integrated-GPU
  laptop when that is within the approved supported specification.
- Test without Rust, developer tools, or this repository installed.
- Record failures and issue IDs honestly. Blank, skipped, coached, or unobserved checks do not pass.
- A full visible two-to-four-hour play/device soak requires start/end times and a human observer; the
  automated render-soak record may support it but cannot replace it.
- SmartScreen reputation, other antivirus results, audio judgment, visual judgment, demo completion,
  and the unsigned-build decision remain human evidence.
- Only the named release owner may accept an important issue or sign the final GO decision.

Generating a packet creates no physical-test evidence and grants no release approval. Its initial
status is always `awaiting_human_evidence`.
