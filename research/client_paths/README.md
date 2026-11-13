# The client's path finder, run under an emulator

`crates/ro-game/src/path.rs` (`client_search`) is a port of the client's
`CPathFinder` (Ragexe 2003-12-18, `0x4d2300`; docs/CLIENT.md). These tools
run the exe's own code on test grids, so the port can be checked against it.
They read the exe at run time and keep nothing of it.

In the workbench (`tools/wb exec bash -c '...'`), once:

    python3 -m venv target/re/venv && target/re/venv/bin/pip install unicorn pefile

Then:

    E=/data/play/RagnarokOnline/Ragexe.exe
    python3 -I research/client_paths/gen_cases.py target/re/cases.json SEED N
    target/re/venv/bin/python3 -I research/client_paths/emu_path.py $E target/re/cases.json target/re/out.json
    python3 -I research/client_paths/to_fixture.py target/re/out.json crates/ro-game/tests/data/client_paths.json
    cargo test -p ro-game --test client_paths

* `gen_cases.py`: synthetic grids (scattered blocks, walls with gaps,
  buildings and diagonal lines, U-shaped traps), 40 start/target pairs each.
  The committed fixture is seed 1, 120 grids (4,800 paths).
* `real_cases.py GRF OUT SEED`: windows of the played GRF's maps. These are
  Gravity's cells: never commit the output. Check them with
  `CLIENT_PATHS=<fixture> cargo test -p ro-game --test client_paths`.
* `emu_path.py`: maps the PE image with unicorn, stands in for `operator
  new` (`0x5be5fb`, a bump allocator), `operator delete` (`0x5afe70`),
  `atexit` (`0x5be687`), `_Lockit` (`0x5afe7b`, `0x5aff17`) and the error
  log (`0x405ac0`), runs the static initialisers `0x4d4800` (the tree's nil
  node) and `0x4fbf10` (the path finder at `0x66e0b8`), then calls
  `0x4d2300(time 0, sx, sy, 8, 8, tx, ty, speed 150, &out)` on a fake map
  object (`+0x110` width, `+0x114` height, `+0x120` cells of 20 bytes, the
  type at `+0x10`).
* `gen_times.py CASES OUT SEED`: the same pairs with a random sub-cell,
  speed and start tick each, plus an open grid of first steps where a float
  or double square root, or the product's rounding, change the time.
  `emu_path.py` then also returns each cell's time (the x87 set to 24-bit
  precision, as Direct3D 7 leaves it in the game), and `to_fixture.py`
  keeps them. The committed `client_walk_times.json` is seed 2, 30 grids:

      python3 -I research/client_paths/gen_cases.py target/re/tc.json 2 30
      python3 -I research/client_paths/gen_times.py target/re/tc.json target/re/tt.json 2
      target/re/venv/bin/python3 -I research/client_paths/emu_path.py $E target/re/tt.json target/re/tt.out.json
      python3 -I research/client_paths/to_fixture.py target/re/tt.out.json crates/ro-game/tests/data/client_walk_times.json

* `rdis.py EXE START END`: linear disassembly, for reading the code.

Checked on 2026-11-13: 4,800 synthetic paths and 5,760 on windows of all
269 maps of the played GRF agree with the port, path for path. Changing the
heap's tie rule or dropping the closed-node quirk makes 22 and 10 of the
synthetic ones differ, so the fixture tells them apart.

Checked on 2026-11-13 too: the times of 925 paths (`path::client_times`).
Taking a double square root, or not rounding the product to a float, makes
7 and 27 of them differ; leaving out the column-0 quirk, 79.
