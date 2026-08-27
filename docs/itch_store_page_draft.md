# Hatchspire itch.io page draft

Status: **PROPOSED COPY — DO NOT PUBLISH WITHOUT HUMAN APPROVAL**  
Audience: Windows 10/11 x64 downloadable-demo page  
Technical tokens are filled by `scripts/create_storefront_review_packet.ps1`.

This draft deliberately separates current facts from promises that depend on the approved demo
scope. Text inside `[[HUMAN APPROVAL REQUIRED: ...]]` must be resolved, not silently deleted.

## Page settings worksheet

| itch.io field | Proposed value | Gate |
| --- | --- | --- |
| Title | Hatchspire | Confirm title and trademark risk |
| Kind of project | Downloadable | Windows ZIP only; do not select HTML5 |
| Classification | Game | Human confirmation |
| Release status | In development until the public-demo GO decision | Human confirmation |
| Platforms | Windows | Two clean physical-PC passes required first |
| Visibility during preparation | Draft | Owner controls account and visibility |
| External candidate test | Restricted, or Public with Unlisted in search & browse | Owner chooses access model |
| Public visibility | Public | Only after explicit GO |
| Pricing | [[HUMAN APPROVAL REQUIRED: free, paid, or pay-what-you-want]] | Owner/tax/payment decision |
| Upload | `{{ZIP_FILENAME}}` | Mark Windows-compatible; confirm whether to mark as a free demo |
| Butler channel | `windows-demo` | Requires approved lowercase owner/project slug |
| Version | `{{BUILD_ID}}` | Candidate technical identity, not marketing version approval |

