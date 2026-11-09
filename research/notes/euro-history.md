# euRO (euro-ro.net): history of the server behind the 2003-12 CD

Researched 2026-11-09. Tags:
**[P]** primary. I read the archived page text myself: the operator's own pages, patch files, forum posts by its staff, SEC filings.
**[C]** contemporary third party. I read it: 2003-04 press and fan forum posts, plus OpenKore config from 2004.
**[H]** hearsay. Later wikis, search-engine summaries, or my own inference, marked as such.

Wayback links use the capture timestamp I fetched. A page I could not load is not cited.
Caveat: most `www.euro-ro.net` captures from April to September 2004 are cut off after the page header. Rules, FAQ, maps, status
and download pages have no body text. News articles and the Feb-2004 mirror on `wpatch.euro-ro.net` are complete.

## Summary

| Claim | Conf. | Source |
|---|---|---|
| euRO was the **official, licensed** RO for DE/AT/CH/IT/TR. Licensee: **Burda Holding International GmbH** (Hubert Burda Media), via its dept. "BHI Online Entertainment" (BHI-OE), Munich. | P | [S2][W1][W2][W8a] |
| Gravity–BHI "Exclusive Ragnarok License and Distribution Agreement" dated **2003-11-26**. Amended 2003-12-02 and 2004-11-18. Renewed 2006-04-15 → 2007-04-14. | P (SEC) / C (onecle copy) | [S1][S2][S3][S4] |
| **Closed beta started 2003-12-20** on Comodo + Q-Pet, with "higher experience" and a planned wipe. CD installer built 2003-12-19. | P | [W1][W2][W9] |
| Full-client download `ragSetup_eu.zip`, 608.38 MB, dated 2003-12-20 | C | [W30] |
| "New Comodo" update in Jan 2004 (2nd-job quests, monster skills, lvl-99 aura…). | P | [W5][W9] |
| Closed beta ended with a full wipe, Jan 31 – Feb 2, 2004. **Open beta 2004-02-04** (slipped from Feb 2). | P | [W6][W7][W10] |
| **Commercial (P2P) start 2004-04-15.** Open-beta characters were kept, with items and zeny rolled back to 2004-04-08. | P | [W12][W13] |
| Registrations: >50,000 by 2004-04-15, "100,000+" by 2004-06-30, **150,699 on 2004-08-14** | P | [W13][W21][W25] |
| Server software: Gravity supplied it ("excluding OS and Database Server Software"). The licensee could not modify the game. It ran on Windows. **Aegis** is inferred; no version found. | P + H | [S3][W26] |
| Juno (Episode 5) + 2-2 jobs went live on euRO **2004-06-09**, as a separate client add-on | P | [W17][W18][W19] |
| Only one world (**Chaos**) until **Loki, announced for 2004-09-16**. 5 character slots announced for 2004-08-18 (presumably 3 before). | P | [W23][W27] |
| WoE existed from the open beta: Sunday 18:00 CET in April 2004 | P | [W14] |
| Exp/drop rates in commercial service: **no primary statement found**. Presumably Gravity default 1x. | H | [S3] §3.4, [H3] |
| CD distribution: the client was free to copy; magazine inserts were announced; registration had a "CD-Code" field. **No proof of a specific cover disc.** | P (intent only) | [W8][W11][W15] |
| `ruro-ro.net`: no 2003-04 captures, no web hits. Russia was only "open beta" for Gravity in 2006. | P (negative) | [W31][S2] |
| End: license talks failed. burda:ic withdrew in July 2010 and the service closed on **2010-09-30**. No sign of legal action against euRO. | C / H | [C5][H1] |
| CD login IP **62.109.138.134:6900** was euRO's login server (OpenKore 2004-11-13: `master_version 20`, `version 1`) | C | [K1] |

## 1. What euRO was

* **Operator.** The Dec-2003 FAQ on bhi-oe.de says it all came from BHI-OE, "eine Gruppe bei der Burda Holding International GmbH, einer
  Tochter der Hubert Burda Media in München". The contact page (Feb 2004) lists "European Ragnarok Online — A project by Burda Holding
  International GmbH, 'Online Entertainment'-Department (BHI-OE)", Arabellastr. 23, Munich. Staff:
  Imdat Solak "PerrI" (General Manager), Achim Kaspers "Prota" (Marketing), Andreas Blobelt "Nathan" (Product), Jens Kappelt "PerrY"
  (Master GM) [W1][W8a]. Press work in DACH went to Between Us! and hosting to Media Professionals AG (press releases of 17 and 26 Apr 2004) [C3][C4].
  The later corporate name "burda:ic" / "BURDA Interactive Communities" appears only in later sources [C5][H1].
