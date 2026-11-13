# The client on the disc

Identification of the CD described in [PRESERVATION.md](PRESERVATION.md).
Facts marked **[disc]** were read from the disc/binaries in this repo's tools;
**[research]** facts come from the notes in `research/notes/` (with their
sources).

## Summary

**euRO (Ragnarok Online Europe) client, December 2003 — almost certainly the
closed-beta distribution.** Engine build `Ragexe.exe` 2003-12-18 06:05:19 UTC,
an English Gravity client of the iRO/pRO lineage, wire protocol = the plain
pre-2004 layout (eAthena `packet_ver 5`).

## Evidence

| What | Value | |
|---|---|---|
| Game exe | `Ragexe.exe`, 2,195,542 bytes, PE timestamp `0x3fe1439f` = 2003-12-18 06:05:19 UTC, MSVC 6, not packed, sha256 `a25b7efa…0eca` | [disc] |
| Patcher | `Ragnarok.exe` 2003-12-12, euRO build: `wpatch.euro-ro.net`, `fpatch.euro-ro.net`, launches `ragexe.exe 1rag1`, merges `.gpf` into `data.grf` via `tmpdata.grf`, knows `2003-10-31gdata_xmas.gpf` | [disc] |
| `data\clientinfo.xml` (in data.grf) | servicetype `germany`, servertype `primary`, display `GRO`, address **62.109.138.134:6900**, version 1, registrationweb `http://www.euro-ro.net` | [disc] |
| Login screen | "http://www.euro-ro.net" artwork | [disc, screenshot] |
| Patches shipped un-merged | `2003-04-30gdata_turtle_ph_1/2.gpf` (Turtle Island, "_ph" = Philippine build), `2003-12-11gdata_yuno_new_1/2.gpf`, `2003-12-11gdata_yuno_update.gpf` (Juno) | [disc] |
| Archives | `data.grf` GRF 0x200 (13,378 files, no encryption); `.gpf` GRF 0x102 (DES-obfuscated names/data, 3,319 files) | [disc] |
| Text | English `msgstringtable.txt`; client service types compiled in: korea, america, japan, china, taiwan, thai, indonesia, philippine, malaysia, singapore, germany | [disc] |
| Installer | InstallShield Express 3.5 MSI, 2003-12-19; support link `www.ruro-ro.net` (probably a typo for euro-ro.net) | [disc] / [research] |
| Operator | Burda Holding International GmbH (licence signed with Gravity 2003-11-26) — the *official* European service, not a private server | [research] euro-history.md |
| Closed beta | started 2003-12-20 (two days after the exe build): Comodo, Q-Pet, higher test EXP, promised wipe; open beta 2004-02-04; paid launch 2004-04-15 | [research] |
| Server software | Gravity's own (Aegis) per the licence terms — an inference, not documented | [research] |
| 62.109.138.134:6900 | euRO's login server (OpenKore 2004 config: `master_version 20`, `version 1`) | [research] |

One file is corrupt in Gravity's own data: `data\wav\worm_tail_damage.wav`
fails its zlib checksum identically in `data.grf` and in the Juno patch.

## Content state

The client data contains Prontera/Geffen/Payon/Morroc/Alberta/Izlude/
Aldebaran/Comodo/Lutie (xmas)/Glast Heim, Turtle Island (patch) and Juno
(patch): 269 maps after merging. Its novice training ground is
`new_zone01..04`. Servers of the time called it `new_1-1..4`, and the client
maps one name to the other itself (`data\resnametable.txt`:
`new_1-1.gat#new_zone01.gat#`). euRO itself only opened Juno and the 2-2
jobs on 2004-06-09, so a faithful "closed beta" world would keep Juno closed
(see ROADMAP).

## Redirecting it (no exe patch)

Tested in the headless rig: this client reads `clientinfo.xml` **only from
data.grf** — a loose `data\clientinfo.xml` is ignored (it still dialled
62.109.138.134). `ragnarok-offline setup` therefore rebuilds data.grf the way
euRO's patcher would have (base + .gpf patches) with our clientinfo.xml
inside; the original is kept as `data.grf.original`. The exe is untouched.
`1rag1` on the command line skips the "please run Ragnarok.exe" patcher check.

Settings live in `HKLM\Software\Gravity Soft\Ragnarok` (`Wow6432Node` on
64-bit systems), written by the client's `Setup.exe`
(`tools/client-test/ragnarok-settings.reg` is a working example).

## The path finder

The client walks every unit along a path it finds itself: the server only
says where from and where to (0x0087, 0x007b). `CPathFinder::FindPath` is at
`0x4d2300` (found through its log strings "find path failed : no path found"
and "find path aborted : buffer overflow"); one instance lives at `0x66e0b8`.
It is an A*:

* f = g + 10 x the Manhattan distance to the target; a straight step costs
  10, a diagonal 14. The neighbours are tried in the order SE, E, NE, N, NW,
  W, SW, S (`0x4d2524`-`0x4d26c6`).
* A step (`0x4d2be0`) needs the cell stepped onto to be in the map and of a
  GAT type other than 1 and 5 (the whole dword is compared); a diagonal
  step needs both cells beside it as well. The cell stepped from is not
  looked at.
* The open list is a binary heap in a `std::vector`, kept with MSVC 6's
  `push_heap`/`pop_heap` (`_Adjust_heap` at `0x4d4770`): equal f go to the
  right child, a node moves up only past a strictly greater f. A node already
  open that is reached more cheaply is found by a linear search of the heap
  and moved up from there.
* Quirk: a closed node that is reached more cheaply gets the new cost and
  parent and is marked open, but is not put back on the heap (`0x4d2a1f`),
  so it is never expanded again. Its children keep pointing at it.
* At most 149 nodes (`0x95`, 28 bytes each in a fixed pool): a search that
  needs more fails. Nothing else limits a path's length; the server's
  32-step limit is Athena's.
* The path (`0x4d2cf0`) keeps each cell's direction and a time: a straight
  step takes the unit's speed in ms, a diagonal `(int)(speed * 1.414f)`.

`crates/ro-game/src/path.rs` (`client_search`) is a port of it, checked
against the exe's code run under an emulator (`research/client_paths/`):
4,800 paths on synthetic grids (`tests/client_paths.rs`) and 5,760 on
windows of the 269 maps, all the same. Athena's path.c, which the server
used before, chose a different route on 22 of 4,122 real-map paths that
both could walk.
