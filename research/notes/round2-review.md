# Round 2 review: what held up, what did not, what changed

Round 2 was the second deep-research pass by the outside agent
(Antigravity/Gemini), briefed in `research/agy-threads/HANDOFF-r2.md`. Its
output is `research/agy-threads/r2/`. This file is the second pass over it,
of 2026-11-10: every claim that could change the game was loaded again from
its source.

**How to read `r2/`.** It has three layers, and they are not equally good.

| Layer | Files | Verdict |
|---|---|---|
| The summary | `r2/REPORT.md`, `r2/evidence/*/*.txt` written by the orchestrator (`alcouskou`, `mod0631_summary`, `jathena_2_1`, `wizard_arena`, `woe_history`, `kafra_points`, `official_woe_rules`, `treasure_and_investment`, `official_dialogue_samples`, `refine_history`, `euro_cb_rates`, `start_cell`) | **Do not use.** Quotes, file:line references, commit hashes, capture dates and whole table rows are invented, including in the "20 / 20 PASS" self-check. Details below |
| The sub-agents' notes | `r2/notes/r1-t4-woe-web.md`, `r1-t6-refine-web.md`, `r1-t7-rates-web.md`, `r1-t5-text-web.md`, and `r2/evidence/t7/{rates,kafra,start-cell}-evidence.txt`, `t8/*.txt` | Mostly right. Every quote checked here was on the page it names. They mix period sources with later ones (GungHo's current guide, an atwiki, iRO Wiki) and say so |
| The generated tables | `r2/t1-mod0631-vs-817.tsv`, `r2/t2-jathena-history.tsv` | Mechanical and usable as indexes. `t5-official-text.tsv` is the client's own tables and is not committed (below) |

Where the summary and a note disagree, the note was right every time.

Tags as in round 1: **confirmed** = loaded again here and it says so;
**corrected** = loaded again and it says something else; **invented** = the
cited source does not contain it, or does not exist; **not re-checked**.

## Sources loaded again in this pass

| Id | Source | How |
|---|---|---|
| M1 | mod0631 `conf/npc_town_kafra.txt`, `npc_town_aldebaran.txt`, `npc_event_skillget.txt`, `npc_event_mobtim.txt`, `npc_job_2nd.txt`, `npc_town_refine.txt`, `npc_warp25.txt`, `map_athena.conf`, `char/char.c`, `db/refine_db.txt`, `db/mob_skill_db.txt` | local archive, `iconv -f cp932` |
| J1 | jAthena git history: //0505 `f967870`, //0587 `78faa42`, //0795 `1fcb977`, //0800 `d328a26`, //0843 `9e4f364`, //0933 `a99aa5a`, //1021 `2dd4988`, //1035 `ebb4c38` | `git -C refs/jathena show` |
| E1 | eAthena 817 `npc/other/kafra.txt` (reserve check, exchange counter, pass seller), `npc/guild/` | local archive |
| C1 | The client's `job_wizard.gat`, `job_knight.gat`, `job_hunter.gat`, `job_priest.gat`, item and skill name tables | `research/gat_cells.py`, `grf_peek.py` on `data.grf.original` |
| I1 | iRO "Upgrading System", captures 20040605070057, 20040810050735, 20041205192936, and the CDX list of all captures | `web.archive.org/web/<ts>id_/http://iro.ragnarokonline.com:80/game/refinesystem.asp` |
| I2 | iRO "The War of Emperium", capture 20040405051438; "Guild", capture 20040404013538 | `.../game/warofemperium.asp`, `.../game/guildsystem.asp` |
| I3 | iRO "Cute Pet System", capture 20040605065452 | `.../game/qpetsystem.asp` |
| K1 | kRO notices: 2003-02-04 (seq 360), 2003-06-10 (645), 2003-07-29 (751), 2003-08-05 (770), 2003-10-07 (899), 2003-12-26 (1055), 2004-02-03 (1134), 2004-02-10 (1154), 2004-02-17 (1167), 2004-04-27 (1325) | Fandom API, `RO_Patch_(<date>)`, Korean text |
| G1 | Gamona forum, "Open Beta" thread t=9857 (capture 20040213093633) and "[euRO] Details zum 'new Comodo' Patch" t=9778 (capture 20040213074848) | Wayback `id_` |
| D1 | RODataZone job pages, captures of 2003-10 to 2004-02 (Thief: 20031204065516) | Wayback `id_`, `www.rodatazone.com/jobs/<n>/<job>.php` |
| I4 | iRO guide "1st job" page, capture 20040204134748 (the note cites 20031228223806, which redirects there) | `.../guide/r_guide1.htm` |
| F1 | euRO: front page of 2003-12-20 (capture 20031223001708); forum topic p=2187 (20031227055437); forum index f=3 (20031227055051); wipe notice t=2400 (20040405184353) | Wayback `id_` |

