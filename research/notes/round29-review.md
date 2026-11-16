# Research round 29: review (2026-11-16)

32 rows (T7 24, T8 2, T9 6), all PASS in `check.txt` and again here
(`check_rows.py --fetch`); pages saved under `research/agy-threads/r29/pages/`.
All three tasks marked "partly".

## How the round did

* **Nothing invented.** All six Wayback captures are in CDX (each is the
  page's first capture, 2010-05). No quote, URL or line made up.
* **One claim past its quote.** T7-061 adds "cannot teleport through walls
  or to different heights", which is in T7-062's quote, not its own.
* **Metadata.** Regions hold this time: admira idRO, Dikiwinky idRO ("idRo
  (Loki)"), aldoteng mRO ("I play in mRO"), Jath "private" ("I do NOT play
  in iRO"), Tangowr iRO (2002, beta). T7-077: CriminallyVulgar never names
  a server (he quotes "the official site (iro.ragnarokonline.com)"), so
  "iRO" is a guess, and his v1.1 line reads "1/12/04" (2005-01-12, after
  v1.0 of 12/22/04), not the row's 2005-01-11.

## What it brought

* **The ensembles (from a saved page, not a row).** Dikiwinky's guide is
  an "Ensemble Support Guide" (idRO, 2004-11-13). Its ensemble section
  agrees with every number built from the iRO guide the same evening:
  Mr. Kim 136% to 180% EXP (jAthena 150% to 250%), Siegfried 40 to 80
  (jAthena 20 to 80), War Drum ATK 50 to 150 / DEF 4 to 12 / 40 to 60 SP,
  Nibelungen 150 to 350 for level-4 weapons, Loki's Veil and Lullaby not on
  bosses, Into the Abyss frees "Gemstones, Traps, etc", Eternal Chaos on
  "all players and monsters (including party/guildsman)". A second
  witness, late (idRO, a year after our client).
* **A song stops on a heavy blow: built.** T7-065 (Jath, private servers)
  and Dikiwinky's stat section ("if we're hitted by enemy and our HP
  decreased 25% in one blow, our song will be cancelled") led to jAthena,
  which has had it since before //0693 (2004-01-09): `pc.c:4494` stops the
  song on a blow over a quarter of max HP, `skill.c:7624` on stone, frozen,
  stunned or asleep. The solo-song port had neither. Both built
  (DEVIATIONS.md, test `ensembles`).
* **Throw Spirit Sphere (T7-054):** admira's 150% to 250% matches the iRO
  guide we built on (both emulators 150% to 350%). One more witness.
* **Blade Stop not on bosses (T7-058):** already built from the guide.
* **Not built, to THREADS #22:** Steel Body making monsters ignore the
  Monk ("enemy won't recognize you", T7-057, idRO 2004-09); Snap passing
  Ice Wall and traps (T8-065, idRO, "Quagmire (unconfirmed)"); Ugly Dance
  10 to 30 SP (Dikiwinky, idRO 2004-11) against the iRO guide's 10 to 50.
* **Cards (T9-407 to -412):** Tangowr's iRO beta guide (2002-10) predates
  our client by a year, and the client's own item text overrides it (Orc
  Lord reflects 25%, not 50%; Zombie Card is HP recovery on shoes, not
  damage on Undead). Nothing to change.
* **Jath's "songs do not work on yourself" (T7-063)** is a private
  server's rule (jAthena's too); the game keeps its deviation
  (`song_on_self`).

New to THREADS: about 5 of 32 rows, the best one through a saved page.
