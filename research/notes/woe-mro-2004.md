# What a siege looked like in numbers: Malaysia's servers, spring 2004

A follow-up to round 3 (research/notes/round3-review.md), of 2026-11-10,
for THREADS #17. Round 3 found a fan site that kept a record of every
castle, siege by siege. This note is what that record says. It is not
euRO, and it is the only dated, siege-by-siege record of an official
server from the first half of 2004 that we have.

## The source

Ro-Guardians, a fan site for the Malaysian servers run by Gameflier
(`sskl.host.sk/ro-guardians/`, forum at `fanbb.com/theguardians/`). Its
forum front page (capture 20040503210327) describes the news board as
"Ro-Guardians and mRO latest news" and has trade boards for "eRO - Iris",
"eRO - Penril", "mRO - Chaos" and "mRO - Loki"; in the one archived topic
(`showtopic=68`, capture 20040523090249) players discuss "malaysia's RO
GMs" and Gameflier. Round 3's rows call these pages "euRO Iris". They are
not.

The site had a "Castle Standing" section per server (`chaos`, `loki`,
`irisi`, `irisii`, `penril`, `odin`, `lidia`) with a "Castle Takeover
History" page per castle: one entry per siege in which the castle changed
hands, listing the guilds that took it, and how long the last of them then
held it. The holder at the end of the siege is first, in bold; the order of
the others is not stated. Coverage starts on 2004-03-18 for Chaos and Iris
and later for the others ("only covered with siege reports from 21st April
2004 onwards", Penril).

Wayback has 22 of these history pages, captured between 2004-06-27 and
2004-07-17: two castles each for Chaos and Iris I with the full history,
one for Penril, and single recent entries for the rest. Parsed, that is 95
entries for 22 castles, in
[woe-mro-2004-takeovers.tsv](woe-mro-2004-takeovers.tsv) (counts and
dates; the guild names are left out). The per-castle pages
(`act=castle`) are empty in all fourteen captures loaded.

The siege commentary board of the forum had a topic per siege from
2004-04-17 to 2004-05-22, with up to 244 replies and a "guild of the week"
poll. Wayback has the topic list (capture 20040523083017) and none of the
topics.

## What it says

* **Twice a week.** 90 of the 95 entries are dated on a Wednesday or a
  Saturday. Four are of Thursday 2004-03-18, the first date in the record,
  and one of a Monday.
* **A contested castle changed hands several times in one siege.** An entry
  lists a median of 3 takeovers; 32 of 95 list five or more, 28 list one.
  On Chaos the median is 4 and the most 9. The most anywhere is 24, by 14
  different guilds, for Prontera II on Iris I on 2004-06-05.
* **Most holders lost the castle at the next siege.** Of the 73 entries
  with a closed "held for", 55 are of 4 days or less: until the next
  Wednesday or Saturday. 17 are of a week or more, 5 of two weeks or more,
  and the longest is 21 days (Geffen II on Chaos, from 2004-04-07).
* **Holding for weeks still meant losing it mid-siege.** Prontera II on
  Iris I has the same guild first on 2004-04-07, 04-21 and 05-05, each time
  "14 days": six weeks through a dozen sieges, in two of which another
  guild's name is listed under it. From 05-19 the same castle has 12, 13,
  14, 9, 5 and 24 takeovers.
* **Castles differed.** In the same weeks Al De Baran V on Iris I has 1 to
  8 takeovers and several sieges without an entry.
* **Sister guilds.** The Iris I table of 2004-06 (round 3's saved page
  `ro_guardians_irisi.html`) has three "Legion of ..." guilds in three
  Geffen castles, three "Council of ..." guilds in three Payon castles, and
  "Fantasy[KinG]", "Fantasy[Venus]", "Fantasy[CluB]" in three more: groups
  larger than one guild's member limit, holding a castle each.

## Limits

* Another region and a later content state than our baseline: the forum of
  May 2004 has boards for Crusader, Sage, Monk, Rogue, Bard and Dancer.
* A fan's record, kept by hand ("Updates are done by Myjavik", "The last
  update took place on 2nd June 2004").
* An entry exists only for a siege with a takeover, so a successful defence
  shows as a gap, not as a row.
* Nothing here says how many people fought, where in the castle, or what
  the treasure was.

## For the rework (a reading, not a finding)

Owning a castle from one siege to the next was the exception. What the
record shows is two hours in which the Emperium of a wanted castle broke
again and again and the last one to break it kept the castle for three or
four days. If the single-player version keeps one thing from this, it is
that a castle is retaken, not kept: the garrison comes back.

## Not followed yet

euRO's own news: for captures between 2004-03 and 2004-10 Wayback's index
lists 76 `www.euro-ro.net/news/` pages (40 `eu`, 27 `de`, 4 `it`, 5 `tr`;
numbers 00010 to 00603) and 19 `events/planned/` pages. Round 3 read four. The first
siege on euRO and the arrival of treasure would be among them.
