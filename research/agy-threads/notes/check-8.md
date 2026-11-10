# Fact-Check: euRO CD Patcher Ragnarok.exe and Lutie Xmas Trigger

The euRO CD patcher executable (Ragnarok.exe, built 2003-12-12) contains hardcoded strings 2003-10-31gdata_xmas.gpf and notice_xmas.txt, and executes a remote check at offset 0x51c4 against patch_allow.txt that automatically triggers the Christmas resource download when the server returns 'xmas', proving Lutie was an integrated permanent town whose seasonal decor was dynamically triggered.

## Verdict
**Partly** (with corrected wording).

### Corrected Wording
The euRO CD patcher executable (`Ragnarok.exe`, built 2003-12-12 01:11:30 UTC) contains the hardcoded strings `2003-10-31gdata_xmas.gpf` and `notice_xmas.txt` and executes a check at file offset `0x51c4` against the remote server response from `patch_allow.txt` for the string `"xmas"`. However, if the server returns `"xmas"`, the patcher does **not** download any Christmas resources; instead, it checks whether `2003-10-31gdata_xmas.gpf` already exists locally via `CreateFileA`. If the file is missing, the patcher displays `notice_xmas.txt`, disables launching the game, and immediately terminates execution. Furthermore, Lutie (`xmas`) was already a permanent official town included in `data.grf` since Episode 2 (December 2002); the patcher routine was an unactivated legacy version gate inherited from Gravity's generic late-2003 patcher template, not a dynamic seasonal decor trigger.

---

## Findings

### 1. Patcher Executable Identity & Build Date (CONFIRMED)
- **Binary**: `Ragnarok.exe` from the euRO installation CD (located at `/data/play/RagnarokOnline/Ragnarok.exe`).
- **File size**: 147,456 bytes.
- **SHA-256**: `91eafb955fcbdf61a18bc1acfbd1e5aa2e127c25127072ddfe3f6e7b191c63c2` (matches `./Ragnarok.exe` in `/data/installed-files.sha256`).
- **PE TimeDateStamp**: `1071191490` -> `2003-12-12 01:11:30 UTC`.
- *Finding*: The claim that the executable was built on 2003-12-12 is strictly **confirmed**.

### 2. Hardcoded Strings in Binary (CONFIRMED)
The strings cited in the claim exist in the `.data` section of `Ragnarok.exe`:
- `notice_xmas.txt` — File offset `0x01d0d4` (Virtual Address `0x0041d0d4`)
- `2003-10-31gdata_xmas.gpf` — File offset `0x01d0e4` (Virtual Address `0x0041d0e4`)
- `xmas` — File offset `0x01d100` (Virtual Address `0x0041d100`)
- `deny` — File offset `0x01d108` (Virtual Address `0x0041d108`)
- `patch_allow.txt` — File offset `0x01d310` (Virtual Address `0x0041d310`)
- `notice_close.txt` — File offset `0x01d350` (Virtual Address `0x0041d350`)
- `notice_patch.txt` — File offset `0x01d390` (Virtual Address `0x0041d390`)
- `patch2.txt` — File offset `0x01d3d0` (Virtual Address `0x0041d3d0`)
- `wpatch.euro-ro.net` — File offset `0x01d210` (Virtual Address `0x0041d210`)

### 3. Remote Check at Offset 0x51c4 against `patch_allow.txt` (CONFIRMED IN LOCATION)
The patcher initializes URLs at `0x00406ec0`-`0x00406f17` and requests `http://wpatch.euro-ro.net/patch/patch_allow.txt` at `0x004050da`.
The received response buffer (`edi`) is evaluated in sequence:
- `0x00405159`–`0x00405169`: `strncmp(edi, "deny", 4)`. If matching, it downloads `notice_close.txt`, displays it, and halts (`jmp 0x00405705`).
- `0x004051c1`: `push 4`
- `0x004051c3`: `push 0x0041d100` (`"xmas"`; the 32-bit pointer operand is located at byte offset `0x51c4` in the file)
- `0x004051c8`: `push edi`
- `0x004051c9`: `call 0x004102c0` (`strncmp`)
- `0x004051ce`: `add esp, 0xc`
- `0x004051d1`: `test eax, eax`
- `0x004051d3`: `jne 0x00405244` (if response is not `"xmas"`, jump directly to normal patch routine)

