Author: AI Agent (Claude Code)
Date: 2026-11-11
Task: TASK-010
Status: Completed

# TASK-010: Align Release And Maintainer Docs With Package Status

## Objective

Make `docs/MAINTAINING.md`, `docs/RELEASING.md`, and root `RELEASING.md` agree with the channel
statuses in `README.md` and the procedures in `docs/PACKAGING.md` (TASK-009), so no release or
maintainer step treats an unpublished channel as published or the tap as the supported channel.

## Background

TASK-009 aligned `docs/PACKAGING.md` and `Formula/README.md` with README and consolidated the
remaining contradictions in its `## Documentation Updates` section ("Follow-up task"). The
maintainer approved this docs part as TASK-010 (2026-11-11, "Yes, TASK-010").

Sources of truth: README Package Status table; `docs/PACKAGING.md` as committed in `c2e201a`;
maintainer commit `c40983a` (Homebrew Planned for homebrew-core, build from source).

Live facts as recorded in TASK-009 (checked 2026-11-11, not re-fetched):

| Item | Fact |
|---|---|
| `AMRSKH/homebrew-tap` | Exists, not archived; formula at v1.1.1; CI does not update it |
| WinGet PR `microsoft/winget-pkgs#409790` | Closed, not merged; package not in `microsoft/winget-pkgs` |
| GitHub latest release | `v1.2.1` with 9 assets: 6 archives (incl. musl), `.deb`, `.rpm`, `checksums.txt` |
| crates.io `angkorfetch` | Latest published version 1.1.1; no `cargo publish` in CI |

Items to resolve (line numbers are for HEAD `4daf6b9`; see TASK-009 for detail):

1. Tap presented as canonical or as a release step: root `RELEASING.md:85`, `:107`, `:150`,
   `:391-393` (the target of the link at `Formula/README.md:39`); `docs/MAINTAINING.md:53`, `:127`.
2. `wingetcreate update` for a package not in `microsoft/winget-pkgs`:
   `docs/MAINTAINING.md:120-123`, root `RELEASING.md:149`. `copr-cli build-package ... --clone-url`
   (`docs/MAINTAINING.md:133`) is unverified.
3. Release steps written as if every channel is published: root `RELEASING.md:105-110`,
   `:148-156`; `docs/MAINTAINING.md:130-146`.
4. `brew audit` presented as enforced: `docs/RELEASING.md:41`; root `RELEASING.md:376-379`.
5. crates.io missing from the release steps: root `RELEASING.md` steps 6-7, `docs/MAINTAINING.md`
   section 4.
6. Stale artifact counts: `docs/RELEASING.md:26`, `:35`; root `RELEASING.md:95`;
   `docs/MAINTAINING.md:102` and root `RELEASING.md:70` ("5 targets", from TASK-008).

## Requirements

- Channel statuses and commands in the three files agree with README and `docs/PACKAGING.md`.
  README and `docs/PACKAGING.md` do not change.
- Steps for unpublished channels are labelled as applying once the channel is published, and first
  submissions point to `docs/PACKAGING.md` rather than duplicating it.
- The tap may be described as existing, but not as canonical, user-facing, or a required release
  step. Whether it should be retired is a maintainer decision and is not made here.
- Artifact and target counts match `.github/workflows/release.yml`; verify against the workflow.
- CI behaviour is described as it is: the formula check is a generator-shape check, not `brew audit`.
- crates.io appears as a manual release step that needs maintainer approval.
- Correct release procedures stay intact.

## Scope

- `docs/MAINTAINING.md`
- `docs/RELEASING.md`
- `RELEASING.md` (repository root)

## Out of Scope

- `README.md`, `docs/PACKAGING.md`, `Formula/README.md`.
- `Formula/angkorfetch.rb`, `winget-pkgs/`, `linux/`, `flatpak/`, `snap/`, scripts, workflows, and
  any SHA256 value (AGENTS.md section 7).
- TASK-009 follow-up items 7-13 (pins, `build-rpm.sh`, PKGBUILD checksum, Flatpak manifest,
  homebrew-core formula, external actions) and the optional `AGENTS.md` / `docs/DEVELOPMENT.md`
  wording.
- Publishing, tap updates, WinGet resubmission, merging the sync branch (external actions).

## Assigned Roles

- Advisor: `advisor` subagent (Claude Code).
- Dev: `dev` subagent (Claude Code).
- QA Review: `qa-review` subagent (Claude Code), independent of Dev.
- Docs: `docs` subagent (Claude Code).
- Orchestration, final validation, commits, and the final report: main session.

## Implementation Plan

Advisor, 2026-11-11. Repository facts were read on this branch (`claude/awesome-pascal-f6d9aa`,
HEAD `a86d6bf`). The three files in scope have not changed since `4daf6b9`, so the line numbers in
Background and below are current. Live facts come from Background and were not re-fetched.

### Approach

| Option | Result | Verdict |
|---|---|---|
| A. Patch only the lines named in Background | Small diff, but leaves wrong text the six items cover: nine more stale counts (root `:13`, `:79`, `:166`, `:206`, `:219`; `docs/RELEASING.md:12`; `docs/MAINTAINING.md:44`, `:104`, `:105`), the asset lists without the musl archive, and root `:3-4` ("Everything below is automated except...") | Rejected |
| **B. Section-by-section correction** of all six items, wherever they occur; every other procedure and the current structure kept; first submissions link to `docs/PACKAGING.md` | The three files agree with README, `docs/PACKAGING.md`, and `release.yml`; correct procedures stay | **Recommended** |
| C. Merge root `RELEASING.md` and `docs/RELEASING.md`, or replace `docs/MAINTAINING.md` section 4 with a link | Removes the duplicated diagram and asset list, but restructures the docs and removes per-release steps that remain valid once channels are published | Rejected (duplication recorded below, not merged) |

Rules for Dev:

- Edit only `docs/MAINTAINING.md`, `docs/RELEASING.md`, root `RELEASING.md` (plus this task file).
  All three are `i/lf w/crlf`: keep CRLF in the working tree, including every new line.
- "Keep" means byte-identical. Do not reword anything this plan does not name. Root `RELEASING.md`
  has em dashes and box-drawing characters in kept lines; leave them.
- No heading changes. These anchors must survive: `#why-the-formula-lives-in-formula`
  (`Formula/README.md:39`), `#cutting-a-release` (`docs/PACKAGING.md:138`, root `:3` after this
  plan), `#why-package-definitions-lag` (root `:126`). `git grep` found no other anchor link into
  the three files. `sync-packages.yml:266` says "See RELEASING.md" (file name only).
- Keep the step numbers of root "Cutting a release"; `docs/PACKAGING.md:138` cites step 1.
- Link to `docs/PACKAGING.md` by file name plus "section N", without anchors.
- Status words are copied from README lines 54-61: Live, Built, Pending, Planned (homebrew-core),
  Planned.
- Line numbers are guides. Match on the old text.

### Counts, derived from `release.yml`

| Item | Count | Source |
|---|---|---|
| Build targets | **6** | `build` matrix, lines 138-157 (gnu x86_64, musl x86_64, gnu aarch64, macOS x86_64, macOS aarch64, Windows x86_64) |
| Files hashed in `checksums.txt` | **8** | 6 archives + `.deb` + `.rpm`: `checksums` job, `sha256sum *` (line 303), run before `checksums.txt` exists in that folder |
| Assets attached to the release | **9** | 8 + `checksums.txt`: `release` job, lines 332-346 (7 names + 2 patterns), `files: artifacts/*` (line 354). Matches the v1.2.1 asset list in Background |
| Assets if `.deb` and `.rpm` are dropped | **7** | 9 - 2 (root `:219` said "Six", from 8 - 2) |

