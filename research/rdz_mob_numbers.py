"""RODataZone's iRO monster table (monsters.js, capture 2004-02-11) against data/db/mobs.toml:
HP, base and job EXP, matched by English name. Run from the repo root on the host.
Output of 2026-11-15: 191 the same, 6 differ, 59 names not matched (THREADS.md #33)."""
import re,tomllib,sys
t=open('/home/headpats/data/ragnarok-cd/refs/web/rodatazone-2004/20040211150750_monsters_monsters.js',encoding='latin-1').read()
rows=[]
for m in re.finditer(r"MONSTERS\[(\d+)\] = new Array\((.*?)\);\n",t,re.S):
    if m.group(1)=='0': continue
    f=re.findall(r"'((?:[^'\\]|\\.)*)'",m.group(2))
    rows.append(f)
db=tomllib.load(open('data/db/mobs.toml','rb')) if __import__('os').path.exists('data/db/mobs.toml') else None
mobs=db['mob']
byname={}
for m in mobs: byname.setdefault(m['display'].lower(),[]).append(m)
same=diff=miss=0; out=[]
for f in rows:
    name,lvl,size,hp,bx,jx=f[:6]
    c=byname.get(name.lower())
    if not c: miss+=1; continue
    m=c[0]
    try: hp,bx,jx=int(hp),int(bx),int(jx)
    except: continue
    if (m['hp'],m['base_exp'],m['job_exp'])==(hp,bx,jx): same+=1
    else: diff+=1; out.append(f"{name:22} ours hp {m['hp']:6} exp {m['base_exp']}/{m['job_exp']}   rdz hp {hp:6} exp {bx}/{jx}")
print('same',same,'diff',diff,'unmatched names',miss)
print('\n'.join(out[:80]))
