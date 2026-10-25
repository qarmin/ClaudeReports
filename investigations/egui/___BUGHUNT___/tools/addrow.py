import sys,json
# usage: addrow.py json
d=json.loads(sys.argv[1]); B='___BUGHUNT___/'
n=d['n']; slug=d['slug']
row=f"| {n} | {d['status']} | {d.get('cls','n/a')} | {d.get('fr','n/a')} | {slug} | {d['root']} | {d.get('intro','n/a')} |  | [`{n}.{slug}/RESULT.md`]({n}.{slug}/RESULT.md) |\n"
open(B+'SUMMARY.md','a').write(row)
if d['status']=='Confirmed':
    open(B+'SUMMARY-SHORT.md','a').write(f"| {n} | {d['cls']} | {d['short']} | {d.get('lines','n/a')} | {d['rootshort']} | {d.get('intro_hash','not determined')} |\n")
