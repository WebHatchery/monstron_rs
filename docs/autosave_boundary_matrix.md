# Hatchspire autosave boundary matrix

Audit date: 27 August 2026

`tests/autosave_boundaries.rs` exercises completed player-action boundaries through the native
atomic save-slot implementation. It uses an isolated, uniquely named application-data directory,
writes a save, reloads it from disk as though the process restarted, compares the full progression
state, and continues from that reloaded state.

The matrix covers:

1. a newly created camp;
2. a town resource action;
3. a facility improvement;
4. tower entry;
5. tower movement;
6. an unresolved combat turn, including replay history;
7. combat victory rewards and return to town; and
8. day advance and recovery.

Every replacement after the first also exercises creation of the previous-good backup slot. The
test cleans its isolated primary and backup files even when an assertion fails.

## What this proves

For each listed completed boundary, the full state can be written through the same native atomic
backup API used by autosave, loaded into a fresh value, and used by the next game system without
loss or repair. A companion structural assertion requires all ten mutating screen routes in the
top-level coordinator—including both placeholder transitions—to pass through the change-aware
`Game::apply_progression` wrapper. This guards the “action completes, autosave returns, process
ends” path and forces newly added progression routes to update the gate deliberately.

## What this does not prove

The matrix does not terminate the process during a filesystem call, kill power, simulate a full
disk, or exercise antivirus interference. Atomic write, backup, corrupt-save, and visible failure
handling have separate tests and recovery evidence. Manual force-close checks on the final packaged
build remain required before claiming that every possible OS interruption is safe.