Official itch.io guidance says a project starts private, supports Draft, Restricted, and Public
access modes, allows up to ten tags, recommends a 315:250 cover ratio (for example 630×500), and
recommends three to five screenshots. It also requires accurate platform and generative-AI metadata.
See [first-page setup](https://itch.io/docs/creators/getting-started),
[access control](https://itch.io/docs/creators/access-control), and
[quality guidelines](https://itch.io/docs/creators/quality-guidelines).

## Short description

Raise tower-born monsters, rebuild a struggling camp, and plan careful expeditions into a ruined
living spire.

Human approval: [[HUMAN APPROVAL REQUIRED: approve or replace this short description]]

## Long description

Hatchspire is a monster-raising town RPG about rebuilding a camp beneath a ruined tower. Care for
tower-born companions, prepare a balanced expedition party, and decide how far to press into the
spire before fatigue and danger force a retreat.

Each trip brings back materials, discoveries, and eggs that help the camp recover. Improve its
facilities, hatch new companions, and learn how each monster's traits shape combat and exploration.

### Proposed public-demo promise

[[HUMAN APPROVAL REQUIRED: only publish the following paragraph after the scoped tutorial, floor-3
finale, results screen, and boundary tests are implemented and approved.]]

The proposed demo follows the opening camp and the first three tower floors through an authored
finale. A first playthrough is intended to take **[[HUMAN APPROVAL REQUIRED: approved playtime]]**.
Progress beyond the ending and compatibility with a later full release follow the approved save and
endpoint decisions; do not promise either here until those decisions are recorded.

### Features

- Raise, care for, recover, and hatch tower-born monsters.
- Rebuild camp facilities with resources recovered from expeditions.
- Plan parties around companion roles, traits, fatigue, and injuries.
- Explore a changing tower map with hazards, landmarks, secrets, camps, and tactical retreats.
- Fight deterministic turn-based battles with visible targets, queued enemy intent, status effects,
  and recovery consequences.
- Play every required action with visible mouse/touch controls; keyboard shortcuts are optional.
- Keep progress locally with autosave, one-step backup recovery, and no account requirement.

Human approval: [[HUMAN APPROVAL REQUIRED: verify every feature against the final scoped build]]

## Download and installation

1. Download `{{ZIP_FILENAME}}`.
2. Extract the entire ZIP to a writable folder.
3. Open the extracted folder and run `hatchspire.exe`.

Do not run the executable from inside the ZIP. Windows may display an unsigned-app warning because
the current candidate has no code signature.

Exact candidate: `{{BUILD_ID}}`  
ZIP size: {{ZIP_MIB}} MiB ({{ZIP_BYTES}} bytes)  
ZIP SHA-256: `{{ZIP_SHA256}}`

## Controls

- Mouse/touch: every required action has a visible target.
- Main menu: choose New Game, Continue, Settings, or Save Options.
- Town: select camp actions, facilities, NPCs, or Enter Tower.
- Tower: select a room, then use ROUTE, EXPLORE, SURVEY, CAMP, RETREAT, or FIELD GUIDE.
- Combat: use ATTACK, SKILL, DEFEND, HERBS, or FLEE.
- Camp menu: use Save, Load, Title, or Exit Game.
- Keyboard: optional shortcuts supplement the visible controls.

## Proposed system requirements

Only the operating-system target is currently substantiated. Do not invent CPU, RAM, GPU, storage,
or display minima before the required physical-PC and modest integrated-GPU tests are complete.

| Requirement | Minimum | Recommended |
| --- | --- | --- |
| OS | Windows 10/11 x64 | Windows 10/11 x64 |
| CPU | [[HUMAN TEST EVIDENCE REQUIRED]] | [[HUMAN TEST EVIDENCE REQUIRED]] |
| Memory | [[HUMAN TEST EVIDENCE REQUIRED]] | [[HUMAN TEST EVIDENCE REQUIRED]] |
| Graphics | [[HUMAN TEST EVIDENCE REQUIRED]] | [[HUMAN TEST EVIDENCE REQUIRED]] |
| Storage | At least {{ZIP_MIB}} MiB for the download, plus extracted files and saves | [[HUMAN TEST EVIDENCE REQUIRED]] |
| Display | [[HUMAN TEST EVIDENCE REQUIRED]] | 1280×720 or larger is the current design target |

## Accessibility and limitations

- Required interactions use visible mouse/touch targets; keyboard-only play is not required.
- Windowed UI scale, fullscreen, reduced motion, and master/music/SFX settings persist locally.
- [[HUMAN APPROVAL REQUIRED: describe final audio, color, text-size, remapping, and accessibility
  support without claiming features that are absent.]]
- One local save slot; no cloud saves, accounts, online multiplayer, achievements, or automatic
  crash upload.
- [[HUMAN APPROVAL REQUIRED: approved demo-save migration and post-ending statement.]]

## Privacy and support

The Windows game does not send analytics, telemetry, saves, settings, or crash reports over the
network. It writes saves, settings, optional tester summaries, and crash logs locally. Files leave
the computer only when the player chooses to share them manually.

Support: [[HUMAN APPROVAL REQUIRED: public support address or form and response expectation]]

## Content and generated-material disclosures

- Fantasy monster combat and expedition danger.
- [[HUMAN APPROVAL REQUIRED: final content warnings after complete human playthrough.]]
- [[HUMAN APPROVAL REQUIRED: accurate itch.io generative-AI metadata and public disclosure text.]]

## Candidate tags

Select no more than ten accurate tags in the live itch.io metadata UI. Proposed concepts to verify:
Monster Taming, Turn-Based Combat, RPG, Management, Strategy, Fantasy, Singleplayer, 2D, Atmospheric,
and Mouse Only. Do not add a tag merely for reach, and do not treat this wording as proof that the
live site offers the exact tag.

## Media worksheet

- Cover: [[HUMAN APPROVAL REQUIRED: approved 630×500 image with 315:250 ratio]]
- Screenshots: [[HUMAN APPROVAL REQUIRED: choose three to five exact-candidate images from the
  candidate-stamped visual-review packet after its review passes]]
- Trailer: [[HUMAN APPROVAL REQUIRED: omit or approve an exact-candidate trailer]]
- Screenshot ordering and captions: [[HUMAN APPROVAL REQUIRED]]

## Known-issues section

Publish only issues that remain true for the approved candidate. The current internal preview still
lacks first-session onboarding, an authored demo finale/boundary, approved audio, final rights and
visual approval, independent playtests, clean-machine evidence, a public support route, and signing.
Those are release gates, not acceptable public-demo footnotes.

Accepted public known issues: [[HUMAN APPROVAL REQUIRED: issue IDs and plain-language impact]]

## FAQ

### What is Hatchspire?

A monster-raising town RPG about rebuilding a camp and preparing companions for expeditions into a
ruined living tower.

### What platforms are supported?

The proposed demo target is Windows 10/11 x64. WebGL is development-only; Linux, macOS, Steam, and
other storefronts are outside this release.

### How long is the demo?

[[HUMAN APPROVAL REQUIRED: publish only the measured, approved playtime range.]]

### Can I continue my save in the full game?

[[HUMAN APPROVAL REQUIRED: state the approved compatibility promise; the safe default is no promise.]]

### Does the game collect analytics?

The current Windows binary sends no analytics or telemetry. It has no accounts and uploads no saves
or crash reports automatically.

### Where are saves stored?

Open **Settings → Help & Support** in the game to see the exact local saved-camp path. The same screen
explains backup recovery and optional manual report files.

### How do I report a problem?

[[HUMAN APPROVAL REQUIRED: public support route.]] Include the build shown in Help, what happened,
what you selected immediately beforehand, and Windows/display details. Share logs or saves only when
comfortable and only through the approved route.
