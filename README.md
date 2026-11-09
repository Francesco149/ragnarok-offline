# ragnarok-offline

A single-player server for one specific, preserved Ragnarok Online client: the
**euRO closed-beta CD of December 2003** (`Ragexe.exe` built 2003-12-18). The
original client connects to it unmodified — no exe patching — and you play
alone on your own machine.

The server is written in Rust. Everything server-side that a period server
decided (NPCs, warps, spawns, damage, HP/SP, regen...) lives in Lua scripts you
can edit while the server runs.

| | |
|---|---|
| What the disc is | [docs/CLIENT.md](docs/CLIENT.md) |
| How it was preserved, where the image is | [docs/PRESERVATION.md](docs/PRESERVATION.md) |
| How the server is built | [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) |
| Wire protocol, and what is confirmed from the exe | [docs/PROTOCOL.md](docs/PROTOCOL.md) |
| Writing NPCs, rules, commands (Lua) | [docs/SCRIPTING.md](docs/SCRIPTING.md) |
| Where every number comes from | [docs/PROVENANCE.md](docs/PROVENANCE.md) |
| What is missing / next | [docs/ROADMAP.md](docs/ROADMAP.md) |
| Building, testing, the headless client rig | [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) |

## Play (Windows)

1. Install the client from the CD (or copy an installed `RagnarokOnline` folder).
2. Unzip `ragnarok-offline-<version>-windows-x86_64.zip` anywhere, e.g. `C:\ragnarok-offline`.
3. In that folder, open a terminal and run once:
   ```bat
   ragnarok-offline.exe setup --client "C:\Program Files\Gravity\RagnarokOnline"
   ragnarok-offline.exe init --client "C:\Program Files\Gravity\RagnarokOnline"
   ```
   `setup` keeps the CD's `data.grf` as `data.grf.original`, writes a new
   `data.grf` with the CD's Turtle Island/Juno patches merged (what euRO's
   patcher would have done) and a `clientinfo.xml` pointing at 127.0.0.1, and
   adds `Play Offline.bat`. `init` writes `ragnarok-offline.toml`.
4. Start the server: double-click `ragnarok-offline.exe` (or `ragnarok-offline.exe run`).
5. Start the game with `Play Offline.bat` in the client folder.
6. Log in with **any new ID and password** (4+ characters): the account is
   created on first login. Add `_F` to the ID once to make a female account
   (`myname_F`), `_M` for male (default).

The server needs Windows 10 or newer (or Windows 7/8 with the Universal C
Runtime update installed). The client needs DirectX 7-era 3D; on modern
Windows it may need a wrapper such as dgVoodoo2.

## Play (Linux)

Same steps with `ragnarok-offline` from `ragnarok-offline-<version>-linux-x86_64.tar.gz` (a
static binary). Run the client with Wine:

```sh
./ragnarok-offline setup --client ~/RagnarokOnline
./ragnarok-offline init --client ~/RagnarokOnline
./ragnarok-offline run &
cd ~/RagnarokOnline && wine Ragexe.exe 1rag1
```

## Change the game while it runs

```sh
ragnarok-offline console                     # Lua REPL inside the running server
ragnarok-offline eval 'player.zeny = 50000'  # one-off, as the first online player
```

Edit any file under `scripts/` and it reloads automatically; in game,
`@reload` does the same and `@help` lists the admin commands (your
auto-registered account is GM level 99 by default).

## Status

Working with the real client: login, character create/delete/select, all
maps of the client incl. Juno and Turtle Island, walking, warps (2,038 era
portals), NPC dialogs (Lua coroutines: mes/next/menu/close/input), shops,
inventory/equipment, items with use effects, monsters (2,001 era spawn groups)
with simple AI, melee combat, EXP and levels, drops, regen, death/respawn,
saving. Not yet: skills, status effects, element/size/card modifiers, parties,
storage, trading, guilds, pets — see [docs/ROADMAP.md](docs/ROADMAP.md).

## Licensing note

`data/db/*.toml` and `scripts/import/*.lua` are generated from Athena-family
emulator data (GPL-2.0-or-later); keep that in mind before publishing. The
client and its data are Gravity's and are never in this repository.
