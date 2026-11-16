# Round 26 review (2026-11-16)

25 rows (T1 2, T4 5, T8 8, T9 10) from six GameFAQs guides: Giga
Bahamut's Archer/Hunter FAQ (20075), lvlika's Agility Wizard FAQ (25510),
peropero11's Double Dagger Assassin guide (29579), TrinityUser's Lancer
FAQ (25432), mangdiws's Spear Knight FAQ (28554) and Weggy's FAQ (20092).
The agent's checker run: 25 PASS. Ours with `--fetch --pages
research/agy-threads/r26/pages`: 15 PASS, 10 WARN.

## Invention

**Quotes: none.** Every quote is on its saved page.

**Two capture timestamps invented (ten rows).** T1-067/068, T8-053/054
and T9-338/339 cite `20100509054944` for 20075; CDX has `20100509054954`
(ten seconds later) and nothing at 054944. T9-342..345 cite
`20100508064530` for 28554; CDX's first capture is `20100508090803`. The
pages saved are those captures' text, so the rows' content holds; the
addresses do not. The same slip as rounds 21 and 22: copy the timestamp
CDX prints.

**Dates: six rows off by half a year.** Giga Bahamut's guide is headed
"Version Final, Last Updated 2003-08-12", and the rows carry that date.
Its title is "Ragnarok Online Beta 2 Archer/Hunter FAQ", its history
finished the Hunter skills on 2002-10-20 and updated them on 2003-02-22
("Big update mainly with hunter skills"), and the last two updates fixed
typos and builds. So T1-067/068, T8-053/054 and T9-338/339 describe iRO's
beta 2 of late 2002 to early 2003, ten months or more before this client.
The rule of round 7 ("date a FAQ's section, not the FAQ") again.

**Regions.** TrinityUser's rows say "iRO Sakray" in the claim and "iRO" in
`region`: Sakray is iRO's test server, where the Comodo patch ran before
the main one. The others hold (pRO for peropero11 and mangdiws, iRO for
lvlika and Weggy).

**Claims: one past its quote.** T9-346 adds "containing no Joker" to a
list that simply names eight monsters. True of the list, but the claim
should say only what the quote does. progress.md lists the rows as they
are.

## What is new

**An unused trap gives its Trap back (T8-054).** "Any unused trap will be
able to be picked up after it's lasting time has depleted" (iRO beta 2).
817 does it (`skill.c:6143`: a player's trap of unit 0x8f-0x98 that runs
out drops a Trap on its cell, Venom Dust excepted); mod0631 does not.
Built (`scripts/rules/skill_units.lua` `skill_unit_timer`, test
`unused_trap_leaves_its_trap`, DEVIATIONS.md).

**Trap counts (T9-338/339): witnesses on both sides.** The iRO beta 2
guide has "(Uses 2 traps)" for both the Claymore and Shockwave Traps.
Set beside what we had:

| Trap | 1 Trap | 2 Traps |
|---|---|---|
| Claymore | this client's skill text (2003-12), mod0631 | iRO beta 2 (2003-02), 817 `skill_require_db` (2004-01), euRO's May-2004 skill text |
| Shockwave | mod0631, 817 | this client's skill text, iRO beta 2 |

Round 25's reading that "euRO raised the Claymore's cost between the two
clients" is weaker now: iRO took two in early 2003 already, so this
client's "1 Trap Needed" may be a slip in the text rather than a cost
euRO had. Both kept at one (mod0631). THREADS.md #22,
`research/notes/may2004-client.md`.

**Glast Heim and Geffen Dungeon (T9-346/347).** Weggy's World Map section
(rewritten in v6.7, 2004-07-09, iRO) lists Geffen Dungeon Level 3 as
exactly our `gef_dun02` (no Joker: round 25's guide put one there,
probably counting floors otherwise). His Castle Level 1 (Joker, Baphomet
Jr., Rideword, Evil Druid, Tirfing, Mysteltainn) shares Joker, Baphomet
Jr., Rideword, Tirfing and Mysteltainn with round 25's list of before
2004-02-01. Neither names Carat, Sage Worm, Alice, Owl Duke or Dark Frame
anywhere in Glast Heim, all of which mod0631 (and Ragu Memo's jRO counts)
put on `gl_cas01` and `gl_in01`. Two English iRO lists against jRO's, but
name lists only, plainly incomplete (Weggy's lacks the Khalitzburg the
other has): not built. THREADS.md #10.

**Frost Diver's freeze (T4-028).** "Level 10 Frost Diver lasts around
20-30 seconds" (lvlika, iRO Loki, v2.0 of 2003-09-29). Ours is 817's
`cast_db`, 6 s at level 1 to 15 s at level 10, less MDEF (Ragu Memo's
formula, thread 21). A player's estimate, and one witness: noted in
thread 21, not changed.

**An MVP without its escort (T9-342).** "Encounter an MVP without its
minions" (pRO, 2004-02): bosses did not always stand among a crowd, as
ours never do. A small point for thread 34's escort question.

**Not new.** The Hunter change at job level 40 with no test (T1-067) is
beta-era, older than the 2-1 quests (round 7). The falcon at 2500z
(T1-068) is ours. Poison (T4-029..032) is a pRO guide of 2004-11; ours keeps
817's numbers. Ice Wall (T8-050..052) is how ours behaves (monsters walk
round an open wall). The Talkie Box's 600 s (T8-053) and Venom Splasher
ending on a hit (T8-055) are built. The Comodo-era Auto Counter (T9-340/
341: the stance holds the Knight, twice the damage, extra HIT) is what we
built from 817 and jAthena; TrinityUser's Brandish Spear and Bowling Bash
of the Comodo patch (T8-056/057, 1 s, 2/3 DEF) are the kRO balance patch
of thread 34, still not on euRO by our evidence. Counter Attack's speed
(T9-343), Bash's stun at 6-10 after Comodo (T9-344) and Two-Hand Sword
Mastery before Counter Attack (T9-345): pRO, after the patch, no change.

## Count

New: the unused trap's return (built), Frost Diver's longer freeze on
iRO (a lead), a second English witness for
Glast Heim's castle (not built), the trap counts' witnesses (not
settled). Six rows misdated, ten with an invented capture second.