* **Licensed, not private.** PerrI wrote on 2003-12-21: "We have a contract with Gravity Corp., Korea" [W2]. Gravity's 2006 20-F lists
  "Europe — Burda Holdings International GmbH — license Nov 2003 — commercial launch April 2004 — expiry April 2006". It covers Germany,
  Austria, Switzerland, Italy and Turkey under one license [S2]. The original agreement (onecle mirror of the SEC exhibit) defines the territory as those five countries.
  It requires beta within 60 days of signing and commercial launch within 90 days of beta. Royalty was 33% of gross plus fixed initial fees [S3].
  The 2006 amendment cut the royalty to 29% and let BHI sue unlicensed servers in the territory [S1].
* **Timeline [P].** Negotiations ran from about June 2003 and were agreed at the end of November [W1 Q19]. The 2003-12-04 fan announcement
  (via ragnarok.gamona.de) gave closed beta Dec 20, open beta Jan 12, commercial March, and servers on a Level3 backbone [C1].
  - Closed beta 2003-12-20 (planned date held): by e-mail application [W1][W9]. The forum peaked at 109 users online on Dec 20 and had 582 registered users by Dec 26 [W3].
  - 2004-01-27: maintenance notice. Three zone servers were added on Jan 28, then a full shutdown and wipe of all characters, items and *accounts* from Jan 31 to Feb 2 [W6].
  - Open beta was postponed "due to some internal problems" to **2004-02-04 10:00 CET** [W7][W10]. The site headline was "START OF OPEN BETA euRO, February 4th, 2004" [W8b]. The Dec 2003 plan had been open beta on Jan 12 and commercial launch on March 1 [W1].
  - 2004-04-08: no wipe at P2P. Items, zeny and castles were rolled back to 08-Apr 09:00 CET. Servers were down from Apr 13 to Apr 15 [W12].
  - **2004-04-15: commercial start.** Prices were €11.95 per month, €29.95 for 3, €57.95 for 6 and €99.95 for 12 months, plus €5.95 for 30 hours from May 17 [W13][W16][C2].
  - From June/July 2004 a transfer was offered for European iRO players, with characters, zeny caps by level and remaining play time [W21][W22][W24].
* **Players [P].** 50,000+ registered during the open beta [W13]. "100,000+ users", "localised already in 4 languages" by June 30 [W21]. Gameswelt reported
  ">100.000 Mitglieder" on 2004-06-09 [C7]. 150,699 registered on 2004-08-14 [W25]. A fan post from 2004-02-15 claims about 200 closed-beta and 2,000+ daily open-beta players [C6, hearsay].
* **End [C/H].** GamersGlobal (2010-07-15): burda:ic had negotiated with Gravity since November 2008 without agreement and has now
  withdrawn its offer [C5]. iRO Wiki: closure on 2010-09-30, free play from Sept 15, Chaos and Loki later merged as "Odin",
  characters transferable to fRO [H1]. OpenKore later notes "euRO was closed on September 30, 2010" [H2]. It ended over the license, not
  in court. The legal-action angle runs the other way: in 2006 Gravity empowered BHI to act against unlicensed servers [S1]. Already in Dec 2003
  BHI said it was talking to European private servers about moving them "auf eine legale Basis" without lawyers. It was taking over their *accounts* for the
  closed beta, but "Quad-EXP oder ähnliches werden wir nicht übernehmen" [W1 Q15, Q27].
* **"GRO".** No archived page uses "GRO" or "gRO". The clientinfo `servicetype germany` matches OpenKore's euRO `master_version 20` [K1].
  The sibling note `emulators.md` §6 [H] maps CLIENTTYPE 20 to GERMANY. So "GRO" was probably Gravity's internal label for the German licensee (unconfirmed).

## 2. Server software

* The contract has Gravity deliver "the Game and any related software, documentation and other means of enabling the operation of the game".
  The only exclusion is "Operating Systems and Database Server Software", and the licensee is "strictly prohibited from any modification… of any part of the
  Game" [S3 §3.1, §3.4]. So euRO ran Gravity's own server, not eAthena, Freya or jAthena.
