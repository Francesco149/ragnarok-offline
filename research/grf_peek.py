#!/usr/bin/env python3
"""Exploration helper: list / cat entries of a GRF v0x200 (unencrypted entries only).

The real reader (all versions, DES) is the Rust crate `ro-grf`; this exists so
early identification work doesn't wait on it.

  grf_peek.py list <file.grf> [substring]
  grf_peek.py cat  <file.grf> <name>
"""
import struct, sys, zlib


def entries(path):
    with open(path, "rb") as f:
        hdr = f.read(46)
        assert hdr[:15] == b"Master of Magic", "not a GRF"
        off, seed, n2, ver = struct.unpack_from("<IIII", hdr, 30)
        assert ver == 0x200, f"version {ver:#x} not handled here"
        count = n2 - seed - 7
        f.seek(off + 46)
        csize, usize = struct.unpack("<II", f.read(8))
        tbl = zlib.decompress(f.read(csize))
        assert len(tbl) == usize
        p = 0
        for _ in range(count):
            z = tbl.index(b"\0", p)
            name = tbl[p:z].decode("cp949", "replace")
            cs, csa, rs, flags, pos = struct.unpack_from("<IIIBI", tbl, z + 1)
            p = z + 1 + 17
            yield name, cs, csa, rs, flags, pos + 46


def main():
    cmd, path = sys.argv[1], sys.argv[2]
    if cmd == "list":
        sub = sys.argv[3].lower() if len(sys.argv) > 3 else ""
        for name, cs, csa, rs, fl, pos in entries(path):
            if sub in name.lower():
                print(f"{fl:02x} {rs:10d} {cs:10d} {name}")
    elif cmd == "cat":
        want = sys.argv[3].lower()
        for name, cs, csa, rs, fl, pos in entries(path):
            if name.lower() == want:
                if fl & 6:
                    sys.exit(f"{name}: encrypted (flags {fl:#x}); use ro-grf")
                with open(path, "rb") as f:
                    f.seek(pos)
                    sys.stdout.buffer.write(zlib.decompress(f.read(csa))[:rs])
                return
        sys.exit("not found")


if __name__ == "__main__":
    main()
