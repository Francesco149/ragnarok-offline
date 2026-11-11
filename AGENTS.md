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
  `~/data/ragnarok-cd/stable/play.sh --smoke` (docs/DEVELOPMENT.md). The owner
  plays it from the app launcher while work goes on: never touch
  `stable/saves`, `stable/wine` or a running `ro-play` container.
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
  what you install. The all-in-one installer output contains Gravity's files:
  never commit or publish it.
