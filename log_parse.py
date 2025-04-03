#!/usr/bin/env python3

for line in open(0).readlines():
    if not line:
        continue
    line = line.split('\t')[-1]
    entries = line.split('|')
    name = entries[0].strip()
    name = name[name.index(' ')+1:]
    rows = int(entries[1].strip().split()[-1].replace('_', ''))
    cells = int(entries[2].strip().split()[-1].replace('_', ''))
    cols = " ".join(entries[4].strip().split()[3:]).replace('_', '')
    assert cols.startswith("[") and cols.endswith("]")
    cols = int(cols[1:-1].split(', ')[0])
    print(f"{name}\t{rows}\t{cols}\t{cells}")
