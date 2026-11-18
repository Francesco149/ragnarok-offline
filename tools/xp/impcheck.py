# impcheck.py EXE DLLDIR [READOBJ]: every function EXE imports that the DLLs in DLLDIR (copied from
# a Windows XP SP3 system32) do not export. Exit status 1 if any.
import subprocess, sys, os, re, collections
RO = sys.argv[3] if len(sys.argv) > 3 else 'llvm-readobj'
def run(*a): return subprocess.run(a, capture_output=True, text=True).stdout
imps = collections.defaultdict(set); cur = None
for line in run(RO, '--coff-imports', sys.argv[1]).splitlines():
    m = re.search(r'Name: (\S+\.dll)', line, re.I)
    if m and 'Import {' not in line: cur = m.group(1).lower(); continue
    m = re.search(r'Symbol: (\S+) \(', line)
    if m and cur: imps[cur].add(m.group(1))
dlls = {f.lower(): os.path.join(sys.argv[2], f) for f in os.listdir(sys.argv[2])}
bad = 0
for dll, syms in sorted(imps.items()):
    if dll not in dlls: print('NO DLL ON XP (or not copied):', dll, sorted(syms)); bad += 1; continue
    ex = set(re.findall(r'Name: (\S+)', run(RO, '--coff-exports', dlls[dll])))
    miss = sorted(syms - ex)
    print(f'{dll}: {len(syms)} imported, {len(miss)} missing', miss if miss else '')
    bad += len(miss)
print("TOTAL MISSING", bad)
sys.exit(1 if bad else 0)
