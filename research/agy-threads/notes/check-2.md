# Fact-Check: Armor Refine DEF Mechanics and 2003-02-04 kRO Rebalance

**Claim:** Armor refine does not grant +1 or +5 hard DEF per upgrade level; rather, Gravity's 2003-02-04 kRO rebalance permanently established that armor refining grants exactly +0.7 DEF per refine level (calculated as sd->def += (refinedef + 50) / 100), intentionally making refined DEF less effective than base equipment DEF.

## Verdict, with the corrected wording
**Verdict:** `partly`

**Corrected Claim:**
Armor refine in Pre-Renewal does not grant +1 or +5 hard DEF per upgrade level; rather, Gravity's 2003-02-04 kRO rebalance established that additional DEF from refining was intentionally made less effective than base equipment DEF (empirically evaluated by the community and emulators as ~0.7 DEF per refine level, with the armor safe refine limit lowered from +5 to +4). However, this change was not permanent (Renewal completely replaced it with tiered integer DEF bonuses), Gravity never published an explicit "0.7" number or formula in its patch notes, and the formula `sd->def += (refinedef + 50) / 100` is an eAthena/rAthena C emulator implementation (using Athena's `sd` player struct and integer rounding division), not Gravity's proprietary server code.

## Findings
- Gravity's 2003-02-04 patch explicitly decreased the efficiency of refine DEF relative to pure armor DEF and lowered safe refine from +5 to +4 — "제련으로 인한 추가 DEF 는 순수한 방어구의 DEF 보다 효율이 낮게 변경 되었습니다. 방어구가 안전하게 제련되는 수치를 +5 → +4 로 변경 하였습니다." — https://mro.gnjoy.com/notice/detail?seq=360 (published 2003-02-03; high)
- Gravity never stated "+0.7 DEF" in official patch notes; official iRO guide documentation as late as June 2004 still claimed +1 defense per refine — "| Armor | | Elunium | not sold | 2,000 zeny | Each +1 adds 1 defense |" — https://web.archive.org/web/20040605070057/http://iro.ragnarokonline.com:80/game/refinesystem.asp (published 2004-06-05; high)
- Early Athena emulators encoded armor refine DEF inconsistently across 2003–2004 snapshots: 1 in mod0631 (Dec 2003), 5 in 817 (Feb 2004), and 70 (0.7) in r2 (Nov 2004) — "Armor DEF per refine is encoded as 1 (641), 5 (817) and 70 = 0.7 (r2) [V]." — /opt/src/ragnarok-offline/research/notes/mechanics-sources.md (published 2026-11-09; high)
- The formula `sd->def += (refinedef + 50) / 100` originated in eAthena/rAthena C emulator source code where `sd` is `struct map_session_data*` — "refinedef += sd->status.inventory[index].refine*refinebonus[0][0]; ... sd->def += (refinedef+50)/100;" — https://raw.githubusercontent.com/rathena/rathena/706e785091b4b16b0626289e69fc5ac4ab3bc068/src/map/pc.c (published 2004-11-04; high)
- eAthena's `refine_db.txt` explicitly treats 70 as 0.7 DEF (70% of 1 DEF) — "// Armor defense is done in percentage (ie 70 = .7 def) \n 70, 4,4, 100,100,100,100, 60, 40, 40, 20, 20, 10,	// Armor" — https://raw.githubusercontent.com/rathena/rathena/706e785091b4b16b0626289e69fc5ac4ab3bc068/db/refine_db.txt (published 2004-11-04; high)
- Integer rounding in `(refinedef + 50) / 100` means individual armor refines do not add isolated +0.7 DEF; instead, accumulated refine bonuses across all equipment are summed and rounded to the nearest integer — "base_status->def += (refinedef+50)/100;" — https://raw.githubusercontent.com/rathena/rathena/master/src/map/status.cpp (published 2026; high)
- Pre-Renewal community consensus and databases treat Pre-Re armor refine as +0.7 DEF per level (+7 DEF at +10) — "| Pre-Re | Each upgrade adds DEF | 0.7 ... Cumulative DEF added | 7 |" — https://ratemyserver.net/index.php?page=refine_db (published 2006; medium)
- Pre-Renewal mechanics documentation noted that 0.7 is an empirical approximation and the exact internal value was not publicly confirmed — "- Armor upgrades: +0.7 per point (inaccurate, exact value unknown)" — https://irowiki.org/classic/Defense (published 2009-11-21; high)
- The ~0.7 DEF scaling was not permanent: Renewal completely eliminated it in favor of tiered integer Hard DEF bonuses (+1 per level up to +4, +2 from +5 to +8, etc.) — "Bonus: 100 ... Bonus: 200 ... Bonus: 300 ... Bonus: 400 ... Bonus: 600 ... Bonus: 800" — https://raw.githubusercontent.com/rathena/rathena/master/db/re/refine.yml (published 2021; high)

