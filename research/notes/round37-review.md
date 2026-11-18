# Research round 37: review (2026-11-18)

33 rows (4 T1, 29 T9), all PASS in `check.txt` and on my own run of
`check_rows.py --fetch`. Four pages saved. `progress.md` matches the rows
claim for claim.

## How the round did

* **Nothing invented.** Every quote is on its page or in its file; the
  jAthena rows (//0505's Peco Peco egg summon, //0556's four weapons) are
  exact (`git show 8ec48b3 -- db/item_db.txt`).
* **One claim past its quote.** T9-566 says //0505 *added* the Peco Peco's
  egg summon. //0505 is where jAthena's history starts (2003-10-30); the
  row was there, nothing says it was added then.
* **Metadata slips in T1.** The four Sage rows are tagged pRO and dated
  2004-05-27 (Calx Zinra's v1.4). The section is in the guide from v1.0
  (2004-05-05); no later version line touches it. And it is not pRO play:
  the author says he does not know the test yet and quotes iRO's site,
  "How to change into a Sage: (www.ragnarokonline.com)".
* **One date a day off.** T9-564 and T9-565 date 817's `db/skill_db.txt`
  2004-02-18; the zip says 2004-02-17 15:00.
* **Nothing new in T1.** iRO's site text (Juno's academy, job level 40,
  memorization tests, item errands, an essay) is the outline of jAthena's
  test, built yesterday. The queue asked for the course choice, the
  questions, the fights: none came.
* **Nothing new in 817's rows.** T9-558 to T9-563 (Piercing and Critical
  attacks of Hornet, Muka, Caramel, Desert Wolf, Riding Word, Orc
  Skeleton) are in mod0631 and our table.

## The gold: Gravity's own monster-skill list

T9-550 to T9-557 quote **Gravity's iRO notice of 2004-02-11**, "Monster
Skill Update" under the Turtle Island update on Sakray
(`renewal.playragnarok.com/news/updatedetail.aspx?id=147`, live; copy in
`data:refs/web/iro-news-2004/updatedetail_147.html`). Read whole, it is a
table of every monster skill by monster: player skills, the monsters' own
skills, and fifteen bosses' lists, then the boss rule ("All boss monsters
will use the Two-Hand Quicken, Increase Agility, Guided Attack, and
Piercing Attack ... will Teleport if they are attacked from beyond their
attacking range"), "Monsters with attributes will use attacks of their
attribute", and "The above content has not finalized".

Set beside unknown13's Monster Skill FAQ (GameFAQs 29608, April 2004),
whose 325 pairs we built monster skills from on 2026-11-14 (round 9):
**the FAQ is a copy of it.** 302 pairs match by name; the other 23 are
spellings (Earth and Sky Petite for Walking and Flying Petite, Lex
Aeterna, Status Recovery, Vagabond Wolf, Tarou), and the FAQ's "Poison
Attack" for the notice's second "Venom Dust" (the one among the monsters'
own skills, on Argos, Verit, Poporing...). The notice has one pair the FAQ
lacks, Marine Sphere's Self-destruction, which mod0631 already has. Its
"Marionette | Freezing Trap" row is Gravity's own slip of the columns; the
FAQ read it right. The FAQ's boss rule, which it said it took from iRO, is
this notice word for word.

So the 223 pairs (315 rows, 22 boss Teleports among them) in
`data/db/mob_skills_faq.toml` are Gravity's list, not a player's: `tools/import/mob_skills_faq.py` and
its TSV now say so (regenerated: only the source text changed). Two
things it does not settle:

* It is a Sakray notice, "not finalized", of 2004-02-11: iRO's test
  server, a month after euRO's "New Comodo" brought monster skills
  (THREADS.md #23). What euRO's main server had in January 2004 is still
  mod0631's table plus these pairs.
* "Monsters with attributes will use attacks of their attribute" is now
  Gravity's line, not only Skygor's (round 32). Our tables give elemental
  attacks to 196 of 410 such monsters, the same monsters RODataZone's
  table of the same month shows using them (iRO's main server, as
  players saw it). Left as it is: the intention against the observed
  table, and the notice says it is not final.

## The rare-weapon rework

T9-571 to T9-578: Ragu Memo's jRO weapon pages captured 2005-03-09 give
Muramasa ATK 170 and CRI + 20, the Assassin Dagger ATK 125 with Dark and
MSP + 30: the rows from **before** jAthena //0556 (2003-11-21), which the
game has used since round 35. So jRO still had the old numbers in March
2005; //0556's rework was not jRO's in our period or long after. That
answers most of the queue's T9 item on when the rework came (for jRO);
kRO and euRO are still open.

## Counted

New to THREADS.md: the notice (one page, but every row the FAQ's pairs
became now cites Gravity) and
the Ragu Memo dating of the rework. Everything else was in or built.