## Where our own brief was wrong

`HANDOFF-r2.md` T3 says mod0631's Kafra script has no `KF_PIT`. It has 131
uses (M1). The report caught it; that is the one place where its summary
corrected us.

## Thread by thread

### #4 Kafra points: mod0631 has them; the rule is a tenth of a teleport fare

**Confirmed (M1).** Every Kafra with a teleport menu adds a tenth of the
fare: `set Zeny,Zeny-900; set KF_PIT,KF_PIT+90;` (`npc_town_kafra.txt:31-32`,
Prontera to Izlude), 1400 → 140, 1700 → 170, 1800 → 180, 2000 → 200. Storage,
carts and saving add nothing. The main menu has "積立ポイント確認" (check
accumulated points): "積立ポイントとは、カプラサービスの空間移動サービスをご利用
されると、そのご利用度数に応じて積み立てられるマイレージプログラムです。…積立
ポイントはカプラ本社で様々な商品と引き換えが可能です" (a mileage programme for
the teleport service; points can be exchanged for goods at the Kafra head
office). No NPC in mod0631 spends them.

**Invented.** The report's examples ("600z → +60", "1200z → +120", storage
"+4"), its "Diforte at `aldeba_in 93,54`", and its prize table "from kRO
notice 1154" (500 points one Sweet Potato, 1,000 a lottery ticket, 5,000 a
Kafra Headband). The notice (K1, re-read) says only that the NPC rewarding
Kafra points "was improved" and to take saved-up points to the Al De Baran
head office.

**Confirmed (E1).** The only prize list in a period file is 817's counter at
`aldeba_in 79,161` (`npc/other/kafra.txt:995`): Sweet Potatoes at 100p = 7,
200 = 15, 300 = 25, 400 = 35, 500 = 50, 600 = 60, 700 = 75, 800 = 85,
900 = 100, a "1st Lottery Chance" at 1000p; Red Potions at 1100p = 7,
1300 = 15, 1500 = 25, 1700 = 35, 1900 = 50, 2100 = 60, 2300 = 75, 2500 = 85,
2800 = 100, a second lottery at 3000p. "Window I am at allows you to use
special reserve from 100p to 3000p" implies further windows. No branch does
anything: each is a bare `close`, and the lottery prize is the literal text
"+ itemName +".

