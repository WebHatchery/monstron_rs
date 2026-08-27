# Hatchspire independent playtest packet

Audit date: 27 August 2026

The local `tester_summary.txt` records saved pacing and balance counters, but it cannot establish
whether a tester is independent, whether coaching occurred, whether the town → raise → tower loop
was understood, or whether observations were collected with consent. Those are human evidence.

After sealing a clean candidate and generating its storefront review packet, run:

```powershell
.\scripts\create_playtest_packet.ps1
```

The ignored packet under `target/playtest-packet/<full-commit>/` contains:

- `CANDIDATE.json`, with exact package and storefront-copy identities;
- `OBSERVER_GUIDE.md`, with consent boundaries and a neutral no-coaching protocol;
- `SESSION_RECORD_TEMPLATE.md`, for human notes outside the repository;
- `SESSION_TEMPLATE.json`, an anonymous structured record for mechanical aggregation;
- `COHORT_RECORD.md`, a blank owner worksheet; and
- `sessions/`, where completed JSON copies may be placed locally.

Do not put names, email addresses, contact details, recordings, quotes, hardware identifiers, saves,
logs, or free-form observation notes in session JSON. Use an anonymous ID such as `T01`. Keep any
identity-to-ID key and consent records outside Git under the human owner’s control.

After placing completed JSON records in `sessions/`, run:

```powershell
.\scripts\summarize_playtest_cohort.ps1
```

The summarizer verifies every record against the exact packet, excludes records without explicit
anonymized-results consent, independence, or observer attestation, deduplicates people by anonymous
tester ID, and calculates these final-candidate targets:

- at least five eligible independent testers;
- at least two unfamiliar and two familiar with monster-raising games;
- at least four reach the first completed expedition without coaching;
- at least three complete the approved demo without coaching;
- at least four understand the town → raise → tower loop; and
- zero report a crash or progress loss.

“Would play more” is reported honestly with no forced pass threshold. The generated cohort status is
`targets_met`, `targets_not_met`, or `insufficient_evidence`; none grants release approval. Multiple
playtest rounds with fixes require separate exact-candidate packets and a human programme record.

The packet must not be given to testers until its unresolved storefront instructions, scope, support
route, consent process, and test route are approved. A developer playing their own build does not
count as an independent tester.