Count conventions: both `RELEASING.md` files use the same convention (a total that includes
`checksums.txt`, formerly 8); only the noun differs (`docs/RELEASING.md` "artifacts", root
"assets"). `docs/MAINTAINING.md:104-105` separates hashed (7) from attached (8), which is also
consistent. All are one target short because the musl build was added without updating them. The
right statement everywhere: **6 targets; 9 assets attached; `checksums.txt` covers the other 8**.

### docs/RELEASING.md

Lines 1-11, 13-25, 36-40: keep.

| Line | Old | New |
|---|---|---|
| 12 | `build (5 targets)` | `build (6 targets)` (same width, the box stays aligned; rest of the line kept) |
| 26 | `Every release produces 8 verifiable artifacts:` | `Every release produces 9 verifiable artifacts:` |

Replace lines 28-35 with:

```markdown
1. `angkorfetch-linux-x86_64.tar.gz`: Linux x86_64 binary (glibc)
2. `angkorfetch-linux-x86_64-musl.tar.gz`: Linux x86_64 static binary (musl, cross-compiled)
3. `angkorfetch-linux-aarch64.tar.gz`: Linux ARM64 binary (cross-compiled)
4. `angkorfetch-macos-x86_64.tar.gz`: macOS Intel binary
5. `angkorfetch-macos-aarch64.tar.gz`: macOS Apple Silicon binary
6. `angkorfetch-windows-x86_64.zip`: Windows x86_64 executable
7. `angkorfetch_<version>_amd64.deb`: Debian/Ubuntu deb package
8. `angkorfetch-<version>-1.x86_64.rpm`: Fedora/RHEL rpm package
9. `checksums.txt`: SHA256 checksums of the 8 files above
```

Reason for line 1: the gnu target is not static (no `crt-static` and no `.cargo/` config in the
repository); "static" belongs to the musl row now that both are listed.

Line 41, change only the middle clause. Old: `Preserves line endings, enforces `brew audit --strict` standards, and fails on missing artifacts.`
New:

```markdown
Preserves line endings, renders the formula from a template written to pass `brew audit --strict`, and fails on missing artifacts. CI does not run `brew audit`; `--check` only confirms the committed formula matches the generator.
```

Item 4. The part of the bullet before `Preserves` is kept.

### Root RELEASING.md

**Lines 3-4 (item 3).** Old: "Reference for maintainers cutting a release. Everything below is automated except
the version bump and the two pull request merges." New:

```markdown
Reference for maintainers cutting a release. The version bump, the tag, the two pull
request merges, and the external channel steps ([Cutting a release](#cutting-a-release),
steps 6 and 7) are manual; the rest is automated.
```

**Counts (item 6).**

| Line | Old | New |
|---|---|---|
| 13 | `build (5 targets)` | `build (6 targets)` (rest of the line kept) |
| 70 | `Release build (5 cross-platform targets)` | `Release build (6 cross-platform targets)` |
| 79 | `GitHub Release (attached 8 assets)` | `GitHub Release (attached 9 assets)` |
| 95 | `  - Cross-compilation for 5 target binaries` | `  - Cross-compilation for 6 target binaries` |
| 166 | Jobs table, `build` row, cell `5 targets` | Cell `6 targets` (rest of the row kept) |
| 206 | `A correct release publishes 8 assets:` | `A correct release publishes 9 assets:` |
| 209, new line after it | | `angkorfetch-linux-x86_64-musl.tar.gz` (position matches `release.yml`) |
| 219 | `Six assets means the packages were dropped` | `Seven assets means the packages were dropped` (rest of the line and line 220 kept) |

**Line 85, flow diagram (items 1, 3, 5).** New:

```text
External package publication (crates.io; once each is published: WinGet, homebrew-core, Copr, AUR, Flathub)
```

**Lines 105-110, Responsibility Matrix (items 1, 3, 5).** Keep line 105; replace lines 106-110 with:

```markdown
  - Publishing the crate to crates.io (manual `cargo publish`, needs maintainer approval)
  - Once each channel is published (first submissions: [docs/PACKAGING.md](docs/PACKAGING.md)):
    - Submitting updated WinGet manifest to `microsoft/winget-pkgs`
    - Triggering Copr build for Fedora DNF
    - Submitting updated `PKGBUILD` to AUR for Arch Linux
    - Opening PR on Flathub repository
  - Homebrew has no release step until the planned homebrew-core formula exists
```

**Lines 148-156, Cutting a release, steps 6-7 (items 1, 2, 3, 5).** Replace with:

```markdown
6. **External Package Submissions** (manual; statuses are in the [Package Status table](README.md#package-status--availability), and first submissions are in [docs/PACKAGING.md](docs/PACKAGING.md)):
   - **crates.io** (Live): Run `cargo publish --locked` from the tagged commit. No workflow does this, and it needs maintainer approval. See [docs/PACKAGING.md](docs/PACKAGING.md), section 8.
   - **WinGet** (Pending): The package is not in `microsoft/winget-pkgs` yet, so resubmit it as a new package with `wingetcreate submit`. Use `wingetcreate update` only after the package has been merged. See [docs/PACKAGING.md](docs/PACKAGING.md), section 1.
   - **Homebrew** (Planned, homebrew-core): No release step yet. `AMRSKH/homebrew-tap` is not the documented install path, and updating it is not part of a release.
   - **Fedora (DNF)** (Built; the `.rpm` is already attached to the release): Build in Copr `amrskh/angkorfetch` once that project exists.
   - **Arch Linux** (Planned): Once the package is on the AUR, publish the updated `linux/arch/PKGBUILD` there.
   - **Flatpak** (Planned): Once the app is on Flathub, submit each update there.

7. **Verify installation & documentation**:
   Verify each published channel resolves the new version (the GitHub Release files and quick installers, plus crates.io after step 6) and update status tables accordingly.
```

**Lines 375-379 (item 4).** Keep lines 375-377 up to "depend". Replace the end of the paragraph, old
"on, so a formula that would fail audit blocks the tag rather than surfacing after
publication." with:

```markdown
on, so a formula that differs from the generator's output blocks the release rather
than surfacing after publication. CI does not run `brew audit`: the template is
written to pass it, and the sync pull request asks the reviewer to run
`brew audit --strict` on macOS if available.
```

Source: `release.yml` has no brew step (only comments at lines 73-76, step name at line 82);
`sync-packages.yml:237` is the reviewer note.

**Lines 381-393, "Why the formula lives in Formula/" (item 1).** Keep the heading and the paragraph at
lines 383-389. Replace lines 391-393 with:

```markdown
The unit tests compare the generator's output with the hand-maintained
`scripts/testdata/angkorfetch.rb.golden`, which pins the formula's shape. Nothing
in this repository or in CI reads or updates `AMRSKH/homebrew-tap`. The tap is not
the documented install path, and its formula pointed to v1.1.1 on 2026-11-11,
behind the v1.2.1 release. The planned Homebrew channel is `Homebrew/homebrew-core`,
with a build-from-source formula; see [docs/PACKAGING.md](docs/PACKAGING.md), section 2.
```