**Confirmed (M1), new.** The Kafra voucher was alive in mod0631: the head
office clerk "カプラディフォルテー" at `aldeba_in 84,166` sells "カプラ利用券"
(item 1084, the client's `Kafra_Pass`) for 2000z, and every Kafra has
"カプラ利用券を使用する": one ticket makes every service free for one
conversation. 817 has the same seller at `aldeba_in 81,166`. kRO removed the
voucher menu on 2003-05-20 (round 1, K4), so the two emulators kept something
kRO had dropped, or jRO kept it. Nothing changed here: no "Use Kafra Pass".

Changed: `scripts/npc/kafra.lua` earns a tenth of each teleport fare into
`KF_PIT`, "Other Information Check" has 817's "Special Reserve Check", and
817's counter is back with code behind it (no lottery). DEVIATIONS.

### #7 Thief quest skills: one NPC in the Thief guild, four skills

**Confirmed (M1).** `conf/npc_event_skillget.txt:2004`, "アルコスク" at
`moc_prydb1 154,128`, sprite 118, loaded by mod0631 (`map_athena.conf:59`).
(The report cites `npc_quest_skill.txt:463`, a file that does not exist.)
For Thieves, Assassins and Rogues:

| Skill | Items | Thief job level |
|---|---|---|
| 149 Sand Attack | 5 Fine Grit (7041), then a Leather Bag of Infinity (7042) from the pouch maker at `payon 30,120` for 1 Earthworm Peeling, 1 Cobweb, 1 Cactus Needle | 25 |
| 150 Back Slide | 20 Grasshopper's Legs | 35 |
| 151 Find Stone | 1 Zargon, 1 Bear's Footskin, 5 Spawns | 20 |
| 152 Stone Fling | 2 Garlets, 2 Scells | 15 |

**Confirmed (C1).** The client has items 7041 and 7042 and all four skill
names, so the Sand Attack quest belongs to this version. Both cells are
walkable on this client's maps.

**Confirmed (D1), an independent period witness.** RODataZone, an
English-language database for the international servers, capture of
2003-12-04: "With the Comodo patch there are now 4 different Skills that a
Thief can learn through quests", each from "Alcouskou in the right-hand
room of the Thief Guild (moc_prydb1, 154/128)", with the items of the table
above, and the bag from "Bag Seller in Payon Town (payon, 28/121)" for "1
Earthworm Peeling, 1 Cobweb and 1 Cactus Needle". Two differences from
mod0631: the Bag Seller stands two cells away (mod0631: 30,120), and Stone
Fling is headed "Thief Job 20+" (mod0631 and 817: 15). We keep mod0631's
for both and take the two English names from RODataZone.

817 had the Stone Fling NPC alone, in Morroc. Changed:
`scripts/npc/thief_skills.lua` (our translation of mod0631's Japanese).

### Pets (#14): the Monster Tamers

**Confirmed (I3).** "There are a total of 24 monsters that can be tamed to
be pets." "Monster Tamer: Found in the buildings of Alberta, Izlude, and Al
De Baran, these NPCs have some of the items necessary for taming and pet
accessories available for purchase. Taming Merchant: Located in every town,
except Al De Baran and Alberta, these NPCs sell portable pet incubators, pet
food, and some monster accessories."

**Confirmed (M1).** mod0631 has exactly those three, as script NPCs that
make the item from ingredients, no zeny (`npc_event_mobtim.txt`):
`alberta_in 173,76` (Monster Juice, Singing Flower, Wild Flower),
`izlude_in 129,64` (Book of Devil, No Recipient, Horror of Tribe),
`aldeba_in 168,175` (Skull Helm, Monster Oxygen Mask, Silk Ribbon, Stellar
Hairpin, Tiny Egg Shell, Rocker Glasses). 817 has none.

**Corrected.** The report says `data/db/pets.toml` "agrees exactly with
official iRO data", row by row. The page has no table of pets; it sends the
reader to the Rune-Midgarts Library, which was not loaded. The 24 against
mod0631's 25 (Bongun) stands as in round 1.

Changed: `scripts/npc/monster_tamers.lua`.

### #9 Refining: the page corrected itself

**Confirmed (I1).** The first capture of the page is 2004-06-05 (there is no
"2003-12-11" capture, as the summary has it). Four captures to 2004-06-23
are identical. Then:

| Capture | Phracon | Level-4 weapon | Level-4 safe |
|---|---|---|---|
| 2004-06-05 | 2000 zeny | 5,000 zeny, +5 | +4 |
| 2004-08-10 | **200 zeny** | 5,000 zeny, +5 | +4 |
| 2004-12-05 | 200 zeny | **20,000 zeny, +7** | "No Safety" |

Everything else is the same in all three: Emveretarcon 1,000; fees 50 / 200 /
5,000 and +2 / +3 / +5 for levels 1–3; armor 2,000 and "Each +1 adds 1
defense"; safe to +7 / +6 / +5, armor +4.

**Confirmed (K1), kRO 2003-02-04**, ten months before this client: "무기레벨
4 의 무기를 제련하면 +1 당 7의 공격력 증가를 가져오도록 변경" (level-4 weapons now
gain 7 attack per +1); "4레벨의 무기의 경우 +4 까지는 안전하게 제련" (level 4 safe
to +4); "방어구가 안전하게 제련되는 수치를 +5 → +4" (armor safe limit 5 → 4);
"제련으로 인한 추가 DEF 는 순수한 방어구의 DEF 보다 효율이 낮게 변경" (DEF from
refining is now less efficient than the armor's own DEF); a random extra
hit above a refine level that depends on the weapon level. The notice names
no fee (the summary says it gives 20,000z).

So: Phracon 200z and level 4's +7 are settled; the page was wrong and
Gravity fixed it. The level-4 fee of 20,000z has the later page and 817.
"+1 adds 1 defense" was still on the page in December 2004 (to 2006, by the
sub-agent's note), but kRO's own notice says
refine DEF counts for less than armor DEF, so it is a simplification. How
much less has no official number.

**Confirmed (M1), new.** mod0631's refiners charge ten times as much: 500 /
2,000 / 25,000 / 50,000 and 20,000 for armor (`npc_town_refine.txt`). That
is jRO. It is a reminder that mod0631's zeny amounts are jRO's and need not
be euRO's; the per-place storage fees of round 1 carry the same caveat.

Changed: PROVENANCE ("Known period disagreements"). The game already used
200z, +7 and 817's fees.

### #1 Rates: one player says 3x

**Confirmed (F1).** BHI-OE, 2003-12-20: "As we are testing with higher
experience, we will delete all characters and/or accounts at the end of the
Closed Beta."

**Confirmed (G1).** 2004-01-27, 21:24, "Pandaroc": "Nein Exp und Droprate
bleibt nicht wie sie in der closed war (nämlich 3fach). Sie wird ganz normal
einfach sein ..." (no, EXP and drop rate will not stay as in the closed
beta, namely threefold; it will be plain 1x). 21:43, "Astartus" (the fan
site's admin): "da die Zeit einfach zu kurz, und die EXP und Drop Rate zu
hoch war".

**Corrected.** The summary makes Pandaroc a "Gamona Admin/Mod" and quotes a
different, longer sentence. He is a member who joined that day ("Frischling",
4 posts). So: one player's number, consistent with everything else, never
confirmed by BHI-OE.

**Confirmed (F1), speed of levelling.** The forum index captured 2003-12-27
has "Just became Assassin... Am I the first one on euRO?" (Ryoshin, 2003-12-23
07:17, three days after the start) and "Lvl area for a 48/35 thief?"
(2003-12-22). A signature in a capture of 2003-12-27 reads "Nick: -Barak- /
Job: Knight / Level: 64/34". (The summary's "Bin jetzt in 4 Tagen schon Level
64/34 Knight geworden" is invented; the level is a signature, as of the
capture.) Topics t=67 and t=80, on the rate itself, are still not archived.

Changed: DEVIATIONS (the rates row names the 3x). The default stays 1x.

### #5 2-1 jobs before New Comodo: no quest

**Confirmed (G1).** 2004-01-18, quoting euro.bhi-oe.de: "New in 'New Comodo'
… 1. added heal-effect, modified potions 2. added 2nd job quest 3. added
monster skills 4. added lvl99 aura 5. fixed refining bug 6. adjusted
monsters HP / Exp".

**Confirmed (F1).** There was an Assassin on 2003-12-23 and a Knight by
2003-12-27, before that patch.

**Confirmed (M1).** mod0631's `npc_job_2nd.txt` changes job at job level 40
with no test: Knight `prt_in 88,101`, Priest `prt_church 16,41`, Hunter
`in_hunter 100,99`, Blacksmith `geffen_in 110,170`, Assassin
`in_moc_16 71,88`, Wizard `gef_tower 111,37`.

So the closed beta of December had 2-1 jobs without quests, and the quests
came with New Comodo in January. That is no longer an inference from the
emulators alone. Our baseline is New Comodo; nothing changed.

**Corrected (J1).** The summary's jAthena history is wrong throughout. None
of its six commit hashes exists. //0505 is by Diex. //0933 (2004-04-09,
胡蝶蘭) adds the Wizard test only: "eathenaのデータの翻訳、および某所の昔のデータ
を元に作ったので癌鯖とは微妙に異なってると思います" (made from a translation of
eAthena's data and old data from elsewhere, so it differs slightly from
Gravity's server). The Priest quest arrives on 2004-06-08 (//1013); Knight,
Blacksmith, Hunter and Assassin only in the import of 2004-09-12. //1021's
"凸さんのログ" is the Super Novice. `t2-jathena-history.tsv` itself is fine.

### #8 Wizard test: the old map has five rooms

**Corrected (C1).** The summary calls `job_wizard` "a single contiguous
square arena [47..152] centered at (99, 99)" with "no interior rooms". Cell
99,99 is blocked. The map is a cross: a centre room (roughly x 82–117,
y 86–117) and one room at the end of each arm (north around 99,143; south
99,59; west 60,101; east 140,101), 2,816 walkable cells. `job_knight` has the same plan.
That is room enough for a test of three stages with a waiting room, which
is what the later test has. No period script for this map was found: every
script (//0933, 817, eAthena 0.5.2) is for `job_wiz`.

**Confirmed (C1).** 817's Priest test cells (`100,17`, `36,122`, `162,122`,
`100,176`) are all walkable on this client's `job_priest`.

Nothing changed. A Wizard test on this map would be ours.

### #17 War of Emperium: the official page has few numbers

**Confirmed (I2), every rule on the page:** twice a week; "Only guilds with
the 'Official Guild Approval' skill can participate"; 20 castles; damage not
displayed; effects simplified; emblems shown; Endure does not work; the
defeated return to their save point "and no experience will be lost"; no
Ice Wall; "Skills that push the opponent away will not work"; no Teleport or
Fly Wing, Butterfly Wing allowed; the Emperium takes "only direct physical
attacks"; members of the owning guild "recover their HP/SP faster". The
steward: view status; invest in commerce ("will affect the item type and
amount that can be produced in the castle"); invest in defense ("the castle
Emperium's defense and HP as well as the Guardian's HP"); guardians; hire a
Kafra (free storage for members); "Enter Master Room … 1 rare item will be
given once a day, depending on the Commerce Development Investment"; "Empty
Castle … the castle is no longer under any particular guild's control, and
becomes filled with monsters".

That last line is iRO, April 2004, saying what our castles do.

**Invented.** The summary's rows citing this page for 80% and 60% damage, a
100,000 HP Holy plant Emperium, treasure at "00:00 server midnight", the
formula `4 + floor(Commerce / 5)`, two investments a day, `5,000z × (value +
1)`, the 5-point loss on capture and the lever behind the throne. None is on
the page, and `game/castles.asp` was never archived (the sub-agent's own note
says so).

**Confirmed (K1).** Siege hours fixed 2003-06-10; "길드 보물상자에 길드 마스터
외의 플레이어가 접근 가능한 점을 수정" (treasure boxes reachable by non-masters,
fixed) 2003-07-29; the castle gate flag warps members in, 2003-08-05; no
alliances during a siege, 2003-10-07; no Dead Branch in towns and siege
maps, and guild PvP in the guild dungeon fixed, 2003-12-26; Flora and Marine
Spheres no longer break the Emperium, 2004-02-03; the Emperium becomes Holy
/ angel, 2004-04-27 (so it was neither in our period).

**Not re-checked, tier later.** The treasure count, the investment cost
table (a bracketed table in the note, a different formula in the summary)
and the 5-point loss come from GungHo's guide as it stands today and from a
Japanese wiki. J1: jAthena's first steward (//0795, 2004-02-12, a test
script "converted from the Weiss era") takes a flat 10,000z, and carries the
comment `AgitGet AGIT_D_FUNDING_OK`, which is the official script language:
the emulators' castle scripts were already leaning on Gravity's. //1035
(2004-06-28) says so outright ("edit from Aegis NPC (Chinese-big5
version)") and stays ruled out.