* The 2004-09-03 outage post says "the whole game environment runs on Windows (sic!)… the stupid server software not recognizing the IP adresses,
  the MS Windows Systems not recognizing each other" [W26]. That fits Aegis, Gravity's Windows/MS-SQL server [H4].
  **No Aegis version or build number appears anywhere.**
* Updates came from Gravity: "Gravity will put future updates on the test server, then on kRO, and about a month later make them available to all
  other licensees, including euRO" [W1 Q12, translated].

## 3. Server settings, Dec 2003 – mid 2004

* **Content state [P].** The closed beta "includes Comodo and Q-Pet" [W2]. "New Comodo" (posted 2004-01-18 on the forum; the website dates it
  2004-01-08) added heal effect and modified potions, the **2nd-job quests**, **monster skills**, the **lvl-99 aura**, a refining-bug fix and
  "adjusted monsters HP/Exp" [W5][W9]. The patch list has `2004-01-09gdata_g.gpf`, `aRagexe`, `skin_g` and a `2004-01-15gdata_g_b` [W11a].
  Juno / Episode 5 went live 2004-06-09 with 2-2 jobs (Crusader, Alchemist, Monk, Rogue, Sage, Bard, Dancer), 10 maps, 30+ monsters, weapon sprites,
  `/bm`, weapon breaking, "new card-stats" and monster relocation [W18][W19]. Juno Kafra via Aldebaran and the "The_Paper" monster came 2004-07-14 [W20].
  Announced roadmap (2004-08-05): 5 slots Aug 18; Amatsu/Gonryun and hairdresser in September (Amatsu/Gonryun *scheduled* for 2004-09-30);
  wedding and Super Novice in October; Niflheim/Umbala in November [W23][W28]. Nothing mentions rebirth/transcendent classes in 2004.
* **Levels.** The lvl-99 aura implies base cap 99 [W5]. The job cap is not stated.
* **Rates.** Closed beta: "As we are testing with higher experience, we will delete all characters… at the end" [W2]. Its forum had threads
  "what's the exactly exp/Droprate atm?" and "[Poll] Exp and Droprate: Higher or high enough" [W4]. **The multiplier is not given in any page
  I could load.** For open beta and P2P no rate is ever announced. The "no modification" clause [S3 §3.4] and later RateMyServer listings of official euRO
  (1x/1x/1x, 2019) [H3] suggest stock 1x (inference).
* **Worlds and slots [P].** One world (Chaos) until Loki, which was announced on 2004-09-14 for 2004-09-16. Players could then keep Chaos characters and have
  "10 Charakterslots" across both [W27]. "With this update every player can make 5 characters on one account" (announced for 2004-08-18) [W23].
  So it was presumably 3 before that, matching the "3 chars per account" transfer rule [W24] (inference).
* **WoE [P].** Already during the open beta: "War of Emperium (Sonntag, 11 April 18Uhr)", with a GM attack on the castles [W14]. September 2004: WoE
  suspended over spell problems, a test WoE on a Wednesday, treasure boxes and guild dungeons disabled. Reinstalling WoE on Chaos, plus 3 more zone servers,
  was scheduled for the 2004-09-22 maintenance [W28][W29].
* **GMs [P].** The GM list of 2004-05-17 names Master and Deputy GMs with "Item- and Monster-Spawn-Rights" and plain GMs without
  them. Support GMs have no in-game rights. "Our GMs are not there to help you to level. They are not even permitted to do so" [W17a]. Weekly maintenance
  was "every Wednesday 07:00 .. 10:00 CET" [W20a]. GM command lists were not found.
* **Accounts [P].** Closed beta: manual selection via e-mail to closed-beta@bhi-oe.de [W9]. Rules: no bots, no multi-accounts, no foul
  language, no flaming [W2]. Open beta (Feb 2004): self-registration with a field "If you have CD-ROM and a code on it… CD-Code", a 7-day trial and
  7 bonus days on first payment [W11]. P2P: 15-day trial [W13]. The form asks for first/last name, e-mail, gender, birthday, country and user ID, and the
  password is e-mailed [W15b]. Under 18 needs parental consent (T&C of 2004-02-09) [W15].
* **Languages.** The client was English first, with German, Italian and Turkish versions and servers planned [W1 Q5]. Announcements were in EN/DE/IT/TR (FR later) [W6][W20].

