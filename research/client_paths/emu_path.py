# emu_path.py EXE CASES.json OUT.json
# Runs the client's own path finder (Ragexe 2003-12-18, 0x4d2300) under unicorn.
# CASES: [{"w":W,"h":H,"cells":"0101..." (row-major, y*w+x; '1' blocked),
#          "pairs":[[sx,sy,tx,ty],...]}, ...]
#   A pair may go on with [subx, suby, speed, start] (default 8, 8, 150, 0):
#   the sub-cell the unit starts from (0-15, 8 the centre), ms a cell, and
#   the move's tick.
# OUT: same shape with "paths": [null | [[x,y,dir,time],...]] per pair, time
#   being when the client has the unit at that cell's centre.
import json, struct, sys
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_32, UC_HOOK_CODE, UC_HOOK_MEM_INVALID
from unicorn.x86_const import *

EXE, CASES, OUT = sys.argv[1:4]
pe = pefile.PE(EXE)
BASE = pe.OPTIONAL_HEADER.ImageBase
img = pe.get_memory_mapped_image()
size = (len(img) + 0xFFFF) & ~0xFFFF

mu = Uc(UC_ARCH_X86, UC_MODE_32)
mu.mem_map(BASE, size)
mu.mem_write(BASE, img)
STACK, STACK_SZ = 0x10000000, 0x100000
HEAP, HEAP_SZ = 0x20000000, 0x4000000
FAKE = 0x30000000  # fake map object + cells
mu.mem_map(STACK, STACK_SZ)
mu.mem_map(HEAP, HEAP_SZ)
mu.mem_map(FAKE, 0x1000000)
SENTINEL = 0x0BADF00C
mu.mem_map(0x0BADF000, 0x1000)

heap_top = [HEAP]

def u32(a): return struct.unpack("<I", mu.mem_read(a, 4))[0]
def i32(a): return struct.unpack("<i", mu.mem_read(a, 4))[0]
def w32(a, v): mu.mem_write(a, struct.pack("<I", v & 0xFFFFFFFF))

def ret_cdecl(eax):
    esp = mu.reg_read(UC_X86_REG_ESP)
    ra = u32(esp)
    mu.reg_write(UC_X86_REG_EAX, eax)
    mu.reg_write(UC_X86_REG_ESP, esp + 4)
    mu.reg_write(UC_X86_REG_EIP, ra)

def op_new(uc, addr, sz, ud):
    n = u32(mu.reg_read(UC_X86_REG_ESP) + 4)
    p = heap_top[0]
    heap_top[0] += (n + 15) & ~15
    assert heap_top[0] < HEAP + HEAP_SZ, "heap exhausted"
    ret_cdecl(p)

def op_noop(uc, addr, sz, ud):
    ret_cdecl(0)

def op_ret(uc, addr, sz, ud):  # thiscall, no args (_Lockit)
    ret_cdecl(0)

HOOKS = {0x5afe7b: op_ret, 0x5aff17: op_ret, 0x5be5fb: op_new, 0x5afe70: op_noop, 0x5be687: op_noop, 0x405ac0: op_noop}
for a, f in HOOKS.items():
    mu.hook_add(UC_HOOK_CODE, f, begin=a, end=a)

def bad(uc, access, addr, size, value, ud):
    print(f"invalid mem access {access} at {addr:#x} eip={uc.reg_read(UC_X86_REG_EIP):#x}", file=sys.stderr)
    return False
mu.hook_add(UC_HOOK_MEM_INVALID, bad)

def call(fn, ecx=0, args=()):
    esp = STACK + STACK_SZ - 0x1000
    for a in reversed(args):
        esp -= 4; w32(esp, a)
    esp -= 4; w32(esp, SENTINEL)
    mu.reg_write(UC_X86_REG_ESP, esp)
    mu.reg_write(UC_X86_REG_ECX, ecx)
    # the x87 as the game runs: Direct3D 7 leaves it at 24-bit precision
    # (SetCooperativeLevel at 0x404251 never asks for DDSCL_FPUPRESERVE)
    mu.reg_write(UC_X86_REG_FPCW, 0x007F)
    mu.emu_start(fn, SENTINEL, count=50_000_000)
    return mu.reg_read(UC_X86_REG_EAX)

call(0x4d4800)   # static init: the tree's nil node (0x66c7dc / 0x66c7d8)
call(0x4fbf10)   # static init: CPathFinder at 0x66e0b8
PF = 0x66e0b8
MARK = heap_top[0]
OUTINFO = FAKE + 0x800000

def run_case(w, h, cells, pairs):
    attr = FAKE
    cell_base = FAKE + 0x1000
    w32(attr + 0x110, w); w32(attr + 0x114, h); w32(attr + 0x120, cell_base)
    buf = bytearray(w * h * 20)
    for i, c in enumerate(cells):
        struct.pack_into("<I", buf, i * 20 + 0x10, 1 if c == "1" else 0)
    mu.mem_write(cell_base, bytes(buf))
    w32(PF, attr)
    res = []
    for p in pairs:  # also takes the fixture's rows
        sx, sy, tx, ty = p[:4]
        rest = [v for v in p[4:8] if isinstance(v, int)]
        subx, suby, speed, start = rest if len(rest) == 4 else (8, 8, 150, 0)
        mu.mem_write(OUTINFO, b"\0" * 0x40)
        # the previous call's tree nodes are erased (read) at the start of the
        # next call, before anything is allocated; the open list's buffer is
        # dropped here so new allocations cannot alias it
        heap_top[0] = MARK
        mu.mem_write(PF + 0x108c, b"\0" * 12)
        ok = call(0x4d2300, PF, (start, sx, sy, subx, suby, tx, ty, speed, OUTINFO)) & 0xFF
        if not ok:
            res.append(None); continue
        b, e = u32(OUTINFO + 4), u32(OUTINFO + 8)
        res.append([[i32(c), i32(c + 4), i32(c + 8), u32(c + 12)] for c in range(b, e, 16)])
    return res

cases = json.load(open(CASES))
for c in cases:
    c["paths"] = run_case(c["w"], c["h"], c["cells"], c["pairs"])
json.dump(cases, open(OUT, "w"))
print("cases", len(cases), "pairs", sum(len(c["pairs"]) for c in cases), "heap used", heap_top[0] - HEAP)
