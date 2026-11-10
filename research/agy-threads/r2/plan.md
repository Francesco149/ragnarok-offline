# Deep Research Plan: Ragnarok Offline Round 2

**Date:** 2026-11-10  
**Brief:** Continue research on open threads for `ragnarok-offline` (euRO 2003-12-18 client) following `HANDOFF-r2.md`. Replace inference with period primary evidence (Grade A/B), checking local period sources (mod0631, jAthena git history, client GRF/strings) and web primary sources (kRO notices, iRO/euRO Wayback captures, GameFAQs period guides, bug boards). Strictly avoid leaked Gravity server files (Aegis). All claims formatted in single graded rows.

## Decision and Context
- Client: euRO CD built 2003-12-18 (`Ragexe.exe`), targeting closed/open beta (Dec 2003 - Feb 2004).
- Server reconstruction: Rust/Lua/TOML, strictly tagged provenance.
- Strict grading: A (period primary <= 2004-06), B (period emulator <= 2004-05), C (independent later sources), D (single later / inference), X (source disagreement).
- Deliverables:
  - `r2/t1-mod0631-vs-817.tsv`
  - `r2/t2-jathena-history.tsv`
  - `r2/t5-official-text.tsv`
  - `r2/evidence/<task>/`
  - `r2/REPORT.md` (and task tables, self-check of 20 rows)

## Sub-Questions / Tasks

1. **t1-mod0631-npcs**: Systematic comparison of mod0631 (2003-12-18) NPC scripts against `scripts/import/npc/` (817). Mapping coordinate matches, differences in requirements/zeny/items, walkable checks via `gat_cells.py`.
2. **t2-jathena-history**: Complete commit history of jAthena up to 2004-06-30 for all NPC/conf/gvg files. Identifying commits from official server logs (`本鯖`, `凸さんのログ`), arrival of 2-1 job quests and tests, and WoE scripts.
3. **t3-kafra-points**: Tracing `KF_PIT` through jAthena and period notices. Accrual rules (1/10th teleport fee?), redemption NPCs (Al De Baran HQ), period reward tables, and official kRO/iRO/euRO notices.
4. **t4-woe-period**: Period WoE rules from official sources only: iRO April 2004 guide pages (`warofemperium.asp`, `guildsystem.asp`), kRO notices on 공성전 / 보물상자 / 투자 / 길드던전, early jAthena `conf/gvg/`, 817 `npc/guild/`.
5. **t5-official-text**: Gathering authentic period English NPC dialogue from iRO bug board, period GameFAQs guides (<= 2004-06), iRO official guide pages, and client data tables (`data.grf`: `msgstringtable.txt`, item/skill descs).
6. **t6-refine-history**: Complete history of iRO refine page across all Wayback captures, kRO 2003-02-04 notice, and mod0631 refine script & db to settle the 2000z Phracon / L3-L4 fees / armor DEF questions.
7. **t7-rates-fares-start**: Period evidence for euRO closed beta rates (Gamona / euRO forum captures), pre-2004-02-24 iRO/euRO Kafra fares, and novice start cell screenshots.
8. **t8-small-checks**: Lutie disassembly in `Ragnarok.exe` (`xmas` branch), monster skills completeness in mod0631 vs 817, and pet system comparison between iRO guide (2004-04) and `pets.toml`.
