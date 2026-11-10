# Fact Check: Novice Training Ground Spawn Point and Video GJO1B9ogKXY

Claim: Gameplay footage of the classic Novice Training Ground (video GJO1B9ogKXY, frames 0:54–1:14) shows newly created Novices spawning at the western grassy platform next to the water and walking east across the wooden bridge toward the training castle, visually validating the physical spawn position of cell (53, 111).

## Verdict
**Partly confirmed** (with critical corrections regarding source provenance, exact timestamps, and discrete coordinate validation):
- **Corrected statement**: Video GJO1B9ogKXY (a 2017 tutorial recorded on the private server *EosRO*, not an official 2003–2004 server) shows a newly created Novice appearing on the western grassy platform next to the water (spawning at frame 0:39 upon finishing character creation, then interacting with a tutorial NPC until 0:59) and walking eastward across the wooden bridge over the water to the castle gate between frames 1:04 and 1:14. While map geometry (`new_1-1.gat` / `new_zone01.gat`) and contemporary 2003 guides (such as Weggy's May 2003 FAQ) confirm that newly created characters began on this western grassy platform and walked east across the bridge into the castle, footage of a private server running an Athena/rAthena derivative (where `start_point: new_1-1.gat, 53, 111` has been the hardcoded default setting since mid-2003) constitutes circular emulator behavior and does not provide independent empirical validation that official Gravity servers in 2003 specifically used cell coordinate (53, 111).

## Findings
- Video GJO1B9ogKXY is not official 2003 footage, but a 2017 private server tutorial — "This tutorial is made in eos Ragnarok online." — https://www.youtube.com/watch?v=GJO1B9ogKXY (published 2017-02-20; high)
- Character spawn occurs at frame 0:39, not 0:54; the character appears on the western grassy island and interacts with the guide NPC before walking east across the bridge (frames 1:04–1:09) and reaching the castle gate at 1:14 — seen: frames 0:39, 0:49, 0:54, 0:59, 1:04, 1:09, 1:14 (/opt/src/ragnarok-offline/research/agy-threads/frames/GJO1B9ogKXY_0.50-1.20-sheet.jpg and GJO1B9ogKXY_0.00-0.50-sheet.jpg) — https://www.youtube.com/watch?v=GJO1B9ogKXY (published 2017-02-20; high)
- Map collision data verifies cell (53, 111) is on the western grassy platform and the bridge spans X=69–108, Y=110–113 eastward to the castle entrance at X=117–148 — decompressed mcache: cell (53,111) is walkable (type 0), facing the bridge across water — ~/data/ragnarok-cd/refs/rust-ro/config/maps/pre-re/new_1-1.mcache (high)
- Athena emulator code hardcoded (53, 111) on `new_1-1.gat` as the default character start point since mid-2003 — "struct point start_point={\"new_1-1.gat\",53,111};" — ~/data/ragnarok-cd/refs/_extract/jathena/char-0587.c (published 2003; high)
- Contemporary May 2003 official game walkthrough confirms novices started on the island and crossed the bridge eastward to the castle — "You'll start in the Novice training ground. Cross the bridge to your right and you will come upon a castle. Inside is the Novice Training Ground." — https://www.ign.com/articles/2003/05/20/ragnarok-online-walkthroughfaq-403797 (published 2003-05-20; high)
- Official server botting macro (pRO 2013) targets cell 53, 111 and NPC at 53, 114 before walking to castle warp at 148, 112 — "do move 53 111\ndo talknpc 53 114 r0 r0 r0 r0 r0\npause 3\ndo move 148 112" — https://forums.openkore.com/viewtopic.php?t=142852 (published 2013-08-30; high)
- Official Aegis script conversion assigns Basic Game Mechanics Instructor Shion to (53, 114) — "new_1-1,53,114,4 script Shion#nv1 727,{ ... The Training Grounds are located just past the bridge located to the right." — https://github.com/rathena/rathena/blob/master/npc/re/jobs/novice/novice.txt (published 2013; medium)

## Contradictions and open points
- **Contradiction**: The claim assumes video GJO1B9ogKXY is representative historical footage providing independent validation of official Gravity server settings. In reality, it is a 2017 video of the private server EosRO, which inherently relies on the Athena/rAthena emulator codebase where (53, 111) has been the default start point since 2003. This is circular evidence.
- **Timing detail**: The claim states frames 0:54–1:14 show novices spawning and walking east. In fact, character creation completes and spawn occurs at 0:39; frames 0:44–0:54 show the player standing still in dialogue with an NPC. Movement across the bridge occurs between 0:59 and 1:14.
- **Coordinate precision**: Video footage alone cannot mathematically confirm a specific cell like (53, 111) without `/where` output or packet logs, though it visually confirms the western platform next to the bridge.

## Sources read
- https://www.youtube.com/watch?v=GJO1B9ogKXY — Primary video source: metadata, description ("eos Ragnarok online"), and visual seek frames 0:00–2:30 inspected.
- ~/data/ragnarok-cd/refs/rust-ro/config/maps/pre-re/new_1-1.mcache — Map geometry and cell walkability verifying spawn platform at X=45–68, bridge at X=69–108, Y=110–113, and castle gate at X=117–148.
- ~/data/ragnarok-cd/refs/_extract/jathena/char-0587.c — Primary emulator source demonstrating start_point default hardcoding in 2003.
- https://www.ign.com/articles/2003/05/20/ragnarok-online-walkthroughfaq-403797 — Weggy's Ragnarok Online FAQ v5.4 (May 20, 2003) confirming classic spawn on the island and crossing bridge to the castle.
- https://forums.openkore.com/viewtopic.php?t=142852 — OpenKore forum macro log from official server (pRO) navigating 53 111 -> 53 114 -> 148 112.
- https://maxion-1.gitbook.io/ragnarok-landverse-whitepaper/game-guide/novice-in-game-guide — Guide describing tutorial sequence: Shion at start platform directing across bridge to castle receptionist.
- https://wiki.uaro.net/remastered-novice-location/ — Classic/remastered novice training grounds overview.
- https://99porings.com/revo-classic/guides/novice/novice — Novice progression guide detailing spawn point and bridge crossing.
- https://github.com/rathena/rathena/blob/master/npc/re/jobs/novice/novice.txt — Aegis-converted novice script showing NPC placements and dialogue.
- /opt/src/ragnarok-offline/research/agy-threads/HANDOFF.md & docs/PROVENANCE.md — Internal documentation of start point provenance and Athena lineage.

## Could not read
- https://ragnarok.fandom.com/wiki/Training_Grounds — Cloudflare 403 bot check.
- https://gamefaqs.gamespot.com/pc/561051-ragnarok-online/faqs/20092 — Cloudflare 403 bot check.
- https://project-r.app/guide/jobs/novice — JavaScript required.
- https://www.reddit.com/r/RagnarokOnline/comments/1ovzqae/how_do_you_access_the_old_training_ground_map/ — JavaScript required.
- https://ratemyserver.net/nsw_npc_search.php?nid=8164&na=1&re=1 — Thin text response / protected query.
