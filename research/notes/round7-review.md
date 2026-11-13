# Round 7: review

Round 7 (`research/agy-threads/r7/`, written 2026-11-13 23:34 to 23:41)
took four tasks of `QUEUE.md`: T1, T9, T10 and T11. 50 rows, three saved
GameFAQs pages, a `progress.md`. Reviewed 2026-11-14 by loading every
quoted page and file again.

## Invention

* **Rows:** no invented quote. `research/check_rows.py --fetch` gives 50
  PASS, 0 FAIL, the same as the round's `check.txt`, and no claim goes
  beyond its quote.
* **Metadata (not checked by the tool):**
  * vgmaster88's guide (T9-002 to T9-004) is called "v3.3 (2003-11-14)".
    The saved page is version 3.2 and its history carries no dates. The
    history places it after iRO's Comodo update and its PvP, before
    Thanksgiving 2003 ("Chances are, I will do another update during the
    Thanksgiving Holidays"): autumn 2003.
  * Two line numbers are off: 817 `map/clif.c` 543 and 627 are 544 and
    630 (T11-016, T11-017). The checker's window let them pass.
  * Four rows' reasoning (T1-001, T1-002, T1-004, T1-007) reads Weggy's
    simple 2-1 job changes as "confirms iRO had no test" in May and June
    2004. They are older text. Weggy came back on 2004-05-17 after "about
    a year long break from RO". On 05-18 (v6.1) he wrote: "I just realized
    that Gravity also added more in-depth quests to the 2-1 classes, so
    i'll hopefully be able to update those soon". On 06-15 (v6.6) he
    "Updated some 2-1 quests", without naming them. So the Knight's
    "2500 zeny" and the Assassin's "666 zeny" describe iRO before his
    break, about mid-2003. The Wizard's and the Blacksmith's sections
    were touched after 05-18 and still say "talk to", so they say little
    either way.
* **`progress.md`:** it names the Priest test's quiz-givers "Father
  Mareusis / Peter S. Alberto", and Weggy names nobody. It also repeats
  the "no Knight test" reading as fact. The rows hold; the summary
  embroiders, as round 2's report did.

## What is new

25 of the 50 rows say something THREADS.md and the notes did not have.

**T1 (2-1 tests), 9 of 10 new.**
* Weggy's FAQ (GameFAQs 20092, iRO): his pre-break text has the Knight for
  2500z at the culvert registration building, the Assassin for 666z, the
  Wizard and the Blacksmith by talking to them. His v6.1 note of
  2004-05-18 says the 2-1 quests were added since. That fits THREADS #5
  (none in the closed beta) and dates nothing more finely.
* Weggy's Priest section (v6.4, 2004-06-07, iRO): "A priest in the
  antechamber to the left will first ask you some questions ... After
  that, you will walk a hall of temptations ... Finally, you will be put
  into rooms where undead monsters spawn. You can actually just run past
  them all". So iRO had a three-part Priest test by 2004-06-07, a day
  before jAthena's script of 2004-06-08. But the order and the rule
  differ from that script, which we play: jAthena has a pilgrimage, then
  an exorcism where thirteen undead must fall before the gate opens (the
  tempters come after them), then a vow.
* Guide_Maker's Forging guide (GameFAQs 28512, pRO, section added
  2004-08-31, hedged "I think"): the Blacksmith test as eAthena 1.0 has
  it: directions in Geffen, a quiz in Alberta, materials delivered, a
  "Reciept" back in Alberta, a quiz in Morroc, the "Hammer of
  Blacksmith". The first player's account of it, still autumn 2004.
* T1-008 (the Master Hunter at the Archer guild or the palace) repeats
  round 5.

**T9, 4 of 4 new.**
* Swordman course (Weggy, v6.0 and v6.2, May 2004): "If you do fall off,
  there are some aggressive Fabres and Chonchons down there." Both kinds:
  817 has Chonchons, mod0631 Fabres.
* Moving HP Recovery (vgmaster88, iRO, autumn 2003): "1 Padded Armor
  (which you get to keep), 500 empty bottles, and a rocker doll." This
  answers THREADS #7: the armour was asked for on the international
  servers, and it was not taken. That is 817's code (it tests for one and
  does not take it), not our draft (wear one, not tested).
* Fatal Blow: vgmaster88 says 10 Tentacles; mod0631, 817 and RODataZone
  say 30. The guide's slip. Auto Berserk agrees with all three.

**T10, 0 of 7 new.** T10-001 to -004 are round 5's Weggy rows again; -005
to -007 are THREADS #6's Usher lines. Leo's questions, the prizes, the
end rooms and the Fabre forest: nothing.

**T11 (May-2004 client), 12 of 29 new.**
* The two bytes the unit packets gained are the base level: eAthena 817
  writes `(base_level>99)?99:base_level` at 0x0078+52, 0x007b+58 and
  0x0079+51 for its packet version 4 and below (`map/clif.c:544`, `630`,
  `855`). That answers the open item in `may2004-client.md`.
* euRO's own skill text of May 2004 (`yuno_lang/data/skilldesctable.txt`),
  against the drafts:
  * Endow failure breaks the weapon (:1794): as drafted (the guide said
    so; now the client does too).
  * Dispell takes a Yellow and a Red Gemstone (:1828), the Chemical
    Protections a Coating Bottle (:1445): as drafted.
  * Defender slows movement and attack (:1591); Grand Cross costs 20% of
    the HP left (:1572; not drafted).
  * **The Strips differ.** Strip Weapon: a monster's ATK down (:1309, no
    number). Strip Shield: DEF −15% (:1316). Strip Armor: **VIT** −40%
    (:1323). Strip Helm: **INT** −40% (:1330). The draft follows the iRO
    guide of 2004-03-27: −10% ATK, and −10% DEF for each of the other
    three.
* The packet diff (7 grown, 0x014e gone, 22 new ids) and the GRF diff's
  totals repeat what the notes had; recomputed, they agree.

## What follows

* Strips: euRO's text of May 2004 is nearer the jobs' opening (June 2004)
  than the guide of March. Re-draft with its numbers, or keep the guide's
  and say why (THREADS #25).
* Moving HP Recovery: ask for a Padded Armour in the bag and leave it
  there, as 817 does (THREADS #7, DEVIATIONS).
* Priest test: whether "run past them all" or "thirteen must fall"
  (THREADS #8, queue T1).
* The pages worth keeping are in `r7/pages/`: Weggy's FAQ as of v6.7
  (2004-07-09) has every job's section. Its pre-break text is a witness of
  iRO in 2003.