## Contradictions and open points
- **Official Documentation vs. Game Reality:** Gravity Interactive's official iRO game guide (`refinesystem.asp`, Wayback 2004-06-05) stated that armor upgrading adds "+1 defense" per +1, contradicting both the 2003-02-04 kRO rebalance notice and actual in-game damage calculations. This was an un-updated legacy description from the pre-2003 era.
- **Emulator Confusion (1 vs 5 vs 0.7):** Early emulator releases struggled to model this: Athena mod0631 (Dec 2003) gave +1 DEF per refine; eAthena 817 (Feb 2004) gave +5 DEF (confusing the safe limit +5 with the bonus); eAthena r2 (Nov 2004) settled on 70 (0.7 DEF) using round-half-up integer division `(refinedef + 50) / 100`.
- **Exact Fraction:** iRO Wiki Classic notes that while +0.7 is the accepted emulator and community figure, Gravity never confirmed the exact fraction or internal floating/fixed-point formula used in Aegis.

## Sources read
- https://mro.gnjoy.com/notice/detail?seq=360 — Primary official kRO patch notice (2003-02-03) detailing the 2003-02-04 rebalance patch. Highly reliable.
- https://ragnarok.fandom.com/api.php?action=parse&page=RO_Patch_(2003_Feb._04)&prop=wikitext&format=json — Community archive reproducing the 2003-02-04 kRO patch notes with Korean text and English translation. Highly reliable.
- https://web.archive.org/web/20040605070057/http://iro.ragnarokonline.com:80/game/refinesystem.asp — Official Gravity Interactive iRO game guide (June 2004) documenting upgrading rules and fees. Highly reliable historical document.
- https://raw.githubusercontent.com/rathena/rathena/706e785091b4b16b0626289e69fc5ac4ab3bc068/db/refine_db.txt — eAthena/rAthena SVN revision 2 `refine_db.txt` (2004-11-04) showing armor refine constant 70. Highly reliable codebase record.
- https://raw.githubusercontent.com/rathena/rathena/706e785091b4b16b0626289e69fc5ac4ab3bc068/src/map/pc.c — eAthena/rAthena SVN revision 2 `pc.c` (2004-11-04) showing `sd->def += (refinedef + 50) / 100`. Highly reliable codebase record.
- https://raw.githubusercontent.com/rathena/rathena/master/db/pre-re/refine.yml — Modern rAthena pre-renewal refine database confirming 70 DEF per level. Highly reliable.
- https://raw.githubusercontent.com/rathena/rathena/master/db/re/refine.yml — Modern rAthena renewal refine database demonstrating tiered integer DEF scaling (+1, +2, +3, etc.). Highly reliable.
- https://raw.githubusercontent.com/rathena/rathena/master/src/map/status.cpp — Modern rAthena `status.cpp` demonstrating `base_status->def += (refinedef + 50) / 100`. Highly reliable.
- https://irowiki.org/classic/Defense — iRO Wiki Classic frozen mechanics documentation explaining hard DEF, soft DEF, and upgrade DEF. Reliable community reference.
- https://irowiki.org/classic/Refinement_System — iRO Wiki Classic refinement reference for safe limits and rates. Reliable community reference.
- https://ratemyserver.net/index.php?page=refine_db — RateMyServer pre-renewal vs renewal refinement tables. Reliable community reference.
- /opt/src/ragnarok-offline/docs/PROVENANCE.md — Internal project provenance guide documenting the 1 vs 5 vs 0.7 emulator divergence. Highly reliable local context.
- /opt/src/ragnarok-offline/research/notes/mechanics-sources.md — Internal project bibliography detailing period sources and emulator discrepancies. Highly reliable local context.

## Could not read
- https://forum.motr-online.com/topic/38450/%D0%BF%D1%80%D0%BE-%D0%B7%D0%B0%D1%82%D0%BE%D1%87%D0%BA%D1%83-%D0%BE%D1%80%D1%83%D0%B6%D0%B8%D1%8F-%D0%BE%D0%B4%D0%B5%D0%B6%D0%B4%D1%8B/41?page=7 — Blocked by Anubis Proof-of-Work bot check. Expected to contain player discussion of the eAthena refine DEF rounding formula.
