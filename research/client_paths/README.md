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
* `rdis.py EXE START END`: linear disassembly, for reading the code.

Checked on 2026-11-13: 4,800 synthetic paths and 5,760 on windows of all
269 maps of the played GRF agree with the port, path for path. Changing the
heap's tie rule or dropping the closed-node quirk makes 22 and 10 of the
synthetic ones differ, so the fixture tells them apart.
