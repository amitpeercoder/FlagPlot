#!/usr/bin/env python3
"""The measurements behind decision 0242 (a map from Papua to Midway).

For each candidate projection, how far a distance on the flat map is from the
true distance on the globe: a leg of 250 nm in 24 directions from each named
place (the shortest and the longest reading, as a share of the true leg), and
the straight line between pairs of places.

The positions are approximate, typed to a tenth of a degree for this
measurement alone. They are not facts of data/oob and nothing reads them.
The build of the map measures again from the rows of
data/oob/place_positions.csv.

Run: python3 scripts/wide-map/projection_error.py
The first part is the first pass (four candidates, on the extent first
thought of, centred on the equator). The second part is the pass the record
quotes: the two leading schemes on the extent of the record.
"""
import math, itertools
R=3437.74677  # nm per radian
P={ # lat, lon east-positive (run on past 180)
'Midway':(28.2,182.6),'Pearl Harbor':(21.35,202.05),'Wake':(19.3,166.6),'Johnston':(16.7,190.5),
'Truk':(7.4,151.8),'Kwajalein':(8.7,167.7),'Tarawa':(1.4,173.0),'Saipan':(15.2,145.7),
'Rabaul':(-4.2,152.2),'Lae':(-6.7,147.0),'Port Moresby':(-9.45,147.2),'Milne Bay':(-10.3,150.4),
'Lunga':(-9.4,160.0),'Espiritu Santo':(-15.5,167.2),'Noumea':(-22.3,166.4),'Suva':(-18.1,178.4),
'Pago Pago':(-14.3,189.3),'Tongatabu':(-21.1,184.8),'Townsville':(-19.25,146.8),'Brisbane':(-27.4,153.1),
'Canton':(-2.8,188.3),'Palmyra':(5.9,197.9),'Nauru':(-0.5,166.9)}
def gc(a,b):
    la1,lo1,la2,lo2=map(math.radians,(*a,*b))
    h=math.sin((la2-la1)/2)**2+math.cos(la1)*math.cos(la2)*math.sin((lo2-lo1)/2)**2
    return 2*R*math.asin(math.sqrt(h))
def dest(a,brg,d):
    la,lo=map(math.radians,a); b=math.radians(brg); c=d/R
    la2=math.asin(math.sin(la)*math.cos(c)+math.cos(la)*math.sin(c)*math.cos(b))
    lo2=lo+math.atan2(math.sin(b)*math.sin(c)*math.cos(la),math.cos(c)-math.sin(la)*math.sin(la2))
    return (math.degrees(la2),math.degrees(lo2))
def equirect(lat0,lon0,f):
    return lambda p:((p[1]-lon0)*60*f,(p[0]-lat0)*60)
def azeq(lat0,lon0,k):
    def pr(p):
        c=gc((lat0,lon0),p)/R
        if c==0: return (0,0)
        la0,lo0,la,lo=map(math.radians,(lat0,lon0,*p))
        y=math.sin(lo-lo0)*math.cos(la); x=math.cos(la0)*math.sin(la)-math.sin(la0)*math.cos(la)*math.cos(lo-lo0)
        az=math.atan2(y,x); r=k*R*c
        return (r*math.sin(az),r*math.cos(az))
    return pr
def stereo(lat0,lon0,k):
    def pr(p):
        c=gc((lat0,lon0),p)/R
        if c==0: return (0,0)
        la0,lo0,la,lo=map(math.radians,(lat0,lon0,*p))
        y=math.sin(lo-lo0)*math.cos(la); x=math.cos(la0)*math.sin(la)-math.sin(la0)*math.cos(la)*math.cos(lo-lo0)
        az=math.atan2(y,x); r=k*R*2*math.tan(c/2)
        return (r*math.sin(az),r*math.cos(az))
    return pr
def laea(lat0,lon0,k):
    def pr(p):
        c=gc((lat0,lon0),p)/R
        if c==0: return (0,0)
        la0,lo0,la,lo=map(math.radians,(lat0,lon0,*p))
        y=math.sin(lo-lo0)*math.cos(la); x=math.cos(la0)*math.sin(la)-math.sin(la0)*math.cos(la)*math.cos(lo-lo0)
        az=math.atan2(y,x); r=k*R*2*math.sin(c/2)
        return (r*math.sin(az),r*math.cos(az))
    return pr
def pd(pr,a,b):
    x1,y1=pr(a);x2,y2=pr(b);return math.hypot(x2-x1,y2-y1)
def local(pr,a,d=250):
    # worst and best ratio map/true for a 250 nm leg in 8 directions from a
    rs=[pd(pr,a,dest(a,b,d))/d for b in range(0,360,15)]
    return min(rs),max(rs)
def report(name,pr):
    worst=0;rows=[]
    for n,a in P.items():
        lo,hi=local(pr,a); e=max(abs(lo-1),abs(hi-1)); worst=max(worst,e); rows.append((n,lo,hi))
    pairs=[(a,b,pd(pr,P[a],P[b])/gc(P[a],P[b])-1) for a,b in itertools.combinations(P,2)]
    wp=max(pairs,key=lambda t:abs(t[2]))
    return worst,rows,wp,pairs
