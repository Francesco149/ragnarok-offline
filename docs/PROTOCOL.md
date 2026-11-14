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
