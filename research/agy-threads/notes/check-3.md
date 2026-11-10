Phracon was sold by official NPC upgrade merchants for 200z and Emveretarcon for 1,000z; the '2000 zeny' price listed on the official iRO Refine System website was an editorial typographical error.

## Verdict
Confirmed. Official NPC upgrade merchants ("Miner" NPCs inside the forge buildings in Prontera, Morroc, Payon, Alberta, and Yuno) sold Phracon for 200z and Emveretarcon for 1,000z. The "2000 zeny" price listed in the upgrade table on the official iRO website (`refinesystem.asp`) was an editorial typographical error—specifically an accidental trailing zero appended to "200", as also evinced by the omission of the thousands comma present on all other four-digit values in the same table ("1,000 zeny", "2,000 zeny", "5,000 zeny"). In the game's tiered economy, Weapon Level 1 upgrades cost 50z plus one Phracon (safe limit +7 total: 1,750z), whereas Weapon Level 2 upgrades cost 200z plus one Emveretarcon (safe limit +6 total: 7,200z). A 2,000z Phracon would have forced a Level 1 safe upgrade to 14,350z, completely inverting the tier progression. The 200z and 1,000z prices are verified by contemporaneous player guides (e.g. RoboShop's Beta 2 Weapon Guide, Nov 2002), Athena mod0631 server item database (Dec 2003), and period eAthena NPC shop scripts (`phraconseller.txt`, 2004).