import sys
def best(make,ks):
    out=None
    for k in ks:
        w,rows,wp,pairs=report('',make(k))
        if out is None or w<out[0]: out=(w,k,rows,wp,pairs)
    return out
C0=(0.0,173.5)
cands={
 'A one factor (as today)':best(lambda f:equirect(0,173.5,f),[x/1000 for x in range(850,1001,2)]),
 'B azimuthal equidistant':best(lambda k:azeq(*C0,k),[x/1000 for x in range(940,1001,2)]),
 'C stereographic (conformal)':best(lambda k:stereo(*C0,k),[x/1000 for x in range(900,1001,2)]),
 'D Lambert equal-area':best(lambda k:laea(*C0,k),[x/1000 for x in range(940,1040,2)]),
}
key=['Midway','Pearl Harbor','Truk','Rabaul','Port Moresby','Lunga','Noumea','Brisbane','Tarawa','Wake']
legs=[('Pearl Harbor','Midway'),('Pearl Harbor','Noumea'),('Truk','Rabaul'),('Truk','Midway'),('Saipan','Midway'),('Rabaul','Lunga'),('Noumea','Lunga'),('Pearl Harbor','Tongatabu'),('Brisbane','Port Moresby'),('Truk','Kwajalein')]
for n,(w,k,rows,wp,pairs) in cands.items():
    print(f"\n{n}: k={k}  worst 250nm leg at a named place {w*100:.1f}%  worst pair {wp[0]}-{wp[1]} {wp[2]*100:+.1f}%")
    d=dict((r[0],r) for r in rows)
    print('  local (shortest..longest, % of true): '+'; '.join(f"{x} {100*(d[x][1]-1):+.1f}/{100*(d[x][2]-1):+.1f}" for x in key))
    pp={(a,b):e for a,b,e in pairs}
    print('  legs: '+'; '.join(f"{a}-{b} {gc(P[a],P[b]):.0f}nm {100*pp.get((a,b),pp.get((b,a))):+.1f}" for a,b in legs))
# corners for each
for n,mk in [('A',equirect(0,173.5,cands['A one factor (as today)'][1])),('B',azeq(*C0,cands['B azimuthal equidistant'][1])),('C',stereo(*C0,cands['C stereographic (conformal)'][1]))]:
    cs=[(32,142),(32,205),(-30,142),(-30,205),(32,173.5),(-30,173.5),(0,142),(0,205)]
    print(n,'corners/edges:',' '.join(f"{100*(local(mk,c)[0]-1):+.0f}/{100*(local(mk,c)[1]-1):+.0f}" for c in cs))

print("\n=== second pass: extent 30 S to 38 N, 142 E to 153 W; centre 4 N 174.5 E ===")
P['Carrier battle off Midway']=(31.5,181.0)
C1=(4.0,174.5)
A=equirect(20.5,174.5,0.936672)
bb=best(lambda k:azeq(*C1,k),[x/1000 for x in range(940,1001,1)])
B=azeq(*C1,bb[1]); print('B k=',bb[1])
show=['Carrier battle off Midway','Midway','Pearl Harbor','Wake','Truk','Kwajalein','Tarawa','Rabaul','Port Moresby','Lunga','Espiritu Santo','Noumea','Suva','Pago Pago','Townsville','Brisbane']
print('place | A shortest/longest | B shortest/longest  (a 250 nm leg, % of true)')
for n in show:
    a=local(A,P[n]); b=local(B,P[n])
    print(f"{n:26s} | {100*(a[0]-1):+5.1f} {100*(a[1]-1):+5.1f} | {100*(b[0]-1):+5.1f} {100*(b[1]-1):+5.1f}")
for nm,c in [('north edge 38N mid',(38,174.5)),('NW corner',(38,142)),('NE corner',(38,207)),('SW corner',(-30,142)),('SE corner',(-30,207)),('south edge mid',(-30,174.5))]:
    a=local(A,c); b=local(B,c)
    print(f"{nm:26s} | {100*(a[0]-1):+5.1f} {100*(a[1]-1):+5.1f} | {100*(b[0]-1):+5.1f} {100*(b[1]-1):+5.1f}")
legs=[('Pearl Harbor','Midway'),('Pearl Harbor','Noumea'),('Pearl Harbor','Tongatabu'),('Truk','Rabaul'),('Truk','Midway'),('Saipan','Midway'),('Rabaul','Lunga'),('Noumea','Lunga'),('Brisbane','Port Moresby'),('Truk','Kwajalein'),('Pearl Harbor','Lunga')]
for a,b in legs:
    t=gc(P[a],P[b]); print(f"{a}-{b}: true {t:.0f} nm, A {100*(pd(A,P[a],P[b])/t-1):+.1f}, B {100*(pd(B,P[a],P[b])/t-1):+.1f}")
# present maps for comparison
SP=equirect(-12,160,0.978148); MW=equirect(28.5,178.5,0.878817)
for n in ['Rabaul','Lunga','Noumea','Port Moresby']:
    a=local(SP,P[n]); print('present south-west Pacific map',n,f"{100*(a[0]-1):+.1f} {100*(a[1]-1):+.1f}")
for n in ['Midway','Carrier battle off Midway']:
    a=local(MW,P[n]); print('present Midway map',n,f"{100*(a[0]-1):+.1f} {100*(a[1]-1):+.1f}")