## 4. The CD

* The CD's dates fit the **closed-beta client**: InstallShield 2003-12-19, Ragexe 2003-12-18, launch 2003-12-20. A 608.38 MB
  `ragSetup_eu.zip` "EuRO Full Client Installer" dated 2003-12-20 was mirrored on daddeln.de [W30]. Its comments ask about a password for `data.cab`.
* Physical media were intended but not documented. The 2004-02-02 client page says "You will also be told when we are releasing the client direct to the public
  as an insert in various magazines" [W8]. Registration had a CD-Code field [W11]. The T&C allow free copying of the client and charging only for the
  carrier ("e.g. CD") [W15]. The newbie guide says "you don't have to purchase any CD to play the game, it is free" [W15a].
  The euRO PC (E-Hands, 2004-06-30) shipped with RO pre-installed "zusätzlich auf CD" [W16a]. **No archived source names a magazine issue or a pressed euRO disc.**
* **ruro-ro.net / ruRO.** Wayback holds a single `robots.txt` from 2007 and nothing else [W31]. Web search finds no trace. Gravity's 2006 20-F lists
  Russia only as "open beta testing" in 2006 [S2], so a 2003-04 ruRO is unlikely. *Inference:* "ruro" is one key off "euro" on QWERTY, so the ARP help link is
  probably a typo for `www.euro-ro.net`.

## 5. Patch history

`wpatch.euro-ro.net/patch/patch2.txt` from 2004-02-21 is the oldest list [W11a]. It has entries 303-306 and 400-414, with 415 commented out.
The June copy extends it to 462 [W11b]. The July `patch50` list adds the Juno block [W11c]. Line labels in parentheses are my reading of the dates.
```
303 2003-12-19gdata_g_b.gpf        (dated the day before closed beta)
304 2004-01-09gdata_g.gpf   305 2004-01-09aRagexe.rgz   306 2004-01-09skin_g.rgz   ("New Comodo")
400 2003-04-30gdata_turtle_ph_1.gpf   401 2003-04-30gdata_turtle_ph_2.gpf   (same names as on the CD)
402 2004-01-15gdata_g_b.gpf   409/410 2004-02-04 dRagexe + gdata_g_b (open beta)   412 2004-02-14   413-415 2004-02-20
417/418 2004-02-25   423 2004-03-02   424 2004-03-04   425/426 2004-03-10
440 2004-04-14gdata_lang_g   442 2004-04-14gdata_egg_g   444 2004-04-15aRagexe   445 2004-04-15gdata_scr_g  (P2P launch)
446-448 2004-04-21..28   451 2004-05-12   453 2004-05-15   459 2004-05-26   462 2004-05-31aRagexe
503-509 2004-05-04gdata_yuno_new1..5_g / yuno_change1..2_g   513 2004-05-08gdata_yuno_g   514-525 2004-05-18 .. 2004-06-24  [W11c]
```
* The CD's `2003-12-11gdata_yuno_new_1/2` and `yuno_update` appear in **no** archived euRO list. euRO shipped Juno as its own `_g` files plus a
  150 MB add-on, activated server-side on 2004-06-09 [W18][W19]. *Inference:* the CD's Juno data came from Gravity's international build, and the server
  did not enable Juno until June 2004.
* The CD patcher's `2003-10-31gdata_xmas.gpf` / `notice_xmas.txt`: not in any archived euRO list or notice. Origin unknown; it may be inherited from Gravity's
  template (unverified).
* Patcher notices [P]: July 2004 says "During the maintenance on July, 14th we add the Kafra service to Juno via Aldebaran… 'The_Paper'" [W20]. Closed notice:
  weekly maintenance [W20a]. A 2005-03-16 capture describes a "February 24th" novice-ground/headgear update (year not stated; probably 2005) and an
  Umbala/Niflheim add-on requirement [W32].
* News timeline Dec 2003–Sep 2004 [P]: closed beta (12-20), New Comodo (01-08/18), open beta (02-04), update plan (04-01: Juno June, Amatsu/Gonryun Sept,
  Niflheim Nov) [W17], wipe decision (04-08), P2P (04-15), loading-screen contest (May 12 in game), Juno add-on (05-28/06-01), Juno live (06-09), squad
  tournament (Jul 3-11), iRO transfer (06-30 → tool 08-20), 5 slots (08-18), King of the Hill (08-28/29), data-centre outage (08-27/28), Loki (09-16),
  WoE back plus 3 more zone servers (09-22), Amatsu/Gonryun (09-30).

