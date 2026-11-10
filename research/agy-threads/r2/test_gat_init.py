import os
import re
import sys
import struct
import zlib

GRF_PATH = "/data/play/RagnarokOnline/data.grf.original"

# 1. Load GRF resnametable and GAT cache
def load_gat_checker(grf_path):
    import sys
    sys.path.insert(0, "/work/research")
    import grf_peek

    gat_cache = {}
    table_data = None
    for n, cs, csa, rs, flags, pos in grf_peek.entries(grf_path):
        if n.lower() == "data\\resnametable.txt":
            with open(grf_path, "rb") as f:
                f.seek(pos)
                table_data = zlib.decompress(f.read(csa))[:rs]
            break
    alias = {}
    if table_data:
        for line in table_data.decode("latin1", "replace").splitlines():
            p = line.split("#")
            if len(p) >= 2:
                alias[p[0].lower()] = p[1]

    def get_gat_data(map_name):
        map_base = map_name.lower().replace(".gat", "")
        gat_name = alias.get(f"{map_base}.gat", f"{map_base}.gat")
        if gat_name in gat_cache:
            return gat_cache[gat_name]
        data = None
        for n, cs, csa, rs, flags, pos in grf_peek.entries(grf_path):
            if n.lower() == f"data\\{gat_name}".lower():
                with open(grf_path, "rb") as f:
                    f.seek(pos)
                    data = zlib.decompress(f.read(csa))[:rs]
                break
        gat_cache[gat_name] = data
        return data

    def is_walkable(map_name, x, y):
        data = get_gat_data(map_name)
        if not data or data[:4] != b"GRAT":
            return False, "missing_or_invalid"
        w, h = struct.unpack_from("<ii", data, 6)
        if not (0 <= x < w and 0 <= y < h):
            return False, "outside"
        kind = struct.unpack_from("<I", data, 14 + (y * w + x) * 20 + 16)[0]
        return (kind not in (1, 5)), f"type_{kind}"

    return is_walkable

print("GAT checker initialized successfully.")
