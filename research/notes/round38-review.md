# Research round 38: review (2026-11-18)

61 rows (16 T1, 15 T8, 30 T9), all PASS in `check.txt` and on my own run of
`check_rows.py --pages research/agy-threads/r38/pages --fetch` (without
`--pages` it looks in `rows/pages/` and fails every row). Four pages saved:
Calx Zinra's Monk FAQ (GameFAQs 36497, Wayback 20100509045649, the first
capture in CDX) and three of Gravity's iRO notices on
`renewal.playragnarok.com` (live; CDX has them from 2020-09-26 for 146 and
148 and from 2023-08-17 for 149). `progress.md` matches the rows.

## How the round did

* **Nothing invented.** Every quote is on its page and on the live page;
  ids 146/148/149 load the notices the rows name.
* **One claim past its quote.** T8-077 says traps *and Fire Pillar* take
  0.4 s to set; the line says "Every trap". (The heading is "Change of
  Trap & Fire Pillar", but nothing gives the pillar the delay.) Milder:
  T1-143's "physical exam" (the quote says only "two quests to choose
  from"; the iRO site text above it says "memory and physical test");
  T8-075's "while casting" (a fair reading of the lines around it).
* **Sixteen rows misdated.** T1-131 to T1-146 are dated 2004-03-25, the
  guide's v1.0. The section they quote came later. v2.4 (2004-05-15) "Added
  ... a How to be a monk section submitted by Louise Stevens Cabarles", and
  v2.5 (2004-05-23) credits the owner ("he took it from someone's Monk
  Guide"): "(From Akemi's iRO Monk Guide at RO-world)". iRO had no Monks
  before Juno (notice 146, 2004-04-07). So it is iRO, April to May 2004. The
  item sets cite `forums.nitro7.com`, server not said. Only T1-131's line
  ("You must be at least Job Level 40", iRO's site text) could be older,
  and no version line dates it.
* **The notices' date is the date they were filed under.** Comodo (148),
  Turtle Island (147, the Sakray monster-skill notice of round 37) and War
  of Emperium (149) all show 2/11/2004. Forever-Zero copied the Comodo list
  as a "Future iRO update" by 2004-01-10 (round 10), and the iRO guide's
  Assassin page of 2004-03-27 still has Venom Splasher at 1/3 where 148 says
  2/3. So the date does not show when the patch went live on iRO.

## New against what we know

* **T1, 16 rows: new.** This is the first player account of the Monk test.
  THREADS.md has the Monk next, from eAthena `cfd39590d` (2004-12-30,
  Dino9021, `npc/jobs/2-2/monk.txt`). Set beside that script:
  * The guide lists **7 item sets**; eAthena picks one of **6**
    (`rand(1,6)`, monk.txt:181). Six match item for item: guide sets 1, 2,
    3, 4, 6, 7 are eAthena's 4, 5, 3, 6, 1, 2. eAthena lacks the guide's
    set 5 (5 Cyfar, 10 White Herb, 10 Yellow Herb).
  * eAthena's set 3 counts 30 Stem (905) but deletes 30 of item 7012
    (monk.txt:270, :294). The guide's "30 Stems" says 905 is meant.
  * The mushroom test: the guide asks 70-80 points (Orange Net 3, Orange
    Gooey 1). eAthena asks 30 of one kind and one of the other
    (monk.txt:1557).
  * The marathon: the guide says 10 laps; eAthena uses `rand(8,10)`
    (monk.txt:1383).
  * The order of the steps (items, recital to the monk north of the temple,
    the choice of mushrooms or marathon in the south-west with a Green
    Potion as the prize, the maze of hidden walls in the centre building,
    the potion and the oath) fits eAthena's NPCs.
* **T8, 15 rows: no new value but one.** Everything else is in THREADS.md
  #22 and #28 through Forever-Zero's copy (round 10), Psycho Battousai's
  preview (round 24) or the iRO guide, and is built or kept on purpose. The
  new value is Venom Splasher's **1 s cast** (T8-072). What else is new is
  the source: Gravity's own words now stand where those copies did.
* **T9 War of Emperium, 10 rows: nothing new.** The iRO guide's War of
  Emperium page (round 11) has all of it: free storage, the daily rare
  item, faster recovery, defence investment, no EXP loss, Ice Wall, Phen
  and Intimidate off. Notice 149's guild-skill text is our client's GD_*
  `skilldesctable.txt` text sentence for sentence.
* **T9 Juno, 20 rows: new, but after our window.** Leads for
  `may2004-client.md` (below).
* **146, 148 and 149 against 147.** 147 is monster skills only, so there is
  no overlap. 148 carries the most, and most of it is in lines the round
  did not quote: the whole first- and second-job balance list, the new
  quest skills, and a section on ailments.

## Notice 148 (Comodo, iRO, filed 2004-02-11), read whole, against our game

Client = this client's `data\skilldesctable.txt` and
`leveluseskillspamount.txt` (2003-12-18). May = euRO's May-2004 text
(`mico07/yuno_lang`). Guide = the iRO guide's skill pages
(`data:refs/web/iro-guide-2004/`, 2004-03-27 to 04-13).

| Notice says | We have | Verdict |
|---|---|---|
| 1H / 2H Sword Mastery 5 → 4 a level | 4 (`battle.lua:558-559`) | matches (client text +5 = before) |
| Fatal Blow: Bash ≥ 6 stuns | `skill.lua:116` (lv - 5) | matches (client text says "level 5 or above") |
| Spear Boomerang Ghost → Neutral, 10 SP | Neutral, 10 (`skills.toml:722`) | matches |
| Spear Boomerang 1 s delay after | none (mod0631 has no cast_db row) | **differs**; 817 cast_db `59,0,1000`, guide Knight page "1 second skill delay" |
| Counter Attack: cannot move, ignores DEF, can miss | DEVIATIONS "Auto Counter", `battle.lua:674,746`, `combat.rs:175` | matches (client text "Critical Attack" = before) |
| Bowling Bash / Brandish Spear 1 s cast, no interruption, DEF to 2/3, 1 s delay | cast 1000, delay 1000 (`skills.toml`), 2/3 built 2026-11-17 | matches (817 has 0.7 s and no delay) |
| Two-Hand Quicken +50% → +30% | `status.lua:367` (70%) | matches |
| Spear Stab reach 4 | range 4 | matches |
| Double Strafe 15 → 12 SP | 12 | matches |
| Arrow Shower 9 → 25 cells, pushes 2 | `skill_effects.lua:140` (ar 2), `battle.lua:857` | matches |
| Flasher 20 → 12, Sandman 12, Freezing Trap 10, Skid Trap 5 → 10 SP | client SP table: 12, 12, 10, 10 | matches; **the 2003-12-18 client already has these** |
| Ankle Snare 10 → 12 SP; hold by AGI, at least 3 s + caster level; items, wings, Teleport usable | client SP 10 (`toml` 12 unused); hold 4-20 s by level, boss /5 (`skill.lua:142`, `skill_units.lua:592`) | differs, **before the patch (keep)**: client SP 10; May text "affected by enemy's AGI" |
| Skid Trap 6 → 10 cells at level 5 | 1-5 (client text, mod0631) | differs, **keep** (THREADS #22) |
| Beast Bane 4 a level, brutes and insects | `battle.lua:555` | matches (client +3, May +5) |
| Traps: hidden again after leaving the screen | `battle.rs:103` | matches (THREADS #28) |
| Traps 0.4 s to set | not built | differs, notice only |
| Traps and Fire Pillar not on a cell where someone stands | allowed (`skill_units.lua:399`, jAthena's `nofootset` off) | **differs**; jAthena //0825 has the option |
| No trap or pillar within 3x3 of another | trap range 1 + new 0 (`skill_units.lua:423`) | matches |
| Traps spare whoever stood in range when they were set | not built | differs, notice and Forever-Zero |
| Frost Diver cast 1 → 0.8 s | 800 | matches |
| Stone Curse reach 2 | range 2 | matches |
| Safety Wall, Pneuma: no overlap | `skill_units.lua:406` | matches |
| Fire Wall: no more than 3 at once | no limit | **differs**; 817 `skill.c:3878` (`fw_count == 3`) |
| Fire Pillar 35 → 25 SP, 30 s at any level | 25, 30000 | matches |
| Storm Gust: a hit every 0.45 s, 10 at most; freezes after 3 hits, not bosses or undead | `skill_units.lua:216`, `skill.lua:133` | matches |
| Soul Strike: 0.3 s off the delay at level 10 | 2800 → 2500 | matches |
| Weaponry Research +2 damage, +2 HIT a level | `battle.lua:760`, `status.lua:307` | matches |
| Impositio Manus +2 → +5, 40 s → 60 s | `battle.lua:142`, `status_change.lua:162` | matches (client +2 / 40 s = before; May +5 / 1 min) |
| Enchant Poison 15 → 20 SP | 20 | matches |
| Enchant Poison ends when equipment changes | no (`skill.lua:990`) | differs, notice only (817 does not) |
| Poison React raises Venom Splasher's damage (max 600%) | no (`battle.lua:883`, 200 + 20/lv) | differs, notice only, no formula |
| Venom Splasher 1/3 → 2/3 HP | 1/3 (`skill_effects.lua:381`) | differs, **keep**: client "1/3", guide 03-27 "1/3"; May "2/3" |
| Venom Splasher 25% on 3x3 | `skill_effects.lua:397` | matches |
| Venom Splasher 1 s cast | none (`skills.toml:1732`) | differs; jAthena skill_cast_db (from 2004-09-14) |
| Cloaking: within a cell of a wall | `skill_effects.lua:362` | matches |
| Envenom 15 → 12 SP | **15** (`skills.toml:638`, mod0631) | **differs**; 817 skill_require_db 12, jAthena //0505 12, guide 12 |
| Steal 20 → 10 SP | **20** (`skills.toml:615`, mod0631) | **differs**; 817 10, jAthena //0505 10, guide 10 |
| First Aid 3 SP / 5 HP, Play Dead 5 SP, Arrow Craft 10 SP, Back Slide 5 cells 7 SP, Change Cart 40, Crazy Uproar STR +4 5 min 8 SP, Energy Coat 30 SP 5 s | as said | matches |
| Moving HP Recovery 25%, skill recovery not counted | `combat.lua:88`, `combat.rs:1467` | matches |
| Berserk: Provoke 10 under 25% HP | `skill.lua:983` | matches |
| Arrow Repel 150%, 1.5 s cast, uninterrupted, 6 cells | 6 cells only (`battle.lua:887`, `skills.toml:1821` no cast) | **differs**; 817 `battle.c:1488` 150% and cast_db 1500, guide Archer page same, jAthena same |
| Sand Attack 15% Blind, Earth, 9 SP, 1 cell | `skill.lua:159`, range 1 | matches (damage 125% = client text) |
| Find Stone 2 SP, 0.5 s cast, not at 50% weight | 2 SP, no cast, no weight check (`skill_effects.lua:689`) | **differs**; guide Thief page same; 817 `recover_weight_rate` |
| Stone Fling 7 cells, 30 ignoring DEF, 5% stun, 2 SP, 1 s cast | range 8, no cast (`skills.toml:1870`) | differs; guide same; mod0631 and 817 range 8, no cast |
| Cart Revolution 150% + cart weight, splash, pushes 2, 12 SP | `battle.lua:889` | matches |
| Holy Light 150% MATK, 2 s, 15 SP | 125% (`battle.lua:1035`) | differs, **keep**: guide 125%, every emulator 125 |
| Poisoned: no HP/SP recovery | recovers (`combat.lua:61`) | **differs**; 817 `pc.c:5089` stops all natural recovery |
| Poison lasts at most 30 s (players 5-30 s) | 10-59 s (`status_change.lua:274`, 817) | differs, notice only |
| Poison and stone stop at 25% HP | `status_change.lua:416` | matches |
| Poison lowers DEF 25%, no longer ATK | `battle.lua:184`, `status.lua:362` | matches |
| Frozen or stone: no dodge, MDEF +25%, DEF -50% | `battle.lua:752`, `:185`, `:191` | matches |
| No Lex Aeterna or Steal on the frozen or stoned; freezing ends both | no check | **differs** for Lex Aeterna: 817 `skill.c:5218`; Steal notice only |
| Stone: Earth element while it lasts | no (`battle.lua:217-223` has only Freeze → Water) | **differs**; 817 `battle_get_element` (`SC_STONE` → 22) |
| Asleep: hit by criticals twice as often | `battle.lua:668` | matches |
| Blind: HIT and Flee -25% | `battle.lua:111,117` | matches |
| Hiding: only ground spells reach | not checked | — |

No numbers on the page (the notice's tables were images): Provoke,
Improve Concentration, Hammer Fall, Angelus, Blessing, Aspersio and Sonic
Blow ("changed"), Ice Wall's durability.

**What this says about our window.** The Dec-2003 client is mostly before
this patch: its text has masteries +5, Impositio +2 for 40 s, Two-Hand
Quicken 15-60 s, Venom Splasher at 1/3, Counter Attack critical, and Ankle
Snare 10 SP. But its SP table already has the new Flasher, Sandman,
Freezing Trap and Skid Trap costs. The May-2004 text is after it. mod0631,
and so our game, already has most of the patch's numbers (Sword Mastery 4,
Impositio +5/60 s, Two-Hand Quicken 30%, Double Strafe 12, Fire Pillar 25
and 30 s, Arrow Shower 5x5...). The gaps below are rows where mod0631
lagged: 817 (2004-02-19) and the iRO guide have the patch's value. Taking
them makes the game agree with itself; it does not settle THREADS #22's
question of when euRO got the patch.

**R6 needs a line.** The guide pages of 2004-03-27 already describe
Juno-era iRO: they have the 2-2 jobs, and Resurrection's table
(6/4/2/0 s cast, 10/30/50/80%), which notice 146 calls a change of
2004-04-07. So the guide is not a witness for February 2004. Our client's
numbers are the state before the Comodo patch (148 gives "from" values that
match them: masteries 5, Impositio 2 and 40 s). Resurrection is still 1
HP/15/30/50 in both euRO clients, December and May.

## Notice 149 (War of Emperium)

All in round 11's notes and built or reworked (castle_lord.lua header,
DEVIATIONS "Guild castles"), except the siege rules, which are not applied
to our castles: no EXP loss on death, no Ice Wall, no knock-back, no
Teleport or Fly Wing (Butterfly Wing yes), no Endure, Phen's no-interrupt
off, no Intimidate, and the Emperium unaffected by support magic. Castle
maps have no flags at all (`data/db/mapflags.toml`), so dying in a castle
in a siege costs EXP now. "Twice a week" is iRO's; euRO's Sunday hour
stands.

## Notice 146 (Juno, 2004-04-07): leads for `may2004-client.md`, not the 2003 game

Already as we have them (jRO / mod0631 values): Iron Ore weight 15, Pet
Food 1,000z, Fortune Sword weapon level 4, the Curse slowing the walk
(`status.lua:363`). Not ours, after the window: Resurrection
10/30/50/80% (May text still 1 HP/15/30/50); the Meteor Storm SP
"description" 30-74 corrected to 20-64 (our server charges the client's
table, 30-74; jAthena of 2003-10-30 has the same); trap elements (Freezing
Water, Blast Mine Wind, Claymore Fire; ours Earth/Fire/Earth); Wizardry Staff level 4 (ours 3);
Book 85 ATK 3 slots, Bible 115 ATK 2 slots, Tablet 125 (ours 100/3, 139/2,
125/1); Novice Breastplate level 10; Sleipnir DEF 1; level requirements on
Megingjard and Brisingamen; Stripe Hairpin without AGI +3; Small Thief Bug
an Insect; Elder Willow and Scorpion aggressive, Hode not; Mummy's HIT,
Sandman's EXP; Novices may not wield Damascus; Power Thrust breaks the
weapon 1%; MVP loot at 10/20/22 s for the next damagers; Turtle Dungeon 4
without wings or Teleport; castle Kafras without the cart; castle boxes
safe from Steal; traps four times longer in a siege; no vending while
trading; Venom Dust + Detoxify "infinite defence" fixed; the north-gate
Kafra moved inside Prontera.

## Recommended changes

Ordered by strength of evidence. "817" = eAthena 817 (2004-02-19), tier
athena2003; "notice" = 148, tier official.

1. **Charge Arrow** 150% damage and a 1.5 s cast: `scripts/rules/battle.lua:887`
   (`pct(w, 150)`), and the cast through `tools/import/athena_import.py`
   (`import_skills`: take 817's cast_db row where mod0631 has none), then
   regenerate `data/db/skills.toml`. Sources: 817 `battle.c:1488` and
   cast_db, the guide's Archer page, notice 148, jAthena.
2. **Spear Boomerang** 1 s after-cast delay: the same importer fallback
   (817 `59,0,1000`, the guide's Knight page, the notice). Among first and
   2-1 skills only 59 and 148 lack a mod0631 row while 817 has one
   (mechanical diff; the rest are 2-2 skills).
3. **Steal 10 SP, Envenom 12 SP**: an importer override over mod0631's 20 and 15
   (817 skill_require_db, jAthena //0505 of 2003-10-30, the guide, the
   notice). Neither is in the client's SP table, so the client does not say
   otherwise.
4. **Fire Wall**: at most three standing at once (817 `skill.c:3878`, the
   notice): `scripts/rules/skill.lua` cast check, counting groups in
   `skill_units.lua`.
5. **Find Stone** refused at 50% weight or more (817 `recover_weight_rate`,
   the guide, the notice); 0.5 s cast (the guide and the notice, no
   emulator).
6. **No natural recovery while poisoned** (817 `pc.c:5089`; the notice:
   "HP/SP recovery skills become inactivated"): `scripts/rules/combat.lua`
   `rules.regen`.
7. **Stone Curse makes the target Earth** while it lasts (817
   `battle_get_element`, the notice): `scripts/rules/battle.lua:217-223`.
8. **No Lex Aeterna on the frozen or stoned** (817 `skill.c:5218`, the
   notice): `scripts/rules/status_change.lua` (refuse SC.AETERNA).

Notice alone (with the guide where said); decide:

9. Venom Splasher 1 s cast (jAthena's later cast_db agrees).
10. Stone Fling 1 s cast and reach 7 (the guide too; mod0631 and 817 have 8, no cast).
11. Traps and Fire Pillar not on a cell where someone stands (jAthena
    //0825's `nofootset`): `skill_units.lua:399`.
12. Poison at most 30 s (`status_change.lua:274`); Enchant Poison ends on an
    equipment change; Steal refused on the frozen or stoned.
13. Siege rules in castles during a siege, at least no EXP loss on death
    (`castle_lord.lua` / the death penalty); the rest are PvP rules.
14. Resurrection and the Leaf of Yggdrasil at the client's 1 HP/15/30/50%
    (`skill_effects.lua:419`): both euRO clients, and notice 146 dates
    10/30/50/80 to April 2004. Almost no effect with one player.

Keep: Skid Trap 1-5, Ankle Snare (client SP 10, hold by level), Venom
Splasher 1/3, Holy Light 125%, Meteor Storm's client SP.

Update THREADS.md #22 (the notice is Gravity's text of the patch, filed
2004-02-11; the client's SP table already carries part of it) and R6 (the
guide is Juno-era; the client's numbers are the pre-Comodo state). Add to
T1 the Monk test's differences from eAthena above.
