#!/usr/bin/env python3
"""Draw a map's cells as text, to lay out a script on THIS client's map.

  gat_ascii.py <data.grf> <map> [x0,y0,x1,y1] [step]

'#' blocked (types 1, 5), '.' walkable, '~' water (type 3), other types as
their digit. North is up; the left and bottom margins give x and y.
"""
import struct
import sys

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from gat_cells import cat  # noqa: E402


def main():
    grf, m = sys.argv[1], sys.argv[2]
    data = cat(grf, f"data\\{m}.gat")
    if data is None:
        sys.exit(f"{m}: not in this client")
    w, h = struct.unpack_from("<ii", data, 6)
    x0, y0, x1, y1 = (map(int, sys.argv[3].split(","))
                      if len(sys.argv) > 3 else (0, 0, w - 1, h - 1))
    step = int(sys.argv[4]) if len(sys.argv) > 4 else 1

    def ch(x, y):
        t = struct.unpack_from("<I", data, 14 + (y * w + x) * 20 + 16)[0]
        return "#" if t in (1, 5) else "." if t == 0 else "~" if t == 3 else str(t)[0]

    print(f"{m}: {w}x{h}")
    print("     " + "".join(str((x // 10) % 10) if x % 10 == 0 else " "
                            for x in range(x0, x1 + 1, step)))
    for y in range(y1, y0 - 1, -step):
        print(f"{y:4} " + "".join(ch(x, y) for x in range(x0, x1 + 1, step)))


if __name__ == "__main__":
    main()