## Findings
- **Official iRO Refine System table typographical error:**
  The official iRO guide table lists:
  `| Weapon Level 1 | | Phracon | 2000 zeny | 50 zeny | Each +1 increases 2 damage |`
  `| Weapon Level 2 | | Emveretarcon | 1,000 zeny | 200 zeny | Each +1 increases 3 damage |`
  Notice that Emveretarcon is formatted with a comma (`1,000 zeny`), as is the Armor fee (`2,000 zeny`) and Weapon Level 3 fee (`5,000 zeny`), whereas Phracon is formatted without a comma as `2000 zeny`, indicating a manual typo adding an extra zero to `200`. — [http://iro.ragnarokonline.com:80/game/refinesystem.asp](https://web.archive.org/web/20040605070057/http://iro.ragnarokonline.com:80/game/refinesystem.asp) (archived 2004-06-05; high)

- **Contemporaneous Beta 2 guide documenting exact NPC prices:**
  RoboShop's Ragnarok Online Beta 2 Weapon Guide explicitly records:
  *"Each upgrade shop houses three NPC's: one who will sell you Phracon and Emveretarcon, one who will change Rough Elunium into Elunium and Rough Oridecon into Oridecon, and a third who will perform the actual upgrades. Phracon costs 200z and Emvertarcon costs 1000z: because they're bought through an NPC conversation interface rather than a shop, Merchant Discount doesn't work, but they can now be bought in multiples."* — [https://gamefaqs.gamespot.com/pc/561051-ragnarok-online/faqs/18644](https://web.archive.org/web/20190505182848/https://gamefaqs.gamespot.com/pc/561051-ragnarok-online/faqs/18644) (published 2002-11-22; high)

- **Mathematical proof in contemporaneous upgrade calculations:**
  RoboShop calculates the exact cumulative cost to reach the safe refine limits:
  *"+7: Anyone can get a Weapon Level 1 to +7 with a mere 1750 zeny investment, with no risk involved and very little time invested."* (7 upgrades × [200z Phracon + 50z fee] = 1,750z).
  *"+6: Anyone can get a Weapon Level 2 to +6. A good level to shoot for if you're short on zeny. It only costs 7200 zeny to get a +6."* (6 upgrades × [1,000z Emveretarcon + 200z fee] = 7,200z).
  If Phracon had been 2,000z, a +7 Level 1 weapon would have cost 7 × 2,050z = 14,350z, nearly double the cost of a +6 Level 2 weapon. — [https://gamefaqs.gamespot.com/pc/561051-ragnarok-online/faqs/18644](https://web.archive.org/web/20190505182848/https://gamefaqs.gamespot.com/pc/561051-ragnarok-online/faqs/18644) (published 2002-11-22; high)

- **Period server item database buy prices:**
  In Athena mod0631 (2003-12-18), imported in `data/db/items.toml`:
  Item ID 1010 (`Phracon`): `buy = 200`, `weight = 200`
  Item ID 1011 (`Emveretarcon`): `buy = 1000`, `weight = 200` — `data/db/items.toml` lines 2752–2769 (tier athena2003, 2003-12-18; high)

- **Period server NPC shop definitions:**
  In `eathena-npc.rar:npc/shops/phraconseller.txt`, the upgrade merchants across all major towns are defined as:
  `prt_in.gat,56,69,4 shop Miner 119,1010:-1,1011:-1`
  `payon_in01.gat,84,26,4 shop Miner 88,1010:-1,1011:-1`
  `morocc_in.gat,65,37,4 shop Miner 99,1010:-1,1011:-1`
  `alberta_in.gat,18,59,5 shop Miner 84,1010:-1,1011:-1`
  `yuno_in01.gat,171,26,6 shop Miner 88,1010:-1,1011:-1`
  In Athena/eAthena script syntax, `-1` as the price parameter specifies that the item is sold at its default database `buy` price (200z for Phracon, 1,000z for Emveretarcon). — `eathena-npc.rar:npc/shops/phraconseller.txt` (timestamp 2004-03-22; high)

- **Refiner dialogue and fees:**
  In `refine.txt`, the blacksmith refiners (Hollgrehenn, Aragham, Antonio, etc.) enforce:
  Level 1 weapon: 1 Phracon (1010) + 50z fee
  Level 2 weapon: 1 Emveretarcon (1011) + 200z fee — `research/agy-threads/refine.txt` lines 122–140 and `eathena-npc.rar:npc/other/refine.txt` (timestamp 2004-03-22; high)

## Contradictions and open points
- None. The table on `iro.ragnarokonline.com` has an obvious typo in `2000 zeny` for Phracon. All historical gameplay mechanics, client item databases, server scripts, and period guides unanimously confirm Phracon was 200z and Emveretarcon was 1,000z.

## Sources read
- `https://web.archive.org/web/20040605070057/http://iro.ragnarokonline.com:80/game/refinesystem.asp` — Official iRO website upgrading guide containing the table with the "2000 zeny" typo for Phracon.
- `https://web.archive.org/web/20190505182848/https://gamefaqs.gamespot.com/pc/561051-ragnarok-online/faqs/18644` — M. Fitzpatrick (RoboShop) Beta2 Weapon Guide (Nov 2002), providing independent contemporaneous verification of 200z / 1,000z prices and exact upgrade cost calculations.
- `eathena-npc.rar:npc/shops/phraconseller.txt` (2004-03-22) — Official eAthena NPC script defining ore shop Miner NPCs using default DB buy prices (`1010:-1,1011:-1`).
- `data/db/items.toml` / Athena mod0631 (2003-12-18) `item_db.txt` — Primary server item database confirming Phracon buy price 200 and Emveretarcon buy price 1,000.
- `research/agy-threads/refine.txt` & `eathena-npc.rar:npc/other/refine.txt` (2004-03-22) — Period refining scripts detailing NPC dialogue, ore requirements, and fee amounts.
- `https://roid-classic.gitbook.io/roid-documentation/enchancements/refinement-+-modified-sales-prices` — Modern classic reference corroborating standard NPC shop prices (Phracon 200z, Emveretarcon 1,000z).
- `https://ro.gnjoy.com/guide/runemidgarts/itemview.asp?itemSeq=7&ID=1010` — Official Gravity Korean RO portal confirming Phracon (ID 1010) as Lv 1 weapon refining ore.

## Could not read
- `https://gamefaqs.gamespot.com/pc/561051-ragnarok-online/faqs/18644` (live URL) — Cloudflare 403 bot block; read via Wayback Machine snapshot (`20190505182848`).
- `https://ratemyserver.net/index.php?page=refine_guide` — 404 / bot-blocked page on RMS; verified via item database and guides.
- `https://forum.motr-online.com/topic/813/...` — Anubis proof-of-work anti-scraping challenge.