Keep lines 395-398. Reasons: "byte-identical by design" cannot be checked from this repository
and stops being true once the v1.2.1 sync is merged; the golden test (`test_sync_package_manifests.py:61-66`)
compares only generator output and the fixture. The text states what is true now and says nothing
about keeping or retiring the tap, so **no maintainer decision is needed for this wording**.

### docs/MAINTAINING.md

Keep everything not named here (sections 1, 3 except the counts below, and 5).

| Line | Old | New |
|---|---|---|
| 44 | `CI builds 5 targets + .deb + .rpm + checksums.txt` | `CI builds 6 targets + .deb + .rpm + checksums.txt` |
| 53 | `Submit / Verify external packages (Winget, Homebrew tap, Copr, AUR, Flathub)` | `Submit / Verify external packages (crates.io; once each is published: WinGet, homebrew-core, Copr, AUR, Flathub)` |
| 102 | `Compiles 5 release targets (Linux x86_64, Linux ARM64,` | `Compiles 6 release targets (Linux x86_64, Linux x86_64 musl, Linux ARM64,` (rest kept) |
| 104 | `for all 7 assets.` | `for the 8 other assets.` |
| 105 | `attaches all 8 assets.` | `attaches all 9 assets.` |

**Section 4 (lines 116-145).** Keep the heading. Directly under it add (blank line before and after):

```markdown
Channel statuses are in the [Package Status table in the README](../README.md#package-status--availability). Of the channels below, only crates.io is published today. The `.deb` and `.rpm` files need no step here, because each release attaches them. Every other step applies once its channel is published; first submissions follow [PACKAGING.md](PACKAGING.md).
```

**WinGet (item 2).** Keep the heading, step 1, and step 3. Add under the heading (before step 1, blank line after):

```markdown
**Status: Pending.** The package is not in `microsoft/winget-pkgs` yet, so the next submission is a new-package pull request: follow [PACKAGING.md](PACKAGING.md), section 1. Step 2 applies only after the package has been merged.
```

Replace step 2 (lines 120-123) with:

````markdown
2. Once the package is in `microsoft/winget-pkgs`, submit each later version via `wingetcreate`:
   ```powershell
   wingetcreate update AMRSKH.AngkorFetch --version X.Y.Z --urls https://github.com/AMRSKH/angkorfetch/releases/download/vX.Y.Z/angkorfetch-windows-x86_64.zip --submit
   ```
````

Reason: same form as `docs/PACKAGING.md:73`. Without `--submit` no pull request is opened, so step 3
would have nothing to monitor.

**Homebrew (items 1, 3).** Keep the heading; replace lines 127-128 with:

```markdown
**Status: Planned (homebrew-core).** There is no Homebrew release step yet. The planned formula builds from source and goes to `Homebrew/homebrew-core`, so it is not the generated `Formula/angkorfetch.rb`. See [PACKAGING.md](PACKAGING.md), section 2.

`AMRSKH/homebrew-tap` still exists, but it is not the documented install path, CI does not update it, and updating it is not part of a release.
```

**Fedora Copr (items 2, 3).** Keep the heading; replace lines 131-134 with:

```markdown
**Status: Built.** The `.rpm` is attached to each GitHub Release, and no Copr project serves it yet. To create the project, follow [PACKAGING.md](PACKAGING.md), section 3. Once it exists, trigger a Copr build for each release. The exact `copr-cli` command is not given here because it has not been verified; check `copr-cli --help` first.
```

`copr-cli build-package ... --clone-url` is removed, not replaced. I'm not sure of the correct
command: `copr-cli` is not installed on this host, and as far as I know `--clone-url` belongs to
`buildscm` / `add-package-scm`. AGENTS.md section 11 rules out documenting it as working, and no
Copr project exists for it to run against. The maintainer can add the verified command later.

**Arch Linux (item 3).** Keep the heading and steps 1-3. Add under the heading (before step 1, blank line after):

```markdown
**Status: Planned.** Not yet on the AUR; the first submission follows [PACKAGING.md](PACKAGING.md), section 5. The steps below apply to each release after that.
```

**Flathub (item 3).** Keep the heading and step 1. Add under the heading (before step 1, blank line after):

```markdown
**Status: Planned.** Not yet on Flathub; the first submission follows [PACKAGING.md](PACKAGING.md), section 6. The step below applies to each release after that.
```

**crates.io (item 5, new).** Add after the Flathub step, before the `---` at line 147:

````markdown
### crates.io (Cargo)
**Status: Live.** No workflow publishes the crate, so each release needs a manual publish from the tagged commit, with maintainer approval and a crates.io token. The published version can lag the release; see [PACKAGING.md](PACKAGING.md), section 8.
```bash
git switch --detach vX.Y.Z
cargo publish --locked
```
````

`cargo publish --help` on this host lists `--locked`. The publish itself is not run in this task.

### Files to change

- `docs/RELEASING.md`: lines 12, 26, 28-35, 41.
- Root `RELEASING.md`: lines 3-4, 13, 70, 79, 85, 95, 106-110, 148-156, 166, 206, one line after 209, 219, 378-379, 391-393.
- `docs/MAINTAINING.md`: lines 44, 53, 102, 104, 105; section 4 as listed.
- No code, scripts, workflows, manifests, `README.md`, `docs/PACKAGING.md`, `Formula/`, or SHA256 values.

### Further contradictions found (record only; out of scope for this task)

| # | Where | Contradiction |
|---|---|---|
| 1 | `docs/MAINTAINING.md:88-92`, `:6`, `:38` vs root `:114`, `:137`, AGENTS.md section 8 | MAINTAINING commits the version bump straight to `main` with `git add -A`; root uses a pull request; AGENTS.md forbids direct edits on `main` and blind `git add -A` |
| 2 | `docs/MAINTAINING.md:65-72` vs root `:116-123` | MAINTAINING's bump list omits `linux/arch/PKGBUILD` and README |
| 3 | Root `:102` vs root `:142`, `docs/MAINTAINING.md:95` | "signed git tag"; the commands create an annotated tag (`git tag -a`) |
| 4 | Root `:163-169`, `docs/MAINTAINING.md:100-106`, both diagrams | `version-check` and `test-installer-containers` are not listed; root's job table also omits `sync_packages`. `build` needs `version-check` too (`release.yml:129`) |
| 5 | Root `:395-398` | The "It fails rather than writing..." paragraph sits under the Formula section, where "It" has no subject (it means the sync script); kept as is |
| 6 | `scripts/test_sync_package_manifests.py:63-65`, `scripts/sync_package_manifests.py:165` | Comment says "canonical tap shape"; docstring says "audited in the AMRSKH/homebrew-tap repo". Scripts are out of scope, and whether the fixture should go on mirroring the tap goes with any decision about the tap |
| 7 | `docs/MAINTAINING.md` section 4, root step 6 | Snap has no per-release step, only the version bump in step 1. It is Planned, so this is a gap for when it is published |
| 8 | Root and `docs/RELEASING.md` | The pipeline diagram and asset list are duplicated, so counts must be changed twice. Not merged, as instructed |

### Risks and mitigations

