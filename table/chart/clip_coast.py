#!/usr/bin/env python3
"""Clips Natural Earth land to a chart's box and writes the coastline the table
draws (see table/README.md, "The chart's coastline"). Standard library only.

    python3 table/chart/clip_coast.py [--map MAP.csv] SOURCE[,SOURCE...] OUT.json

SOURCE is a GeoJSON file or an ESRI shapefile (.shp) of polygons: Natural Earth's
ne_10m_land and ne_10m_minor_islands. Only outer rings are kept. A shapefile's
outer rings are the clockwise ones.

With --map, the box is the extent in the header of that map file (read as the
table's bridge reads it, either kind of frame: one factor or about a centre, decision
0242), padded by 3 degrees on every side, and each ring is
brought into the map's longitude run whole: where the map crosses the date line
(its longitudes above 180), a ring lying wholly west of the map's western edge
has 360 added to every point, and a ring written twice is written once. The output then holds longitudes in the run, and
the page draws it as it is. With no --map the box is the fixed one of the
slice, 142 to 175 E and 26.5 S to 2 N, and the output is as it has always been.
"""
import argparse, json, os, struct, sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from serve import read_frame_and_edges


def shp_polygons(path):
    """Each polygon record of a shapefile as a list of its outer rings
    (the clockwise parts), each a list of (lon, lat)."""
    with open(path, "rb") as handle:
        data = handle.read()
    pos, polys = 100, []
    while pos + 8 <= len(data):
        length = struct.unpack(">i", data[pos + 4:pos + 8])[0] * 2
        body = data[pos + 8:pos + 8 + length]
        pos += 8 + length
        kind = struct.unpack("<i", body[:4])[0]
        if kind == 0:
            continue
        if kind not in (5, 15, 25):
            raise SystemExit(f"{path}: shape type {kind} is not a polygon")
        nparts, npoints = struct.unpack("<ii", body[36:44])
        parts = list(struct.unpack(f"<{nparts}i", body[44:44 + 4 * nparts]))
        at = 44 + 4 * nparts
        points = [struct.unpack("<dd", body[at + 16 * i:at + 16 * i + 16]) for i in range(npoints)]
        bounds = parts + [npoints]
        rings = []
        for i in range(nparts):
            ring = points[bounds[i]:bounds[i + 1]]
            area = sum(ring[k][0] * ring[(k + 1) % len(ring)][1] - ring[(k + 1) % len(ring)][0] * ring[k][1] for k in range(len(ring)))
            if area < 0:
                rings.append(ring)
        polys.append(rings)
    return polys


def geojson_polygons(path):
    polys = []
    for f in json.load(open(path))["features"]:
        g = f["geometry"]
        for poly in (g["coordinates"] if g["type"] == "MultiPolygon" else [g["coordinates"]]):
            polys.append([[(c[0], c[1]) for c in poly[0]]])
    return polys


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("src")
    ap.add_argument("out")
    ap.add_argument("--map", help="a generated map file: its extent, padded by 3 degrees, is the box")
    args = ap.parse_args()
    W, E, Sx, N = 142.0, 175.0, -26.5, 2.0
    frame = None
    if args.map:
        found = read_frame_and_edges(args.map)
        if not found:
            raise SystemExit(f"{args.map} has no frame in its header")
        frame, north, south = found
        W, E, Sx, N = frame["west"] - 3, frame["east"] + 3, south - 3, north + 3

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
    for path in args.src.split(","):
        polys = shp_polygons(path) if path.endswith(".shp") else geojson_polygons(path)
        for poly in polys:
            for ring in poly[:1] if not path.endswith(".shp") else poly:
                xs = [c[0] for c in ring]; ys = [c[1] for c in ring]
                if frame and frame["east"] > 180 and max(xs) < frame["west"]:
                    ring = [(x + 360, y) for x, y in ring]
                    xs = [x + 360 for x in xs]
                if max(xs) < W or min(xs) > E or max(ys) < Sx or min(ys) > N: continue
                p = clip(ring)
                if len(p) < 3: continue
                flat = []
                last = None
                for x, y in p:
                    q = (round(x, 3), round(y, 3))
                    if q != last: flat += [q[0], q[1]]; last = q
                # The two Natural Earth files hold some islands twice: with --map a ring
                # already written is not written again (the slice's file keeps its repeats).
                if len(flat) >= 6 and not (frame and flat in rings): rings.append(flat)
    json.dump({"rings": rings}, open(args.out, "w"), separators=(",", ":"))
    print(len(rings), sum(len(r) for r in rings) // 2)


main()
