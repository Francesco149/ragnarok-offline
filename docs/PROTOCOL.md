# Protocol of the 2003-12-18 client

eAthena called this layout `packet_ver 5`: the plain, pre-July-2004 packet
set (no shuffled ids, no padded walk/skill packets). Cross-references and the
packet-version history are in `research/notes/emulators.md`.

## Ground truth: the length table

`research/extract_packet_lengths.py` walks the client's packet-map
initialiser (`sub_418030`, two idioms: `map::insert` → `[eax+0x10]` and
`map::operator[]` → `[eax]`) and writes `data/client/packet_lengths.tsv`:
**386 packets, 0x0064–0x01f0**, -1 = variable (u16 length after the id).

`ro-proto` embeds that file. Every packet the server builds passes
`Writer::finish`, which panics if a fixed-length packet's size differs from
the client's table, so a layout mistake can never silently desync the client.

The table is what the client uses to frame what it **receives**, and also
what it sends: every packet goes out through `SendPacket(len, buf)`
(`0x417d80`, on the connection at `0x6182e0`), and a fixed-size one takes
`len` from the same table (`0x417ec0(id)`). Fourteen of the 44 call sites
compute `len` themselves. Their ids are ones the table marks variable
(chat, whisper, chat rooms, vending, guild, the ignore list; two guild
packets are built in helpers at `0x515ba0` and `0x515d00`, not read), except
two that are sent variable-length although the table lists them fixed
(`ro_proto::lengths::CLIENT_SEND_OVERRIDES`):

* `0x01d5` (NPC text input, table 8), built at `0x513dd0`: id, u16 length
  = 8 + the text with its NUL, u32 NPC id, the text and its NUL;
* `0x019c` (local broadcast, table 4), built at `0x511e7c` by the same code
  as `0x0099`: id, u16 length = 4 + the text, the text.

One more send is odd and seems unreachable: case 0x81 of the zone mode's
`SendMsg` (`0x50f390`, case at `0x5143d4`) sends a 4-byte `0x008c` whose
length field says 2. No caller of that slot passes 0x81 as an immediate.
A server that frames by the length field (Athena, and us) would refuse it.

## Layouts confirmed from the client's receive code

Every packet the server sends has been read in the client's handler
(2026-11-14). Message numbers are 0-based lines of `msgstringtable.txt`.