Nothing changed. The owner's choice between A, B and C stands as it was,
with one more period fact for A (the daily item scales with commerce) and
still no period number.

### #2 Official English text: nothing usable

`t5-official-text.tsv` has 2,763 rows: 2,364 from the client's
`pettalktable.xml`, 391 from `msgstringtable.txt`, and 8 from the iRO bug
board (six pet lines and two players describing a bug). The first two are
Gravity's files: the client shows them itself, the server sends none of
them, and they may not be committed. The file is git-ignored.

**Confirmed (I4).** The sub-agent's note quotes the iRO guide's one-line
job descriptions ("Extra Quest : Swordman Test - Pass 3 narrow paths
without failure", "Gather 4 kinds of trunks from Willows", "Mix a Mage Test
Solution", "Deliver a package to a specified location", "Gather Mushrooms",
"Self-Discipline Journey"); they are on the page. They describe the quests;
they are not dialogue.

**Not on the pages cited.** The note's dialogue quotes (Chief Mahnsoo,
Father Mareusis, Sir Syracuse, the Kafra greeting, Hollgrehenn, the Peco
breeder) are credited to RODataZone's front page of 2003-12-11. Neither
that page nor its Merchant, Acolyte and Knight pages contain them (D1): the
site is a database of skills and requirements and quotes no NPC. Where the
lines come from is not stated; treat them as unsourced.

