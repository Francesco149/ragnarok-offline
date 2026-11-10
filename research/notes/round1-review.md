# Round 1 review: what held up, what did not, what changed

Round 1 was a deep-research pass by an outside agent (Antigravity/Gemini),
briefed in `research/agy-threads/HANDOFF.md`. Its report is
`research/agy-threads/REPORT-antigravity.md`, with its own notes and
excerpts next to it. This file is the second pass, of 2026-11-10: which of
its claims were loaded again from the source, what they turned out to say,
and what was changed in the game because of them.

Read the report with this file open. It states most things with "High"
confidence, including claims that rest on later sources (rAthena, iRO Wiki,
RateMyServer, Divine Pride) and a few that the sources contradict.

Tags: **confirmed** = the source was loaded again here and says it;
**corrected** = loaded again, and it says something else; **not re-checked**
= still only the report's word.

## Sources loaded again in this pass

| Id | Source | How |
|---|---|---|
| C1 | `data\resnametable.txt` and `data\indoorrswtable.txt` in the CD's `data.grf` | `research/grf_peek.py cat` on `data.grf.original` |
| C2 | The client's maps, through the running server (`walkable`, `map_size`) | `ragnarok-offline eval` |
| M1 | mod0631 `conf/npc_town_kafra.txt` (archived 2003-12-18; 5,927 lines, Japanese, jAthena lineage) | local archive |
| J1 | jAthena git history, `conf/npc/npc_town_kafra.txt` at `3afe25f` (2004-01-31, //0763) | local clone |
| E1 | eAthena 817 `npc/mobs/gvg.txt`, `npc/other/arena.txt`, `npc/other/kafra.txt` | local archive |
| K1 | kRO notice of 2004-02-24 (gnjoy seq 1182), Korean text | Fandom API, `RO_Patch_(2004_Feb._24)` |
| K2 | kRO notice of 2004-10-12 | Fandom API, `RO_Patch_(2004_Oct._12)` |
| K3 | kRO notices of 2004-02-10 and 2004-02-17 | Fandom API |
| K4 | kRO notice of 2003-05-20 | Fandom API |
| I1 | iRO guide "Upgrading System", capture of 2004-06-05 | `web.archive.org/web/20040605070057/http://iro.ragnarokonline.com:80/game/refinesystem.asp` |

## Thread by thread

### #6 Start cell: the map is confirmed, the cell is still Athena's

**Confirmed (C1).** The client redirects the newer map names itself:

```
new_1-1.gat#new_zone01.gat#      (also new_2-1 .. new_5-1, and .gnd/.rsw)
new_1-2.gat#new_zone02.gat#      ... new_1-4 -> new_zone04
sword_1-1.gat#job_sword1.gat#    priest_1-1.gat#job_priest.gat#
knight_1-1.gat#job_knight.gat#   hunter_1-1.gat#job_hunter.gat#
wizard_1-1.gat#job_wizard.gat#   force_1-1.gat#force_map1.gat#  (-2, -3)
ordeal_1-1.gat#ordeal_a00.gat#   ordeal_1-2 -> a02, 1-3 -> a03, 1-4 -> a04
pvp_y_N-1 -> prontera, -2 izlude, -3 payon, -4 alberta, -5 morocc
pvp_n_N-1 -> prt_maze02, -2 job_hunter, -3 job_wizard, -4 job_priest, -5 job_knight
guild_vs1 -> job_wizard, guild_vs2 -> job_priest, guild_vs3 -> job_knight
```

143 `.gat` lines in all. So a server of December 2003 sent `new_1-1` and
this client drew `new_zone01`: "the training ground under an older name" is
the client's own statement now, not only our cell check (THREADS R2).

**Still open.** The cell `53,111` remains what every Athena of 2003–04 put
in its config. The report's "originated from live observation of jRO" is an
inference from the changelog, and its video is a 2017 private server. Its
"Sprakki at 53,114" argument rests on a 2013 rAthena script.

**Corrected (mod0631 `conf/npc_job_newbie.txt:64-66`).** The report calls
817's usher gift of 150z "an emulator bug" for 50z, because 817's English
line says "Training Subsidy 50 Zeny supplied". mod0631 gives 150 and says
150 ("修練補助費、150Zenyを支給します"). 817's number agrees with it; the
English "50" is the odd one out. Nothing changed: the usher gives 150z.

**New, from C2.** `job_wiz`, `job_knt`, `job_prist` and `job_hunte` (warps
in mod0631, the Wizard test in eAthena 0.5.2) are *not* this client's
`job_wizard`, `job_knight`, `job_priest`, `job_hunter`: 3 of 10, 0 of 4,
1 of 3 and 1 of 2 of their warp cells are walkable. They are later maps.
`ordeal_a03`/`a04` are named in the table but are not in the client's data.

Changed: `tools/import/maps.py` (evidence recorded), PROVENANCE (start
point: map `client`, cell `athena2003`).

### #3 Kafra fares: confirmed, from a better source than the report's

The report argued from jAthena's current `master` and from rAthena. The
period files say the same, and more.

**Confirmed (M1, J1).** mod0631, archived the day the client was built, has
a full Kafra script. It shows and takes Prontera→Geffen 1400 and
Alberta→Prontera 2100 at both Alberta Kafras. So 817's 1800 (charged for
Geffen, shown for one Alberta Kafra) were 817's slips. jAthena of
2004-01-31 agrees on every fare.