### 4. Behavior When Server Returns 'xmas' (CONTRADICTED: Local Check & Halt, NOT Automatic Download)
Disassembly of instructions following a positive `"xmas"` match:
```assembly
004051d5: push 0x0041d0e4              ; "2003-10-31gdata_xmas.gpf"
004051da: lea  ecx, [ebp-0x38]
004051dd: call 0x00404b8d              ; CreateFileA("2003-10-31gdata_xmas.gpf", GENERIC_READ, OPEN_EXISTING)
004051e2: test al, al                  ; Returns al = 1 if file exists, 0 if missing
004051e4: jne  0x00405244              ; IF FILE ALREADY EXISTS: proceed to normal patch2.txt pipeline!
004051e6: push 0x0041d0d4              ; "notice_xmas.txt"
004051eb: mov  ecx, esi
004051ed: call 0x00407d7b              ; Download & display notice_xmas.txt in notice window
004051f2: lea  ecx, [esi+0x48]
004051f5: call 0x004016f6              ; Disable Start button
...
0040523f: jmp  0x00405705              ; EXIT PATCHER ROUTINE (terminates without patching or starting)
```
- Subroutine `0x00404b8d` calls Windows API `CreateFileA` with `dwCreationDisposition = OPEN_EXISTING` (`3`) and `dwDesiredAccess = GENERIC_READ` (`0x80000000`). It tests if the file exists on the player's local disk.
- There is only **one** reference to `2003-10-31gdata_xmas.gpf` in the entire binary (at `0x004051d6`), and it is passed only to `CreateFileA`.
- The binary contains **no** download routine or HTTP request for `2003-10-31gdata_xmas.gpf`.
- If the server returned `"xmas"` and the user did not have `2003-10-31gdata_xmas.gpf` in their folder, the patcher refused to patch and instructed the player to read `notice_xmas.txt`.
- *Finding*: The claim that the patcher "automatically triggers the Christmas resource download" is strictly **contradicted**.

### 5. Lutie Permanence and Seasonal Decor Hypothesis (CONTRADICTED)
- **Lutie was already permanent**: kRO Episode 2 ("A Call to the Snowland: Lutie") went live on December 17, 2002.
- **CD Base Files**: The euRO CD client's `data.grf` (dated 2003-12-18) already contains all Lutie maps (`data\xmas.gat`, `data\xmas.rsw`, `data\xmas.gnd`, `data\xmas_dun01.gat`, `data\xmas_dun02.gat`, `data\xmas_fild01.gat`, `data\xmas_in.gat`) and over 320 models/textures in `data\model\크리스마스마을\`. Lutie did not require any patcher download to exist.
- **euRO Never Used 'xmas' Trigger**: In euRO patch history (`research/notes/euro-history.md`), `patch_allow.txt` on `wpatch.euro-ro.net` returned `"allow"` during beta operation and `"deny"` during maintenance. Neither `2003-10-31gdata_xmas.gpf` nor `notice_xmas.txt` appeared in any archived euRO patch index (`patch2.txt`) or announcement. The filename date `2003-10-31` reflects Gravity's kRO Sakray/main patch from late October 2003, which was incorporated into Gravity's generic patcher client codebase and inherited by the euRO build.
- **Cited URL Evaluation**: `http://web.archive.org/web/20040207012248/http://www.euro-ro.net/server/` contains announcements for the Closed Beta launch (Dec 20, 2003) and the New Comodo update (Jan 8, 2004). It contains zero mentions of Lutie, seasonal decor, `patch_allow.txt`, or Christmas downloads.

---

## Contradictions and Open Points

1. **Gate Check vs. Automated Downloader**:
   The primary contradiction in the original claim is confusing a gating prerequisite check (`CreateFileA` + modal stop) with an automated downloader. The code was an enforcement mechanism requiring an existing local patch rather than an automated fetcher.
2. **Dynamic Seasonal Decor vs. Permanent Town Assets**:
   Town assets for Lutie (`xmas`) were not seasonal overlays; they were permanent Episode 2 assets embedded directly in the main `data.grf` distributed on the CD.

---

## Sources Read

1. `/data/play/RagnarokOnline/Ragnarok.exe` (SHA-256: `91eafb955fcbdf61a18bc1acfbd1e5aa2e127c25127072ddfe3f6e7b191c63c2`) — Primary euRO CD client patcher binary (built 2003-12-12 01:11:30 UTC). Disassembly of `0x004050c6`–`0x00405244` confirms string locations and execution flow.
2. `/data/data.grf.list` & `/data/play/RagnarokOnline/data.grf` — Primary euRO CD client archive (built 2003-12-18/19). Confirms all Lutie maps (`xmas.gat`, `xmas_dun01/02.gat`, `xmas_fild01.gat`, `xmas_in.gat`) and 320 Christmas town models were pre-packaged on the disc.
3. `http://web.archive.org/web/20040207012248/http://www.euro-ro.net/server/` — Official euRO website capture (2004-02-04). Documents Closed Beta (Dec 20, 2003) and New Comodo (Jan 8, 2004); no reference to Lutie or Christmas patch.
4. `http://nn.ai4rei.net/dev/rsu/help/usage.htm` — Technical documentation of Ragnarok Online patchers and `patch_allow.txt` by Ai4rei (RO Patcher Lite developer). Confirms official server usage of `patch_allow.txt` (`allow`/`deny`).
5. `/opt/src/ragnarok-offline/research/notes/euro-history.md` — Project historical research notes detailing `patch2.txt` history from `wpatch.euro-ro.net` and analysis of unactivated Gravity template files.
6. `/opt/src/ragnarok-offline/research/agy-threads/notes/r1-maps-pets-arena-lutie.md` — Initial research note tracking Thread #19 and identifying string offsets in `Ragnarok.exe`.

---

## Could Not Read
- None (all primary binary, archive, and web sources successfully fetched and analyzed).
