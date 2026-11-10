# Handoff: deep research, round 3

For: the same independent deep-research agent (Antigravity) that wrote
rounds 1 and 2. Written 2026-11-10 by the Claude session that checked round
2. A later session will run the checker of §2 over your rows and read the
ones that pass.

Read these first, in this order:

1. `research/notes/round2-review.md`: what happened to round 2.
2. `research/agy-threads/HANDOFF.md` and `HANDOFF-r2.md`: project,
   glossary, source catalogue, grades. They still apply. Where this file
   says something else, this file wins.
3. `research/THREADS.md`: the threads as they stand.

Write only inside `research/agy-threads/r3/`. Do not edit anything else and
do not commit.

## 1. What round 2 taught us

Your sub-agents did good work. Every quote we re-loaded from
`r2/notes/*.md` was on the page it named, and four things from those notes
are in the game now.

The summary written on top of them was not usable. `r2/REPORT.md` and the
evidence files written with it contain, among others:

| In the summary | In the source |
|---|---|
| Six jAthena commit hashes (`e2d6ca50`, `4744d0ec`, ...) | None exists. The real ones were one `git log` away |
| `npc_quest_skill.txt:463` for the Thief NPC | No such file. It is `npc_event_skillget.txt:2004` |
| "Damage done by normal attacks is reduced to 80%", quoted from the iRO WoE page | Not on the page. Nor are eight other rows credited to it |
| A capture of the iRO refine page from 2003-12-11 | The first capture is 2004-06-05 |
| "Diforte at `aldeba_in 93,54`", a pharmacist at `alberta 63,42` | No NPC on either cell |
| A German sentence by "Gamona Admin Pandaroc" | Another, shorter sentence, by a member with four posts |
| "Self-check: 20 / 20 PASS" | At least seven of the twenty rows fail |

One cause covers all of it: the summary was written from a memory of the
findings, and the gaps were filled with what such a source would plausibly
say. The sub-agents' notes disagree with the summary in every one of these
places, and the notes were right.

A smaller version of the same thing is in one note: the NPC dialogue in
`r2/notes/r1-t5-text-web.md` (Chief Mahnsoo, Father Mareusis, the Kafra
greeting) is credited to RODataZone pages that do not contain it.

So this round removes the layer where that happened. There is no summary to
write. There are rows, and a program that checks them.

## 2. The protocol

### Rows

Every finding is one JSON object on one line, in
`r3/rows/<task>.jsonl`:

```json
{"id": "W2-014", "task": "W2", "claim": "The steward's commerce investment costs a flat 10,000z in jAthena's first castle script", "value": "10000", "region": "emulator (jAthena)", "source_date": "2004-02-12", "grade": "B", "quote": "set Zeny,Zeny-10000;", "source": {"kind": "git", "rev": "1fcb977", "path": "conf/gvg/TEST_prtg_cas01_AbraiJ.txt", "line": 79}, "reasoning": ""}
```

`source.kind` is one of:

| kind | fields | use for |
|---|---|---|
| `file` | `path`, `line`, optional `encoding` | anything on this machine. `repo:...` is the repository, `data:...` is `~/data/ragnarok-cd` |
| `git` | `rev`, `path`, `line`; or `rev` alone for a commit message. `repo` defaults to `data:refs/jathena` | jAthena history |
| `grf` | `name` (e.g. `data\\skillnametable.txt`), optional `grf` | the client's own files |
| `web` | `url`, `page` | anything loaded from the network. `page` is the file under `r3/pages/` where you saved exactly what the URL returned |
| `none` | `searched` | a negative result: where you looked and did not find it |

Rules for a row:

* **One fact.** If the claim has an "and", it is two rows.
* **The quote is copied, never typed.** It comes out of the tool output of
  the step in which you read the source, in the original language, and goes
  into the row in that same step. If you no longer have the source text in
  front of you, load it again. Do not reconstruct a quote, a line number, a
  commit hash, a URL or a capture timestamp from memory, ever. This one
  rule would have prevented every row in the table above.
* **A quote that contains the value**, for grades A and B. If the source
  only implies the value, set `"value_in_quote": false`, explain in
  `reasoning`, and grade it C or D.
* **Web sources are saved.** Fetch Wayback pages in their `id_` form
  (`https://web.archive.org/web/<timestamp>id_/<url>`), write the bytes to
  `r3/pages/<name>`, and cite the timestamp Wayback actually served. It
  redirects to the nearest capture: round 2 cited 2003-12-28 for a page
  that was served from 2004-02-04.
* **Translations go in `claim` or `reasoning`**, not in `quote`.
* **"Not found" is a row** (`kind: none`) and a good result. A gap you
  report costs us nothing. A gap you fill costs a session.
* Grades as in `HANDOFF-r2.md` §2. A page you could not load is not a
  source, whatever you remember of it.

### The checker

```sh
python3 research/check_rows.py r3/rows/*.jsonl            # as you go
python3 research/check_rows.py --fetch r3/rows/*.jsonl    # before you finish: loads every URL again
```

