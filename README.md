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
| What we do differently from 2003, on purpose | [docs/DEVIATIONS.md](docs/DEVIATIONS.md) |
| Open research questions | [research/THREADS.md](research/THREADS.md) |
| What is missing / next | [docs/ROADMAP.md](docs/ROADMAP.md) |
| Building, testing, the headless client rig | [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) |
| Modern Windows: launcher, all-in-one installer, what the 2003 CD does on Win11 | [docs/WINDOWS.md](docs/WINDOWS.md) |

## Play (Windows 10/11)

1. Install the client from the CD (or copy an installed `RagnarokOnline` folder).
2. Put **`RagnarokOffline.exe`** into that folder (next to `Ragexe.exe`) and
   double-click it. That's all: the first start prepares the folder once
   (UAC prompt only if it is under Program Files). Then, as in 2003, the
   CD's patcher shows euRO's notice (an archived snapshot, served locally)
   and you click **Start Game**.
3. Log in with **any new ID and password** (4+ characters): the account is
   created on first login. Add `_F` to the ID once to make a female account
   (`myname_F`), `_M` for male (default).

`RagnarokOffline.exe` contains the server, its scripts and database, and
[DDrawCompat](https://github.com/narzoul/DDrawCompat) (DirectX 7 fixes for
modern Windows, XP-style text). Fullscreen is emulated (no display mode
change, 4:3 with black bars); the client's own `Setup.exe` still changes
resolution, sound and so on. Your saves, scripts and `ragnarok-offline.toml`
end up in `<game>\offline\` (or `%LOCALAPPDATA%\Ragnarok Offline\` if the
game folder is read-only).

Own the CD? `tools/aio/build.sh` turns it into an **all-in-one installer**:
the original setup wizard, plus all of the above, prepared at install time.
Details, and what the 2003 installer and client do on Windows 11:
[docs/WINDOWS.md](docs/WINDOWS.md).

### By hand (server CLI)

Unzip `ragnarok-offline-<version>-windows-x86_64.zip` anywhere, then in a terminal:

```bat
ragnarok-offline.exe setup --client "C:\Program Files (x86)\Gravity\RagnarokOnline"
ragnarok-offline.exe init --client "C:\Program Files (x86)\Gravity\RagnarokOnline"
ragnarok-offline.exe run
```

`setup` keeps the CD's `data.grf` as `data.grf.original`, writes a new
`data.grf` with the CD's Turtle Island/Juno patches merged (what euRO's
patcher would have done) and a `clientinfo.xml` pointing at 127.0.0.1, and
adds `Play Offline.bat`; `init` writes `ragnarok-offline.toml`. Start the game
with `Play Offline.bat`. The server needs Windows 10 or newer (or 7/8 with the
Universal C Runtime update); the client needs DDrawCompat or a similar
wrapper for fullscreen on modern Windows (`RagnarokOffline.exe` installs it).

## Play (Linux)

Put `RagnarokOffline` from `ragnarok-offline-<version>-linux-x86_64.tar.gz` into the
client folder and run it (server + `wine Ragexe.exe 1rag1`; configure the client
with `wine Setup.exe` first). Or by hand, with the static `ragnarok-offline` binary:

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
portals), the era's NPCs (611 from eAthena 817: towns, the novice training
ground, job quests, guides, Kafras, refiners, quests), shops,
inventory/equipment, items with use effects, Kafra storage, the skill tree
and skill points, monsters (2,001 era spawn groups) with simple AI, melee
combat, EXP and levels, drops, regen, death/respawn, saving. Not yet: using
skills, status effects, element/size/card modifiers, carts' contents, pets;
parties, trading and guilds need other players — see
[docs/ROADMAP.md](docs/ROADMAP.md) and [docs/DEVIATIONS.md](docs/DEVIATIONS.md).

## License

GPL-3.0-or-later ([LICENSE](LICENSE)). `data/db/*.toml` and
`scripts/import/**` are generated from, and the NPC scripts are ported from,
the GPL-licensed Athena-family emulators of 2003–04 (Athena, AppleMod,
eAthena); the project as a whole is GPL so they can live in it. Bundled
DDrawCompat is 0BSD (its licence ships next to it). The client and its data
are Gravity's and are never in this repository.
