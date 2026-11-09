# Reference emulators & protocol docs for the 2003-12-18 Ragexe

Researched 2026-11-09. **[V]** = I read the source or file myself (path/commit given). **[H]** = hearsay:
web summaries, secondary pages, or inferred, not checked against source. Local copies live under
`~/data/ragnarok-cd/refs/` (`/data/refs/` inside `tools/wb`). The comparison tool is
`refs/_tools/pktcmp.py`. It diffs packet length tables (C `packet_len_table`, eA `packet_db.txt`,
Hercules `packetLen()` headers evaluated at a date, OpenKore `recvpackets.txt`).

## 1. Packet-version candidates

| Scheme / value | Date / client | Evidence |
|---|---|---|
| jAthena compile-time `PACKETVER=3` | kRO after the Juno update (Athena changelog entry `//0512`) | [V] 0x01de skill damage and 0x01c9 skill unit; 0xa8 becomes 0x01c8. rathena `706e7850` `Readme-jap` L11035-11042 (CP932). The mirror's 2003-10-30 `//0505` Makefile already has `=3`. Athena 0601 (archive 2003-12-13) defaults to 3. |
| `PACKETVER=4` | Athena `//0627`/`//0629`; `athena-dev-2.1.1-mod0629.rar` is dated **2003-12-17** | [V] 0x01d8/0x01d9/0x01da spawn/stand/walk (54/53/60), 0x01d7 changelook. `Readme-jap` L9018-9061. mod0629/0631/0652, AppleMod 641 and eA 715/817 Makefiles all default to 4. jAthena git defaults to 4 from `//0741` `fc6498f2` (2004-01-25). |
| `PACKETVER=5` | kRO 0x01ee from **2003-12-08**; 0x01ef/0x01f0 from **2003-12-18** | [V] item lists 0x01ee (inventory), 0x01ef (cart), 0x01f0 (storage) replace 0xa3/0x123/0xa5. jAthena `//0867` `d690c6b9` (2004-03-05) sets `=5`; eAthena 0.5.2 (2004-05-20) is the first archived eA with `=5`. |
| eAthena runtime `packet_ver: 5` | "old": every client before 2004-07-05 | [V] rathena `706e7850` `src/map/map.h:157`: `// 5: old, 6: 7july04, 7: 13july04, 8: 26july04, 9: 9aug04/16aug04/17aug04, 10: 6sept04 (by [Yor])`. The `packet_ver: 5` block was added to `db/packet_db.txt` in `6e3d4f80` (2005-01-13). Last copy before deletion: `0d103bf6` L47. |
| `packet_ver: 6`…`11` | 6=2004-07-05aSakexe, 7=07-13, 8=07-26, 9=08-09 (also 08-16/08-17), 10=09-06, 11=09-20 | [V] rathena `0d103bf6:db/packet_db.txt` L477-567. Packet shuffling starts at 6 and **does not apply to us**. |
| Hercules/rAthena date `PACKETVER` | base list applies when `< 20040705`; Hercules 2003 length tables have dates 20031028…20031223 | [V] Hercules `04a422ce` `src/map/packets.h` (base list, then `//2004-07-05aSakexe` at L166); `src/common/packets/packets2003_len_main.h`. |
| roBrowser | version key `0` = base (< 20040705) | [V] `e4b5b53a` `src/Network/PacketVersions.js` L18-43. |

**Conclusion.** The client sits in eA `packet_ver 5` and jAthena PACKETVER ≥ 4. It is PACKETVER 5 if its table has 0x01ee-0x01f0. In Hercules terms it is
probably `PACKETVER 20031218`. The fingerprint in §2 will settle it from the exe table. OpenKore's iRO table "16 December 2003"
(§4) already lists 0x01ee/0x01ef/0x01f0, so a 2003-12 international client is the kRO 2003-12-18 class.