## 6. Not found / open

Exact closed-beta rates. Any rate statement for 2004. An Aegis version. GM command list. Server-rules text (captures truncated).
A magazine or cover-disc record. The meaning of "GRO". The origin of the xmas patch names. Whether Turtle Island was enabled server-side in Dec 2003.
Next steps: the closed-beta topics `viewtopic.php?t=67` / `t=80` / `t=16` on forum.euro.bhi-oe.de have **no Wayback captures**. Topics
on `www.euro-ro.net/community/forum` (thousands of 2004 captures) could be searched page by page. German fan sites `rns.gamona.de` and
`ragnarokonline.de` had 2004 coverage.

## 7. Implications for emulation

* **Era profiles.** Offer (a) **"euRO closed beta"** (2003-12-20 → 2004-01-31): Comodo + Q-Pet, 1st/2nd (2-1) jobs, rates boosted
  by an *unknown* factor. Expose this as a knob, default 1x, and say in the UI that the value is unknown.
  (b) **"euRO open beta / launch"** (2004-02-04 → 2004-06-08): New Comodo feature set, base 99, 2-1 jobs only, WoE, 3 slots, 1x [H].
  (c) Optional **"Juno"** (from 2004-06-09): 2-2 jobs and Juno maps. The CD already carries that data.
* **Content cutoff for this disc:** the **New Comodo** state (Jan 2004) is the most faithful default. Keep Juno and 2-2 jobs behind a toggle, off by default.
  No rebirth, Super Novice, marriage, Amatsu/Gonryun, Niflheim or Umbala in any 2004-H1 profile.
* **No custom features.** The licensee was contractually barred from modifying the game [S3 §3.4], so use stock Gravity behaviour. GM accounts should be
  tiered (item/monster-spawn rights vs. plain).
* **Protocol identity.** The client expects login at `62.109.138.134:6900` with `version 1` and clienttype/master_version 20 [K1]. Rewrite the
  clientinfo to localhost but keep those values. Login is plain `0x0064` per OpenKore (see `emulators.md` §7).
* **Flavour.** Optional loading screens and notices in EN/DE/IT/TR. Wednesday maintenance and WoE Sunday 18:00 CET could be scheduled events.

## Sources

