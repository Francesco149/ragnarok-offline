# Rounds 5 and 6 review: nothing invented, and one find on our own disk

Rounds 5 and 6 are the outside agent's (Antigravity/Gemini) passes over
the queue (`research/agy-threads/QUEUE.md` of 2026-11-11). Output:
`research/agy-threads/r5/` (T1, T2, T10; 55 rows) and `r6/` (T3, T4, T5;
72 rows), each with `rows/*.jsonl`, saved pages, `check.txt` and
`progress.md`. Reviewed 2026-11-11 (sixth session of the day). Every row
was read against its source.

## How much was invented

| Layer | Result |
|---|---|
| Quotes, files, lines, URLs | **Nothing invented.** `check_rows.py --fetch` on both rounds: 126 PASS, 0 FAIL, 1 NONE, the same as the agent's two `check.txt` |
| `claim` | **Nothing false.** Four claims say a little more than their quote, and it is true in the file: T10-009 (the Helper's cell), T10-018 ("level 9"), T1-022/023 ("the test takes place" from an NPC header), T4-016 (an absence). The cells in T5-017 to -027 are the NPC headers of the same file |
| `source_date` | Mostly right this time: the file's own date (mod0631 `npc_event_rental.txt` 2003-11-06, `skill.c` 2003-12-12, 817 2004-02-03/-18) or empty for the GameFAQs guides. T3-006, T3-007, T5-001 and T5-010 carry 2026-11-09, the day the file came to this machine |
| Tiers | Two rows mis-tiered: T5-001 and T5-010 grade `research/agy-threads/euRO_quests.txt` "A, euRO", but that table is a much later client's quest log (it lists Transcendent quests; no 2004 client had a quest window). The 817 rows of T4 are labelled region "kRO" and the emulator code grade A |
| `progress.md` | The details it adds (Astartus, 2004-01-27, "13:00 CET") are all on the saved pages. One generalisation: "status changes used 10-minute placeholder durations" for mod0631 is Lex Aeterna's alone (`skill.c:3831`). T1 and T10 are marked "Answered"; neither is (below) |

The rules of the round-4 review held: no invented quote, no false claim,
no made-up date for a file on disk.

## What is new, and what is not

| Task | Rows | New to us |
|---|---|---|
| T1 2-1 tests | 27 | **The Hunter test on iRO, from a player**: Forever-Zero's Archer/Hunter guide on GameFAQs (faq 27633, captured 2010), whose version history says "1.9 (02/15/04) - Added the Hunter Quest", Turtle Island having brought it ("if the Turtle Island update was not implimented yet, the lady at the front desk ... would instantly change you"). T1-001 to -007. The Wizard guide (faq 30054; "2.1 Jul 30, 2004 ... Added Wizard Quest Section"): failing sends you back saved outside the tower, the skip after 5-6 failures for a Worn Out Scroll, one examiner question (T1-019 to -021). The rest quotes jAthena's Knight (2004-09-12, known) and Priest (in the game), 817's stubs and mod0631's breeders |
| T2 later client | 10 | **We had it all along.** The MICO07 DVD (ripped 2026-11-09, `data:mico07/`) carries euRO's May-2004 installer: `Ragexe-2004-05-07.exe` and `2004-05-06gdata_yuno_lang_g.gpf`. T2-007 to -010. Checked below |
| T10 training ground | 18 | Weggy's FAQ (faq 20092, the base text older than its 6.0 of 2004-05-17): "you should have 100 zeny, or 200 zeny if the final few NPCs guessed your class correctly", and skipping the training "whisks you away to a random town" (T10-012/013). The other sixteen rows quote eAthena 0.5.2's `novice.txt`, which is 817's script two months on, not a second witness; Leo's questions on an international server: nothing |
| T3 euRO news | 27 | Little: nearly every row is already in `research/notes/euro-history.md` (closed beta, 3x, wipe, open beta, P2P, prices, WoE of 2004-04-11, Juno, slots, Loki). The pages themselves are saved in `r6/pages/` |
| T4 ailments | 18 | **jAthena's first resistance**: commit `71d06d6` (//0729, 2004-01-21) adds `sc_def_mdef = 100 - mdef`, `sc_def_vit = 100 - (vit + luk/3)`, `sc_def_int = 100 - (int + luk/3)` for players and monsters; `188a761` (//0736, 2004-01-23) makes it shorten the time as well as the chance, and takes Stone Curse's time from the "マジスレテンプレ" (the 2channel mage thread's template). T4-001 to -007. The 817 and mod0631 rows restate THREADS.md #21 |
| T5 Turtle Island | 27 | Nothing: mod0631's `npc_turtle.txt` and `npc_event_ma_sword.txt` (in the game) and 817's `headgearquest.txt` (imported). Nothing from an international server |

So about twenty rows of 127 are new, and the most useful thing in the
rounds is a pointer into our own data.

## T2, checked

* `data:mico07/packet_lengths-Ragexe-2004-05-07.tsv` (made 2026-11-09):
  0x01cd 30, 0x01ce 6, 0x01cf 28, 0x01d0 8, 0x01d1 14, 0x01d2 10,
  0x01e1 8, the six that THREADS.md #25 says this client lacks, plus
  0x01f1 to 0x01ff. Against ours: the four spawn/idle packets 0x0078 to
  0x007b and 0x01d8 to 0x01da are two bytes longer, and 0x014e is gone.
* `strings Ragexe-2004-05-07.exe`: 128 Aegis names of 2-2 skills
  (`CR_`, `MO_`, `SA_`, `RG_`, `AM_`, `BA_`, `DC_`, `BD_`).
* `2004-05-06gdata_yuno_lang_g.gpf` (GRF 0x102, read with
  `cargo run --example grf_dump -p ro-grf`): `data\skillnametable.txt`
  has every 2-2 skill (`MO_TRIPLEATTACK#Triple_Attack#`,
  `CR_GRANDCROSS#Grand_Cross#`, `SA_AUTOSPELL#Auto_Spell#` ...), and
  `skilldesctable.txt` their English descriptions. Shield Reflect's
  answers a question of the queue (T7): "Reflect damage to enemy against
  short-distance attack ... Reflect only physical attack and you can use
  this skill only when you putting on shield."

The memory of the investigation of 2026-11-09 called the DVD "the
May-2004 build", and nobody connected it with thread 25 when the 2-2
skills came up two days later. Recorded in THREADS.md #25.

## For the next brief

* Keep the rules; they held.
* A file that is not a period source (later clients' tables, our own
  extractions) is grade D, whatever region it names.
* A guide on GameFAQs is dated by the version-history line that added the
  section quoted, not by the capture: say which line in the row.
* Before searching the web for a file, `ls` `data:` and read
  `docs/PRESERVATION.md`: the owner's discs hold more than the CD.