## 2. Date fingerprint (kRO Ragexe tables; Hercules `packets2003_len_main.h`/`packets2004_len_main.h`) [V]

| id | 10-28 | 11-03 | 11-10 | 11-17 | 11-24 | 12-08 | **12-18** | 12-23 | 2004-01-07 |
|---|---|---|---|---|---|---|---|---|---|
| 0x00f9 / 0x0104 | 26/79 | 26/79 | **28/81** | 26/79 | 26/79 | 26/79 | 26/79 | 26/79 | 26/79 |
| 0x01e6 | 28 | 28 | 26 | 26 | 26 | 26 | 26 | 26 | 26 |
| 0x01e7 | 81 | 81 | 2 | 2 | 2 | 2 | 2 | 2 | 2 |
| 0x01e8 / 0x01e9 | 26/– | 26/2 | –/– | 28/81 | 28/81 | 28/81 | 28/81 | 28/81 | 28/81 |
| 0x01ea | – | – | – | 6 | 6 | 6 | 6 | 6 | 6 |
| 0x01eb-0x01ed | – | – | – | – | 10/26/2 | ✓ | ✓ | ✓ | ✓ |
| 0x01ee | – | – | – | – | – | -1 | -1 | -1 | -1 |
| 0x01ef / 0x01f0 | – | – | – | – | – | – | **-1/-1** | -1/-1 | -1/-1 |
| 0x01f1 / 0x01f2 | – | – | – | – | – | – | – | -1/– | -1/20 |

The Sakexe table (`packets2003_len_sak.h`) has the same values; its first entry is 20031029.
The next new packets are 0x01f3 (2004-02-16), 0x01f4/0x01f5 (03-02), 0x01f6-0x01f9 (03-08) and 0x01fa (03-22).
0x01fa adds the account-select login. The 2004-01-07 table is still the unshuffled base layout:
0x0072=19, 0x0085=5, 0x0089=7, 0x007e=6, 0x008c=-1, 0x0094=6, 0x009b=5.
All of these hold until 20040705.

## 3. How the 2004 shuffled versions differ (context only, our client predates them) [V rA `0d103bf6:db/packet_db.txt`]

| id | v5 (base) | v6 07-05 | v7 07-13 | v8 07-26 | v9 08-09 | v10 09-06 |
|---|---|---|---|---|---|---|
| 0x0072 | 19 wanttoconnection 2:6:10:14:18 | 22 (5:9:13:17:21) | 39 (12:22:30:34:38) | 14 dropitem | 17 dropitem | 20 useitem |
| 0x007e | 6 ticksend 2 | = | = | 33 wanttoconnection | 37 wanttoconnection | 19 movetokafra |
| 0x0085 | 5 walktoxy 2 | 8 (5) | 9 (6) | 20 useskilltoid | 26 useskilltoid | 23 actionrequest |
| 0x0089 | 7 actionrequest 2:6 | = | = | 15 getcharnamerequest | 12 | 9 walktoxy |
| 0x008c | -1 globalmessage | = | = | 23 useskilltopos | 40 | 105 useskilltoposinfo |
| 0x009b | 5 changedir | = | 13 (5:12) | 6 walktoxy | 15 walktoxy | 14 getcharnamerequest |
| 0x0113 / 0x0116 | 10 / 10 | 15 / 15 | 19 / 19 | 16 movetokafra / 2 closekafra | 23 movetokafra / = | 11 takeitem / 11 ticksend |

Since v6, the server identifies the client version by the length of its `wanttoconnection` packet. This is the walk/connect
obfuscation people mention; a 2003 client never uses it.

## 4. Cross-reference divergences to check in our exe table [V, `pktcmp.py` output]

Compared with kRO 2003-12-18:

* **OpenKore iRO 2003-12-16** matches every id it lists. It omits 0x0198-0x01a4, 0x01cd-0x01d2, 0x01e4, 0x01e6 and 0x01ea, and marks
  0x0066/0x0070 as 0 (variable). Commit `81c06af5…` `trunk/tablepack/tables/recvpackets.txt`, which says
  "Packet lengths for RO 4.0 / Monster skills without Turtle Island / iRO: 16 December 2003".
  Follow-ups: `3530d544` (2004-01-06, adds 0x01a4=11), `2921838c` (2004-01-09 "iRO (guild wars)", 0x019b=10),
  `8ce669fd` (2004-02-07, adds 0x0198-0x01a4, 0x01e4, 0x01e6, 0x01ea, 0x01f1).
* **Athena 0629/0631/0652, AppleMod 641, eA 715/817** (PACKETVER 4) all lack 0x01e0 and 0x01e2-0x01f0. They also differ on these ids:
  0x00cd **6** (kRO 3), 0x019c **-1** (kRO 4), 0x01cd **29** (kRO 30; fixed by jA 0867), and an invented 0x01f4=7
  (kRO's real 0x01f4=32 arrives 2004-03-02).
* **eA 0.5.2 / rA r2 / rA 2017 v5** also differ: 0x01d5 -1 (kRO 8), 0x019c -1 (kRO 4), rA r2 0x008b=2 (kRO 23),
  and rA 2017 0x0066/0x0070=6.
* Caveat [V+H]: 0x01d5 (CZ_INPUT_EDITDLGSTR) and 0x019c (CZ_LOCALBROADCAST) are client→server packets. Every server
  treats them as variable-length, yet the client's own table says 8 and 4 until kRO 2004-12-13.
  The exe table is only authoritative for **S→C** lengths; C→S lengths must be read from the send code.

## 5. Contemporary emulators and where to get them (Q2)

| Project | Era | Source today | Notes |
|---|---|---|---|
| Athena / jAthena (Japanese, from 0052.lzh) | 2003-2006 | [V] https://www.eathena.org/jathena/: `athena-dev-2.1.1-mod0629.rar` (2003-12-17), **`mod0631.rar` (2003-12-18 16:49)**, 0633…0652 (2003-12-27), 0676…1106. `raw/` has per-snapshot diffs 0505…0696, but **0588-0675 are missing**. Git: https://github.com/flaviojs/jathena (hg→git of bitbucket zart/jathena, GPL-2.0), `//0505` `f9678703` (2003-10-30) … `//1093` `13854f20` (2005-01-02). Gaps 0588-0675 and 0697-0726. [H] SourceForge `jathena` files start at 1045 (2004-08-25). | Snapshots are numbered (`//NNNN by …`). `Readme-jap` is the changelog. |
| AppleMod → eAthena (pre-SVN) | 2003-12 → 2004-11 | [V] https://www.eathena.org/eathena/archive/: `601mod.rar` 2003-12-13 (=Athena 0601, PV3), `641mod_rev2.rar` 2003-12-25 (newest Readme entry `//0631`, PV4), `athena0659TestEngmod.rar` 2004-01-01, `eathenamod715testSrc.zip` 2004-01-18, `athena-817-src.zip` 2004-02-19 (PV4), `eathena-v0.5.2.rar` 2004-05-20 (PV5), 1.0 RC1-RC5 (2004-07…10) | [H] AppleMod 641 was the "Christmas release" (2003-12-24); eA 715 2004-01-18; 0.4.1 2004-05-08; RC1 2004-07-23; omniAthena merge 2004-10-14; RC5 2004-10-15; public SVN 2004-12-07 (EverybodyWiki). |
| eAthena SVN → rAthena / Hercules | 2004-11 → today | [V] Git history starts at **svn r2, 2004-11-04 23:25 UTC** everywhere: rathena `706e7850…` (`svn.code.sf.net/p/rathena/svn/athena@2`), https://github.com/eathena/eathena `845fd099…` (`eathena.googlecode.com/svn/athena@2`), Hercules `195dffc2…`. Nothing earlier is in git. | r2 `clif.c` carries `$Id: clif.c 164 2004-10-01`, so an earlier SVN existed that was never imported. |
| Freya | 2004 (from eA RC4) → 2006+ | [V] https://github.com/flaviojs/freya (GPL-2.0, 820 commits), first commit `7eae034f` = `ookoo.org/svn/freya/trunk@1` **2006-08-22** | Its 2004-era history is gone. Not useful for a 2003 client. |
| OmniAthena | 2004 | [H] https://sourceforge.net/projects/omniathena/ (GPLv2), merged into eAthena 2004-10-14 | Not checked. |
| Aegis (Gravity) | — | [H] Only binaries leaked, never source. The first leak was reported 2003-01-29 on the EOR BBS, followed by the Athena team's announcement 2003-02-05 (ropp.fandom timeline, via search snippet; page blocked). Later packages circulated on ASB. | It needs MSSQL and adds nothing to a packet-table cross-check. |
| rAthena / Hercules HEAD | — | [V] rathena `d4b8e7b8` (2026-10-06), Hercules `04a422ce` (v2026.09) | Both still define the base table, but it is untested for 2003 and the structs assume newer clients. Useful as a dictionary, not as a reference server. |

### Recommended reference revisions (Q2 answer)

1. **Packet lengths, the authoritative cross-check:** Hercules `04a422ce60402c731d9cff0724cca9a5bc3c53e9`,
   `src/common/packets/packets2003_len_main.h` (blob `f88d6699…`), evaluated at `PACKETVER 20031218`. Add `packets2004_len_main.h`
   to locate the 2004 boundary and `packets2003_len_sak.h` for Sakexe. These headers are autogenerated from real kRO executables
   (header comment; Andrei Karas, added in `77841351` on 2018-11-05). https://github.com/HerculesWS/Hercules/tree/stable/src/common/packets
2. **International-client check:** OpenKore `81c06af516949dcbb1bd9ee12b4d34fe2376e6a5`, `trunk/tablepack/tables/recvpackets.txt`
   (iRO 2003-12-16). https://github.com/OpenKore/openkore
3. **Contemporary server logic, the closest in time:** `athena-dev-2.1.1-mod0631.rar` (archive dated 2003-12-18;
   sha256 `37fd1f1036bb9318e7c8de10bd929d883137c5309ee449ea579b9f2804a2a163`), PACKETVER 4, `-DNEW_006b`, with 0x01dd/0x01db
   in `login/login.c`. https://www.eathena.org/jathena/athena-dev-2.1.1-mod0631.rar. The closest in git is jathena `//0676`
   `5415aa02ff47c5d3abb5664d378c34516e369ea4` (2004-01-04), whose packet table matches mod0631. For the PV5 item-list code, use jathena `//0867`
   `d690c6b9e9c74c6e6dc8b02ebb3017078aead30f` (2004-03-05).
4. **Cleanest readable eAthena-era single-table server, with English comments, all in git:** rathena
   `706e785091b4b16b0626289e69fc5ac4ab3bc068` (svn r2). Files: `db/packet_db.txt` (base table ending `//jROはここまで`),
   `src/login/login.c` (0x64/0x1dd/0x1db), `src/char/char.c` (0x65 echo, 0x6b), `src/map/clif.c` (PACKETVER=5 via the
   top-level `Makefile`: `-DPACKETVER=5 -DNEW_006b`), and `Readme-jap` (the PACKETVER history).
5. **packet_ver dictionary:** rathena `0d103bf6a788a09a13512e5ca20fa94fc907360c` (the parent of `f0ab8d2a5` "Overhauled and
   dropped packet db", 2017-07-15), `db/packet_db.txt`.
6. **Aegis field names:** roBrowser `e4b5b53aa1f8b7e429bfa987ab321c96502ba1da` `src/Network/PacketStructure.js`. Also
   rust-ro `tools/packets/packets_db`. Both use Aegis-style names; rust-ro's layouts are post-2004 (§7).

## 6. Era protocol notes (Q3). [V] = eA r2 / jA 0587 source unless noted

Offsets are relative to the start of the packet (id at 0). All values are little-endian.

**Login (port 6900)**
* C→S `0x0064` (55): `u32 version@2, char id[24]@6, char pass[24]@30, u8 clienttype@54`.
* Encrypted login: C→S `0x01db` (2). S→C `0x01dc` (var: `u16 len@2, key@4`; eA sends 12-15 random non-zero bytes).
  Then C→S `0x01dd` (47): `u32 version@2, id[24]@6, md5[16]@30, u8 clienttype@46`. eA `mmo_auth`: passwdenc 1 = `MD5(key‖pass)`,
  2 = `MD5(pass‖key)`; `PASSWORDENC=3` tries both. OpenKore 1.6.4 `Send.pm` `sendMasterSecureLogin` uses odd type for key‖pass
  and even for pass‖key. The client has the strings `passwordencrypt` and `passwordencrypt2` [V `ragexe.strings` L6411-6412],
  presumably modes 1 and 2 [H, verify in the exe]. 0x01fa (adds a u8 account selector) only exists from 2004-03-22.
* S→C `0x0069` (47+32n): `u16 len@2, u32 login_id1@4, u32 aid@8, u32 login_id2@12, u32 0@16, char lastlogin[24]@20, u8 sex@46`.
  Each server is `u32 ip, u16 port, char name[20], u16 users, u16 maintenance, u16 new`. roBrowser names the fields
  AuthCode/AID/userLevel/lastLoginIP/lastLoginTime[26]/Sex.
* S→C `0x006a` (23): `u8 error@2` (+`char[20]` unban date when error=6). `0x0081` (3) is a disconnect reason.

**Char**
* C→S `0x0065` (17): `u32 aid@2, u32 login_id1@6, u32 login_id2@10, u16 clientType@14 (ignored), u8 sex@16`.
* **Quirk:** the server immediately writes **4 raw bytes = AID**, with no packet id, before anything else
  (`WFIFOL(fd,0)=RFIFOL(fd,2); WFIFOSET(fd,4);`, jA 0587 `char/char.c` L908, eA r2 `src/char/char.c` L2530).
  roBrowser `CharEngine.js` L76-79 reads it.
* S→C `0x006b`: with `NEW_006b` (the default in every 2003 Makefile) the header is **24** bytes: id, len, then 20 bytes that
  roBrowser names `dummy1_beginbilling, code, time1, time2, dummy2_endbilling[7]`. Then 106-byte `CHARACTER_INFO`
  entries: `char_id, base_exp, zeny, job_exp, job_level, 0, 0, option, karma, manner, u16 status_point, hp, max_hp, sp, max_sp,
  speed, class, hair, weapon, base_level, skill_point, head_bottom, shield, head_top, head_mid, hair_color, clothes_color,
  name[24]@74, u8 str..luk@98-103, u8 slot@104` (eA r2 `mmo_char_send006b`). Without NEW_006b the header is 4 bytes.
  Check which one the exe expects.
* `0x0066` (3, slot) → S→C `0x0071` (28): `u32 char_id@2, char map[16]@6, u32 ip@22, u16 port@26`.
  `0x0067` (37) create → `0x006d` (108) / `0x006e` (3). `0x0068` (46: char_id + email[40]) → `0x006f` (2) / `0x0070` (3).
  `0x0187` (6) is the keepalive. `0x006c` (3) refuses.

**Map**
* C→S `0x0072` (19): `u32 aid@2, u32 char_id@6, u32 login_id1@10, u32 client_tick@14, u8 sex@18`.
* **Quirk:** the server writes **4 raw bytes = AID** right away (`clif_parse_WantToConnection`; jA 0587 `map/clif.c`
  L4512, eA r2 L7246). roBrowser `MapEngine.js` L105-111 notes that this lasts until PACKETVER 20070521, when 0x0283 replaced it.
  Next comes S→C `0x0073` (11): `u32 tick@2, pos3@6, u8 5, u8 5`, or `0x0074` (3) to refuse.
* C→S `0x007d` (2) LoadEndAck; `0x007e` (6, tick) → `0x007f` (6, server tick).
* C→S `0x0085` (5) walk; `0x0089` (7) `u32 target@2, u8 action@6`. Positions are packed in 3 bytes:
  `b0=x>>2, b1=(x<<6)|((y>>4)&0x3f), b2=y<<4|dir` (eA r2 `WBUFPOS` L83; `WBUFPOS2` for 5-byte move pairs).
  There is **no** packet shuffling or junk padding: that is kRO 2004-07-05+. iRO's "padded packets" correspond to OpenKore serverType 1/2
  (`Send.pm` 1.6.4 L420-445); their dates are [H] but they are later than this client.
* Packets the client can understand at PV ≥4/5: 0x01d8-0x01da (vs 0x0078-0x007b), 0x01d7 (vs 0x00c3), 0x01de (vs 0x0114),
  0x01c9 (vs 0x011f), 0x01c8 (vs 0x00a8), 0x01ee/0x01ef/0x01f0 (vs 0x00a3/0x0123/0x00a5). The old ids stay valid, so a
  PV3 or PV4 server also works with a newer client.
* `1rag1` [V string only]: in `ragexe.strings` L8608, next to `/account:`, `clientinfo.xml` and `data.grf`, with
  "Please run Ragnarok.exe"/`Patchup.exe` at L6400-6401. [H] It is the token the launcher passes so that Ragexe runs without the patcher.
  The exe also has `npkeysdk` (nProtect KeyCrypt) strings at L8609-8625, the `servertype` values `primary/sakray/local`,
  `hideaccountlist`, `registrationweb` and `langtype`.
* `clienttype` byte (OpenKore calls it `master_version`) [V servers.txt]: euRO **20**, iRO 1, iRO Sakray 14, idRO 12, pRO 15,
  mRO 16/17, inRO 21, bRO 22, oRO 23. [H] These match Aegis's CLIENTTYPE enum (20 = GERMANY). eA ignores this byte; it only
  checks `version` when `check_client_version` is on. The exe has the servicetype strings korea…germany
  [V L6422-6432, stored in reverse order]; [H] these are SERVICETYPE 0-10.

## 7. OpenKore and euRO (Q1 detail) [V]

* `89134c4e04646445a129ae9f29a6c4131744230c` (2004-11-13, first `servers.txt`): `[Europe - euRO] ip 62.109.138.134 port 6900
  master_version 20 version 1`. It has no `secureLogin`, so euRO used plain `0x0064`. pRO and idRO had `secureLogin 1`.
* OpenKore 1.6 (`BRANCH-1-6`, 2006-02): euRO is at 217.68.157.42 with `patchserver wpatch.euro-ro.net`, `serverEncoding Western` and
  `storageEncryptKey …`. Tags 1.9.2/2.0.6 add `serverType 0`. `796c40ee26187aa0dfeedaf1c390d8d548dc780e` (2007-12-09)
  says: "Set serverType to 0 and Paddedpackets off for euRO". `6e8a510a` (2007-12-08) adds `recvpackets-euro.txt`. `66c5006d`
  (2010-01-16) adds `Network::Send::euRO`/`Receive::euRO`, both subclasses of **ServerType0** (plain `0x0085` move).
  `c554eaac` (2010-09-30) removes them: "server closed 30.09.2010".
* **What this implies:** euRO never adopted kRO's shuffled CZ packets. serverType 0 is the base layout
  (`0x0072` 19 bytes, `0x0085` 5, `0x0089` 7: `a12a49af` `tags/RELEASE-1-6-4/openkore/src/Network/Send.pm` L420-422,
  L1059-1060, L1226-1244). For our client, eA `packet_ver 5`/base semantics apply throughout.
* Kore history goes back to CVS (2003-11-07, `7b9c700b`). OpenKore 1.0 was released 2003-11-12 (News.txt). Its skore-revamped changelog ("October 1st", year implied 2003) shows bot authors
  adding 0x01d8/0x01d9/0x01da/0x01de/0x01d7 handling, which matches iRO having PV4 packets by then.

## 8. Rust emulators (Q4)

| Repo | License | State / version target | Architecture worth borrowing? |
|---|---|---|---|
| https://github.com/nmeylan/rust-ro `14193fcd2a285a9690cf28c923d9af56fe9587a5` (2026-10-12), 210★ | MIT [V] | [V] Active hobby server. Targets PRE-RE **PACKETVER 20120307** (README §1-2). Login, char and map run in one process. Postgres via sqlx, tokio, a game-loop thread plus persistence and notification threads driven by message passing (`docs/Architecture.md`, ADRs in `docs/adr/`). Has a client↔rAthena proxy for packet capture (`server/proxy`). | **Yes, partly.** `tools/packets/packets_db` is a C-struct DSL (Aegis names) with `#V GTE/LT/RANGE` field conditions and `id:packetver` id lists, fed to a generator that emits `parse(buf, packetver)` and serializers (`lib/packets/src`). That design fits a version-parametrized emulator. But its layouts are **post-2003**: for example, 0x0078 has `isPKModeON`, `clevel` (55 bytes vs kRO-2003's 54), and 0x006b uses `CHARACTER_INFO_NEO_UNION`. It would need a 2003 profile. |
| https://github.com/nmeylan/rathena-script-lang-interpreter | **no license file** [V repo listing] | [H] ANTLR grammar, compiler and VM for rAthena NPC script, used by rust-ro | Only useful if we want rAthena-script NPCs. Without a license, reuse is all-rights-reserved; ask the author or write our own. |
| https://github.com/vE5li/korangar `639eaabf…` (client, 371★) | MIT [V] | [V] Rust/Vulkan **client**. Its crates `ragnarok-packets`, `ragnarok-bytes`, `ragnarok-macros` use `#[derive(Packet, ByteConvertable)] #[header(0x0064)] #[length(24)]`. Only `SupportedPacketVersion::_20220406` exists (`korangar-networking/src/packet_versions/mod.rs`). | **Yes, for style.** The derive-macro packet definitions are clean and MIT-licensed. Its packet set is 2022, so we only borrow the approach. |
| https://github.com/ViteFalcon/rusthena `16967134` (2015) | MIT | [V] A toy: 3 files (`main.rs`, `db/txtdb.rs`) | No. |
| https://github.com/hanar3/rust-athena `a768c1d5` (2022) | none | [V] Login and char WIP using Diesel, made for modern rAthena-era clients (README points to WARP) | No (no license, modern packets). |
| https://github.com/trapilot/rust-ro-server (2026-10 fork) | MIT | [H] A fork of rust-ro | — |

**None of these targets a pre-2004 client.** Recommendation: write our own small packet crate in korangar's derive style.
Feed it from the length table extracted from the exe, cross-checked against Hercules 2003 + OpenKore iRO-2003-12-16.
Take struct layouts from eA r2 / Athena 0631 sources, using roBrowser/rust-ro only for names.

## 9. Pointers not followed up
* [H] https://github.com/MrAntares/roBrowserLegacy is the maintained roBrowser fork with more packet versions. I did not check it.
* [H] https://www.eathena.org/client/DiffsSakexe/ has old client diffs. I did not open it.
* [H] Aegis Support Board (ASB) for leaked Aegis data packages, organised by episode.
