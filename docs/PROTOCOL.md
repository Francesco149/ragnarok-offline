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

The table is what the client uses to frame what it **receives**. Two
client→server packets are sent variable-length although the receive table
lists them fixed (`ro_proto::lengths::CLIENT_SEND_OVERRIDES`): `0x01d5`
(NPC text input) and `0x019c` (local broadcast). Still to verify from the
send code (ROADMAP).

## Layouts confirmed from the client's receive code

| Packet | What the client does | Where |
|---|---|---|
| 0x0069 login accept | login ids at +4/+8/+12, sex byte at +46, server records of 32 bytes from +47 (`count = (len-47) >> 5`) | `sub_5306b0` |
| 0x006a login refused | error code byte at +2, switch over 0..0x63 | `sub_530770` |
| 0x006b char list | `count = (len-4)/106`, records read from `4 + (len-4) % 106` — so both a bare 4-byte header and the era's 4+20-byte header work | `0x52f75a`, `sub_5309f0` |
| char record | 106 bytes; slot byte at +104, class u16 at +52 | `CLoginMode` array at `+0xe30` |
| 0x006d char created | copies one 106-byte record | `0x52f7f6` |
| login-mode dispatch | ids 0x69..0x74 via jump table at `0x52fa88`, 0x81 separately | `0x52f70d` |

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
