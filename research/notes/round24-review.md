# Round 24 review (2026-11-15)

20 rows, all T9 (threads 10, 32, 34, 35): 18 quoting six GameFAQs guides
and four Ragu Memo item pages, 2 `none`. The agent's checker run: 18 PASS,
2 NONE. Ours with `--fetch --pages research/agy-threads/r24/pages`: the same.

## Invention

**Quotes and URLs: none.** Every quote is on its page, every capture is
served.

**Dates and regions: five rows wrong, and one claim past its quote.**

* T9-285..287 (Psycho Battousai 21546, "The Komodo Patch") are dated
  2003-12-18, which is this client's date, not the guide's (header
  2003-02-25, v1.2 "Finished ... the Komodo Patch"), and tagged iRO, which
  the guide never names. The section is a **preview**, in the future tense
  ("when the Komodo patch is out", "soon to be +5% - Komodo Patch"), built
  from patch notes the author found on "an alternative site". It is
  translated kRO notes, not play. (Round 23 called the header date
  impossible because iRO got Comodo in 2003-10. A preview can be older
  than iRO's patch, and the author's "late April/early May" has no year,
  so the date is not settled either way.)
* T9-288 (the only KoW, 24428) is tagged iRO. The author: "I am not
  currently playing iRO! You can find me on cRO ... and kRO". The guide is
  of 2003-07-13; the server is uncertain.
* T9-290/291 (admira, 26968) are dated "v1.1, 2004-01-20" and tagged iRO.
  The page is v3.1 of 2004-10-18; no version line has a date, and the
  author's circle is idRO ("my Brother ... on Iris server of idRO"). The
  Resurrection table is quoted from a third party as a *coming* iRO patch
  ("Later, res will have casting time (source = iRO)"). T9-291's "rather
  than restoring lost EXP" is in no quote: the claim goes past it.
* T9-289 (Phelan, 27955) is tagged iRO; the guide names no server. A
  screenshot named `271203-screenLoki002.jpg` points to a Loki server,
  which iRO and pRO both had (Mad Monarch's credits: "pRO Loki").

**Ids in the claims: one wrong.** T9-300 gives Crescent Scythe as 1464
(that is a Halberd; mod0631's クレセントサイダー is 1466): an id from
memory again (rounds 16, 17). T9-298's Guisarme 1452 is right (the slotted
one, as 817's Gryphon has it).

progress.md matches the rows this time.

## What is new

**The Ragu Memo drop rates (T9-292..301)**: the rows pick ten items; the
pages are whole tables. Fetched all of Ragu Memo's item pages at the
capture nearest 2003-12-18 (armor1-8, weapon1-14, item0-7, itemc, card;
`data:refs/web/ragmemo-2003/`) and compared them mechanically
(`research/ragmemo_drops.py`). The results are under THREADS #32.

**Payon Cave 1F (T9-289)**: new and useful. Phelan's Battle Priest guide
(by 2004-01-13, an English server) lists the floor's monsters as
"Familiars, Zombies ... spores", with no Skeletons. A Battle Priest guide
would name 25 undead if they were there. This is the first non-jRO account
of 2004 for THREADS #10 (see there). Psycho Battousai's pre-Comodo list
has the same three.

**Auto Counter, Bowling Bash (T9-284..287)**: a kRO preview of the Comodo
patch, not play. Its Auto Counter "no longer causes critical hits ... 2x
the damage of what critical blow would do ... +20 HIT" differs from what we
built (817 and jAthena //0751: twice the critical rate, HIT +20). It is the
only account that says so, a half-understood translation, and both
emulators disagree, so it is recorded and not built. Bowling Bash and
Brandish Spear "1 second casting time ... equipment's DEF reduced to 2/3
... not canceled when hit" is a kRO figure older than jAthena //0785's
0.7 s (2004-02-08). Thread 22's question is unchanged: did euRO have the
balance patch at 2003-12-18? Mad Monarch (pRO, 2003-10-02): before Comodo,
"countered by a critical hit". This client's own text says "Strike back
enemy with Critical Attack" (release-day text).

**Hammer Fall 3x3 (T9-288)**: kRO/cRO 2003-07, level 1 only ("14% chance
to stun enemies in a 3x3 tile area"). Ours is 5x5 with mod0631 and 817,
and jAthena //0880 says the real reach was unknown. One line from a guide
whose other levels say nothing of area: recorded in #34, not built.

**Death 1% (T9-290)**: "lost 1% exp", idRO, undated. It does not say 1% of
what, so thread 35 stays open.

**T9-302/303 (none rows)**: searched six guides. Fine as negatives, but
these are not guides that would discuss plant damage or ground units.

## Count

New to THREADS: Phelan's Payon Cave list, the kRO Komodo preview's Auto
Counter wording, KoW's Hammer Fall line. The ten drop rows led to the
whole-table comparison, which is the round's real yield.
