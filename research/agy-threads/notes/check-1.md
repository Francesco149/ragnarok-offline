# Fact-Check: euRO Closed Beta 2-1 Job Advancements & New Comodo Update

> **Claim:** During the euRO closed beta (Dec 20, 2003 – Jan 8, 2004), 2-1 second job advancements (Knight, Priest, Wizard, Blacksmith, Hunter, Assassin) required zero quests or test trials, granting instant advancement at Job Level 40 via plain guild NPCs under an in-game decree from King Tristram III; 2-1 job quests were only introduced on euRO with the 'New Comodo' update on January 8/18, 2004.

## Verdict
**Partly confirmed** (with significant corrections regarding dates and the origin of the King Tristram III dialogue).

### Corrected Wording
During the initial period of the euRO closed beta (December 20, 2003 – January 8/18, 2004), 2-1 second job advancements required zero quests or test trials, allowing instant advancement at Job Level 40; 2-1 job quests were officially introduced to euRO with the "New Comodo" update dated January 8, 2004 (announced on the forum on January 18, 2004). However:
1. The euRO closed beta did **not** end on January 8, 2004; it continued until January 31 – February 2, 2004 (with open beta launching on February 4, 2004). January 8 was merely the date of the "New Comodo" server update during closed beta.
2. The dialogue attributing instant job advancement to an in-game decree from King Tristram III was **not** an authentic official euRO or Gravity Aegis decree. It is fan-written placeholder dialogue authored by contemporary Athena emulator scriptwriters (specifically RoVeRT for `knight.txt` and Darkchild for `hunter.txt` in late 2003) to justify their simple Job Level 40 job-changer scripts before official quests were implemented in Athena. euRO ran official Gravity Aegis server software under a strict license barring game modifications.

---

## Findings

