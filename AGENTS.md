# Agent guide

Single-player server for the 2003-12-18 euRO client (see README.md).

## Rules

- **Run everything in the workbench** (`tools/wb exec ...`): 2 GiB cap, no
  swap, self-destructing container. Never build, run Wine or the server on the
  host. maximus has no swap and has frozen from uncapped jobs before.
- `pkill -f PATTERN` inside `tools/wb exec bash -c '...'` matches its own shell;
  use the bracket trick (`pkill -f "[s]erver"`).
- **Client files are Gravity's**: never commit anything from the CD or the
  unpacked client. They live in `~/data/ragnarok-cd` (`/data` in the
  workbench); the disc image is archived on cold (docs/PRESERVATION.md).
- **Tag provenance.** Every server-side value has a source and a tier
  (docs/PROVENANCE.md). Prefer `client` evidence; cite file:line for formulas
  taken from period emulators; never replace a period value with a later one
  without saying so.
- Keep confirmed behaviour in Rust and reconstructed behaviour in Lua/TOML
  (docs/ARCHITECTURE.md).
- euRO's May-2004 client (MICO07 DVD) gets its own version after this
  one is complete: don't build for it now, but note anything relevant to
  it in research/notes/may2004-client.md.
- Every deliberate difference from the period game (single-player
  reworks, fixes to emulator scripts) goes in docs/DEVIATIONS.md; every
  open question worth chasing goes in research/THREADS.md.
- Don't edit generated files (`scripts/import/**`, `data/db/*.toml`):
  regenerate them, or override in `scripts/npc/`.
- Every packet goes through `ro_proto::Writer`; don't bypass the length check.
- Test with `cargo test --workspace`; for client-visible changes also run the
  headless client (docs/DEVELOPMENT.md) and look at a screenshot.
- **End every session with a fresh playable snapshot**: from the committed
  tree, `tools/wb exec tools/play/update.sh`, then on the host
  `~/data/ragnarok-cd/stable/play.sh --smoke`, then `tools/strays.sh --clean`
  (docs/DEVELOPMENT.md; run `tools/strays.sh` at the start too). The owner
  plays it from the app launcher while work goes on: never touch
  `stable/saves`, `stable/wine` or a running `ro-play` container.
- **Releases**: at session start run `python3 tools/ci/daily_release.py`
  (the post-commit hook runs it too, once `git config core.hooksPath
  tools/git-hooks` is set). When the rolling `nightly` release is more than
  a day behind master it pushes master and rebuilds the release on GitHub
  (docs/DEVELOPMENT.md "Releases"). The release must never contain
  anything of Gravity's: the setup builder fetches the disc at run time.
- Commits: small, logical, with `Co-Authored-By:` naming your model.
- Work on `master`. Use a branch or worktree only for a good reason (several
  agents on different features at once), and merge it back before the
  session ends. The one exception is work that leaves the game broken: leave
  it on its branch, say so in research/THREADS.md or the session notes, and
  the next session finishes and merges it first.

## Map

`crates/` (Rust), `scripts/` (Lua: rules, commands, NPCs, imports),
`data/db/` (TOML game data), `data/client/packet_lengths.tsv` (from the exe),
`research/` (tools + notes), `tools/` (workbench, client rig, real-Windows
rig `win-test/`, all-in-one installer `aio/`, importer, packaging),
`third_party/` (pinned downloads, git-ignored), `docs/`.

- The Windows rig (`tools/win-test`) drives someone's real desktop: no
  native fullscreen mode switches, keep files under its `WIN_DIR`, uninstall
  what you install. A full disk there once wedged its sshd: `win.sh` refuses to
  start below 20 GB free on C: or `WIN_DIR`'s drive. If it refuses, stop the
  Windows work and tell the orchestrator; never free space yourself to get
  past it. Delete large files you pushed (ISOs, client copies) once a test is
  done. The all-in-one installer output contains Gravity's files:
  never commit or publish it.
