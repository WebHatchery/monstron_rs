# Hatchspire Windows Demo — Human-Required Checklist

Audit date: 26 August 2026

This file contains only work that requires a human’s identity, authority, independent judgment, physical access, or social participation. The companion `AI_AGENT_PC_DEMO_RELEASE_PLAN.md` covers implementation, testing automation, packaging, drafting, and technical preparation.

Assumed release: a Windows 10/11 x64 downloadable demo on itch.io. Steam, WebGL, Linux, and macOS are out of scope.

## Estimate and commitment

Use **12–16 weeks as the working schedule** and keep **20–24 weeks as the safe outside range**. The 3–6 month estimate is therefore reasonable for one part-time human owner working with an AI agent.

Before work begins, record:

- [ ] Target release month: ____________________
- [ ] Hours per week the human owner can reliably provide: ____________________
- [ ] Maximum cash budget for audio, art, testing, legal advice, and optional signing: ____________________
- [ ] One named release owner with final authority: ____________________
- [ ] A normal decision-response time of no more than two working days.

If the owner cannot provide at least 5–8 focused hours most weeks, use the six-month end of the estimate.

## 1. Make the product decisions an agent must not invent

- [ ] Approve the audience and content tone.
- [ ] Approve the demo’s purpose: festival/press pitch, public feedback build, mailing-list growth, or general discovery.
- [ ] Approve the promised playtime. Recommended: 60–120 minutes.
- [ ] Approve the playable boundary. Recommended: the opening town and floors 1–3, ending in an authored demo finale.
- [ ] Decide whether progress after the demo endpoint is locked, permitted with a warning, or retained for the eventual full game.
- [ ] Decide whether demo saves should migrate into a later full release. Do not promise migration casually.
- [ ] Approve which raising, breeding, building, and tower systems must be experienced before the ending.
- [ ] Approve the emotional promise: what should the player feel about Pip, the camp, and the tower by the end?
- [ ] Freeze nonessential features until after release.
- [ ] Define the minimum acceptable visual, audio, accessibility, stability, and balance bar.

Human decision record:

| Decision | Choice | Date | Owner |
| --- | --- | --- | --- |
| Demo endpoint |  |  |  |
| Target playtime |  |  |  |
| Save compatibility promise |  |  |  |
| Free / paid / pay-what-you-want |  |  |  |
| Public / unlisted / restricted test |  |  |  |
| Release month |  |  |  |

## 2. Own the game’s taste and identity

An agent can generate candidates and perform consistency checks. A human must decide what represents Hatchspire publicly.

- [ ] Play the current title, town, tower, combat, facilities, and ending-path builds personally.
- [ ] Choose the canonical monster designs, crops, palettes, and animation/readability tradeoffs.
- [ ] Reject visible generation defects, chroma leakage, nonsensical details, inconsistent scale, and imagery that does not fit the game.
- [ ] Approve the final logo, icon, cover, thumbnail, screenshots, trailer, and store-page ordering.
- [ ] Approve music and sound choices after listening on speakers and headphones.
- [ ] Decide whether generated assets are acceptable for this commercial/public release and public disclosure policy.
- [ ] Approve the final writing voice, names, tutorial tone, ending text, content warnings, and marketing claims.
- [ ] Confirm the combat screen is understandable at a glance; the current verification capture is not release-ready.

Do not delegate “looks good enough” to the same system that produced or implemented the candidates. Human taste is a release gate.

## 3. Supply and attest rights information

An agent can make a provenance ledger, scan dependencies, and draft notices. Only the rights holder can truthfully attest permission to publish.

- [ ] Identify the legal person or entity publishing Hatchspire.
- [ ] Confirm ownership or a valid commercial license for the title, logo, source code, generated images, edited images, fonts, sound effects, music, and marketing material.
- [ ] Provide source URLs, invoices, license texts, model/tool terms, commission agreements, and creator names where applicable.
- [ ] Resolve the repository metadata that currently points to `monstron_rs`; confirm whether any inherited Monstron material is authorized and whether the public link is correct.
- [ ] Decide the game’s license and whether the source repository is public.
- [ ] Review and approve credits and third-party notices.
- [ ] Review the dependency-license report and resolve anything unclear.
- [ ] Decide whether a privacy policy is required. If the demo has no analytics, accounts, network calls, or crash upload, approve a clear no-telemetry statement.
- [ ] Obtain qualified legal advice for any uncertain ownership, consumer-law, privacy, tax, trademark, or license question. An AI audit is not legal advice.

