# Hatchspire Windows preview — known issues

This is an internal development build, not the approved public demo.

- The first-expedition and first-egg guides are implemented, but their pacing and clarity still
  need independent tester validation.
- Floors 1–3 do not yet end in an authored demo finale or progression boundary.
- Original procedural music and sound effects now ship through the shared runtime and obey the
  existing audio settings, but their final listening and mix approval is pending.
- Combat has deterministic four-size clarity captures, but final human visual review is pending.
- Gamepad support is not implemented.
- The executable has verified Windows version/product fields and a custom icon derived from the
  title art. Rights approval for that art and a code signature remain absent, so Windows may show
  an unsigned-app warning.
- The build uses one local save slot and has no cloud save or account support.
- Version `0.1.0` is a development placeholder, not an approved demo version. The `+g<commit>`
  suffix identifies the exact build for testing.
- Final visual-asset provenance, dependency notices, credits, and publisher rights approval are
  incomplete. This build must not be distributed publicly.

Report any crash, hang, progress loss, missing asset, unreadable required control, or recovery
failure immediately through the private test channel.