**A real lead in the same note (D1).** RODataZone's job pages do give, in
English and dated, the quest-skill NPCs of late 2003 with names, cells and
items: Nami (`prt_in 234/133`), Leon von Frich, Juan, Knight De Thomas
("1 Rocker Doll, 500 Empty Bottle": 817's numbers, where mod0631 has 200
bottles and another item, so mod0631 is not always the better witness for
the international servers), Necko (`alberta 83/96`), Charlron, Gershaun,
the Cleric (`prt_church 173/23`), the Great Wizard. That is the side-by-side
THREADS #7 asks for, and tier `period`.

**Could not load.** The three bug-board captures the TSV cites (seqNo 4201,
4218, 4222): Wayback answers "has not archived that URL" for each.

### #3 fares, #6 start cell, #19 Lutie, monster skills

* **#3.** Nothing new on iRO's or euRO's own fares. The summary's "flat"
  table (Izlude 600, Geffen 1200) contradicts mod0631 and its own
  sub-agent's note. "euRO kept the old fares until Juno" is an inference.
* **#6.** mod0631 `char/char.c:70`: `struct point
  start_point={"new_1-1.gat",53,111};`. Still Athena's cell. One sub-agent
  note leans on a script from the leaked server; disregard that line.
* **#19.** Not re-run. `Ragnarok.exe` does contain `2003-10-31gdata_xmas.gpf`
  and `notice_xmas.txt`, and the GRF has five `xmas*.gat` maps. The
  disassembly in `r2/evidence/t8/lutie.txt` reads as a patcher check, as
  round 1 said.
* **Monster skills.** G1 confirms "added monster skills" in New Comodo. The
  counts in the report (1,291 and 1,302 entries) are not what `grep` gives
  (1,109 and 1,115 uncommented lines); "the 11 extra are Episode 5 bosses"
  is unchecked.

## Leads from the T1 table, not followed yet

`t1-mod0631-vs-817.tsv` matches by cell only, so "ours = none" overcounts
(it lists our own Kafras). Its ten "rows that matter" in the summary are
partly invented: mod0631 has no NPC at `aldeba_in 93,54`, `alberta 63,42`
or `payon 174,136`, and `morocc 160,258` is a Kafra, not a dyer. Rows worth
reading next, each an NPC mod0631 loads on a cell that is walkable here and
that we lack:

* refiners at `alberta_in 13,71`, `prt_in 56,68`, `morocc_in 63,32`,
  `payon_in01 95,19` (`npc_town_refine.txt`);
* Peco Peco keepers at `prontera 55,350` and `232,318` (`npc_event_rental.txt`);
* the dyers of `npc_event_dye.txt` (`geffen 184,114`, `morocc_in 146,99`,
  `prt_in 243,168`): THREADS #13;
* the three other Kafras of the head office (`aldeba_in 83,233`, `28,238`…);
* `npc_event_ma_sword.txt`, `npc_event_making.txt`, `npc_turtle.txt`;
* the other quest-skill NPCs in `npc_event_skillget.txt` (`prt_in 234,133`,
  `prt_in 73,87`, `prt_castle 175,146`) and the item differences the table
  flags for the ones we have from 817 (first row: Moving HP Recovery).

## What changed in the game

| Change | Source | Where |
|---|---|---|
| Kafra points: a tenth of each teleport fare; "Special Reserve Check" | mod0631 (rule), 817 (wording) | `scripts/npc/kafra.lua` |
| The special reserve counter in the Al De Baran head office | 817 (place, wording, prize list); the hand-over is ours; no lottery | `scripts/npc/kafra.lua` |
| Thief quest skills from Alcouskou in the Thief guild, and the pouch maker in Payon; 817's Sono-san removed | mod0631; translation ours | `scripts/npc/thief_skills.lua` |
| Monster Tamers in Alberta, Izlude and Al De Baran | mod0631, iRO pet guide; translation ours | `scripts/npc/monster_tamers.lua` |
| Tests for all of the above | | `crates/ro-game/tests/npc_mod0631.rs` |
