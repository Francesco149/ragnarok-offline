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

## The playable snapshot (app launcher)

`tools/play` keeps a build to play while the tree is being worked on. It
shares nothing that changes with the rig: its own container (`ro-play`, the
workbench image, 2 GiB, own loopback network), server binary, scripts,
database, client folder, `data.grf` (ports 6910/6131/5131, console 6133),
saves, Wine prefix and config, all under `~/data/ragnarok-cd/stable`.

```sh
tools/wb exec tools/play/update.sh          # end of every session: build + snapshot (builds/<stamp>, `current`)
~/data/ragnarok-cd/stable/play.sh --smoke   # host: headless check, server + client + a login (logs/smoke.png)
tools/play/install.sh                       # host, once: "Ragnarok Offline" in the app launcher
```

`play.sh` (copied next to the snapshot, so the entry does not depend on the
tree) shows the client through the desktop's X socket with sound over
PulseAudio; `--fullscreen` uses Wine's Wayland driver (not yet tried),
`--settings` runs the client's Setup.exe, `--stop` saves and quits. A running
game stays on the build it started with; `update.sh` only adds a build and
moves `current`. `stable/ragnarok-offline.toml` and `stable/saves` are the
player's: never overwritten.

## A real Windows machine

`tools/win-test/win.sh` drives a Windows 10/11 box over ssh (Windows' own
OpenSSH, PowerShell as the default shell). ssh sessions run in session 0 and
cannot see the desktop, so `up` starts two PowerShell agents inside the
logged-in desktop session (scheduled tasks; one medium integrity, one
elevated, because UIPI stops a medium process from driving elevated windows
such as msiexec's UI). Everything it creates lives under `WIN_DIR` (default
`F:\ro-test`); `WIN_HOST` (default `win-wifi`) is the ssh alias.

```sh
W=tools/win-test/win.sh
$W up                                   # copy agent.ps1, start both agents
$W push ~/data/ragnarok-cd/RAGNAROK.iso 'F:\ro-test\RAGNAROK.iso'
$W ps  'Mount-DiskImage F:\ro-test\RAGNAROK.iso'        # ssh session (elevated, no desktop)
$W do  'Launch D:\setup.exe'                             # desktop session, like a double-click
$W doa 'Windows "msiexec"; Click 1055 708'               # elevated desktop agent
$W shot name [admin]                    # -> ~/data/ragnarok-cd/shots/win11/name.png
$W down
```

Snippet helpers (agent.ps1): `Shot`, `Click`/`DClick`/`RClick`, `MoveTo`,
`Key`, `Send`, `Run`, `Launch` (starts a program from the foreground, as
Explorer does), `Windows [regex over "title class process"]`, `Focus`,
`Restore`, and `GameClick X Y` (clicks inside a DDrawCompat fullscreen game
at game coordinates). Gotchas: fullscreen games minimize when anything else
takes the foreground (restarting the agents does); the elevated agent's
hidden console can own the foreground, so restore from `doa`; use Shift, not
Alt, to unlock SetForegroundWindow (Alt sends windows into menu mode).

## Windows launcher and all-in-one installer

`crates/ro-launcher` builds `RagnarokOffline.exe` (GUI subsystem; manifest and
version info via mingw `windres`). It embeds `scripts/`, `data/db/` and, when
`tools/fetch-third-party.sh` has fetched it, DDrawCompat's `ddraw.dll`
(pinned, sha256-checked, into the git-ignored `third_party/`). `tools/package.sh`
runs the fetch. `tools/aio/build.sh` turns a CD (default `/data/iso`) into the
all-in-one installer folder and ISO under `/data/aio/`: Gravity's files plus
ours, never committed. See WINDOWS.md for what both do and why.

### The patcher under Wine

The launcher's patcher path works in the headless rig too, with the Windows
build run in Wine. The patcher always dials FTP port 21, and a rootless
container can't let a normal user bind it, so forward it as root and point
the patch server at 2121 in `<game>\offline\ragnarok-offline.toml`:

```sh
podman exec -u 0 -d ro-wb socat TCP-LISTEN:21,bind=127.0.0.1,fork,reuseaddr TCP:127.0.0.1:2121
```

The patch server logs every HTTP request and FTP transfer to `launcher.log`.

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