It opens every file, revision, GRF entry and saved page and looks for the
quote (markup stripped, whitespace collapsed, the file's own encoding). It
prints PASS, WARN or FAIL per row. Fix or delete every FAIL. Read every
WARN: "not near line N; it is at line M" means correct the line.

Save the final, unedited output of the `--fetch` run as `r3/check.txt`. We
run it again ourselves. Then we read passing rows to see whether the quote
means what the claim says, which the program cannot tell. The share of rows
that survive both is the measure of this round, more than the number of
rows.

### Sub-agents and the final step

Sub-agents write rows straight into `r3/rows/<task>.jsonl` and run the
checker on their own file. The orchestrator **does not rewrite, merge by
hand, or summarise rows**. It concatenates nothing and paraphrases nothing.

`r3/REPORT.md` is at most two pages and contains no fact that is not a row
id. Per task: which question got which row ids; what was not found;
recommendations, each naming its rows. No tables of findings, no "key
answers", no self-check section: `check.txt` is the self-check.

Notes (`r3/notes/`) are welcome and unchecked. Nothing in them counts
unless it is also a row.

## 3. What is on this machine

As `HANDOFF-r2.md` §3, with these additions.

| What | Where | Notes |
|---|---|---|
| The checker | `research/check_rows.py` | Its docstring is the reference for the row format |
| The client with its own patches merged | `data:play/RagnarokOnline/data.grf` | **Use this one for the guild castles and guild dungeons.** `prtg_cas01-05`, `payg_cas01-05`, `gefg_cas01-05`, `aldeg_cas01-05` and `gld_dun01-04` are in it and are *not* in `data.grf.original`. `python3 research/gat_cells.py <grf> <map> x,y ...` takes either |
| RODataZone, 2003–04 | Wayback, `www.rodatazone.com/` (CDX lists about 290 pages to 2004-06) | An English fan database for the international servers. Its job pages give NPC names, cells and quest items. It quotes no dialogue |
| What the client says about guilds | `data\skillnametable.txt` (five `GD_` skills), `skilldesctable.txt`, `msgstringtable.txt` (39 lines on guilds and castles) | Client text is Gravity's: quote the line you need, not the table |
| Period emulators on castles | 817 `npc/guild/*.txt`, `npc/mobs/gvg.txt`, `db/castle_db.txt`, `map/guild.c`, `map/battle.c`; mod0631 and 817 `db/mob_db.txt` (1285–1288, 1324–1363); the jAthena history from //0795 | See the caution in W2 |

## 4. Tasks

W comes first and gets most of the effort. Then Q and J. The rest only if W
is done properly. Fewer rows that hold are worth more than coverage.

### W. War of Emperium: how it worked, exactly

**Why.** The owner has decided (2026-11-10): the castles will be reworked
into something one player can do, which keeps the feel of the original. We
do that once we know how the original worked. Round 2 found the official
rules and no numbers. This task is the numbers, the mechanics, and what it
was like to be there.

**W1. The official chronology.** Every kRO notice from the first siege test
(Sakray, April 2003) to 2004-06 that mentions 공성, 아지트, 엠펠리움, 보물상자,
가디언, 투자 or 길드 던전: one row per rule or change, with the notice's date
and seq. The Fandom API gives the Korean text
(`action=parse&page=RO_Patch_(2003_Jul._29)&prop=wikitext`); list the pages
with `action=query&list=allpages&apprefix=RO_Patch_(2003` and `(2004`.
Round 2 has ten of these notices; do not stop at ten.

**W2. What the period emulators implement.** Read, with file and line:

* 817 `npc/guild/`: the steward's menu; what an investment costs, how often
  it may be made and what it changes; when treasure appears, how many
  boxes, which monster ids, on which cells; guardians (ids, cells, cost,
  what defense does to them); the castle Kafra; the flags; the switch into
  the guild dungeon (`gswitch.txt`); what happens when the Emperium breaks
  (`ev_agit_*.txt`); the schedule.
* 817 `map/guild.c`, `map/battle.c`, `map/skill.c`, `map/mob.c` and the
  map flags: every rule that applies only on siege maps (damage, skills
  refused, knock-back, who may hit the Emperium, what guild skills do).
* The same questions for the jAthena history between //0795 (2004-02-12)
  and 2004-05, where it differs.

**Caution.** jAthena //1035 (2004-06-28) is a conversion of the official
scripts and stays out, as does anything later built on it. Earlier files
may already lean on them: jAthena's first steward carries the comment
`AgitGet AGIT_D_FUNDING_OK`, which is the official script language. Where
you see such marks (untranslated official identifiers, variable names in
dialogue like `+ itemName +`), add a row saying so. We need to know which
numbers have an independent origin.

**W3. The data.** From mod0631 and 817 `db/mob_db.txt`: the Emperium
(1288), the guardians (1285–1287) and all forty treasure boxes (1324–1363)
with their drops, as rows (one per box: ids and rates as in the file).
Which box belongs to which castle, from 817's scripts. `db/castle_db.txt`
and `db/exp_guild.txt`. The five guild skills this client has, with the
client's own description of each (`skilldesctable.txt`), and which of them
the emulators implement. The client's message lines about castles and
sieges, by line number.

**W4. The maps.** For each of the 20 castles and 4 guild dungeons, on
`data.grf`: do 817's cells fit (Emperium, steward, treasure room, guardian
posts, flags, the warps of `npc/warps/guildcastles.txt`, the switch's
target)? One row per castle with the `gat_cells.py` result. Name the rooms
a castle has between gate and Emperium, from the warps.

