# The May-2004 client: notes for its own version

**Decision (owner, 2026-11-11):** the game is finished for the 2003-12-18
client first. euRO's client of May 2004 gets its own version of the game
afterwards. Until then, nothing is built for it. Whatever a session learns
that matters to that version goes here, so that the work starts from a list
and not from a search. Add to this file; don't build for that client yet.

## What it is, and where

The owner's *Il Mio Computer* n.108 DVD (July 2004), ripped 2026-11-09 to
`~/data/ragnarok-cd/mico07/` (`/data/mico07` in the workbench; Gravity's
files, never committed; on archive.org as `il-mio-computer-108-luglio-2004-dvd`).
`Giochi/RAGSETUP_EU/` is euRO's May-2004 installer. Installed by `msiextract`
into `mico07/client/out/Program Files/Gravity/RagnarokOnline/`:

* `2004-05-07aRagexe.rgz`, unpacked as `mico07/Ragexe-2004-05-07.exe` (PE
  timestamp 2004-05-07). Its packet table: `mico07/packet_lengths-Ragexe-2004-05-07.tsv`
  (`research/extract_packet_lengths.py`).
* `2004-05-04gdata_yuno_change1_g.gpf`, `_change2`, `_new1` to `_new5`
  (Juno and the maps it changes) and `2004-05-06gdata_yuno_lang_g.gpf`
  (English tables; unpacked to `mico07/yuno_lang/data/`).
* `data.grf` (452 MB): the base archive of May 2004.
  `mico07/datagrf-diff-dec2003-may2004.txt` compares it with the CD's
  **base** `data.grf.original` (134 changed, 2004 added, none removed);
  "added" includes the Turtle Island patch maps that we merge from the CD's
  gpf, so read that list against the merged GRF, not as new content.

On euRO this exe sits between patch-list entries 444 (`2004-04-15aRagexe.rgz`)
and 462 (`2004-05-31aRagexe.rgz`, the one the 2-2 jobs opened with on
2004-06-09; not found anywhere). `research/notes/euro-history.md` §5.

Read any of the archives with
`cargo run --release --example grf_dump -p ro-grf -- <grf|gpf> [substring] [outdir]`
(GRF 0x102 with DES included; `research/grf_peek.py` reads only 0x200).

## The client against ours

**Packets** (`data/client/packet_lengths.tsv` against the May table):

| Id | Dec 2003 | May 2004 | What (kRO numbering) |
|---|---|---|---|
| 0x0078, 0x0079, 0x007a, 0x007b | 52, 51, 56, 58 | 54, 53, 58, 60 | unit idle / spawn / walk: two bytes more each; which field is queue T11 |
| 0x01d8, 0x01d9, 0x01da | 52, 51, 58 | 54, 53, 60 | the same for the later forms |
| 0x014e | 6 | absent | |
| 0x01cd to 0x01d2, 0x01e1 | absent | 30, 6, 28, 8, 14, 10, 8 | Auto Spell list and choice, Devotion's lines, spirit spheres (two forms), Blade Stop, combo delay (THREADS.md #25) |
| 0x01f1 to 0x01ff | absent | 15 new ids | not yet looked at |

So a server for that client needs its own packet lengths and the two
extra bytes in the unit packets; everything else it shares with ours.

**Skills.** The exe holds 128 Aegis names of 2-2 skills (`CR_`, `MO_`,
`SA_`, `RG_`, `AM_`, `BA_`, `DC_`, `BD_`); `yuno_lang/data/skillnametable.txt`
and `skilldesctable.txt` name and describe every one in English. Ours stop
at the 2-1 jobs, which is why the 2-2 drafts are GM-command only here.
Lines of its text that settle a question:

* Shield Reflect: "Reflect damage to enemy against short-distance attack
  ... Reflect only physical attack and you can use this skill only when
  you putting on shield." (Our draft follows the iRO guide's "full
  damage taken"; THREADS.md #25, DEVIATIONS.)

**Other tables changed** in the language patch or the base GRF: item names,
descriptions and resource names (`idnum2item*`, `num2item*`),
`msgstringtable.txt`, `mp3nametable.txt`, `pettalktable.xml`,
`mapnametable.txt`, `cardpostfixnametable.txt`, `num2cardillustnametable.txt`,
`metalprocessitemlist.txt`, and two new ones (`ba_frostjoke.txt`,
`dc_scream.txt`: the lines of Frost Joke and Scream).

**Maps.** With the Juno patch files applied, 35 maps that we have differ
from ours cell for cell (compared with the merged GRF we play, 2026-11-11):
`aldebaran beach_dun2 c_tower1 c_tower2 c_tower4 cmd_fild02 cmd_fild07
cmd_in01 gef_fild07 gef_fild11 gefg_cas02 gl_cas02 gl_sew02 gl_sew03
in_sphinx1 iz_dun03 job_sword1 mag_dun02 mjolnir_01 mjolnir_05 moc_fild07
moc_pryd01 pay_fild10 payon_in02 prontera prt_church prt_fild01 prt_fild05
prt_in prt_maze02 quiz_00 tur_dun01 tur_dun03 xmas yuno_in03`. Three are
new: `sec_in01 sec_in02 sec_pri`. The base GRF also adds `tur_dun06`
(THREADS.md #26 asks how `tur_dun05` was entered). Every NPC, warp and
script cell on those maps must be re-checked for that version: run
`ragnarok-offline check` against its data, and `research/gat_cells.py`
on single cells. Places this game already relies on that are among them:
the Hunter test's guild master (`payon_in02 21,31`) and the Hunter guild's
door (`pay_fild10`), the Swordman course (`job_sword1`), the Priest test's
church (`prt_church`), Prontera and its inns (`prt_in`), the Clock Tower's
random warps (`c_tower1`, `c_tower2`, `c_tower4`), Turtle Island
(`tur_dun01`, `tur_dun03`), the Morroc pyramids (`moc_pryd01`).

## What that version would switch on

* `jobs2_2` by default, and the 2-2 skills that need the packets above:
  the Monk's spirit spheres and combos, Auto Spell's list, Devotion,
  Blade Stop; the drafts behind `jobs2_2` now (DEVIATIONS.md: the
  Crusader's shield skills, Providence, the endows, Dispell, the Strips,
  the Chemical Protections) checked against its skill text.
* Juno, if it is in that client's data and on its patch list by then
  (euRO opened Juno with the jobs, 2004-06-09).
* Whatever else euRO changed between 2003-12 and 2004-05 that the client
  shows (the item tables above).

## Open, for later

* What the two extra bytes in 0x0078 to 0x007b are (queue T11).
* What 0x01f1 to 0x01ff are, and whether the server must send any.
* The exe of 2004-05-31 (patch entry 462) is still lost; 2004-05-07 is
  what we have.

## The path finder (2026-11-13)

The 2003-12 exe's `CPathFinder` (`0x4d2300`, docs/CLIENT.md) is ported in
`path.rs` and checked under an emulator (`research/client_paths/`). For the
May-2004 exe: find the same function by its log strings ("find path
failed : no path found"), point `emu_path.py`'s addresses at it (the
function, `operator new`/`delete`, `atexit`, `_Lockit`, the error log, the
two static initialisers, the instance) and run the committed fixture
through it. If any path differs, that version needs its own finder.