- **New Comodo update introduced 2-1 job quests to euRO on Jan 8/18, 2004:**
  - *"New in 'New Comodo' (among others): Added Heal-Effect, Modified Potions, Added 2nd Job Quest, Added Monster Skills, Added lvl99-Aura, Fixed Refining Bug, Adjusted Monsters HP/Exp"* — [http://www.euro-ro.net/server/](https://web.archive.org/web/20040207012248/http://www.euro-ro.net/server/) (published 2004-01-08 / 2004-02-04; high).
  - *"New in 'New Comodo' ... 2. added 2nd job quest"* — [http://forum.euro.bhi-oe.de/viewtopic.php?t=1791](https://web.archive.org/web/20040723203021/http://forum.euro.bhi-oe.de/viewtopic.php?t=1791) (published 2004-01-18; high).
  - Player response confirming quest addition and initial bugs: *"very good the 2nd job Quest but buggy, i can't be a Priest \*sigh\* but very good ^^ i like the Monsterskills"* (Condor, Jan 18, 2004 9:11 am) — [http://forum.euro.bhi-oe.de/viewtopic.php?t=1791](https://web.archive.org/web/20040723203021/http://forum.euro.bhi-oe.de/viewtopic.php?t=1791) (published 2004-01-18; high).

- **Closed Beta dates were Dec 20, 2003 – Jan 31/Feb 2, 2004, NOT ending Jan 8:**
  - *"Prontera (pna) - The Prontera Tourism Office today announced the official launch of the Closed Beta of Raganrok Online (RO) for Europe (euro-ro)... Closed Beta Started | December 20th, 2003"* — [http://www.euro-ro.net/server/](https://web.archive.org/web/20040207012248/http://www.euro-ro.net/server/) (published 2003-12-20 / 2004-02-04; high).
  - Closed beta ended with full wipe between Jan 31 and Feb 2, 2004, with open beta launching Feb 4, 2004 — `research/notes/euro-history.md` §1 citing official maintenance notices `viewtopic.php?t=2400` and `viewtopic.php?t=2539` (high).
  - On Jan 18, 2004, the forum section where "New Comodo" was posted was still titled *"euRO euRO closed beta"* — [http://forum.euro.bhi-oe.de/viewtopic.php?t=1791](https://web.archive.org/web/20040723203021/http://forum.euro.bhi-oe.de/viewtopic.php?t=1791) (published 2004-01-18; high).

- **Origin of King Tristram III decree dialogue (Athena emulator fan scripts, not Aegis/euRO):**
  - Knight script: `//Knight Quest for Athena by RoVeRT` contains:
    *"His Majesty Tristram 3rd has been encouraging people to be Knights without any difficult requirements except Job Level 40 or above. Anyway according to the Rumor, people say another requirements would be added soon when the Society become stabilized. If it were so,you got to become a Knight now."* — `research/agy-threads/knight.txt` / `scripts/import/npc/jobs/knight.lua` lines 46-52 (written late 2003, imported from eathena 817 2004-02-19; high).
  - Hunter script: `//= Hunter Job Quest Script //= By: Darkchild` contains:
    *"It seems the King needs more Hunters! So he established a new constitution of Hunters, so you can be a Hunter anytime if you reach at Job Level 40 or above. This is the good chance to become a Hunter! I assume the Government will introduce many Strict requirements for Hunter later if there are too many people want to be Hunters!"* — `research/agy-threads/hunter.txt` / `scripts/import/npc/jobs/hunter.lua` lines 51-57 (written late 2003, imported from eathena 817; high).
  - Blacksmith script: `//BlackSmith NPC for Athena by RoVeRT` contains:
    *"Actually Rune-Midgard is an Underdevelopment Country in the Metal Industry! That's why the Govenment encourage people to join us,Blachsmiths more and more. To make our nation an Advanced Country, we establish the requirements of Blacksmiths only Job Level 40!"* — `research/agy-threads/blacksmith.txt` / `scripts/import/npc/jobs/blacksmith.lua` lines 45-46 (high).
  - Wizard script from Athena 659 (2004-01-01):
    *"To Utilise the Magical Power of Geffen, Rune-Midgard encourages Mages to Grow Wizards. Why don't you apply on the Wizard if you reach at Job Level 40?"* — `scripts/import/npc/jobs/wizard.lua` lines 46-47 (high).
  - Script provenance tag in ragnarok-offline:
    `-- Tier athena2003: Athena-family emulator script, contemporary with the client; dialogue is the emulator authors' (often fan-translated), not Gravity's.` — `scripts/import/npc/jobs/knight.lua` line 2-3 (high).
  - euRO server software was Gravity's official Aegis, licensed under a strict agreement prohibiting any modification:
    *"licensee is 'strictly prohibited from any modification… of any part of the Game'"* — SEC filing Exhibit 4.4, Gravity-Burda Agreement dated 2003-11-26 §3.4; `research/notes/euro-history.md` §2 (high).

---

## Contradictions and Open Points

1. **Closed Beta Timeline vs New Comodo Date:**
   - The claim asserted closed beta lasted "Dec 20, 2003 – Jan 8, 2004". Jan 8, 2004 is the date the official website assigns to the "New Comodo" update. Closed beta itself continued through the end of January 2004.
2. **Official In-Game Lore vs Emulator Developer Narrative:**
   - The claim stated that instant advancement was granted "under an in-game decree from King Tristram III". In reality, King Tristram III was used as an in-universe narrative device by Athena emulator contributors (RoVeRT and Darkchild) to explain why their simplified job changers did not have quests yet. There is no evidence Gravity's official Aegis scripts featured this decree; official servers simply lacked the 2nd job quest scripts prior to their rollout in the New Comodo patch.

---

## Sources Read

- `https://web.archive.org/web/20040723203021/http://forum.euro.bhi-oe.de/viewtopic.php?t=1791` — Primary: euRO official forum thread by BHI-OE Director PerrI listing New Comodo additions (Jan 18, 2004). Highly reliable.
- `https://web.archive.org/web/20040207012248/http://www.euro-ro.net/server/` — Primary: euRO official website server news archive detailing Closed Beta start (Dec 20, 2003) and New Comodo update (Jan 8, 2004). Highly reliable.
- `/opt/src/ragnarok-offline/research/notes/euro-history.md` — Primary/Secondary repository historical study synthesizing SEC filings, official euRO news, and patch logs. Highly reliable.
- `/opt/src/ragnarok-offline/scripts/import/npc/jobs/knight.lua` — Primary repository import of eathena 817 `jobs/knight.txt` authored by RoVeRT.
- `/opt/src/ragnarok-offline/scripts/import/npc/jobs/hunter.lua` — Primary repository import of eathena 817 `jobs/hunter.txt` authored by Darkchild.
- `/opt/src/ragnarok-offline/scripts/import/npc/jobs/wizard.lua` — Primary repository import of Athena 659 `jobs/npc_wizard.txt`.
- `/opt/src/ragnarok-offline/scripts/import/npc/jobs/blacksmith.lua` — Primary repository import of eathena 817 `jobs/blacksmith.txt` authored by RoVeRT.
- `/opt/src/ragnarok-offline/scripts/import/npc/jobs/assassin.lua` — Primary repository import of eathena 817 `jobs/assassin.txt` authored by RoVeRT.
- `/opt/src/ragnarok-offline/scripts/import/npc/jobs/priest.lua` — Primary repository import of eathena 817 `jobs/priest.txt` authored by DiaDz (dated Jan 9, 2004).
- `/opt/src/ragnarok-offline/research/agy-threads/knight.txt`, `hunter.txt`, `blacksmith.txt`, `assassin.txt`, `priest.txt`, `wizard.txt` — Period Athena script sources showing exact headers, authors, and script logic.
- `/opt/src/ragnarok-offline/research/agy-threads/digest-r1.md` — Subagent digest summarizing Thread #5 claims.
- `https://web.archive.org/cdx/search/cdx?url=forum.euro.bhi-oe.de/*` — Wayback Machine CDX API query verifying available forum captures.

---

## Could Not Read

- `https://gamefaqs.gamespot.com/pc/561051-ragnarok-online/faqs/29457` — BLOCKED: HTTP 403 / Cloudflare bot check. Expected to find guide text for 2003-2004 job quests, but sufficient primary documentation was found via archived euRO site and script files.
