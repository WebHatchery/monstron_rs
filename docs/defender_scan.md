# Hatchspire exact-package Microsoft Defender scan

Audit date: 27 August 2026

`scripts/defender_scan.ps1` records a non-remediating Microsoft Defender custom scan of the exact
sealed Windows ZIP and its extracted executable. Before scanning, it verifies the archive byte size
and SHA-256 against the external manifest and checksum sidecar, requires clean Hatchspire and shared-
toolkit source identities, extracts exactly one `hatchspire.exe`, and rehashes it against the manifest.

Run it after sealing a clean package:

```powershell
.\scripts\defender_scan.ps1
```

The gate requires Defender antivirus and real-time protection to be active. Signatures must be no
more than three days old by default; `-MaxSignatureAgeDays` can narrow that window. Both scans use
Defender's custom-scan `-DisableRemediation` mode, which ignores exclusions and scans archives while
avoiding quarantine, event-log, and Windows Security UI changes. Any nonzero scan result fails.

The ignored `target/defender-scan/summary.json` records the exact build, toolkit, archive and EXE
hashes, Defender engine/signature versions and update time, scanner version, timestamps, exit codes,
and console results. The temporary extracted executable is removed after the scan.

This result is evidence only for the installed Defender engine and signatures on the development
host at that moment. It does not establish SmartScreen reputation, code-signing trust, behavior under
another antivirus product, a clean Windows installation, or results on either required physical test
machine. Those remain human release gates.
