# Handoff: deep research, round 2

For: the same independent deep-research agent (Antigravity) that wrote round
1. Written 2026-11-10 by the Claude session that checked round 1 and
changed the game from it. A later session will again load every source you
cite before anything goes into the game.

Read these first, in this order:

1. `research/notes/round1-review.md`: what happened to your round-1
   report. Which claims held when the source was loaded again, which did
   not, and which nobody could check.
2. `research/agy-threads/HANDOFF.md`: the round-1 brief. The project
   description, glossary and source catalogue there still apply and are not
   repeated here.
3. `research/THREADS.md`: the threads as they stand now.

Write only inside `research/agy-threads/r2/`. Do not edit anything else and
do not commit.

## 1. What round 1 taught us

Round 1 found real things: the client's `resnametable.txt`, the kRO arena
date, the castle-monster notices, the iRO refine page. Those are in the
game now.

It also had four errors, and they have one cause. Each was a claim stated
with "High" confidence that rested on a later source or on an inference,
while a period source that said otherwise was within reach:

| Round-1 claim | What the period source says |
|---|---|
| Kafra points were disabled everywhere in late 2003 | kRO's notice of 2004-02-10 improves the NPC that rewards Kafra points. Your report cites that same notice for the castle monsters |
| The usher's 150z is an 817 bug for 50z | mod0631 gives 150z and says "150Zeny" (`conf/npc_job_newbie.txt:64-66`) |
| Take the Wizard test from eAthena 0.5.2 | It runs on `job_wiz`. This client's map is `job_wizard`, and the cells do not fit (3 of 10) |
| `indoorrswtable.txt` has 31 maps and they block Teleport | It has 22, and nothing in it is about Teleport |

A fifth pattern: the start-cell check used a map cache from `refs/rust-ro`,
which describes a later version of the map, not this client's.

So this round changes the method more than the questions. The most useful
period sources turned out to be **already on this machine**, and round 1
mostly went to the web and to rAthena instead. In particular, mod0631 (the
emulator release archived the day the client was built) carries full NPC
scripts in Japanese that nobody has read against our English ones.

## 2. Rules for this round

The round-1 rules stand (read-only; no posting; no CAPTCHAs; no executables
or disc images). These are added or sharpened.

