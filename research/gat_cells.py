#!/usr/bin/env python3
"""Is a cell walkable on THIS client's map? Reads the .gat straight from a GRF 0x200.

  gat_cells.py <data.grf> <map> [x,y ...]      e.g. gat_cells.py data.grf.original job_wizard 99,160 60,165
  gat_cells.py <data.grf> <map>                size and count of walkable cells
  gat_cells.py <data.grf> <map> --heights      "W H", then each row's cell heights (mean of the four
                                               corners, negative is up), from row 0 (tools/client-test/play.sh)

Use it to test whether a script written for some map name fits the map this
client has (research/notes/round1-review.md: `job_wiz` does not fit
`job_wizard`). Map caches of later emulators describe later maps; don't use
them for this. Cell types as Athena read them: 1 and 5 are blocked, the rest
walkable. Names are resolved through the client's data\\resnametable.txt, so
`new_1-1` answers for `new_zone01`.
"""
import struct
import sys
import zlib

sys.path.insert(0, __file__.rsplit("/", 1)[0])
import grf_peek  # noqa: E402


def cat(grf, name):
    for n, cs, csa, rs, flags, pos in grf_peek.entries(grf):
        if n.lower() == name.lower():
            if flags & 6:
                sys.exit(f"{n}: encrypted; use ro-grf")
            with open(grf, "rb") as f:
                f.seek(pos)
                return zlib.decompress(f.read(csa))[:rs]
    return None


def main():
    grf, m = sys.argv[1], sys.argv[2]
    table = cat(grf, "data\\resnametable.txt") or b""
    alias = {}
    for line in table.decode("latin1").splitlines():
        p = line.split("#")
        if len(p) >= 2:
            alias[p[0].lower()] = p[1]
    gat = alias.get(f"{m}.gat".lower(), f"{m}.gat")
    data = cat(grf, "data\\" + gat)
    if data is None:
        sys.exit(f"{gat}: not in this client")
    assert data[:4] == b"GRAT", "not a .gat"
    w, h = struct.unpack_from("<ii", data, 6)

    def kind(x, y):
        return struct.unpack_from("<I", data, 14 + (y * w + x) * 20 + 16)[0]

    def ok(x, y):
        return 0 <= x < w and 0 <= y < h and kind(x, y) not in (1, 5)

    if sys.argv[3:] == ["--heights"]:
        print(w, h)
        for y in range(h):
            print(" ".join(str(round(sum(struct.unpack_from("<4f", data, 14 + (y * w + x) * 20)) / 4)) for x in range(w)))
        return
    cells = [tuple(int(v) for v in a.split(",")) for a in sys.argv[3:]]
    if not cells:
        n = sum(ok(x, y) for y in range(h) for x in range(w))
        print(f"{gat}: {w}x{h}, {n} walkable cells")
        return
    good = 0
    for x, y in cells:
        good += ok(x, y)
        t = kind(x, y) if 0 <= x < w and 0 <= y < h else "outside"
        print(f"{gat} {x},{y}: {'walkable' if ok(x, y) else 'blocked'} (type {t})")
    print(f"{good} of {len(cells)} walkable")


if __name__ == "__main__":
    main()
