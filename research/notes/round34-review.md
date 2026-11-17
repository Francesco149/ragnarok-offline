# Research round 34: review (2026-11-17)

40 rows (7 T1, 33 T9), all PASS in `check.txt` and on my own run of
`check_rows.py --fetch`. Three pages saved: ArislanX's Alchemist guide
(GameFAQs 31925), unknown13's Monster Skill FAQ (GameFAQs 29608) and Ragu
Memo's `rec_item.html` (capture 2003-12-05).

## How the round did

* **progress.md matches the rows**: same 40 ids, claims and quotes.
* **Captures are real**: all three timestamps are in CDX with status 200,
  and each is the first capture of its page.
* **jAthena line numbers are exact**: all ten rows' lines hold the quoted
  line in //1046 (6332410, 2004-09-12), and the commit date is right.
* **One date found nowhere**: the three FAQ 29608 rows are dated
  2004-04-09. The page's header says "Version 1.0, Last Updated
  2004-04-15" and its history has v0.9 "April 4 2004", v0.95 "April 5
  2004" and v1.0 "April ?? 2004". 04-09 is on no line.
* **Nothing new in T1**: ArislanX's guide was read in rounds 11 and 28 and
  the Alchemist test was built from jAthena //1046 that afternoon.
* **Nothing new in the T9 monster rows**: Mental Breaker for Clock,
  Phreeoni, Nightmare and Myst and Khalitzburg's Decrease AGI were already
  built from jAthena //0953 (`data/db/mob_skills_faq.toml`). The other
  five rows (Bathory's Energy Drain, Cat O'Nine Tails' Heal, Giant
  Hornet's Stun Attack, the Giant Spider's and Gullinbursti's summons) are
  skills that neither the FAQ nor RODataZone gives those monsters; mod0631
  has every one of them already (`mob_skills.toml`). The queue asked for
  witnesses to the *guessed* rows (Bathory's HP Absorption, the Giant
  Spider's Envenom, ...); none came.

## What was built

* **Healing items count VIT and INT at 2% a point, not 1%.** Ragu Memo's
  page (jRO, 2003-12-05, the same text in the captures of 2004-02-22 and
  2004-06-19): `HP回復量 = 基本回復量 * (1 + VIT/50 + HP回復向上Lv/10) *
  (1 + ラーニングポーションLv/20)`, INT/50 for SP. Behind it, RODataZone's
  iRO stats page, captured 2003-12-04 (`www.rodatazone.com/mechanics/stats.php`,
  CDX's first): "Healing items effect increased by +2% per point" of VIT.
  jAthena made the same change on 2004-03-10 (//0880, `paramc[2]<<1`, no
  reason given, among the Potion Pitcher work); mod0631, 817 and
  eAthena mod 715 count 1%. Built in `scripts/rules/combat.lua`
  `item_heal` and in Aid Potion (where the "jAthena counts VIT twice"
  deviation is gone: that was the period rule). Tests
  `trade_potions_and_weight`, `alchemist_acid_bomb_aid`. Learning Potion's
  5% a level and the recovery skills' 10% already matched.
  RODataZone's INT line says nothing on SP items: for SP, Ragu Memo and
  jAthena are the witnesses.
* **Base amounts read whole**: Ragu Memo's 19 items set beside our
  `use_script`s: 15 match exactly. Four differ at the top only: Orange
  Potion and Ice Cream 105-143 (ours 105-144), White Potion and Royal
  Jelly 325-402 (ours 325-404). A measured range can miss its top value,
  so nothing changed; noted in THREADS.

Pages copied to `data:refs/web/` (`ragmemo-2003/ragmemo_rec_item_*`,
`rodatazone-2004/20031204072212_mechanics_stats.html`).

## Trick

When a row gives a formula, grep our rules for the same formula before
anything else. Here the round's only new row turned a constant we had
taken from mod0631 without a second look.