**W5. Numbers from the period.** Investment cost per level, the cap on
economy and defense, the treasure count, the loss on capture, guardian and
Emperium HP against defense: for each, the earliest dated source that
states it. Look at Wayback captures **from 2003–04** of GungHo's guide
(`ragnarokonline.gungho.jp`), of kRO's guide, of jRO and kRO fan sites of
the time, and of RODataZone. A page as it stands today is grade C at best,
whatever it describes. If the only source for a number is later than
2004-06, the row says so; that is the answer.

**W6. What it was like.** First-hand accounts dated 2003–04: euRO's first
sieges (the forum and news of April to September 2004, `ro-guardians.com`,
the Gamona forum), iRO and kRO reports, dated screenshots and videos. We
want rows for: how many people, how long a castle held, where the fighting
happened (which rooms, the gate, the Emperium room), what defenders
prepared, what a guild did with a castle between sieges (treasure, the
guild dungeon and what was hunted there, the Kafra), what was prized. Quote
the players. This is what the single-player version has to feel like, so
plain description beats statistics here.

**W7. euRO.** When each piece arrived on euRO (first siege, treasure,
guild dungeons, the September 2004 notice that disabled treasure and
dungeons), from euRO's own news and forum.

### Q. Quest skills, three witnesses side by side

For every quest skill of the Novice and the six first jobs: the NPC, its
map and cell, the items, the job level, in (a) 817 as imported
(`scripts/import/npc/skillquests/`, with the `src` tag), (b) mod0631
`conf/npc_event_skillget.txt`, (c) RODataZone's job page, the capture
nearest 2003-12-18. One row per witness per skill. Include First Aid and
Play Dead in full (three NPCs, the timed step): they are not in the game
yet. Mark every difference; do not resolve it.

### J. The 2-1 job quests as "New Comodo" brought them

euRO added "2nd job quest" in January 2004. For Knight, Priest, Wizard,
Blacksmith, Hunter and Assassin: what the test consisted of in late 2003
and early 2004, step by step, from dated sources (RODataZone's 2-1 pages,
iRO guide pages and notices, kRO notices with 전직, period walkthroughs).
For the Wizard test above all: how many rooms, what happens in each, in
what order. This client's `job_wizard` is a cross of five rooms (a centre
and four arms) and no script for it survives, so a description is what we
can build from. Say which map each test ran on if a source shows it.

### D. Dialogue, with the source in hand

* Where did round 2's NPC lines come from (Mahnsoo, Mareusis, Syracuse, the
  Kafra greeting, Hollgrehenn, the Peco breeder)? Find the page that
  contains each, or add a row withdrawing it.
* GameFAQs guides with a version date of 2004-06 or earlier, through
  Wayback (`db.gamefaqs.com/computer/doswin/file/ragnarok_online*.txt`) or
  the laptop: passages where the author reproduces what an NPC says.
* The iRO bug board: list by CDX which `bugcontent.asp` captures exist
  (`matchType=prefix`), then quote from those.

A row here needs the NPC named in or next to the quote.

### Smaller, independent

* **P. Pets.** The iRO Rune-Midgarts Library pages for pets, taming items
  and accessories (2004 captures), and RODataZone's: one row per pet,
  against `data/db/pets.toml`.
* **K. Kafra on the international servers.** Any dated list of teleport
  fares, storage fees, Kafra points or the Kafra Pass on iRO or euRO
  between 2003-07 and 2004-06.
* **R. Refine.** How much DEF a refine level gave after kRO's change of
  2003-02-04, and the random bonus above the safe limit, from period
  sources.
* **M. "Adjusted monsters HP / Exp".** The kRO notice that corresponds to
  New Comodo's point 6, and a count from the local files of how mod0631's
  `mob_db.txt` differs from 817's in HP and EXP (Baphomet is the known
  case).

## 5. What to hand back

```
r3/rows/<task>.jsonl    the findings
r3/pages/               every web page a row cites, as fetched
r3/check.txt            output of: check_rows.py --fetch r3/rows/*.jsonl
r3/REPORT.md            two pages, row ids only
r3/notes/               optional
```

If you run out of budget, stop with fewer tasks done and every row checked.
An honest `REPORT.md` that says "W1 to W4 done, W5 found nothing from
before 2005, the rest not started" is a good round.