**Confirmed (M1, J1).** Izlude's "save inside the city" is `izlude 94,103`
(M1:1532). Teleports land on `91,105`.

**Confirmed (M1), with different numbers than the report's.** Storage is
priced per place: Prontera 40 (also `prt_fild01`), `prt_fild05` 30, Izlude
30, Byalan 35, Geffen 30, Morroc 40, the Pyramid (`moc_ruins`) 55, Alberta
50, the Sunken Ship 55, Payon 40, the Archer Village 45, Al De Baran 40,
the Orc Dungeon 55, Mt. Mjolnir 55, Comodo 80 (40 in `cmd_in02`), Pharos
80. 817, 659 and 715 charge 30 everywhere. A cart is 800 at every Kafra that rents one.

A second witness in the same archive: the training ground's instructor
(mod0631 `conf/npc_job_newbie.txt:717`) tells novices "倉庫の保管料は 30 Zeny
から 55 Zenyまで 地域ごとに違うから注意してくれ" (the storage fee runs from 30
to 55 Zeny depending on the region). Comodo's 80 is outside that range:
the lecture is older than Comodo.

**Confirmed (M1).** Routes 817 lacks: Prontera→Comodo 2000, Morroc→Comodo
1500, Al De Baran→Geffen 1600 and →Mt. Mjolnir 1700, and Kafras in Comodo
(town, inn, beach/Pharos), at Mt. Mjolnir and at Prontera's east and west
gates. The client has the Comodo Kafra's sprite (721) and cut-in
(`kafra_07`), and draws them.

**Confirmed (K1).** The kRO notice lists only the new fares. It also says
the Kafras at the Mjolnir mine, the Orc Dungeon, the Pyramid, the Sunken
Ship, Byalan and the Archer Village "no longer offer the save service"
(저장 서비스). So until then they saved, and Kafras stood at Byalan and Mt.
Mjolnir. Its new table has Comodo and the Pharos lighthouse as Kafra
stations, which fits M1's.

**Caveat.** M1 and J1 follow jRO. That kRO, iRO and euRO had the same
numbers in December 2003 is likely (817's English menus match fare for
fare) but not shown. The fares of iRO and euRO themselves are still
unattested.

Changed: `scripts/npc/kafra.lua`: storage fee per Kafra, Izlude's save
cell, the Comodo and Al De Baran routes, six Kafras 817 did not have.

### #4 Kafra points: the report is wrong

The report says points and pass were "disabled across official servers in
late 2003" and should stay off.

**Confirmed (K4).** 2003-05-20: "카프라이용권 메뉴가 사라졌습니다" (the Kafra
*voucher* menu is gone).

**Corrected (K3).** 2004-02-10: "카프라 포인트를 가져가면 보상해주는 NPC가
강화되었습니다. 묵혀놨던 카프라 포인트를 싸들고 바로 알데바란 본사로 달려가 보시기
바랍니다." The NPC that rewards Kafra *points* was improved; take your
saved-up points to the Al De Baran headquarters. So points existed on kRO
in February 2004 and had been accruing before. J1 adds a tenth of every
teleport fare to `KF_PIT` and has a "check Kafra points" menu entry.

So the voucher went in May 2003, the points stayed. 817's exchange NPC in
the Al De Baran headquarters had a real counterpart. It is still not
imported (its menu never worked in 817), and the thread stays open: what
did points buy, and did iRO/euRO have them?

### #18 Arena: confirmed

**Confirmed (K2).** 2004-10-12, main servers: "오랜시간의 재공사끝에 이즈루드
마을에 아레나가 추가되었습니다" (after a long reconstruction, an arena was added
to Izlude).

**Confirmed (E1).** 817's arena is entered from an NPC in the middle of
Prontera (`prontera 160,185`) and runs on `prt_are_in` and `force_1-1`. It
is the emulator's own.

The client of December 2003 does ship `prt_are_in`, `arena_room` and
`force_map1-3` (C1 lists the first two as indoor maps), so an arena was
being built. It was not open.

