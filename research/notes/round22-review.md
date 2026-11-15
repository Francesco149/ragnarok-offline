# Round 22 review (2026-11-15)

19 rows, all T9. The agent's checker run: 19 PASS. Ours with `--fetch
--pages research/agy-threads/r22/pages`: 14 PASS, 5 WARN.

## Invention

**One Wayback timestamp invented, on five rows.** T9-268 and T9-274 to
T9-277 cite `web.archive.org/web/20100508064434id_/.../faqs/20092`. CDX has
no such capture: the first of 2010 is `20100508105531`, which is what
Wayback serves and what the rows should cite. The quotes are on the saved
page. (The other GameFAQs page, `29538` at `20100508064548`, is real.)

Also slipped, unchecked by the tool:

* **A claim past its quote:** T9-260 says jAthena's `Readme0951` "confirms
  official loot mobs keep at most 10 items". The line is about pet looting
  and says a pet picks up "10, the same as loot mobs" (ルートmob同様): it
  describes jAthena's own value, not the official server's. T9-270 says 15z
  a Net Mushroom where the quote multiplies by 5; it is right (817's line
  416 counts each mushroom as 3 points first), but not from the quote.
* **Region:** T9-261 and T9-262 call anjingbalap's Dagger Assassin guide
  iRO; its author writes "in idRO we are underrated build" (Indonesia).
* **Dates:** T9-266 dates mod0631's `npc_job_aco.txt` 2003-08-10 (the file
  says 2003-10-23); T9-263 dates `mob.c` 2003-12-15 (2003-12-16). The
  juice rows date Weggy's Item Creation section by v6.0 (2004-05-17), but
  no version line from 6.0 to 6.7 touches that section: like the 2-1 text
  round 7 misdated, it is most likely from before his year-long break
  (about mid-2003). T9-268's v6.4 (2004-06-07, "Acolyte this time") does
  fit the Monk paragraph it quotes; the page itself is v6.7 (2004-07-09).
* Line numbers of the anjingbalap quotes are 364 and 385 in the rows; the
  page text has them some 40 lines earlier, depending on how it is split.
* Grades: B throughout, as before.

## What is new

About four rows of 19; one changed the game.

* **jAthena set a looter's bag to 10 on 2004-02-01** (T9-259, //0764,
  `cf52bed`, `map/map.h:14`, from mod0631's 20), in the commit that also
  made EXP "like the real server" (本鯖のように). With RODataZone's iRO
  Monster AI page ("a maximum of 10 items", capture 2004-02-11) that is two
  witnesses of early 2004: **built** (`combat.rs` `LOOT_SIZE`, test
  `looter_keeps_ten`, DEVIATIONS.md). The full bag still lets the oldest
  item go, as in mod0631 and jAthena's default `monster_loot_type: 0`;
  RODataZone's "items picked up in excess of this are gone forever" cannot
  tell the two rules apart from a player's side.
* **Detect** (T9-261/262): anjingbalap (idRO, v2.2, 2004-06-02) has
  insects, demons and bosses see through Hiding and Cloaking: with mod0631
  (the Demon race), against RODataZone's "Dark property". Not the client's
  own skill text (which says only "Detected by Certain Skills as Sight").
  Kept as we have it.
* **The Thief test's mushrooms** (T9-268): Weggy (iRO, v6.4, 2004-06-07)
  says the Monk's mushroom quest "use[s] the same items, so you could
  carry them over". mod0631 takes one of each at the job change
  (`npc_job_thief.txt:415-418`); 817's `delitem 1069,99` takes none when
  fewer are held, and so does ours (`take_item`). The game keeps them
  already.
* **Marx Hansen's juices** (T9-274..277): one fruit, an Empty Bottle and
  3z, in Weggy's Item Creation section (iRO, likely mid-2003): with what
  the game does (mod0631) and the two pRO guides, against RODataZone's
  three. No change.

Padding: T9-263 (our own mod0631 line, quoted in THREADS #23 already),
T9-264/265 (the Shockwave Trap split, which the queue item stated),
T9-266/267, T9-269/270/272/273 (the emulator lines the queue quoted).
