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
(patch): 269 maps after merging. It has the *older* novice training ground
`new_zone01..04`, not the 2004 `new_1-x` maps the Athena servers of the time
used. euRO itself only opened Juno and the 2-2 jobs on 2004-06-09, so a
faithful "closed beta" world would keep Juno closed (see ROADMAP).

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