| Risk | Mitigation |
|---|---|
| A count is missed in one of the duplicated places | Test plan step 3 greps all count forms; every occurrence is listed above |
| Dated tap fact goes stale | Stated with its date; same fact and date as `docs/PACKAGING.md:86` |
| Wording implies a decision on the tap | The text says what is true now and only that it is not a release step, which Requirements already mandate; retirement is not mentioned |
| Inbound anchors break | No heading is edited; test plan step 4 |
| Mixed line endings from the edits | Test plan step 7 (`git ls-files --eol`, no bare LF) |
| Box diagram misaligned | Only the digit changes (`5` to `6`) |
| A reader misses the removed `copr-cli` command | The Copr text says it is unverified and where to check; history keeps the old line |
| `wingetcreate`, `copr-cli`, `brew` cannot run here | Maintainer steps; the commands reuse `docs/PACKAGING.md`'s form, and QA records them as not run |

### Test plan

1. `python scripts/sync_package_manifests.py --check`: exit 0.
2. `python -m unittest discover -s scripts -p "test_*.py"`: OK.
3. Grep:
   `grep -nE 'canonical|user-facing|\btap\b|homebrew-tap|wingetcreate|brew audit|clone-url|copr-cli|5 (cross-platform |release )?targets?|8 (verifiable|assets)|7 (binary|assets)|Six assets' docs/MAINTAINING.md docs/RELEASING.md RELEASING.md`

   Allowed hits only:

   | File | Pattern | Allowed where |
   |---|---|---|
   | `docs/MAINTAINING.md` | `wingetcreate` | WinGet step 2 sentence and its `update` block |
   | `docs/MAINTAINING.md` | `homebrew-tap`, `\btap\b` | The Homebrew "still exists" paragraph |
   | `docs/MAINTAINING.md` | `copr-cli` | The Copr "has not been verified" sentence |
   | `docs/RELEASING.md` | `brew audit` | Script bullet (template written to pass it; CI does not run it) |
   | Root `RELEASING.md` | `wingetcreate` | Step 6 WinGet bullet |
   | Root `RELEASING.md` | `homebrew-tap`, `\btap\b` | Step 6 Homebrew bullet; line 383 (generic Homebrew "tap"); line 389 (kept); the replaced paragraph under "Why the formula lives in Formula/" |
   | Root `RELEASING.md` | `brew audit` | Line 356, line 388 (kept); the `--check` paragraph ("CI does not run"; reviewer note) |

   None at all for `canonical`, `user-facing`, `clone-url`, `5 target(s)`, `5 cross-platform`,
   `5 release`, `8 verifiable`, `8 assets`, `7 binary`, `7 assets`, `Six assets`.
4. Links: every `](...)` target in the three files that is not `http` or a bare `#` resolves
   relative to its file. Anchors: `#package-status--availability` = README line 50;
   `#cutting-a-release` = root `## Cutting a release`; `#why-package-definitions-lag` = root
   `### Why package definitions lag`; inbound `#why-the-formula-lives-in-formula` =
   `### Why the formula lives in Formula/`. `git grep -nE '(RELEASING|MAINTAINING)\.md#'` shows no
   other anchor.
5. Counts against the workflow: `grep -cE '^\s+asset:' .github/workflows/release.yml` is 6. In the
   docs: `grep -nE '6 (cross-platform |release )?targets?|9 (verifiable|assets)|Seven assets|8 other assets|8 files above|-musl\.tar\.gz' docs/MAINTAINING.md docs/RELEASING.md RELEASING.md`
   shows every line from the tables above. Confirm 9 against the v1.2.1 row in Background.
6. crates.io and labels: `grep -nE 'crates\.io|cargo publish' docs/MAINTAINING.md RELEASING.md` shows
   root line 85, the matrix bullet, step 6, step 7, MAINTAINING line 53, section 4 intro, and the new
   section. Each section 4 channel has its `**Status:**` line, equal to README's value. No hits:
   `git grep -n 'cargo publish' .github/workflows/`.
7. `git diff --stat`: only the three files; `git status --short` also shows this task file;
   `git diff --check` clean; `git ls-files --eol` still `w/crlf` and no bare LF in the three files.
8. Correct procedures still present: root steps 1-5, jobs table, Prerelease versions, sync-packages
   sections, Verifying a release by hand; MAINTAINING sections 1, 3, 5. Check in `git diff` that no
   hunk touches them except the named count lines.
9. Not runnable here; record as not verified: `wingetcreate`, `copr-cli`, `brew`, `cargo publish`.
10. Rust checks are not required (no Rust or `Cargo.*` change).

### Approvals required

None for the edits or the local work commit. A push or PR needs maintainer approval. The actions
these docs describe (publishing to crates.io, WinGet resubmission, Copr/AUR/Flathub submissions, any
change to the tap) are external and outside this task; each needs maintainer approval.

### Dev implementation (2026-11-11, `dev` subagent, Claude Code)

Option B applied as written, with no deviation and no correction. Each replacement was applied by a
throwaway script in the session scratchpad that requires exactly one match per old text, works on
bytes, and writes CRLF; a single mismatch would have left all three files untouched.

| File | Replacements | `git diff --stat` |
|---|---|---|
| `docs/RELEASING.md` | 4 | 23 lines changed |
| Root `RELEASING.md` | 14 | 64 lines changed |
| `docs/MAINTAINING.md` | 12 | 39 lines changed |

Total: 3 files, 75 insertions, 51 deletions. No SHA256 value, heading, or file outside the scope
changed; the one heading in the diff is the planned new `### crates.io (Cargo)`.

Facts re-verified before editing:

| Fact the new text relies on | Evidence | Result |
|---|---|---|
| 6 build targets | `release.yml` lines 138-157; `grep -cE '^\s+asset:'` = 6 | Plan correct |
| 8 hashed files | `checksums` job: `sha256sum *` (line 303) before `checksums.txt` is in that folder; 6 archives + `.deb` + `.rpm` | Plan correct |
| 9 attached assets | `release` job lines 332-346 (7 names + 2 patterns), `files: artifacts/*` (line 354) | Plan correct |
| Dropping `.deb` and `.rpm` leaves 7 | 9 - 2 | Plan correct |
| Musl and aarch64 are cross-compiled; gnu x86_64 is not static | `use_cross: true` on those two rows only; no `.cargo/` directory; no `crt-static` in `Cargo.toml` | Plan correct |
| Status words | README lines 54-61 and `docs/PACKAGING.md` lines 11-18: Live, Built, Pending, Planned (homebrew-core), Planned | Plan correct |
| CI runs no `brew audit` | `release.yml` has only comments (lines 73-76) and a step name (line 82) | Plan correct |
| Sync pull request asks for `brew audit --strict` | `sync-packages.yml:237` | Plan correct |
| No workflow runs `cargo publish` | `git grep -n 'cargo publish' .github/workflows/`: no hits | Plan correct |
| Nothing in the repository reads or updates the tap | `git grep homebrew-tap` outside docs and `team/`: a `.gitignore` entry, the `Formula/README.md` note, one docstring, one test comment; no code path | Plan correct |
| Fixture is maintained by hand | `scripts/test_sync_package_manifests.py:63` | Plan correct |
| `wingetcreate update --urls ... --submit` form | `docs/PACKAGING.md:73` | Plan correct (form copied from it; the tool was not run) |
| Anchors | README line 50; root `## Cutting a release`, `### Why package definitions lag`, `### Why the formula lives in Formula/` | Plan correct |

## Dependencies