Changed: `[game.content] arena`, off by default.

### #17 Castles: one period fact that needs no invention

**Confirmed (K3).** 2004-02-10: "when a guild that owned a castle
disbanded, monsters appeared in the castle in large numbers", guilds
disbanded and re-formed to level on them, and the patch stopped it.
2004-02-17 repeats it for castles given up.

**Confirmed (E1).** 817's `npc/mobs/gvg.txt` has one `mobcaller` per
castle: at `OnAgitStart`, if the castle has no owner, dozens of monsters
(58 to 90 in the four castles counted: Raydric Archers, Khalitzburgs,
Abysmal Knights, Wanderers, a Bloody Knight, a Dark Lord...).

A world with one player is a world where no castle has an owner. Until
2004-02-10 that meant monsters. That part needs no design.

Changed: `scripts/npc/castles.lua` and the translated
`castles/monsters.lua`, behind `[game.content] castle_monsters` (on).

**Not re-checked.** Everything else the report says about WoE: the
treasure-room formula `floor(economy / 5) + 4`, the drop rates, investment
costs, the guild-dungeon levers and MVP timers. Its sources there are
rAthena's current scripts, iRO Wiki Classic and Divine Pride: tier `later`.
The iRO guide pages it cites for the master room ("1 rare item will be
given once a day") are period and worth loading. The three design options
are the report's own. The decision is the owner's.

### #9 Refining: the official page says what it says

**Confirmed (I1).** "If the upgrade fails, the item, Zeny used, and any
cards in the slots will all be destroyed completely".

**Confirmed (I1), against the report's reading.** The same page: Phracon
"2000 zeny", Emveretarcon "1,000 zeny"; fees 50 / 200 / 5,000 / 5,000 for
weapon levels 1–4 and 2,000 for armor; "+1 increases 2 / 3 / 5 / 5 damage";
armor "Each +1 adds 1 defense"; safe to +7 / +6 / +5 / +4, armor +4.

The report calls the 2000z a typo, the level-4 row a typo and the armor
line wrong (0.7), on the strength of emulator data and a 2002 guide it
could not quote. That may be right: the level-4 row repeats level 3's, and
mod0631 sells Phracon for 200. But it is the only official statement we
have, and it is a known disagreement, not a settled one (PROVENANCE,
"Known period disagreements"). Nothing changed.

### #12 Map flags: the count is wrong and the inference does not hold

**Corrected (C1).** `indoorrswtable.txt` has 22 lines, not 31. They are
`alberta_in izlude_in gef_tower geffen_in moc_castle morocc_in payon_in01
payon_in02 prt_in prt_castle prt_church in_orcs01 in_moc_16 aldeba_in
monk_in prt_are_in arena_room sword_1-1 sword_2-1 sword_3-1 xmas_in
cmd_in01 cmd_in02`.

That table tells the client which maps are interiors (camera, lighting).
Nothing in it says the server refused Teleport there. Nothing changed.
"nopenalty in towns was an emulator invention" rests on a current rAthena
file: not re-checked, nothing changed.

### #8 Wizard test: the script the report recommends is for another map

The report says to take the three-stage test from eAthena 0.5.2. That test
runs on `job_wiz`. This client's test map is `job_wizard` (the client
aliases `wizard_1-1` to it), and the two are different maps (see #6). A
port needs the rooms found on the older map first. K3 does show the test
existed on kRO by 2004-02-10 ("fixed: no recovery when moving to the next
stage of the Wizard job test"). Nothing changed.

### Not re-checked at all

| Thread | The report says | Why it needs a second look |
|---|---|---|
| #1 rates | No number was ever published; open beta 1x | Plausible; its Wayback captures were not loaded here |
| #5 2-1 jobs | Instant at job 40 before New Comodo, quests from 2004-01-08/18 | Its evidence for "instant" is the emulators plus the "New Comodo adds 2nd job quest" line: an inference, as before |
| #2 NPC text | Replace 817's dialogue with rAthena's "Aegis conversions" | Tier `later` (2006+), and their origin is the leaked Aegis scripts this project rules out as a source. Not done. The iRO bug-board captures are a real lead |
| #7 quest skills | Thief skills from "Alcouskou" in the Pyramid | From a 2006 GameFAQs revision and RateMyServer |
| #14 pets | 24 pets in the iRO guide, mod0631 has 25 | The pet lines quoted from iRO bug 4201 are worth loading |
| #19 Lutie | Permanent town; the patcher's `xmas` branch only checks for a local file | The patcher claim is checkable in `Ragnarok.exe` |

The report's "next step" for #17 suggests reading discussions of the leaked
Aegis server. That stays ruled out (THREADS.md).