**Leaked Gravity server files stay out, and so does what is made from
them.** That includes rAthena's and Hercules' `npc/pre-re` and `npc/re`
scripts where the header says they were converted from the official
(Aegis) scripts. You may use them as a **lead**, to learn what to look for
(an NPC's name, that a quest exists). You may not report a value, a
coordinate or a line of dialogue on their authority. Do not open
discussions of the leak itself.

**Every finding is one row with these fields**, so that it can be checked
without reading your prose:

| Field | Content |
|---|---|
| `claim` | One fact. Not two joined by "and" |
| `value` | The number, cell, name or text itself |
| `region` | kRO / iRO / jRO / euRO / emulator (and which lineage it follows, if known) |
| `source_date` | When the source was written or captured. Not when the fact was true |
| `source` | Local: `path:line` (for git history: `rev:path:line`). Web: full URL with the Wayback timestamp |
| `quote` | The source's own words, original language, enough to find it. Japanese and Korean stay in the original, with your translation after |
| `grade` | See below |
| `reasoning` | Only if the claim is more than the quote says: what you inferred, and from what |

**Grades replace "confidence".** Do not write high, medium or low.

| Grade | Means |
|---|---|
| **A** | A period primary source says it and you quote it: the client's own files, or official material (notices, guide pages, publisher posts) dated 2004-06 or earlier |
| **B** | A period emulator file (2004-05 or earlier) says it and you quote it with file and line. Say which official server it follows, if the file shows it |
| **C** | Only later sources (after 2004-06), two or more, independent of each other. Say why they would still describe 2003 |
| **D** | One later source, or your inference |
| **X** | You looked and the sources disagree. Give both rows |

A claim graded A or B must carry a quote that contains the value. If the
quote does not contain it, the grade is D.

**Before you grade anything C or D, check the local period sources (§3) for
the same fact.** Say in the row that you did, and what they say. This is
the check that would have caught every round-1 error.

**Coordinates are checked against this client's map**, with
`research/gat_cells.py` (§3). A script whose cells are blocked on our map
was written for another map; say so.

**Separate what the source says from what we should do.** Recommendations
go in their own section at the end, and each names the rows it rests on.
We decide.

**A source you could not load is a lead, not evidence.** List it under
"could not load" with the error. Do not fill the gap from memory.

## 3. What is on this machine

Paths are on the host. In the workbench container (`tools/wb exec ...`),
`~/data/ragnarok-cd` is `/data` and the repository is `/work`.

| What | Where | Notes |
|---|---|---|
| The client's data | `~/data/ragnarok-cd/play/RagnarokOnline/data.grf.original` | `python3 research/grf_peek.py list <grf> [substring]` and `... cat <grf> 'data\name.txt'`. Text is CP949 |
| The client's maps | same | `python3 research/gat_cells.py <grf> <map> x,y ...` says whether cells are walkable **on this client**. It follows `resnametable.txt`, so `new_1-1` answers for `new_zone01` |
| The client's strings | `~/data/ragnarok-cd/ragexe.strings`, `Ragexe.exe`, `Ragnarok.exe` in the play folder | rizin is in the workbench |
| mod0631 (2003-12-18) | `~/data/ragnarok-cd/refs/eathena-archive/x/athena-dev-2.1.1-mod0631/athena-dev-2.1.1-mod0631/` | `conf/npc_*.txt`: full NPC scripts, **Japanese, CP932**, jAthena lineage. `db/`, `map/*.c`. Our numbers source |
| Athena 659 (2004-01-01), 715, 817 (2004-02-19), eAthena 0.5.2 (2004-05), 601, 629, 641, 652 | same directory, one folder each | English. 817 is our NPC source |
| jAthena history | `~/data/ragnarok-cd/refs/jathena` (a git clone without a checkout) | 463 commits from 2003-10-30 (//0505). `git -C <dir> log --date=short -- <path>`, `git -C <dir> show <rev>:<path> \| iconv -f cp932 -t utf8`. Commit messages are the changelog and often say where a script came from |
| euRO pages already fetched | `~/data/ragnarok-cd/refs/euro-web-wayback`, `euro-patch-wayback` | |
| Later emulators | `refs/rathena`, `refs/hercules`, `refs/rust-ro`, `refs/roBrowser` | Grade C/D at best, and see the rule on converted scripts |
| What the game has now | `scripts/import/npc/**` (817, translated), `scripts/npc/*.lua` (ours), `data/db/*.toml` | Each imported NPC has `src = "817:file:line"` |

Web access as in round 1. Two things that worked for us: the Fandom API
(`https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2004_Feb._10)&prop=wikitext&format=json`)
gives the Korean notice text, and Wayback's `id_` form
(`https://web.archive.org/web/<timestamp>id_/<url>`) gives the page as it
was.

## 4. Tasks

In order of how much they unblock. Do T1 to T4 properly before the rest. A
task is done when its table exists, not when a paragraph about it does.

### T1. Read mod0631's NPC scripts against ours

**Why.** Our NPCs are 817's (English, February 2004). mod0631's are two
months older, from the day of the client build, and follow jRO closely. The
Kafra script alone gave us per-place storage fees, six missing Kafras and
the Comodo routes. Nobody has read the rest.

**Do.** For each of these mod0631 files, compare every NPC with the NPC on
the same cell (or with the same role) in `scripts/import/npc/`:

`npc_event_skillget.txt` (quest skills), `npc_job_1st.txt`,
`npc_job_2nd.txt`, `npc_job_aco.txt`, `npc_job_merchant.txt`,
`npc_job_thief.txt`, `npc_job_newbie.txt`, `npc_town_refine.txt`,
`npc_town_guide.txt`, `npc_town_*.txt` (the towns), `npc_event_*.txt`,
`npc_cTower.txt`, `npc_turtle.txt`, `npc_pvp*.txt`, `npc_test_arena.txt`.
`conf/map_athena.conf` says which of them mod0631 actually loaded; note
that per file.

**Deliver** `r2/t1-mod0631-vs-817.tsv`, one row per NPC:

`m631_file`, `m631_line`, `map`, `x`, `y`, `sprite`, `name_ja`, `name_en`
(your translation), `role`, `ours` (the `src` tag of our NPC, or `none`),
`same_cell` (yes/no), `differences` (requirements, items and amounts, job
levels, zeny, rewards, warps: each as `ours → mod0631`), `cells_ok` (from
`gat_cells.py`).

Then a short list of what matters most: NPCs mod0631 has and we lack; NPCs
we have that mod0631 lacks; numbers that differ. Known example to start
from: mod0631 has the Thief skill NPC `アルコスク` at `moc_prydb1 154,128`,
where 817 has a broken NPC in Morroc.

Translate requirements and rewards exactly. Do not translate dialogue in
full; one line on what the NPC says is enough.

### T2. Date the jAthena scripts

**Why.** A script in mod0631 tells us what jAthena had on 2003-12-18. The
git history tells us *when each quest arrived and where it came from*.
Commit messages say things like "凸さんのログを基に作成しました" (made from 凸's
logs) and "本鯖使用に変更" (changed to what the real server does), both in
//1021 of 2004-06-11. Those mark scripts transcribed from the official jRO
server, which is the closest thing to an official source we have.

**Do.** For every NPC script file under `conf/` (later `conf/npc/`,
`conf/gvg/`, `script/npc/`): first commit, and each later commit that
changed it up to 2004-06-30, with date, revision (`//NNNN`), author and the
part of the commit message about that file.

**Deliver** `r2/t2-jathena-history.tsv`: `path`, `rev`, `date`, `author`,
`message_ja`, `message_en`, `from_official_logs` (yes / no / unclear, with
the words that say so). Then answer from it, with rows cited:

* When did each 2-1 job quest (Knight, Priest, Wizard, Blacksmith, Hunter,
  Assassin) appear, and was it a simple job change before?
* Which map does each job test use in the first version that has one, and
  do its cells fit this client's `job_knight`, `job_priest`, `job_wizard`,
  `job_hunter` (`gat_cells.py`)? We need a Wizard test that runs on
  `job_wizard` (also reachable as `wizard_1-1`). 817's unfinished
  `npc/jobs/wizard.txt` and 659's `conf/npc/jobs/npc_wizard.txt` are worth
  the same check.
* When did the WoE scripts (`conf/gvg/`) arrive, and from what?

### T3. Kafra points

**Why.** Round 1 got this backwards. Points existed; we do not know what
they did.

**Do.** Trace `KF_PIT` through the jAthena history. It is in
`conf/npc_town_kafra.txt` from the first snapshot (2003-10-30, 129 uses);
mod0631's copy of that script has none, which is worth explaining. List
every script that adds to it or spends it, with the rule
(a tenth of the fare?), the reward NPC, its place, the reward table, and
how that changed up to 2004-06. Then the official side: every kRO notice of
2003 and early 2004 that mentions 카프라 포인트 or 카프라이용권; the iRO guide and
notices; anything from euRO.

**Deliver** rows as in §2, and one table: what a player earned points for,
and what points bought, with the date each rule is attested.

### T4. War of Emperium, from period sources only

**Why.** The owner has to choose how a single player gets at castle
content. Round 1's numbers (treasure count, drop rates, investment costs,
guild-dungeon rules) came from rAthena, iRO Wiki and Divine Pride. They may
be right for 2003; we cannot tell.

**Do.**
* Load the iRO guide pages of April 2004 and quote every rule on them in
  full: `game/warofemperium.asp` (capture `20040405051438`),
  `game/guildsystem.asp` (`20040404013538`), and whatever they link to.
* kRO notices from the introduction of 공성전 (find the date) to 2004-06:
  every rule about castles, treasure (보물상자), investment (투자), guardians,
  the guild dungeon (길드 던전 / 아지트 던전).
* The first jAthena `conf/gvg/` scripts (T2 gives the dates) and 817's
  `npc/guild/`: treasure spawn rule, investment cost and effect, the lever
  into `gld_dun`, guardian HP, with file and line.
* euRO's own WoE rules and news, spring to autumn 2004.

**Deliver** one table of rules, each row graded, period sources first. Mark
every number for which you found *no* period source. The design options
from round 1 do not need repeating; say only whether anything you found
changes them.

### T5. Official English NPC text

**Why.** Our dialogue is 817's fan translation. Replacing it needs
Gravity's English text from 2003–04, line by line, with a source. Round 1
proposed rAthena's converted scripts; see the rule in §2.

**Do.**
* The iRO bug board (`community/bugcontent.asp`, `bugList.asp`): list the
  captures by CDX, load as many as you can, and extract every passage where
  a player quotes an NPC.
* GameFAQs guides for RO whose version date is 2004-06 or earlier (use
  Wayback captures to get the old versions): passages quoting NPCs.
* iRO's own guide pages and screenshots that show a dialogue window.
* The client: list every text table in `data.grf` (`grf_peek.py list <grf>
  .txt`, also `.xml`) and say for each what server-side fact or wording it
  fixes. `msgstringtable.txt`, the item and skill description tables and
  `tipOfTheDay.txt` are official English text of December 2003.

**Deliver** `r2/t5-official-text.tsv`: `npc_or_context`, `text` (verbatim),
`region`, `source_date`, `source`, and `ours` (our line for the same
moment, with file and line, if you can find it). Quantity matters here: a
hundred attested lines are worth more than an essay.

### T6. The iRO refine page over time

**Why.** The page of 2004-06-05 says Phracon 2000z, the same fee and bonus
for weapon levels 3 and 4, and "+1 adds 1 defense". Round 1 called all
three typos. A typo test exists: other captures.

**Do.** List every capture of `iro.ragnarokonline.com/game/refinesystem.asp`
(and the same page under other hosts or names), load each, and tabulate the
whole table per capture date. Add kRO's notice of 2003-02-04 in full, and
what mod0631's `npc_town_refine.txt` and `db/refine_db.txt` charge and give.

### T7. Rates, fares and the start cell on the official servers

Lower odds, so bound the effort; a documented negative result is a result.

* **Rates (#1).** Load and quote the two Gamona captures round 1 cited
  (`act=ST&f=31&t=9857`, `t=9778`) and the euRO forum captures. Then look
  for any dated statement of a closed-beta character's level and play
  time.
* **Kafra fares on iRO/euRO (#3).** Any screenshot or text of a Kafra
  teleport menu on iRO or euRO between 2003-07 and 2004-06, and the date
  euRO took kRO's new table.
* **Start cell (#6).** Any screenshot of a brand-new character on an
  official server in 2003 that shows the mini-map or `/where`.

### T8. Small checks

* **Lutie (#19).** In `Ragnarok.exe`, what does the `xmas` branch do? Give
  addresses and the disassembly you rely on.
* **Monster skills.** euRO's New Comodo notes list "monster skills" as new.
  mod0631 has `db/mob_skill_db.txt`. How complete is it, how does it differ
  from 817's, and does any period source say which monsters cast what?
* **Pets (#14).** Load the iRO pet guide (`game/qpetsystem.asp`, 2004-04)
  and compare its table row by row with `data/db/pets.toml`.

## 5. The report

`research/agy-threads/r2/REPORT.md`:

1. **Findings**, per task: the rows (§2), or a pointer to the TSV with the
   ten rows that matter most.
2. **Disagreements** (grade X): both sides, and what would settle each.
3. **Could not load**: source, error, what you wanted from it.
4. **Where round 1 or our notes are wrong.**
5. **Recommendations**, each citing its rows by task and number.
6. **Self-check.** Before you finish, pick twenty of your own A and B rows
   at random, open each source again, and confirm the quote is there and
   contains the value. Report how many held and fix the rest.

Excerpts go in `r2/evidence/<task>/` as plain text with the URL or path at
the top. No downloaded archives, no images except period screenshots that
are themselves the evidence.