| Packet | What the client does | Where |
|---|---|---|
| 0x0069 login accept | login ids at +4/+8/+12, sex byte at +46, server records of 32 bytes from +47 (`count = (len-47) >> 5`) | `sub_5306b0` |
| 0x006a login refused | error code byte at +2, switch over 0..0x63 | `sub_530770` |
| 0x006b char list | `count = (len-4)/106`, records read from `4 + (len-4) % 106` — so both a bare 4-byte header and the era's 4+20-byte header work | `0x52f75a`, `sub_5309f0` |
| char record | 106 bytes; slot byte at +104, class u16 at +52 | `CLoginMode` array at `+0xe30` |
| 0x006d char created | copies one 106-byte record | `0x52f7f6` |
| login-mode dispatch | ids 0x69..0x74 via jump table at `0x52fa88`, 0x81 separately | `0x52f70d` |
| zone-mode dispatch | `id - 0x73` indexes a byte table at `0x518b2c`, then a jump table at `0x5187b4`; each case calls a handler with the packet | `0x516642` |
| 0x0078 / 0x0079 / 0x007b units | id, speed, opt1, opt2, option, class, hair, weapon, head bottom (0x007b: then a u32 tick), shield, head top, head mid, hair colour, clothes colour, head direction as i16; guild u32; then **three i16**: emblem, manner (below 0 the unit is marked, effect 0x22 at `0x535d90`), one more it stores; karma, sex bytes; position (0x007b: the 6-byte move) and two bytes; 0x0078 ends with the state (0 stand, 1 dead, 2 sit). 0x007b's last two bytes are never read. Athena writes a u32 emblem and the manner after it: one i16 off (fixed 2026-11-14) | `0x5191d0`, `0x518dc0`, `0x51b780` |
| 0x008a action | src, dst, tick, src delay, dst delay (u32; a delay of 0 becomes 1440), damage, div (i16; damage -30000 drops the packet), kind byte, damage2 | `0x5198a0` |
| 0x009d / 0x009e floor item | id u32, nameid, identified, x, y, then amount and sub-cell (0x009d) or sub-cell and amount (0x009e) | `0x51c100`, `0x51c270` |
| 0x00a0 item added | idx, amount, nameid, identified, attribute, refine, four cards, location, type, result (switch over 0..6); type 10 is given the arrow slot by the client itself | `0x51c3f0` |
| 0x00a3 item list | `(len-4)/10` records: idx, nameid, type, identified, amount, wear (mod0631: 0x8000 for arrows) | `0x51ca00` |
| 0x00a4 equipment list | `(len-4)/20` records: idx, nameid, type, identified, location, worn location, attribute, refine, four cards. A pet egg (9001-9025) here with a non-zero attribute marks the egg whose pet is out; mod0631 lists eggs here as type 4, we list them in 0x00a3 (not tried in the client) | `0x51cc50` |
| 0x00bd status | status points, the six stats with their costs, atk, atk2, matk max, matk min, def, def2, mdef, mdef2, hit, flee, flee2, crit, then the attack delay (the window shows (2000 - it) / 10, as for 0x00b0 SP_ASPD) and a word it stores but never reads. Athena sends karma and manner in those two (fixed 2026-11-14) | `0x51e390` |
| 0x0119 state | id u32, opt1, opt2, option (i16), karma byte | `0x5226f0` |
| 0x01d7 look | id u32, type byte, then a u32 (type 2: weapon in the low word, shield in the high one). Other units ignore type 8 here | `0x52af20` |
| 0x0080 vanish | id u32, type 0..3 (0 out of sight, 1 died: for oneself the restart window, 2 logged out with effect 7, 3 teleported with effect 0x22) | `0x51bb50` |
| 0x0086 / 0x0087 move | 0x0086: id u32, the 6-byte move, tick u32. 0x0087: tick u32, the 6-byte move (12 bytes: no trailing byte) | `0x5166c6`, `0x516734` |
| 0x0088 stop | id u32, x, y (i16) | `0x516ac9` |
| 0x00b0 parameter | u16 type at +2 (switch to 0x82), u32 value. **Types 11 and 55 (base and job level) play the level-up sound and effect on every receipt**, so they go out on a real change only (fixed 2026-11-14; the full status refresh sent them each time). Weight in tenths; 24 crossing 50% and 90% gives the client's own messages | `0x51ce73` |
| 0x00b1 / 0x00be / 0x0141 | 0x00b1 types 1, 2, 20, 22, 23 (u32); 0x00be stat cost (types 32..37, u8); 0x0141 type read as u32, base and bonus u32 | `0x51d9e0`, `0x51e5e0`, `0x524e20` |
| 0x0196 status icon | type i16, id u32, flag byte (0 off) | `0x527e10` |
| 0x00b4 / 0x00b7 NPC text, menu | npc id u32 at +4, text from +8. 0x00b7's text is copied unchecked into a 2048-byte buffer and split on `:` (empty options dropped): our menus are capped to fit (2026-11-14) | `0x516c1c`, `0x51ded0` |
| 0x010f skill list | `(len-4)/37` records: id (1..158, others skipped), inf read as u32, level, sp, range; the 24-byte name is never read | `0x521e90` |
| 0x013e casting | src, dst (0: on the ground at x, y), x, y, skill, element u32 (picks the circle for magic), cast ms. Brandish Spear and Bowling Bash get no bar | `0x523ba0` |
| 0x011a / 0x01de / 0x0117 skill results | as Athena's. For skills 12, 18, 21, 70, 79, 83, 85, 89 and 110, 0x011a's target is read as a packed cell to turn the caster; we send Hammer Fall a unit id there, as mod0631 does | `0x523e20`, `0x52bd40`, `0x522620` |
| 0x0073 / 0x007f clock | 0x0073: tick u32, packed position; its last two bytes are never read. 0x007f: tick u32; the first sets the clock, later ones adjust it and feed the ping | `0x51a340`, `0x519cf0` |
| 0x008d / 0x008e chat | 0x008d: id u32 (ignore list checked), text from +8; 0x008e: text from +4. **Both copy the text unchecked, without a NUL, into a 256-byte stack buffer**: longer text crashes the client. Ours are cut to 255 bytes and a NUL (2026-11-14). The bubble needs the unit in the client's list | `0x519bd0`, `0x5167a0` |
| 0x0091 map move | 16-byte name the client NUL-terminates itself, then drops its last 4 characters and adds `.rsw`; x, y as i16. The same map repositions without a reload. We kept a NUL inside the 16, which would cut `mjolnir_04_1.gat` (fixed 2026-11-14) | `0x519e00` |
| 0x0095 name | id u32, 24-byte name. **For the player's own pet the name is used as a sprintf format** (as in 0x01a2): pet names lose `%` (DEVIATIONS.md) | `0x5168a8` |
| 0x0098 whisper result | 0 shows the whisper sent, 1 "no such character" (msg 148), 2 and 3 msgs 149, 150; above 3 as 1 | `0x51afd0` |
| 0x009a broadcast | at most 1023 bytes, NUL added. **The client edits the text**: from a `#` to the next `:` is cut out, and a leading `blue` (any case) is removed and turns the line blue (Athena's announce flag 0x10, which we send the same way) | `0x51a770` |
| 0x009c / 0x00a1 / 0x00c0 / 0x00c2 | direction: id, head i16, body byte. Floor item gone: id. Emotion: id, byte (oneself included). User count: i32 into msg 178 | `0x51697c`, `0x516a58`, `0x516daa`, `0x516e29` |
| 0x00b3 / 0x018b quit | 0x00b3: only 1 acts (back to character select). 0x018b: 0 quits, 1 "You can't quit now" (msg 502) | `0x516d3b`, `0x51829f` |
| 0x00b5 / 0x00b6 dialog | npc id u32. 0x00b6 reads the id only when no window is waiting | `0x516c93`, `0x516cca` |
| 0x00c3 look | id u32, type byte 0..8, value **u8**; for oneself it also updates the status globals | `0x51e760` |
| 0x0142 / 0x01d4 input boxes | npc id u32. **They open only for an NPC the client has as a unit**: an invisible NPC asking for a number or text would wait forever. None of ours does (2026-11-14) | `0x517abd`, `0x52abb0` |
| 0x0144 / 0x0145 | Compass: the npc id is never read; type u32 (2 removes), x, y u32, mark byte, colour u32. Cut-in 0x0145: 16-byte name, position byte; we send 0x01b3, read the same way with 64 bytes | `0x524fb0`, `0x517b11` |
| 0x019b effect | id u32, type u32 0..8 → effect. **It looks the id up only among other units**, so one aimed at the player draws nothing. Level-ups and refining show theirs anyway, through 0x00b0 and 0x0188 | `0x528c70` |
| 0x01dc login key | second login dispatch (0x187..0x1dc, `0x52f9aa`); key from +4, length = len - 4; answers 0x01dd with MD5 of key + password (or password + key) | `0x5314e0` |
| 0x00a5 / 0x00a6 / 0x0122 / 0x0123 lists | storage and cart lists, laid out as 0x00a3 (10 bytes) and 0x00a4 (20 bytes) | `0x520670`, `0x5208d0`, `0x522d80`, `0x522b30` |
| 0x00f4 / 0x0124 added, 0x00f6 / 0x0125 removed | idx, amount **u32**, nameid, identified, attribute, refine, four cards: no type, so the client stores type 0. Removed: idx, amount u32 | `0x520a20`, `0x522f50`, `0x520b10`, `0x523040` |
| 0x00f2 / 0x0121 counts | count, max (i16); the cart's weights are i32 **in tenths** | `0x517324`, `0x517780` |
| 0x00a8 item used | idx, **amount left** (i16), result. 0: silent | `0x51db50` |
| 0x00aa / 0x00ac equip | 0x00aa result 1 puts on the item at idx (msg 170) and takes it out of the inventory list; 0 and 2 "You can't put this item on." 0x00ac: **idx is never read**, the lowest set bit of the location picks the slot. A pet accessory put on must get no 0x00aa (mod0631 `pet.c:653`; we sent a success until 2026-11-14) | `0x51dc10`, `0x516b7a` |
| 0x00af item gone | idx, amount | `0x516bb9` |
| 0x00c4 / 0x00c6 / 0x00c7 shop | deal window for the npc id; buy list 11 bytes (price, discounted price, type, nameid); sell list 10 bytes (idx, price, overcharged; indices not held are skipped) | `0x516ea0`, `0x51ea00`, `0x51eb10` |
| 0x00ca / 0x00cb deal result | 0x00ca: 0 done (msg 54), 1 no zeny (55), 2 overweight (56), 3 no room (221); **any result closes the shop**. 0x00cb: 0 done, 1 failed (57). We could send 2 and then 0 for a half-bought list (fixed 2026-11-14, as mod0631 `npc.c:370`) | `0x51eca0`, `0x51ed80` |
| 0x0188 refine result | result i16 (0 effect 0x9a and msg 498, 1 effect 0x9b and msg 499), idx, new refine | `0x5278c0` |
| 0x00bc stat raised | type, ok (0: silent), value byte | `0x51dff0` |
| 0x010a / 0x010b / 0x010c MVP | item (shown by its unidentified name), EXP (printed signed), the effect over a unit | `0x521af0`, `0x51756a`, `0x5175ce` |
| 0x010e skill update | skill, level, sp, range, upgradable; inf kept from 0x010f | `0x521dc0` |
| 0x0110 skill failed | skill, basic type u32, result, cause: 1 SP, 2 HP, 3 no memo, 4 delay, 5 zeny, 6 weapon, 7/8 gemstones, 9 overweight, 10 "failed" (behind a flag). Cause 0 by skill (27, 50, 52 have their own text) | `0x522040` |
| 0x0115 knock-back damage | 0x01de's fields with x, y (i16) where 0x01de has -1, -1; damage i16 | `0x522500` |
| 0x011c / 0x011e warp | four names read with strlen (each must end in a NUL: ours do), then "cancel". /memo: 0 saved, 1 skill too low, 2 no Warp Portal | `0x5229d0`, `0x517737` |
| 0x0120 / 0x01c9 ground units | 0x0120 removes by id. 0x01c9: id, src, x, y, type, a byte the client overwrites, Graffiti flag; with the flag the text is **strcpy'd unbounded** (ours ends in a NUL). Types below 0x2d are not drawn. **A unit whose src is not the player toggles**: the client keeps a list of such ids, which 0x0120 does not clear, and a second 0x01c9 for the same id is drawn hidden (`0x52a9a6`-`0x52aa75`), a third shown again. Every 0x0091 clears the list, same map or not (`0x4fde60`, from `0x519f4e`). Hidden is the unit's byte `+0x85`: neither drawn (`0x5a91f0`) nor given its effect (`0x5a9e80`), **whatever its type**; it is shown all the same when the placer is in the list at session `+0xe4c` (`0x58d6c0`: the party, filled by 0x00fb through `0x58cf40`, thinned by 0x0105). The byte the server sends at +15 (`isVisible`, 1 in jAthena's captures) is never read. The server therefore re-sends only traps (THREADS.md #28) | `0x5176e7`, `0x52a890`, unit message 0x2d `0x5a95b8` |
| 0x0191 Talkie Box | id, 80 bytes of text. The id is a **ground unit's**: looked up in the list 0x0120 removes from (`0x4c8cc0`, world `+0x30`) and given unit message 7, which puts the text in a balloon over the unit, 35 characters a line, colour 0xff8080 (`0x5a99b8`); an empty text shows nothing. 817 sent the caster a made-up id ("not working"); jAthena //0988 (2004-05-25) sends the trap's id. Played 2026-11-14 | `0x5183c8`, `0x5a99b8` |
| 0x01ac reveal | id u32: a unit of type 0x8b-0x99 drawn hidden is shown again (unit message 0x77, `0x5a9923`); others are left alone. Sent by Detect | `0x529870` |
| 0x013a / 0x013b / 0x013c arrows | attack range i16; arrow message 0..3 (3 "Arrow has been equipped."); quiver idx | `0x517a28`, `0x523990`, `0x517a4d` |
| 0x013d / 0x0148 / 0x01b9 | recovery: type 5 HP, 7 SP, amount i16. Resurrected: id (the u16 after it is never read). Cast cut: id | `0x523a70`, `0x525300`, `0x529d80` |
| 0x0147 item skill | as a 0x010f record (inf u32); the name is never read. Inf 4 (Teleport) is used at once | `0x525180` |
| 0x0189 teleport refused | 0 "Unavailable area to teleport" in red, 1 the same in yellow | `0x518273` |
| 0x018c Sense | class, level, size, HP u32, def, race (a number), mdef, element (`% 20`), then nine element bytes | `0x5182f1` |
| 0x0192 cell type | x, y, type (i16); the map name is never read; only 1 and 5 block. A freed Ice Wall cell now gets 3 back on water (2026-11-14) | `0x5183fa` |
| 0x0177 / 0x017b / 0x01a6 / 0x01ad pick lists | `(len-4)/2` i16 inventory indices (unknown ones skipped); 0x01ad holds **item ids**, not indices. An empty list still opens the window | `0x527000`, `0x527200`, `0x529220`, `0x5298c0` |
| 0x0179 / 0x017d results | identify: idx, 0 done / 1 failed; card: equip idx, card idx, 0 / 1. On success the client marks the item, or fills the first free card slot and drops a card, itself | `0x527100`, `0x527320` |
| 0x018d / 0x018f forging | list: 8-byte records, **only the item id is read** (Athena's 0x0012 and maker id are ignored); empty: "You can't create items yet." Result: 0 effect 0x9a, 1 0x9b, 2 0x131, 3+ 0x132; its message is built and never shown | `0x51833c`, `0x527bb0` |
| 0x0194 char name | id u32, 24 bytes (no NUL needed) | `0x51842f` |
| 0x019e / 0x01a0 taming | start (body unread), result byte (stops the slot machine on frame 7 or 3) | `0x528ed0`, `0x528fe0` |
| 0x01a2 / 0x01a3 / 0x01a4 / 0x01aa pets | status: name[24] (**a sprintf format**), renamed, level, hunger, intimacy, accessory; level ≥ 10000 fills the item-info window instead. Feed: ok, food; the reaction uses the hunger from before, so 0x01a4 hunger goes first (as ours). Data: kind 0 spawn, 1 intimacy, 2 hunger, 3 accessory (10001-10020 have sprites), 4 performance, 5 ignores its value. Emote: below 50000 an emotion, 50000+ and 90000+ pet talk | `0x529320`, `0x529430`, `0x529030`, `0x5295b0` |

## Confirmed by running the real client against the server

(headless Wine rig, see DEVELOPMENT.md): 0x0064/0x0069 login, the raw 4-byte
account id the char and map servers send before their first packet, 0x006b
list, 0x0067/0x006d creation, 0x0066/0x0071 hand-off, 0x0072/0x0073 map entry,
0x007d load ack, status block (0x00bd, 0x00b0, 0x00b1, 0x0141, 0x00be),
0x00a3/0x00a4 inventory, 0x0085/0x0087 walking, 0x0091 map change, 0x0078
NPC/monster display (52 bytes), 0x0086 monster moves, 0x008c/0x008e chat,
0x0090 NPC click, 0x00b4/0x00b5/0x00b7/0x00b8/0x00b9/0x00b6 dialogs,
0x0089/0x008a attacks with damage/miss display, 0x0094/0x0095 names, 0x009e
item drop.

## Notable differences from later clients

* 0x0078/0x0079/0x007b/0x01d8–0x01da are **2 bytes shorter** than eAthena's
  2004 defaults: no trailing u16 base level (kRO added it in 2004 for the
  level-99 aura). Our 0x0078 = Athena's 54-byte layout minus that field; the
  client draws NPCs and monsters correctly with it.
* Walk is 0x0085 with a 3-byte packed target; map login is 0x0072 with 19
  bytes; skills 0x0113/0x0116 are 10 bytes.
* Packed positions: 3 bytes `x:10 y:10 dir:4`; moves 6 bytes `x0 y0 x1 y1`
  + sub-cell byte 0x88.
* Text is Windows-1252 (`servicetype germany`).

## Packet log

`packet_log = true` in the config writes every packet in both directions to
`logs/packets.log` (`<session> <dir> <id> <len> <hex>`). Combined with the
headless rig this is how new packets get confirmed: perform the action in
the client, read what it sent, compare what we answer with the length table
and the client's handler.
