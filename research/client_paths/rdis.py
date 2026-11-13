# dis.py EXE START END  -> linear disassembly (VA), with string/imm annotations
import sys, pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_32
pe = pefile.PE(sys.argv[1]); base = pe.OPTIONAL_HEADER.ImageBase
img = pe.get_memory_mapped_image()
s, e = int(sys.argv[2], 16), int(sys.argv[3], 16)
md = Cs(CS_ARCH_X86, CS_MODE_32)
for i in md.disasm(img[s-base:e-base], s):
    print(f"{i.address:08x}  {i.mnemonic:6} {i.op_str}")
