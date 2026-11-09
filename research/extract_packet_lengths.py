#!/usr/bin/env python3
"""Extract the packet length table from a 2003-era Ragexe.exe.

The client builds `std::map<int,int> packetLen` in one long function. Two
idioms, both: store the id to a stack slot, call into the map, write the length
through the returned pointer (an immediate, or a register loaded earlier with
`mov reg, imm` / `xor reg, reg` / `or reg, -1`):

  1. map::insert(make_pair(id, 0)) -> iterator; `mov [eax+0x10], LEN`
  2. map::operator[](id) -> int&;              `mov [eax], LEN`
     (the compiler often stages the *next* id before this store)

`or dword ptr [mem], -1` is VC6's short form of storing -1. -1 means
"variable length: a u16 total length follows the id".

This walks that function linearly with a tiny register model. It is the
protocol's ground truth for this client: every length the server sends must
match it, or the client desyncs.

  extract_packet_lengths.py <Ragexe.exe> [func_va] > packet_lengths.tsv
"""
import sys

import pefile
from capstone import CS_ARCH_X86, CS_MODE_32, Cs
from capstone.x86 import X86_OP_IMM, X86_OP_MEM, X86_OP_REG

DEFAULT_FUNC = 0x418030  # Ragexe.exe 2003-12-18 (PE ts 0x3fe1439f)


def main():
    path = sys.argv[1]
    func = int(sys.argv[2], 0) if len(sys.argv) > 2 else DEFAULT_FUNC
    pe = pefile.PE(path)
    base = pe.OPTIONAL_HEADER.ImageBase
    text = pe.sections[0]
    data = text.get_data()
    start = func - base - text.VirtualAddress
    md = Cs(CS_ARCH_X86, CS_MODE_32)
    md.detail = True

    regs = {}          # reg name -> known int value
    last_key = None    # last packet id stored to a stack slot
    pending = None     # id whose map slot the last call returned
    out = []           # (id, len, va)

    def store(val, va):
        nonlocal pending
        if pending is None:
            sys.exit(f"length store without a pending id at {va:#x}")
        ln = val - (1 << 32) if val & 0x80000000 else val
        out.append((pending, ln, va))
        pending = None

    def is_slot_ptr(mem):
        # [eax+0x10] (insert iterator node) or [eax] (operator[] int&)
        return mem.base and ins.reg_name(mem.base) == "eax" and not mem.index \
            and mem.disp in (0, 0x10)

    for ins in md.disasm(data[start:start + 0x20000], func):
        if ins.mnemonic == "ret":
            break
        ops = ins.operands
        m = ins.mnemonic
        if m == "call":
            if last_key is not None:
                pending = last_key
            for r in ("eax", "ecx", "edx"):     # caller-saved
                regs.pop(r, None)
            continue
        if m == "mov" and len(ops) == 2:
            dst, src = ops
            val = None
            if src.type == X86_OP_IMM:
                val = src.imm & 0xFFFFFFFF
            elif src.type == X86_OP_REG:
                val = regs.get(ins.reg_name(src.reg))
            if dst.type == X86_OP_REG:
                name = ins.reg_name(dst.reg)
                if val is None:
                    regs.pop(name, None)
                else:
                    regs[name] = val
            elif dst.type == X86_OP_MEM:
                mem = dst.mem
                basereg = ins.reg_name(mem.base) if mem.base else None
                if basereg == "ebp" and mem.disp < 0:
                    if val is not None and 0x60 <= val < 0x400:
                        last_key = val
                elif is_slot_ptr(mem):
                    if val is None:
                        sys.exit(f"unknown length value at {ins.address:#x}: {ins.op_str}")
                    store(val, ins.address)
            continue
        if m == "or" and len(ops) == 2 and ops[1].type == X86_OP_IMM and ops[1].imm in (-1, 0xFFFFFFFF):
            if ops[0].type == X86_OP_REG:
                regs[ins.reg_name(ops[0].reg)] = 0xFFFFFFFF
            elif ops[0].type == X86_OP_MEM and is_slot_ptr(ops[0].mem):
                store(0xFFFFFFFF, ins.address)
            continue
        if m == "xor" and len(ops) == 2 and ops[0].type == ops[1].type == X86_OP_REG \
                and ops[0].reg == ops[1].reg:
            regs[ins.reg_name(ops[0].reg)] = 0
            continue
        # any other write to a tracked register invalidates it
        if ops and ops[0].type == X86_OP_REG:
            regs.pop(ins.reg_name(ops[0].reg), None)

    if pending is not None:
        # operator[] default-constructs int() == 0 when nothing is stored
        print(f"# NOTE {pending:#06x}: slot created but never assigned -> 0", file=sys.stderr)
        out.append((pending, 0, 0))

    seen = {}
    print("# id\tlen\tva   (from Ragexe.exe @%#x; -1 = variable)" % func)
    for pid, ln, va in out:
        if pid in seen and seen[pid] != ln:
            print(f"# WARNING {pid:#06x} redefined {seen[pid]} -> {ln}", file=sys.stderr)
        seen[pid] = ln
    for pid in sorted(seen):
        va = [v for p, _, v in out if p == pid][-1]
        print(f"0x{pid:04x}\t{seen[pid]}\t{va:#x}")
    print(f"# {len(seen)} packets, {len(out)} stores", file=sys.stderr)


if __name__ == "__main__":
    main()