Wayback (`https://web.archive.org/web/<ts>/<url>`):
[W1] 20031220035317 http://www.bhi-oe.de/ (euRO FAQ, 2003-12-12) · [W2] 20031223001708 http://euro.bhi-oe.de/ ·
[W3] 20031226111840 http://forum.euro.bhi-oe.de/ · [W4] 20031227055051 http://forum.euro.bhi-oe.de/viewforum.php?f=3&amp ·
[W5] 20040723203021 http://forum.euro.bhi-oe.de/viewtopic.php?t=1791 · [W6] 20040405184353 http://forum.euro.bhi-oe.de/viewtopic.php?t=2400 ·
[W7] 20040405185100 http://forum.euro.bhi-oe.de/viewtopic.php?t=2539 · [W10] 20040203230318 http://forum.euro.bhi-oe.de/viewforum.php?f=1&amp ·
[W8] 20040221041909 http://wpatch.euro-ro.net/client/ · [W8a] 20040221050209 http://wpatch.euro-ro.net/corp/contact/ ·
[W8b] 20040221010012 http://wpatch.euro-ro.net/ ·
[W9] 20040207012248 http://www.euro-ro.net/server/ · [W11] 20040221051644 http://wpatch.euro-ro.net/register/ ·
[W11a] 20040221010205 http://wpatch.euro-ro.net/patch/patch2.txt · [W11b] 20040606225542 (same URL) ·
[W11c] 20040717180302 http://wpatch.euro-ro.net/patch50/patch2.txt · [W12] 20040423025155 http://www.euro-ro.net/news/eu/00054/ ·
[W13] 20040423025128 http://www.euro-ro.net/news/eu/00109/ · [W14] 20040414010235 http://www.euro-ro.net/events/planned/de/00074/ ·
[W15] 20040221050748 http://wpatch.euro-ro.net/corp/terms/ · [W15a] 20040221051523 http://wpatch.euro-ro.net/game/newbie/ ·
[W15b] 20040502182350 http://www.euro-ro.net/apps/usermmt.app/register_user.php · [W16] 20040528233955 http://www.euro-ro.net/news/eu/00218/ ·
[W16a] 20040710072425 http://www.euro-ro.net/news/de/00334/ · [W17] 20040406030155 http://www.euro-ro.net/news/eu/00010/ ·
[W17a] 20040810041428 http://www.euro-ro.net/server/gm_info/eu/00228/ · [W18] 20040607224848 http://www.euro-ro.net/news/eu/00240/ ·
[W19] 20040619132213 http://www.euro-ro.net/server/eu/00267/ (also news/eu/00270 @20040619131855) ·
[W20] 20040718124248 http://wpatch.euro-ro.net/patch/notice_patch.txt · [W20a] 20040718054701 http://wpatch.euro-ro.net/patch/notice_close.txt ·
[W21] 20040811083458 http://www.euro-ro.net/news/eu/00336/ · [W22] 20040811022314 http://www.euro-ro.net/news/eu/00361/ ·
[W23] 20040815010305 http://www.euro-ro.net/news/eu/00414/ (+ plan 20040810040604 /news/eu/00424/) ·
[W24] 20040901140408 http://www.euro-ro.net/support/faq/eu/00447/index.php · [W25] 20040822230213 http://www.euro-ro.net/news/de/00434/ ·
[W26] 20040910081257 http://www.euro-ro.net/news/eu/00500/ · [W27] 20040915072318 http://www.euro-ro.net/news/de/00517/ ·
[W28] 20040923110420 http://www.euro-ro.net/news/de/00520/ · [W29] 20040910081217 http://www.euro-ro.net/news/eu/00499/ ·
[W30] 20040330161222 http://www.daddeln.de/index.php?section=filedb&action=details&file_id=44623 ·
[W31] CDX https://web.archive.org/cdx/search/cdx?url=ruro-ro.net&matchType=domain (one row: 20071021151603 robots.txt) ·
[W32] 20050316063457 http://wpatch.euro-ro.net/patch/notice_patch.txt, 20050316063832 http://wpatch.euro-ro.net/patch50/notice_close.txt

SEC / contracts: [S1] https://www.sec.gov/Archives/edgar/data/1313310/000114554907001173/h01298exv4w44.htm ·
[S2] https://www.sec.gov/Archives/edgar/data/1313310/000114554906000930/h00617e20vf.htm (licensee table and note 12) ·
[S3] https://contracts.onecle.com/gravity/burda.lic.2003.11.26.shtml · [S4] https://contracts.onecle.com/gravity/burda.lic.2003.12.02.shtml

Contemporary third party: [K1] OpenKore `89134c4e` (2004-11-13) `trunk/confpack/control/servers.txt` L68-72,
https://github.com/OpenKore/openkore/blob/89134c4e04646445a129ae9f29a6c4131744230c/trunk/confpack/control/servers.txt ·
[C1] consolewars.de thread, 2003-12-04, https://forums.consolewars.de/threads/euro.12662/ ·
[C2] GameStar 2004-03-27, https://web.archive.org/web/20210516100125/https://www.gamestar.de/artikel/ragnarok-online-kosten-fuer-das-online-rollenspiel,1345299.html ·
[C3] Spieleflut 2004-04-17, http://www.spieleflut.de/news/11273/index.html · [C4] Spieleflut 2004-04-26, http://www.spieleflut.de/news/11387/index.html ·
[C5] GamersGlobal 2010-07-15, https://www.gamersglobal.de/news/24918 · [C6] supernature-forum 2004-02-15,
https://www.supernature-forum.de/threads/tipp-mmorpg-ragnarok-online-europa-euro.33620/ · [C7] Gameswelt 2004-06-09,
https://www.gameswelt.ch/ragnarok-online/news/erfolg-in-europa-76837
(C1, C3-C7 were read through a fetch-and-summarise tool; quotes are the excerpts it returned.)

Hearsay / later: [H1] https://irowiki.org/wiki/Servers · [H2] OpenKore `tables/servers.txt` (master) L310-313, `dead_message` ·
[H3] https://ratemyserver.net/index.php?page=detailedlistserver&serid=20589 (search snippet only) ·
[H4] https://github.com/rathena/rathena/wiki/AEGIS (search snippet only; the page timed out)