Release gate: no asset ships with “unknown,” “probably allowed,” or missing provenance.

## 4. Create and control the itch.io publisher identity

The agent must never impersonate the owner, accept contracts, expose credentials, or make the game public without explicit authorization.

- [ ] Create or select the itch.io account that will own the game.
- [ ] Secure the account with a unique password and available account-security controls.
- [ ] Create the Hatchspire project page manually; Butler cannot create it.
- [ ] Choose and provide the exact lowercase itch owner/project slug for `itch.json`.
- [ ] Install/authenticate Butler or authorize the itch app’s upload feature on the publishing machine.
- [ ] Keep Butler/API credentials secret and outside Git.
- [ ] Choose free, paid, or pay-what-you-want distribution.
- [ ] If accepting money, select the payment mode, accept seller terms, provide real tax identity information, select a payout method, and obtain accounting/tax advice appropriate to the owner’s jurisdiction.
- [ ] Configure page visibility and access: use Draft while building the page, then Restricted or unlisted Public for external release-candidate testing.
- [ ] Mark the uploaded file as Windows-compatible and verify the channel/file labels on the page.
- [ ] Configure comments/community, download access, release date, pricing, and any donation prompts.

Official itch.io references: [first project page](https://itch.io/docs/creators/getting-started.amp), [access modes](https://itch.io/docs/creators/access-control.amp), [Butler authentication](https://itch.io/docs/butler/login.html), [pushing builds](https://itch.io/docs/butler/pushing.html), and [payments/tax interview](https://itch.io/docs/creators/payments).

## 5. Recruit independent players and observe them

Automated tests cannot prove onboarding, fun, pacing, or emotional clarity. The developer playing their own game also does not count as an independent usability test.

- [ ] Recruit at least five people who did not build the game; eight to twelve is better.
- [ ] Include at least two players unfamiliar with monster-raising games and two familiar with the genre.
- [ ] Include at least one low-spec Windows PC and a mix of 1080p and smaller laptop displays.
- [ ] Give testers the ZIP and store-page instructions that public players will receive—no repository README and no live coaching.
- [ ] Observe the first session silently where consent and logistics allow.
- [ ] Record where players hesitate, misread, fail, become bored, or ask what to do.
- [ ] Ask neutral follow-up questions; do not explain the intended answer first.
- [ ] Collect consent before recording screens, voices, names, hardware details, or quotes.
- [ ] Keep personal data out of the repository unless there is a clear, consented reason.
- [ ] Run at least two rounds, with fixes between them.

Minimum human evidence before release:

| Measure | Target | Result |
| --- | ---: | ---: |
| Independent testers | 5 minimum |  |
| Reach first expedition without coaching | 4 of 5 |  |
| Complete the demo without coaching | 3 of 5 |  |
| Understand the town → raise → tower loop | 4 of 5 |  |
| Encounter a crash or progress loss | 0 |  |
| Would voluntarily play more | Record honestly; no forced pass mark |  |

## 6. Test on real Windows machines

An agent can produce a matrix and diagnose results, but it cannot claim tests on hardware it has not physically used.

- [ ] Download the exact processed itch build rather than copying a developer build.
- [ ] Test it on at least two clean Windows machines owned by different people.
- [ ] Include one modest integrated-GPU laptop if that is within the supported specification.
- [ ] Test mouse and keyboard, laptop scaling, multiple resolutions, fullscreen, resize, Alt+Tab, sleep/wake, and clean quit.
- [ ] Test from a path containing spaces and, if possible, non-ASCII characters.
- [ ] Test without Rust, developer tools, or the repository installed.
- [ ] Confirm save creation, restart/load, autosave recovery, New Game warning, and corrupt-save help.
- [ ] Scan the ZIP and executable with the owner’s normal security tools and note SmartScreen/antivirus behavior.
- [ ] Decide whether an unsigned ZIP is acceptable for this free itch demo. If signing is desired, the human must purchase/control the certificate and protect its private key.
- [ ] Verify headphones/speakers, mute, and all volume controls.
- [ ] Complete the entire demo from a fresh save on the release candidate.

Record machine, Windows version, GPU, display scale, result, tester, date, and build checksum for every physical test.

## 7. Approve the public promise

- [ ] Read every sentence on the itch page as a potential buyer/player.
- [ ] Confirm genre, demo length, platform, controls, accessibility claims, system requirements, save limitations, and planned/full-game language are accurate.
- [ ] Confirm screenshots and trailer footage come from the actual release candidate or are clearly labeled.
- [ ] Do not promise a full-release date, features, platforms, or save migration that have not been approved and resourced.
- [ ] Approve support contact/instructions and a sustainable response expectation.
- [ ] Approve any content warning and generated-content disclosure appropriate to the game and storefront rules.
- [ ] Review the final ZIP contents, version, checksum, release notes, credits, notices, known issues, and rollback build.

## 8. Make the go/no-go decision

The human release owner must personally verify each item:

- [ ] The demo starts cleanly, teaches itself, fulfills its stated playtime, and ends deliberately.
- [ ] The raising and tower loop is enjoyable enough to represent the future game.
- [ ] No blocker, crash, progress loss, illegible required UI, obvious art defect, or rights uncertainty remains.
- [ ] Outside playtest evidence meets the agreed bar.
- [ ] The exact itch download passes on the required physical PCs.
- [ ] The owner accepts the known issues that remain.
- [ ] The release page and payment/access settings are correct.
- [ ] A previous build can be restored if the release is bad.
- [ ] The owner has time to watch and respond during the first 48 hours.

Go/no-go record:

- Candidate version: ____________________
- Git commit: ____________________
- SHA-256: ____________________
- Decision: GO / NO-GO
- Decision maker: ____________________
- Date and time: ____________________
- Accepted known issues: ____________________

## 9. Perform the public action and communicate

- [ ] Explicitly authorize the agent to upload the approved candidate, or upload it personally.
- [ ] Personally change itch visibility/access to the approved public state.
- [ ] Confirm the page and download work while signed out or in a private browser session.
- [ ] Publish the approved release post and social/community messages.
- [ ] Answer player questions, moderation issues, refund/payment matters, press requests, and community judgment calls.
- [ ] Preserve honest reports even when feedback is uncomfortable.

Publishing is not complete when Butler says the upload succeeded. It is complete only after a human verifies the public page and its exact downloadable build as an unauthenticated player.

## 10. Own post-release priorities

- [ ] Monitor reports during the first 48 hours and again after seven days.
- [ ] Decide whether each finding is a hotfix, normal patch, post-demo backlog item, or declined request.
- [ ] Authorize rollback if the public build crashes, loses saves, cannot launch, or exposes a serious rights/security issue.
- [ ] Approve any patch that changes balance, presentation, scope, store claims, or save compatibility.
- [ ] Thank testers and obtain permission before using quotes publicly.
- [ ] At day 7, decide whether to keep promoting, pause for fixes, or reduce access.
- [ ] At day 30, decide whether evidence supports continuing toward 1.0 and whether the 5–8 month full-release estimate should be revised.

## Human blockers that extend the schedule

Any of these can turn a three-month plan into six months or more:

- delayed scope decisions or repeated reopening of frozen features;
- no reliable independent testers;
- unresolved ownership or licensing of generated art, logo, music, or inherited prototype material;
- slow approval of visual/audio candidates;
- no access to a second clean Windows machine;
- an uncreated or unauthenticated itch.io page;
- choosing Steam, paid distribution, localization, or additional platforms mid-plan;
- promising all ten floors rather than shipping a curated demo slice;
- insufficient weekly time for the owner to play current builds and answer decisions.

The best schedule protection is a small, explicit demo promise and fast human feedback—not asking the agent to add more systems.
