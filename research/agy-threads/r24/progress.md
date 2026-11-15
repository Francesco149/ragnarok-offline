# Round 24 Progress

## Tasks and Status

| Task | Topic | Status | Rows |
|---|---|---|---|
| T9 | Small, independent (threads 10, 32, 34, 35) | partly (Auto Counter pre-Comodo sure critical confirmed in Mad Monarch Gyl FAQ 26008; Auto Counter post-Comodo no-critical, DEF ignore, 2x damage, +20 HIT, and stance immobility confirmed in Psycho Battousai FAQ 21546 Section XV; Bowling Bash and Brandish Spear 1s cast time and 2/3 equipment DEF confirmed in Psycho Battousai FAQ 21546 Section XV; Hammer Fall 3x3 tile reach confirmed in the only KoW FAQ 24428; Payon Cave 1F monster spawns without Skeletons confirmed in Phelan FAQ 27955; death 1% EXP penalty and Resurrection HP-only restoration confirmed in admira FAQ 26968; period drop rates for mod0631 empty holes confirmed in Ragu Memo armor5 and weapon1/2/4 for Bongun Hat 1/5000, Poring Hat 1/5000, Ribbon 1/28, Heaven Ring 1/5000, Nut Shell 1/80, Pole Axe 1/5000, Guisarme 1/6, Stiletto 1/70, Crescent Scythe 1/50, Cutlas 1/66; plant damage from star crumbs and ground skill vanishing off-screen searched with negative result) | T9-284 – T9-303 |

Total: 20 rows across 1 task.
Checker result (`check.txt`): 18 PASS, 0 WARN, 0 FAIL, 2 NONE of 20 rows verified with `python3 ../../check_rows.py --pages pages rows/*.jsonl`.

## Task Details and Row Mapping

### T9: Small, Independent (Threads 10, 32, 34, 35)
- **Status:** partly
- **Rows:** `T9-284` through `T9-303`
  - `T9-284`: Mad Monarch Gyl Lancer/Spear Knight Guide v2.6 (2003-10-02) records pre-Comodo Auto-Counter countered any attack with an immediate critical hit (source: `gamefaqs_spearknight_madmonarch_26008.html`, value: `countered by a critical hit`).
  - `T9-285`: Psycho Battousai Swordsman/Knight Guide v1.2/v1.3 Section XV The Komodo Patch records Auto-Counter no longer causes critical hits, ignores monster DEF, deals 2x critical damage, scales with enemy FLEE, and grants +20 HIT rate (source: `gamefaqs_swordsmanknight_psychobattousai_21546.html`, value: `+20 HIT rate`).
  - `T9-286`: Psycho Battousai Swordsman/Knight Guide v1.2/v1.3 Section XV The Komodo Patch records the Knight cannot move once starting Auto-Counter until struck by an enemy (source: `gamefaqs_swordsmanknight_psychobattousai_21546.html`, value: `can't move until it's done`).
  - `T9-287`: Psycho Battousai Swordsman/Knight Guide v1.2/v1.3 Section XV The Komodo Patch records Bowling Bash and Brandish Spear have an uninterruptible 1 second casting time during which equipment DEF is reduced to 2/3 (source: `gamefaqs_swordsmanknight_psychobattousai_21546.html`, value: `1 second casting time`).
  - `T9-288`: the only KoW Battlesmith Guide (2003-07-13, iRO) records Hammer Fall affects a 3x3 tile area with a 14% base stun chance at Level 1 (source: `gamefaqs_battlesmith_theonlykow_24428.html`, value: `3x3 tile area`).
  - `T9-289`: Phelan Battle Priest Guide v1.1 (2004-01-13, iRO) records Payon Cave Level 1 monsters as Familiars and Zombies, retaining the pre-rebalance composition without Skeletons (source: `gamefaqs_battlepriest_phelan_27955.html`, value: `Monsters-Familiars, Zombies`).
  - `T9-290`: admira Priest Guide v1.1 (2004, iRO) records character death incurs a loss of 1% EXP (source: `gamefaqs_priest_admira_26968.html`, value: `1% exp`).
  - `T9-291`: admira Priest Guide v1.1 (2004, iRO) records Resurrection restores 10% to 80% HP by skill level rather than restoring lost EXP (source: `gamefaqs_priest_admira_26968.html`, value: `HP restored`).
  - `T9-292`: Ragu Memo (2003-12-12, jRO) records Bongun (1188) drops Bongun Hat (5046) at rate 1/5000 (source: `ragmemo_armor5_20031212.html`, value: `1/5000`).
  - `T9-293`: Ragu Memo (2003-12-12, jRO) records Marin (1242) drops Poring Hat (5035) at rate 1/5000 (source: `ragmemo_armor5_20031212.html`, value: `1/5000`).
  - `T9-294`: Ragu Memo (2003-12-12, jRO) records Sting (1207) drops Ribbon (2209) at rate 1/28 (source: `ragmemo_armor5_20031212.html`, value: `1/28`).
  - `T9-295`: Ragu Memo (2003-12-12, jRO) records Skel Prisoner (1196) drops Soul Ring (Heaven Ring, 2282) at rate 1/5000 (source: `ragmemo_armor5_20031212.html`, value: `1/5000`).
  - `T9-296`: Ragu Memo (2003-12-12, jRO) records Wild Rose (1261) drops Nut Shell (5037) at rate 1/80 (source: `ragmemo_armor5_20031212.html`, value: `1/80`).
  - `T9-297`: Ragu Memo (2003-12-18, jRO) records Gryphon (1259) drops Pole Axe (1417) at rate 1/5000 (source: `ragmemo_weapon4_20031218.html`, value: `1/5000`).
  - `T9-298`: Ragu Memo (2003-12-18, jRO) records Gryphon (1259) drops Guisarme (1452) at rate 1/6 (source: `ragmemo_weapon4_20031218.html`, value: `1/6`).
  - `T9-299`: Ragu Memo (2003-12-18, jRO) records Desert Wolf (1106) drops Stiletto (1217) at rate 1/70 (source: `ragmemo_weapon1_20031218.html`, value: `1/70`).
  - `T9-300`: Ragu Memo (2003-12-18, jRO) records Baphomet (1039) drops Crescent Scythe (1464) at rate 1/50 (source: `ragmemo_weapon4_20031218.html`, value: `1/50`).
  - `T9-301`: Ragu Memo (2003-12-18, jRO) records Drake (1112) drops Cutlas (1135) at rate 1/66 (source: `ragmemo_weapon2_20031218.html`, value: `1/66`).
  - `T9-302`: Negative result: searched GameFAQs guides 25263, 24428, 27955, 26968, 26008, 21546 for plant/mushroom damage scaling. Negative result: period player guides record no special damage bonus exceeding 1 damage to plants or mushrooms from refined weapons or star crumbs.
  - `T9-303`: Negative result: searched GameFAQs guides 24428, 27955, 26968, 26008, 21546 for ground skill disappearing off-screen. Negative result: period player guides record no reports of player or monster Fire Wall, Safety Wall, or Pneuma vanishing when walking off screen.
