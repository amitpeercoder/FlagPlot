import json, sys
src, out = sys.argv[1], sys.argv[2]
W, E, Sx, N = 142.0, 175.0, -26.5, 2.0
def clip(ring):
    def cut(pts, inside, inter):
        res = []
        for i in range(len(pts)):
            a, b = pts[i - 1], pts[i]
            ia, ib = inside(a), inside(b)
            if ia and ib: res.append(b)
            elif ia and not ib: res.append(inter(a, b))
            elif not ia and ib: res.append(inter(a, b)); res.append(b)
        return res
    def ix(x):
        return lambda a, b: (x, a[1] + (b[1] - a[1]) * (x - a[0]) / (b[0] - a[0]))
    def iy(y):
        return lambda a, b: (a[0] + (b[0] - a[0]) * (y - a[1]) / (b[1] - a[1]), y)
    p = [(c[0], c[1]) for c in ring]
    for inside, inter in ((lambda q: q[0] >= W, ix(W)), (lambda q: q[0] <= E, ix(E)), (lambda q: q[1] >= Sx, iy(Sx)), (lambda q: q[1] <= N, iy(N))):
        if not p: break
        p = cut(p, inside, inter)
    return p
rings = []
for path in src.split(','):
    for f in json.load(open(path))['features']:
        g = f['geometry']
        polys = g['coordinates'] if g['type'] == 'MultiPolygon' else [g['coordinates']]
        for poly in polys:
            ring = poly[0]
            xs = [c[0] for c in ring]; ys = [c[1] for c in ring]
            if max(xs) < W or min(xs) > E or max(ys) < Sx or min(ys) > N: continue
            p = clip(ring)
            if len(p) < 3: continue
            flat = []
            last = None
            for x, y in p:
                q = (round(x, 3), round(y, 3))
                if q != last: flat += [q[0], q[1]]; last = q
            if len(flat) >= 6: rings.append(flat)
json.dump({"rings": rings}, open(out, 'w'), separators=(',', ':'))
print(len(rings), sum(len(r) for r in rings) // 2)
