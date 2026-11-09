# Development

## The workbench (always)

All builds, analysis and client tests run in a rootless podman container:

```sh
tools/wb build          # image: Rust (+ windows-gnu, musl), Wine, Xvfb, msitools, rizin, capstone
tools/wb exec <cmd>     # run in it (starts it if needed)
tools/wb sh             # shell
tools/wb down           # kill it now
tools/wb status
```

Hard cap 2 GiB RAM, no swap, 4096 pids (`WB_MEM` to raise for a specific
job). PID 1 is `sleep 6h` and the container is `--rm`, so it tears itself down
with everything inside even if forgotten; `exec` recycles it after 4 h. Mounts:
repo → `/work`, `~/data/ragnarok-cd` → `/data` (disc image, unpacked client,
play copy, Wine prefix, saves, reference emulators — never in git), cargo
registry → volume `ro-wb-cargo`.

## Build and test

```sh
tools/wb exec cargo test --workspace     # unit tests + in-process full session test
tools/wb exec cargo build --release -p ro-server
tools/wb exec tools/package.sh           # dist/: static Linux tarball + Windows zip
```

`crates/ro-game/tests/session.rs` plays a complete session (login → create →
enter → walk → chat → @command → kill a Poring → EXP → save) against `Game`
in-process on a synthetic map; it needs no client files.

## The real client, headless

`/data/play/RagnarokOnline` is a hard-linked copy of the unpacked CD (the
pristine extract under `/data/client/out` is read-only) on which `setup` has
run. `tools/client-test/rig.sh` drives it in Wine on Xvfb :99:

```sh
R=tools/client-test/rig.sh
tools/wb exec $R server-start            # dev.toml: packet log on, saves in /data/saves
tools/wb exec $R client-start
tools/wb exec $R login tester secret1    # from launch to standing on the map
tools/wb exec $R shot name               # -> ~/data/ragnarok-cd/shots/name.png
tools/wb exec $R click X Y | type TEXT | key KEY
tools/wb exec ./target/debug/ragnarok-offline -c dev.toml eval 'nearby(10)'
```

Notes: the client needs press-and-hold clicks and an activated window
(the rig does both); there is no sound device, so the rig's registry file sets
`SOUNDMODE=0`; Escape opens the client's exit menu. `logs/packets.log` has
every packet both ways.

## Reverse engineering

`research/extract_packet_lengths.py` (capstone) produced the length table;
`research/grf_peek.py` lists GRF 0x200 archives; rizin is in the image.
Addresses found so far are in docs/PROTOCOL.md.

## Data import

`tools/import/athena_import.py <refs/eathena-archive/x> <repo>` regenerates
`data/db/*.toml` and `scripts/import/*.lua` from the archived Athena releases
(fetched by the research notes into `/data/refs`). Hand edits to generated
files should go in a commit of their own with the reason.

## Preservation assets

See PRESERVATION.md; the image and its manifests are on
`cold:/gigavault/archive/discs/ragnarok-online-euro-2003-12/`.