- TASK-008 (README statuses), TASK-009 (`docs/PACKAGING.md`, consolidated follow-up list).
- Files read for the plan: `AGENTS.md`; `team/Advisor/README.md`; `team/Commands/advisor.md`;
  `team/Tasks/TASK-009-align-packaging-docs.md` (plan and Documentation Updates); `docs/MAINTAINING.md`;
  `docs/RELEASING.md`; root `RELEASING.md`; `docs/PACKAGING.md`; `Formula/README.md`; `README.md`
  (Installation and Package Status); `.github/workflows/release.yml`;
  `.github/workflows/sync-packages.yml`; `scripts/test_sync_package_manifests.py` (golden test);
  `scripts/sync_package_manifests.py` (formula docstring).
- Checked with read-only commands: `git grep` for inbound links, tap mentions, and counts;
  `git ls-files --eol`; `cargo publish --help` (`--locked` exists); `Cargo.toml` and `.cargo/` (no
  `crt-static`). `copr-cli`, `wingetcreate`, and `brew` are not installed on this host.

## Acceptance Criteria

- [x] No file in scope calls the tap canonical, user-facing, or a required release step
- [x] No `wingetcreate update` shown as usable before the package is in `microsoft/winget-pkgs`
- [x] Steps for unpublished channels are labelled; first submissions link to `docs/PACKAGING.md`
- [x] `brew audit` is not described as enforced by CI or the script
- [x] crates.io appears as a manual, approval-gated release step
- [x] Artifact and target counts match `release.yml`
- [x] Correct release procedures still present
- [x] `python scripts/sync_package_manifests.py --check` and script tests still pass
- [x] All relative links and anchors resolve, including `Formula/README.md` to `RELEASING.md`
- [x] Independent QA accepted

## Testing Requirements

- `python scripts/sync_package_manifests.py --check`
- `python -m unittest discover -s scripts -p "test_*.py"`
- Link and anchor check for the three files and for inbound links to their headings.
- Grep the three files for `canonical`, `user-facing`, `tap`, `wingetcreate update`, `brew audit`,
  `5 target`, `8 verifiable`, `7 binary`, and confirm each remaining hit is accurate and labelled.
- `git diff --stat` shows only the files in scope; `git diff --check` is clean.

### Dev results (2026-11-11, `dev` subagent, Claude Code)

Host: Windows 11, Git Bash, Python 3.14. Base commit `a86d6bf`. Numbers follow the Advisor's test plan.

| # | Check | Actual result | Verdict |
|---|---|---|---|
| 1 | `python scripts/sync_package_manifests.py --check` | `Formula/angkorfetch.rb matches the generated shape (v1.1.1)`, exit 0 | Pass |
| 2 | `python -m unittest discover -s scripts -p "test_*.py"` | `Ran 35 tests`, `OK` | Pass |
| 3 | Forbidden-term grep | 14 hits, all on the allowed list (classified below); none for `canonical`, `user-facing`, `clone-url`, `5 target(s)`, `5 cross-platform`, `5 release`, `8 verifiable`, `8 assets`, `7 binary`, `7 assets`, `Six assets` | Pass |
| 4 | Links and anchors (throwaway `python -I` script, fenced code skipped, GitHub heading slugs) | 23 relative targets, 0 failures, including inbound `Formula/README.md` to `#why-the-formula-lives-in-formula` and `docs/PACKAGING.md` to `#cutting-a-release`. `git grep -nE '(RELEASING\|MAINTAINING)\.md#'` outside `team/Tasks` shows only those 2 | Pass |
| 5 | Counts | Workflow `asset:` count 6. The docs grep shows every planned line: MAINTAINING 44, 102, 104, 105; `docs/RELEASING.md` 12, 26, 29, 36; root 14, 71, 80, 96, 170, 210, 214, 224. 9 agrees with the v1.2.1 row in Background (not re-fetched) | Pass |
| 6 | crates.io and labels | Root 86, 107, 152, 160; MAINTAINING 53, 118, 153, 154, 157. Section 4 `**Status:**` lines: WinGet Pending (121), Homebrew Planned (homebrew-core) (131), Copr Built (136), AUR Planned (139), Flathub Planned (149), crates.io Live (154), each equal to README. `git grep -n 'cargo publish' .github/workflows/`: no hits | Pass |
| 7 | Scope and line endings | `git diff --stat`: only the 3 files. `git status --short`: those 3 plus this task file. `git diff --check`: clean, exit 0. `git ls-files --eol`: all 3 still `i/lf w/crlf`. Byte count: bare LF 0, bare CR 0 (CRLF lines: MAINTAINING 167, `docs/RELEASING.md` 42, root 420) | Pass |
| 8 | Correct procedures intact | `git diff -U0` hunks touch only the planned lines. Root steps 1-5, Prerelease versions, sync-packages sections, Verifying a release by hand: no hunk. Jobs table: only the `build` count cell. MAINTAINING sections 1, 3, 5: only the named count lines in section 3 | Pass |
| 9 | Tools not runnable here | See below | Not verified |
| 10 | Rust checks | Not run: no Rust or `Cargo.*` change | Not required |

Grep hits (test plan step 3), every one classified:

| File:line | Pattern | Allowed entry |
|---|---|---|
| `docs/MAINTAINING.md:124` | `wingetcreate` | WinGet step 2 sentence |
| `docs/MAINTAINING.md:126` | `wingetcreate` | WinGet step 2 `update` block |
| `docs/MAINTAINING.md:133` | `homebrew-tap` | Homebrew "still exists" paragraph |
| `docs/MAINTAINING.md:136` | `copr-cli` | Copr "has not been verified" sentence |
| `docs/RELEASING.md:42` | `brew audit` | Script bullet (template written to pass it; CI does not run it) |
| `RELEASING.md:153` | `wingetcreate` | Step 6 WinGet bullet |
| `RELEASING.md:154` | `homebrew-tap` | Step 6 Homebrew bullet |
| `RELEASING.md:361` | `brew audit` | Plan's line 356, kept, shifted by 5 |
| `RELEASING.md:384` | `brew audit` | `--check` paragraph ("CI does not run") |
| `RELEASING.md:386` | `brew audit` | `--check` paragraph (reviewer note) |
| `RELEASING.md:390` | `\btap\b` | Plan's line 383, generic Homebrew "tap", kept |
| `RELEASING.md:395` | `brew audit` | Plan's line 388, kept |
| `RELEASING.md:396` | `homebrew-tap` | Plan's line 389, kept |
| `RELEASING.md:400` | `homebrew-tap`, `\btap\b` | Replaced paragraph under "Why the formula lives in Formula/" |

Not verified:

- `wingetcreate`, `copr-cli`, and `brew` are not installed on this host (`command -v` found none), so
  the WinGet form, the Copr note, and the audit statements were not exercised.
- `cargo publish` was not run in any form. That `--locked` is accepted comes from the Advisor's
  `cargo publish --help` check and was not repeated.
