# Round 11 review (2026-11-14)

63 rows in six tasks (T1 9, T5 7, T6 7, T7 17, T8 3, T9 20), eleven saved
pages: GameFAQs 30054 (Ghent, Mage/Wizard), 29457 (Edge, Knight), 29037
(blood3d, mixing quests, pRO), 31925 (ArislanX, Alchemist, iRO), 29854 and
30168 (Calx Zinra, Rogue and Sage), 28552 (Tangowr, Crusader), 27646
(Battousai X, quest skills, pRO), 26381 (Godhand, Arrow Crafting, idRO),
28786 (Arctic Breeze, WoE, iRO); euRO forum topic 1791.

Round 12 was being written when this was reviewed (pages only, no rows):
not touched.

## Invention

* **Rows:** `research/check_rows.py --fetch --pages r11/pages` run again
  here: 60 PASS, 3 WARN (Wayback serves a neighbouring capture), 0 FAIL, as
  the agent's `check.txt`. No claim is false and none says more than its
  quote. (Without `--pages` the checker looks for `rows/pages/`, which this
  round does not have: 63 FAIL "saved page missing". Not the agent's fault.)
* **Metadata, the usual slip and two new ones:**
  * T7-001..005 are dated 2004-04-16, the Rogue guide's "Created" line; the
    page is v2.8 of 2005-05-13 and the Strip section is not dated by any
    version entry. Read as "by 2005".
  * Calx Zinra's guides (T7-001..012) are graded B, region pRO. Both say
    they draw on "pRO ... private servers ... and reading other FAQs"; the
    Sage text is the iRO guide's word for word, with other SP and duration
    numbers (below). Not a witness for any official server.
  * T9-006..020 are all region iRO. Section 5.2 (treasure materials by
    castle) and 5.3 (guild dungeons) say they come from a Taiwanese fan site
    by way of a Singaporean one: twRO or kRO data. And T9-006's "trade
    rating goes down by 2" is repeated in section 5.4 with "(Unconfirmed.
    Please send me an e-mail ...)", which the row leaves out.
  * T5-005/006 and T6-004..007 are dated 2004-02-28 (v1.0). The "Created
    at" lines came with v1.01 (2004-03-06, "added the locations of the
    quests"), and v1.05 (2004-08-08) "Fixed the Mixture and Counteragent
    Quest": the fees may be of August.
* **`progress.md`:** "Alberta headgear quests: Boy's Cap, Antler, and Sea
  Otter Raccoon Hat": the Raccoon Hat is not an Alberta quest in the guide.
  T9-004's reasoning names the Arrow Crafting NPC "Kyulnari"; the guide has
  no name (mod0631 has ギョルナリ at `morocc 121,109`: a reading of that,
  not the page). T1-009's "post by Condor Poring" is on the page.

## What is new

About 25 of 63 rows; the rest restate THREADS.md (T1-001..008: the Wizard
guide and Edge's Knight line, both in #8 since round 5; T8-003 and T1-009:
topic 1791, in #5) or numbers we already have (Strip SP and chances, Grand
Cross SP, Magic Rod, Dispell, Alchemist SP: all as in `data/db/skills.toml`).

* **Charge Arrow and Gershaun (thread 7), built.** Godhand (idRO,
  2003-10-23) and Battousai X (pRO, 2004-01-22) list a Crossbow for Charge
  Arrow and say he gives it back; Battousai X's Gershaun wants a Banana
  Juice unless you already have the items, which is mod0631's script. In
  the game: `scripts/npc/skill_quests.lua`, test
  `charge_arrow_crossbow_and_gershauns_juice`. Battousai X also gives Cart
  Revolution 30 Iron and Charge Arrow job level 37: noted in #7, not built.
* **The doomed swords (thread 24).** blood3d (pRO, v1.01 of 2004-03-06)
  gives the smith's three lists. They are what our smith takes, item for
  item and count for count (Mysteltain one each of four drops; Grimtooth
  10 + 5 Cursed Ruby + 100 Ogre Tooth + Old Hilt; Executioner 2/10/3/50),
  i.e. what mod0631's smith *asks* for, not what it takes. One snag: the
  guide calls Grimtooth's ten "Bloody Edge (Executioner)", where mod0631
  asks for Blade Lost in Darkness (7023); Bloody Edge (7024) is
  Executioner's own. Probably the guide's slip (the parenthesis is the
  drop's monster).
* **Alberta's hat quests (thread 24).** Boy's Cap (10 Tiger's Footskin, an
  old man in Alberta) and Antler (20 Evil Horn, upstairs in a mansion) are
  in the guide's v1.0 (2004-02-28), and the same guide dates pRO's Turtle
  Island to 2004-03-31. So on pRO these quests came before the island: they
  are not the island's "new quests". 817 has both (`headgearquest.txt`:
  Grampa, Cherokee) with the same items.
* **The treasure boxes and the castles (thread 17).** "Which box belongs to
  which castle has no period source" (round 3). Arctic Breeze's table
  (twRO/kRO data, in an iRO guide of 2004-02-18) names the god-item
  material each castle yields. Read 817's forty `TREASURE_BOX`es in pairs
  in region order (Al De Baran 1-5, Geffen 1-5, Payon 1-5, Prontera 1-5)
  and the even box of each pair drops exactly that material, all twenty.
  The pairing is therefore period. The guide's rate is 1/250 a box (0.4%);
  817 drops it at 10/10000 (0.1%).
* **The rest of thread 17**, from the same guide: two investments a day,
  each twice the last; a castle Kafra who stores free for the guild and
  warps to every town; guild dungeon spawns with respawn times (Eddga,
  Doppelganger, Dark Lord, Maya 8 h; Ghostring 4 h; Vagabond Wolf 1 h);
  guild levels' EXP; in WoE no EXP loss, no knock-back, no Endure,
  Anodyne, Ice Wall, Teleport, Fly Wing, Phen card or Intimidate; damage
  numbers hidden. 817 already refuses Endure, Teleport, Ice Wall, Back
  Slide and knock-back (round 3); Phen, Anodyne and Intimidate are new.
* **Dyeing (thread 13).** ArislanX (iRO, 2004-08-20): "Louitz in Alberta
  (loc 133/55)", "Aure Dupon (loc 181/114)", "Old Molgy ... Geffen
  Blacksmith Shop (loc 141/140)" who is "Molgensten" one line earlier. Our
  cells are the same; 817's "Morgenshtein" is kept for now. blood3d's fees
  (pRO, possibly of 2004-08): Counteragent 3000z, Mixture 4000z, Black
  Dye 7000z, Blue Dye 3500z.
* **Land Protector (T7), for when the Sage is built.** The iRO guide on
  disk (2004-03-27) gives 2 min, 2:45, 3:30, 4:15, 5:00 and SP 66 to 50;
  817's `skill_db` 120 to 240 s. Calx Zinra's 15-75 s and SP 74-90, and
  Volcano's 52-60 SP, are from somewhere else (a private server?): ignore.
* **Alchemist (T7):** ArislanX: Demonstration's broken weapon lowers a
  monster's ATK by 25%, Acid Terror's broken armour its DEF by 25%.
  Tangowr: Grand Cross reflects "2/3" of its damage on the caster.
* **T8:** pRO got the Comodo patch around 2003-12-17 (Battousai X's
  disclaimer); Ghent's guide was updated "for Comodo Patch to iRO" on
  2003-10-12. Nothing on when euRO got iRO's skill balance patch.

## Lesson

The same as rounds 9 and 10: the best of the round was in a saved page's
other sections (the WoE guide's 5.2 table against 817's box order), and a
guide's own sourcing line (5.2's "obtained from Reesian RO") overturns the
row's region. Read the section's provenance line, not only the guide's.
