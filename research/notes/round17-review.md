# Round 17 review (2026-11-14)

67 rows: T1 1, T4 2, T9 62, T10 2. The agent's checker run: 67 PASS. Ours
with `--fetch --pages research/agy-threads/r17/pages`: 67 PASS.

## Invention

**No quote invented.** Dates are right this time: each GameFAQs row takes
the saved page's own "Last Updated" header, the Ragu Memo rows their
captures. Slips:

* **T9-203 gives the wrong item.** Soujiro's "phantom opera mask DEF 2 for
  mid-lower" is 2281 (Phantom of Opera: DEF 2, Mid and Lower, "Every Job"
  in the client), not 5043 (Opera Ghost Mask: DEF 1, Mid only), which the
  claim names. An id from memory again, as in round 16. 2281 already lets
  a Novice wear it: the row confirms nothing in dispute.
* **progress.md** joins two items as one: "Fortune Sword (Mama's Knife,
  1229)". The guide lists both; 1229 is Mama's Knife, the Fortune Sword is
  1223. It also writes RODataZone's array index as the Wanderer's id
  ("Wanderer (242)"; the monster is 1208), and says T4-021's "unknown" holds
  "through August 2004" from captures it did not save and no row cites.
* **T1-042's region "iRO"**: the guide's author writes "I am not currently
  playing iRO! You can find me on cRO (Sara) and kRO (Sakray)". The
  server is not settled by the page.
* Claims past their quotes, true on the page: T1-042's "Job Level 40" (the
  sentence before the quote), the Soujiro rows' "novice can use" (the
  list's heading). The heading's next sentence is left out of all of them:
  "most of those weapons are in the database but cannot obtain in iRO yet.
  Also, some of the infos may be inaccurate."

## What is new

About five rows of 67:

* **T9-154, T9-155: RODataZone has the Clock Tower Manager**, as "Tower
  Keeper" (level 63, HP 18600, EXP 4378/2850, as mod0631). It names the
  Hinalle Leaflet and the Memorize Book that 817 put in mod0631's empty
  slots. THREADS #32's open list loses its last monster.
* **T4-021**: Ragu Memo's ailment page of 2004-02 still calls Curse's
  length unknown (#21).
* **The Ragu Memo hat pages, set side by side** (my reading, from the
  saved pages, not the rows): `turtle_armor.html` of 2003-08-21, the night
  jRO's Turtle Island patch went in, marks the new hats (Cross Hairband,
  Poring Hat, Deviruchi Hat, Spore, Panda, Miner's Helmet, Opera Ghost
  Mask) On for every job, the Novice too, and the Helm of the Solar God
  for Swordman, Thief and Merchant: our client's text, line for line.
  `armor5.html` of 2003-12-12 has the same hats Off for the Novice, the
  Miner's Helmet for the Swordman, Acolyte and Merchant trees only, the
  Solar God helm for second jobs only: mod0631, and RODataZone's iRO table
  where it has them. So the client's "All" is release-day text; either
  jRO changed the masks within four months or Ragu Memo first wrote down
  the description and corrected it once tested. Either way the later
  masks, which the game has, are what players met (#31).
* T1-042: no test for the Blacksmith in a guide of 2003-04/07 (cRO or kRO
  as likely as iRO), as Edge's account already says (#1).

Not new: the 21 Soujiro weapon rows (below), T4-020 (poison's lengths, in
#21 since round 13), T10-021/022 (the same Mistiknoname3k guide #6 cites),
T9-156..174 (slots `mob_drop_holes.py` already matched, and the five
negatives #32 lists), T9-175..179 (the Clock Tower page: all 34 of its
spawn groups, 1F to B4F, are mod0631's counts; one 4F Clock Tower Manager
is on 60 minutes there, 30 on the page; jRO against jRO), T9-204..213
(Ragu Memo's job marks, which the queue said not to quote again: jRO).

**Soujiro's list is no third witness for #31.** It names exactly the
client's disputed Novice weapons (all nine swords, the twelve daggers, and
Spike, Slash, Quadrille and Grand Cross under "Mace class"), with the
client's own effect lines ("Pierce Defence of the Veggie Monster"), from
"the database", for weapons "cannot obtain in iRO yet". A fan database of
2003 was built from the same item descriptions the client carries: this is
the client's text a second time, not a player who equipped one.

## Queue

T9's job-mask item: a list copied from an item database is the item text,
not a witness; wanted is someone who wore or was refused the item. The
drops item loses the Clock Tower Manager. New rule: an item id is checked
against the source's own numbers (DEF, slot, location) before it goes in a
claim.
