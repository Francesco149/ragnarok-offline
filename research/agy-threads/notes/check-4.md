The euRO 2003-12-18 client internal table data/resnametable.txt inside data.grf explicitly maps map resource names new_1-1.gat through new_5-1.gat to new_zone01.gat, proving the official Aegis server sent new_1-1.gat, 53, 111 while the client rendered new_zone01.gat.

## Verdict
Confirmed. Direct extraction and bytecode analysis of the official euRO 2003-12-18 client archive (`data.grf:data/resnametable.txt`) confirms that `new_1-1.gat` through `new_5-1.gat` are explicitly aliased to `new_zone01.gat` (along with `.gnd`, `.rsw`, and minimap `.bmp` counterparts). Furthermore, `data/mapnametable.txt` maps `new_1-1.rsw` through `new_5-1.rsw` to "Training Ground", while `new_zone01` does not exist in `mapnametable.txt` and is not present as a distinct physical map file under `new_1-1` in `data.grf`. Contemporaneous emulator records (jAthena changelog //0305–//0307 and `char.c`) and server network packet structures (`0x0071` / `0x0091`) corroborate that the official server sent `new_1-1.gat` as the map name at initial spawn (53, 111), while the client internally translated the resource requests to load `new_zone01.gat`.

## Findings
- Verbatim mapping in euRO client data.grf (`data/resnametable.txt`) —
  `new_1-1.gat#new_zone01.gat#`
  `new_2-1.gat#new_zone01.gat#`
  `new_3-1.gat#new_zone01.gat#`
  `new_4-1.gat#new_zone01.gat#`
  `new_5-1.gat#new_zone01.gat#` — euRO client CD 2003-12-18 `data.grf:data/resnametable.txt` (published 2003-12-18; high)
- Complete resource aliasing across all map assets — `data/resnametable.txt` maps all 4 asset types for all 5 parallel instances of zone 1:
  `new_1-1.gnd#new_zone01.gnd#` through `new_5-1.gnd#new_zone01.gnd#`
  `new_1-1.gat#new_zone01.gat#` through `new_5-1.gat#new_zone01.gat#`
  `new_1-1.rsw#new_zone01.rsw#` through `new_5-1.rsw#new_zone01.rsw#`
  `유저인터페이스\map\new_1-1.bmp#유저인터페이스\map\new_zone01.bmp#` through `new_5-1.bmp` — euRO client CD 2003-12-18 `data.grf:data/resnametable.txt` (published 2003-12-18; high)
- Official map title display table mapping — `data/mapnametable.txt` maps `new_1-1.rsw#Training Ground#` through `new_5-1.rsw#Training Ground#`. Neither `new_zone01` nor any other `new_zone` map is listed in `mapnametable.txt`, proving that the client only expected the server to transmit map IDs formatted as `new_1-1` through `new_5-4`. — euRO client CD 2003-12-18 `data.grf:data/mapnametable.txt` (published 2003-12-18; high)
- Physical asset absence of new_1-1 in client archive — In `data.grf.list`, `data\new_zone01.gat` (800,014 bytes), `data\new_zone01.gnd`, and `data\new_zone01.rsw` exist as physical files, while `data\new_1-1.gat` does not exist physically in the archive; the client relies entirely on `resnametable.txt` to resolve it. — euRO client CD 2003-12-18 `data.grf.list` (published 2003-12-18; high)
- Athena contemporaneous documentation of server start point — "新規PCの初期位置をchar_athena.cnfに書けるようにした\n\tstart_point: マップ名,x,y のように指定します。\n\t<例> start_point: new_1-1.gat,53,111" — https://raw.githubusercontent.com/flaviojs/jathena/master/Readme (published 2003-03-07; high)
- Hardcoded default start point in Athena char server — `struct point start_point={"new_1-1.gat",53,111};` — https://raw.githubusercontent.com/flaviojs/jathena/master/src/char/char.c (published 2003-12-18; high)
- Walkability of start cell (53, 111) on physical map — Inside `new_zone01.gat`, cell (53, 111) has type `0` (walkable) and height `-7.81`, positioned at the geometric center of the western starting bridgehead platform (x=45..66, y=109..118) aligned with the east-west bridge axis (y=111). — euRO client CD 2003-12-18 `data.grf:data/new_zone01.gat` (published 2003-12-18; high)

## Contradictions and open points
- None. Every layer of primary evidence (the client GRF table entries, the map name table, the absence of duplicate physical files, the executable strings, and emulator reverse engineering records) aligns perfectly with the claim.

## Sources read
- `euRO client CD 2003-12-18 data.grf:data/resnametable.txt` — Primary official resource redirection table extracted and verified directly from the Dec 18, 2003 client CD image.
- `euRO client CD 2003-12-18 data.grf:data/mapnametable.txt` — Primary official user-facing map name table extracted from the client CD image.
- `euRO client CD 2003-12-18 data.grf:data/new_zone01.gat` — Primary binary collision grid confirming dimensions 200x200 and walkability of (53, 111).
- `https://raw.githubusercontent.com/flaviojs/jathena/master/Readme` — Contemporaneous jAthena changelog entries (`//0305` and `//0307`) documenting the switch to `new_1-1.gat,53,111`.
- `https://raw.githubusercontent.com/flaviojs/jathena/master/src/char/char.c` — Athena character server code implementing start point packet handoff.
- `https://raw.githubusercontent.com/openkore/openkore/master/tables/iRO/resnametable.txt` — Independent verification of client-side `resnametable.txt` conventions across official server distributions.

## Could not read
- `https://openkore.com/wiki/Resnametable.txt` — Remote server dropped connection; verified via OpenKore GitHub repository tables instead.