- Live facts (tap at v1.1.1, 9 assets on v1.2.1, PR #409790 closed, crates.io at 1.1.1) come from
  Background and were not re-fetched.
- External links were not fetched. CI was not run.
- `cargo fmt`, Clippy, `cargo test`, and installer syntax checks were not run; no matching file changed.

## QA Results

### Round 2 (QA, 2026-11-11, qa-review subagent)
Verdict: Accepted

Independent review: ran as the `qa-review` subagent in Claude Code, separate from the Advisor and
Dev agents. Focused re-check of the Round 1 fixes. Host: Windows 11, Git Bash. Base commit
`a86d6bf`, uncommitted diff, nothing staged.

No new findings.

Round 1 fixes:

| Fix | Result | Evidence |
|---|---|---|
| `RELEASING.md:3-5`, local verification added to the manual list | Correct | The manual list now maps to steps 1 (bump), 2 (local verification, `:131-138`), 3 and 5 (the two merges), 4 (tag), 6 and 7 (external channels). No manual step in "Cutting a release" is left under "the rest is automated" |
| `RELEASING.md:96`, "Release builds for 6 target binaries" | Correct | `release.yml:138-157` has 6 matrix rows, all built with `--release` (`:175`, `:179`). `use_cross` is on 2 rows only (`:144`, `:148`), so the old "Cross-compilation" wording no longer applies. No `Cross-compilation for` text remains in the three files |

The other three Round 1 Lows are recorded as accepted in Notes and Limitations and are unchanged:
`RELEASING.md:276` (also `:193`), `RELEASING.md:405`, and `docs/MAINTAINING.md:106` are not in any
diff hunk, and each text is still present in `HEAD`.

Scope: I read every hunk of the current diff for the three files. Each one maps to a named plan change
or to the two fixes above. There is no Round 1 snapshot to diff against, so "nothing else changed"
rests on that hunk review plus the unchanged totals.

Commands run, with results:

| Command | Result |
|---|---|
| `python scripts/sync_package_manifests.py --check` | `Formula/angkorfetch.rb matches the generated shape (v1.1.1)`, exit 0 |
| `python -m unittest discover -s scripts -p "test_*.py"` | `Ran 35 tests`, `OK`, exit 0 |
| Link and anchor check (own `python -I` script, fenced code skipped, GitHub heading slugs) over the three files plus `Formula/README.md` and `docs/PACKAGING.md` | 29 relative targets, 0 failures, including `#cutting-a-release`, `#why-package-definitions-lag`, `#package-status--availability`, `#why-the-formula-lives-in-formula` |
| `git grep -nE '(RELEASING\|MAINTAINING)\.md#'` outside `team/Tasks` | Only `Formula/README.md:39` and `docs/PACKAGING.md:138`; both resolve |
| Plan's forbidden-term grep | Same 14 hits on the same lines as Round 1, all on the allowed list; none in the "none at all" group |
| `grep -cE '^\s+asset:' .github/workflows/release.yml` | 6 |
| `git diff --check` | Clean, exit 0 |
| `git ls-files --eol RELEASING.md docs/MAINTAINING.md docs/RELEASING.md` | All 3 `i/lf w/crlf`; byte count bare LF 0, bare CR 0 (CRLF lines: root 420, MAINTAINING 167, `docs/RELEASING.md` 42) |
| `git status --short` | The 3 files plus this task file; nothing staged |
| `git diff --shortstat` | 3 files, 75 insertions, 51 deletions |
| `git diff --name-only HEAD` on `Formula`, `winget-pkgs`, `scripts`, `.github`, `linux`, `flatpak`, `snap`, `README.md`, `docs/PACKAGING.md`, `AGENTS.md`, `Cargo.*`, installers | No output |
| 64-hex scan of changed lines in `git diff -U0` | No hits, so no SHA256 change |

Not verified:

- `cargo publish` was not run in any form, so `--locked` is still not confirmed by QA; the only evidence is the Advisor's recorded `--help` check.
- `wingetcreate`, `copr-cli`, and `brew` were not run; Round 1 found them not installed on this host. The documented forms were only compared with `docs/PACKAGING.md`.
- Live facts (tap at v1.1.1, 9 assets on v1.2.1, PR #409790 closed, crates.io at 1.1.1) were not re-fetched in this round. External links were not fetched. CI was not run.
- `cargo fmt`, Clippy, `cargo test`, and installer syntax checks were not run: no matching file changed.

### Round 1 (QA, 2026-11-11, qa-review subagent)
Verdict: Accepted

Independent review: ran as the `qa-review` subagent in Claude Code, separate from the Advisor and
Dev agents. Host: Windows 11, Git Bash, Python 3.14.10. Base commit `a86d6bf`, uncommitted diff.

No Critical, High, or Medium findings. All five Lows below are accepted; none blocks the commit.

| Severity | file:line | Finding, evidence, and concrete fix |
|---|---|---|
| Low | `RELEASING.md:3-5` | New sentence ends "the rest is automated", but step 2 (local verification, `:131-138`) is manual and is not in the list. Fix: add "the local verification" to the list of manual items. |
| Low | `RELEASING.md:96` | "Cross-compilation for 6 target binaries": only the digit changed, but `release.yml:144,148` set `use_cross` on 2 of the 6 rows, and `docs/RELEASING.md:29-30` labels only those two as cross-compiled. Fix: "Release builds for 6 target binaries". |
| Low | `RELEASING.md:276` (also `:193`) | Kept rationale says stale definitions would break "both package managers", although neither Homebrew nor WinGet is served from these definitions today. It is an explanation, not a release step, so item 3 is still met. Fix, if wanted: "both package definitions". |
| Low | `docs/MAINTAINING.md:106` | Kept text says the sync will "patch Homebrew formula"; `RELEASING.md:359` says the formula is rendered from a template, and only WinGet is patched. Pre-existing and outside the six items. Fix: "render the Homebrew formula and patch the WinGet manifests". |
| Low | `RELEASING.md:405` | "It fails rather than writing..." still has no subject in this section (the Advisor's contradiction 5). Kept as planned. Fix: start with "The sync script fails...". |

Acceptance criteria:

| Criterion | Result | Evidence |
|---|---|---|
| Tap not canonical, user-facing, or a required step | Met | Grep has no `canonical` or `user-facing`. Remaining tap text (`MAINTAINING.md:133`; `RELEASING.md:154`, `:396`, `:400-401`) says it exists and is not a release step, and says nothing about retiring it. Live: not archived, formula at v1.1.1 |
| No usable `wingetcreate update` before the package is merged | Met | `MAINTAINING.md:121,124` and `RELEASING.md:153` are gated. Live: PR #409790 closed and not merged; `manifests/a/AMRSKH/AngkorFetch` is 404 in `microsoft/winget-pkgs`. The command matches `docs/PACKAGING.md:73` apart from the placeholder |
| Unpublished steps labelled; first submissions link to PACKAGING | Met | Each section 4 channel has a `**Status:**` line equal to README lines 54-61. Section numbers 1, 2, 3, 5, 6, 8 match the PACKAGING headings. No first-submission procedure is duplicated |
| `brew audit` not described as enforced | Met | No workflow runs it (only comments at `release.yml:73-76` and the reviewer note at `sync-packages.yml:237`). `RELEASING.md:373` "CI enforces this" refers to the `--check` and unit test commands, which is true (`release.yml:78,83`) |
| crates.io is manual and approval-gated | Met | `RELEASING.md:107,152`, `MAINTAINING.md:154`. `git grep 'cargo publish' .github/workflows/` has no hits. No text implies automatic publishing |
| Counts match `release.yml` | Met | Derived independently: 6 matrix rows; 8 hashed by `sha256sum *` before `checksums.txt` exists; 9 attached. Live v1.2.1: 9 assets, and its `checksums.txt` lists exactly the other 8. Dropping `.deb` and `.rpm` leaves 7. Asset names and order at `RELEASING.md:213-221` match `release.yml:333-339` |
| Correct procedures intact | Met | Every hunk in the full diff maps to a named plan change. No heading changed; the only new heading is `### crates.io (Cargo)`. Root steps 1-7 keep their numbers and order |
| `--check` and script tests pass | Met | See below |
| Links and anchors resolve, including inbound | Met | See below |

The procedure reads coherently end to end: step 5 (merge the sync pull request) comes before step 6,
as `docs/PACKAGING.md:61` requires for WinGet; step 7 verifies only published channels and refers to
step 6 correctly; no step refers to anything removed. AGENTS.md section 7 is not contradicted, and the
tag, push, and publish steps stay manual. Plan's `crt-static` claim checked: none in `Cargo.toml`, no
`.cargo/` directory, so "(glibc)" and "static binary (musl...)" are correct.

Commands run, with results:

| Command | Result |
|---|---|
| `python scripts/sync_package_manifests.py --check` | `matches the generated shape (v1.1.1)`, exit 0 |
| `python -m unittest discover -s scripts -p "test_*.py"` | `OK`, exit 0 |
| Plan's forbidden-term grep | 14 hits, all on the allowed list: `MAINTAINING.md` 124, 126, 133, 136; `docs/RELEASING.md` 42; root 153, 154, 361, 384, 386, 390, 395, 396, 400. No hit in the "none at all" group |
| Extra grep (number words, `all N`, `N assets/artifacts/binaries/files/targets`, `-u https`, `tap/core`, `byte-identical`, `enforce`) | No stale count; only the Lows above |
| Link and anchor check (own `python -I` script, fenced code skipped) | 29 relative targets in the three files plus `Formula/README.md` and `docs/PACKAGING.md`, 0 failures, including `#why-the-formula-lives-in-formula`, `#cutting-a-release`, `#why-package-definitions-lag`, `#package-status--availability` |
| `git grep -E '(RELEASING\|MAINTAINING)\.md#'` outside `team/Tasks` | Only `Formula/README.md:39` and `docs/PACKAGING.md:138` |
| `grep -cE '^\s+asset:' release.yml` | 6 |
| `git diff --check` | Clean, exit 0 |
| `git ls-files --eol` | All 3 `i/lf w/crlf`; byte count bare LF 0, bare CR 0 (CRLF lines: root 420, MAINTAINING 167, `docs/RELEASING.md` 42) |
| `git status --short`, `git diff --stat` | The 3 files plus this task file; 75 insertions, 51 deletions. No SHA256, script, workflow, or manifest change |
| Read-only `gh api` / `gh release download` (checksums only, to the scratchpad) | v1.2.1 latest with 9 assets; tap not archived at v1.1.1; PR #409790 closed, unmerged; WinGet directory 404 |

Not verified:

- `wingetcreate`, `copr-cli`, and `brew` are not installed on this host. I'm not sure the `wingetcreate submit` and `wingetcreate update` forms work as written; they were only compared with `docs/PACKAGING.md`.
- `cargo publish` was not run in any form, so `--locked` was not confirmed by this review; the only evidence is the Advisor's recorded `--help` check.
- crates.io latest version 1.1.1 was not re-fetched. External links were not fetched. CI was not run.
- `cargo fmt`, Clippy, `cargo test`, and installer syntax checks were not run: no matching file changed.

### Dev fixes (Round 1 Lows, 2026-11-11, dev subagent)

Only the two Lows on lines this task changed, both in root `RELEASING.md`. Applied by a throwaway
script in the session scratchpad (exactly one match per old text, CRLF written). `Status` stays `In Review`.

| Line | Old | New |
|---|---|---|
| 3-5 | `The version bump, the tag, the two pull request merges, and the external channel steps ([Cutting a release](#cutting-a-release), steps 6 and 7) are manual; the rest is automated.` | `The version bump, local verification, the tag, the two pull request merges, and the external channel steps ([Cutting a release](#cutting-a-release), steps 6 and 7) are manual; the rest is automated.` (still 3 source lines; wrapped differently) |
| 96 | `  - Cross-compilation for 6 target binaries` | `  - Release builds for 6 target binaries` |

The other three Lows are pre-existing text and were not changed; see Notes and Limitations.

| Check | Actual result | Verdict |
|---|---|---|
| `python scripts/sync_package_manifests.py --check` | `Formula/angkorfetch.rb matches the generated shape (v1.1.1)`, exit 0 | Pass |
| `python -m unittest discover -s scripts -p "test_*.py"` | `Ran 35 tests`, `OK` | Pass |
| Links and anchors (same throwaway script as the Dev results) | 23 targets, 0 failures: all relative targets in the three files, plus the 2 inbound anchors. `git grep -nE '(RELEASING\|MAINTAINING)\.md#'` outside `team/Tasks`: only `Formula/README.md:39` and `docs/PACKAGING.md:138` | Pass |
| Plan's forbidden-term grep | Same 14 hits on the same lines as before, all on the allowed list (`docs/MAINTAINING.md` 124, 126, 133, 136; `docs/RELEASING.md` 42; root 153, 154, 361, 384, 386, 390, 395, 396, 400); none in the "none at all" group | Pass |
| `git diff --check` | Clean, exit 0 | Pass |
| `git ls-files --eol` | All 3 `i/lf w/crlf`; bare LF 0, bare CR 0 (CRLF lines: root 420, MAINTAINING 167, `docs/RELEASING.md` 42) | Pass |
| `git diff --stat` | `RELEASING.md` 64, `docs/MAINTAINING.md` 39, `docs/RELEASING.md` 23; 3 files, 75 insertions, 51 deletions (unchanged, because the fixes edit lines already in the diff). `git status --short`: those 3 plus this task file | Pass |

Not verified: the same items as in the Dev results (`wingetcreate`, `copr-cli`, `brew`, and `cargo publish` not
run; external links not fetched; CI not run).

## Documentation Updates

2026-11-11. This stage ran in the `docs` subagent (Claude Code), after QA Round 2 accepted the
uncommitted diff of the three files in scope.

**No further documentation changes needed.** No file outside the task's scope was edited, and this
task introduced no direct contradiction in `README.md` or `docs/PACKAGING.md`.

Method: read the full diff, then `git grep` across tracked files (outside `team/Tasks/`) for target
and asset counts, `musl`, `tap`, `homebrew-tap`, `canonical`, `wingetcreate`, `copr-cli`,
`brew audit`, `cargo publish`, links to the two `RELEASING.md` files and `MAINTAINING.md`, and
"step N" references.

| File checked | Result and reason |
|---|---|
| `README.md` | No change. Manual Installation (`:207-217`) lists the same 9 files as `docs/RELEASING.md` and root `RELEASING.md`, and `checksums.txt` is "SHA256 of every file above", which matches "the 8 files above". Status table (`:54-61`) equals every status word added by this task. Platform table: prebuilt binaries match the 6 targets. No count of targets or assets appears |
| `docs/PACKAGING.md` | No change. The link at `:138` still resolves, and step 1 is still the version bump. Status rows (`:11-18`) agree. Section numbers cited by this task (1, 2, 3, 5, 6, 8) match its headings. WinGet (`:67-73`), `brew audit` (`:99`), tap (`:86`), and crates.io (`:325`) say the same as the new text. `copr-cli create` (`:142`) creates a project; the unverified command that `docs/MAINTAINING.md` declines to give is the per-release build, so they do not conflict |
| `Formula/README.md` | No change. The link at `:39` still resolves; its Formula directory explanation matches the kept paragraph of that section. Its tap statements (`:13-15`) agree with the replaced paragraph |
| `CONTRIBUTING.md` | No change. Only links to the release docs (`:120-121`) and a generic packaging checklist (`:109-114`) |
| `docs/DEVELOPMENT.md` | No change. The tree (`:47-48`) and "Validate Homebrew Formula Shape" (`:86`) say nothing this task altered |
| `AGENTS.md` | No change. Section 7 gates (formula `--check`), distribution summary, and approval table agree; `cargo publish` stays approval-gated |
| `team/` role and command docs, `.claude/`, `.opencode/`, `.agents/` wrappers | No change. No statement about targets, assets, the tap, or channel steps. "Step N" hits are the `/task` stages |
| `improve plan.md` | No change. A planning note, not a statement of current behaviour; its tap mentions (`:206`, `:909`) argue against tap-only installs |
| Comments in `.github/workflows/`, `Formula/angkorfetch.rb`, `scripts/` | Not documentation for this stage and not editable here. The two stale wordings are follow-up 6 below |

Not verified in this stage: no command in the docs was run, because this stage edited no command
text. External links and live facts were not re-fetched (they come from Background and QA). GitHub
rendering was not previewed.

### Follow-up list (consolidated; proposed, not created)

Consolidates the Advisor's "Further contradictions found", the three QA Round 1 Lows left unchanged,
and TASK-009 follow-up items 7-13. Line numbers are for this task's working tree. Item 7 is new in
this stage. None of these was introduced by this task.

Docs (one docs task):

1. **Version bump procedure differs.** `docs/MAINTAINING.md:88-92`, `:6`, `:38` commit the bump
   straight to `main` with `git add -A`; root `RELEASING.md:117` uses a pull request; AGENTS.md
   section 8 forbids direct edits on `main` and blind `git add -A`.
2. **Bump list incomplete.** `docs/MAINTAINING.md:65-72` omits `linux/arch/PKGBUILD` and README,
   which root `RELEASING.md:119-126` lists.
3. **"Signed" tag.** Root `RELEASING.md:103` says "signed git tag"; root `:145` and
   `docs/MAINTAINING.md:95` create an annotated tag (`git tag -a`).
4. **Job lists incomplete.** Root `RELEASING.md:167-173`, `docs/MAINTAINING.md:101-106`, and both
   diagrams omit `version-check` and `test-installer-containers`; root's job table also omits
   `sync_packages`. `build` needs `version-check` too (`release.yml:129`).
5. **Small wording errors in kept text.** Root `RELEASING.md:405` "It fails..." has no subject in its
   section (Advisor 5, QA Low). Root `:193`, `:276` "both package managers", although neither is
   served from these definitions today (QA Low). `docs/MAINTAINING.md:106` "patch Homebrew formula";
   the formula is rendered from a template (QA Low).
6. **Tap wording in scripts.** `scripts/test_sync_package_manifests.py:63-65` ("mirrors
   `AMRSKH/homebrew-tap`", "canonical tap shape") and `scripts/sync_package_manifests.py:165`
   ("audited in the AMRSKH/homebrew-tap repo"). Goes with any decision about the tap (Advisor 6).
7. ***New*: `Formula/README.md:13-14` "holds a copy of this formula".** True today (TASK-009 QA
   found the two files byte-identical at v1.1.1), but it stops being true once the v1.2.1 sync
   pull request (item 13 below) is merged, for the same reason this task removed "byte-identical by
   design" from root `RELEASING.md`. Reword when that merge happens.
8. **Snap has no per-release step** in `docs/MAINTAINING.md` section 4 or root step 6, only the
   version bump. It is Planned, so this is a gap for when it is published (Advisor 7).
9. **Duplicated diagram and asset list** in root `RELEASING.md` and `docs/RELEASING.md`, so counts
   must be changed twice (Advisor 8).
10. Optional harness wording from TASK-009: `AGENTS.md:44-45` and the `docs/DEVELOPMENT.md` tree.

Repository files (code or packaging task; no SHA256 by hand), TASK-009 items 7-11:

11. Hand-set pins at 1.1.1: `linux/arch/PKGBUILD`, `snap/snapcraft.yaml`, `linux/deb/build-deb.sh`,
    `linux/rpm/build-rpm.sh`, `linux/rpm/angkorfetch.spec` (no 1.2.1 changelog entry), the Flatpak
    manifest `tag`, and the metainfo `<release>`.
12. `linux/rpm/build-rpm.sh` fails from every directory; `linux/arch/PKGBUILD` uses
    `sha256sums=('SKIP')`; the Flatpak manifest has no Rust SDK extension or vendored Cargo sources;
    no build-from-source Homebrew formula exists for homebrew-core.

External actions (each needs maintainer approval), TASK-009 items 12-13:

13. Merge `origin/automation/sync-packages-v1.2.1` so `Formula/` and `winget-pkgs/` move off 1.1.1,
    before any WinGet resubmission or tap update.
14. Publish 1.2.1 to crates.io; resubmit to WinGet; update or retire `AMRSKH/homebrew-tap`; Copr,
    AUR, Flathub, Snap, and homebrew-core submissions.

## Final Validation (main session, 2026-11-11)

| Check | Result |
|---|---|
| `python -m unittest discover -s scripts -p "test_*.py"` | Ran 35 tests, OK, exit 0 |
| `python scripts/sync_package_manifests.py --check` | `Formula/angkorfetch.rb matches the generated shape (v1.1.1)`, exit 0 |
| `git diff --check` | clean, exit 0 |
| `git ls-files --eol` | all three docs `i/lf w/crlf` |
| `git diff --stat` | `RELEASING.md` 64, `docs/MAINTAINING.md` 39, `docs/RELEASING.md` 23; 75 insertions, 51 deletions |
| 64-hex (SHA256-like) values on diff lines | 0 |

Not run: Rust checks and installer syntax (no matching file changed).

## Git Commit

- Commit Message: `docs(release): align release and maintainer docs with package status [TASK-010]`
- Commit Hash: `616bd384af4ba2e11315b5c0b077b976216c3ef3`

## Final Report

- `docs/MAINTAINING.md`, `docs/RELEASING.md`, root `RELEASING.md`: counts corrected from
  `release.yml` (6 targets, 9 assets, `checksums.txt` covers the other 8); the tap is no longer
  canonical or a release step; unpublished channels labelled, with first submissions pointing to
  `docs/PACKAGING.md`; `wingetcreate update` only after the package is merged; `--check` described
  as a generator-shape check, not `brew audit`; crates.io added as a manual, approval-gated step.
- Stages: Advisor, Dev, QA (Rounds 1-2, both Accepted; two Lows fixed, three pre-existing Lows
  accepted and left), and Docs ran in subagents; orchestration, final validation, and commits in
  the main session.
- Open: the follow-up list under Documentation Updates.
- Not verified: commands for `wingetcreate`, `copr-cli`, `brew` (not installed here); `cargo publish`
  (never run); external links; CI.

## Notes and Limitations

- Live facts above are as of 2026-11-11 and can change.
- Three QA Round 1 Lows are on pre-existing text, accepted by QA, and left unchanged:
  - `RELEASING.md:276` (also `:193`): the rationale says "both package managers", although neither
    Homebrew nor WinGet is served from these definitions today.
  - `docs/MAINTAINING.md:106`: says the sync will "patch Homebrew formula"; the formula is rendered
    from a template, and only WinGet is patched.
  - `RELEASING.md:405`: "It fails rather than writing..." has no subject in its section.
