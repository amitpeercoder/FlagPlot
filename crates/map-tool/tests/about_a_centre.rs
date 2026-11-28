//! The projection about a centre (decision 0242, rules 3, 12 and 16), checked
//! against a hand calculation.
//!
//! # The hand calculation
//!
//! Written by the architect on Nov 28, 2026, before any code. Nobody else
//! changes a number here; a mismatch goes back to the architect.
//!
//! ## The frame
//!
//! The frame of decision 0242, rule 1: kind "about a centre", centre 4 N,
//! 174.5 E, scale 0.964, extent 38 N to 30 S and 142 E to 153 W (207 in the
//! run). The miles in a radian are 3437.746771. Hexes are 40 nm between
//! centres and rows 34.641016 nm apart, as on every map: hex (q, r)'s centre
//! is 40 x (q + r/2) nm east and 34.641016 x r nm south of hex (0,0)'s.
//!
//! ## Part 1: a position to the plane, and to its hex
//!
//! For a point at latitude `lat` and longitude `lon` (in the run), with the
//! centre at `lat0`, `lon0`, all in radians:
//!
//! - `cos c = sin lat0 sin lat + cos lat0 cos lat cos(lon - lon0)`; `c` is the
//!   angle from the centre.
//! - the bearing is `atan2(sin(lon - lon0) cos lat,
//!   cos lat0 sin lat - sin lat0 cos lat cos(lon - lon0))`.
//! - the distance on the map is `c x 3437.746771 x 0.964`; east is the
//!   distance times the sine of the bearing, north the distance times the
//!   cosine.
//!
//! Each position is a row of `data/oob/place_positions.csv`, as the row gives
//! it. A longitude west of 142 E has 360 added.
//!
//! | Row | lat | lon | angle, rad | bearing | east, nm | north, nm | hex | to its centre, nm | next nearest |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | the centre | 4.0 | 174.5 | 0 | none | 0.000 | 0.000 | (0,0) | 0.00 | |
//! | 5100 Midway | 28.232 | -177.370 | 0.44395982 | 16.8629 | 426.793 | 1408.015 | (31,-41) | 14.02 | (31,-40) 25.98 |
//! | 5107 Wake | 19.299 | 166.624 | 0.29883292 | 333.9413 | -435.042 | 889.657 | (2,-26) | 12.07 | (2,-25) 28.01 |
//! | 5113 Point Luck | 32.0 | -173.0 | 0.52982716 | 21.2963 | 637.706 | 1635.942 | (39,-47) | 19.35 | (40,-47) 23.62 |
//! | 46 Truk | 7.454 | 151.808 | 0.39857090 | 279.7315 | -1301.853 | 223.266 | (-29,-7) | 19.31 | (-30,-6) 23.81 |
//! | 1 Rabaul | -4.22224 | 152.16544 | 0.41507194 | 250.0193 | -1292.746 | -470.029 | (-39,14) | 19.64 | (-39,13) 20.99 |
//! | 6 Lunga Point | -9.40538 | 160.02861 | 0.34365516 | 227.0303 | -833.327 | -776.266 | (-32,22) | 15.66 | (-32,23) 24.43 |
//! | 16 Noumea | -22.25675 | 166.42019 | 0.47856679 | 196.4078 | -447.991 | -1521.377 | (-33,44) | 8.48 | (-34,44) 32.13 |
//! | 5108 Saipan | 15.194 | 145.748 | 0.53103177 | 293.5611 | -1613.123 | 703.453 | (-30,-20) | 16.89 | (-30,-21) 24.97 |
//!
//! East and north hold to 0.001 nm, the angle to 0.00000001, the bearing to
//! 0.0001 degree, a distance to a centre to 0.01 nm. The hex is exact.
//!
//! ## Part 2: a hex to its position
//!
//! The other way: for a point `east`, `north` in the plane, `rho` is its
//! distance from the origin, `c = rho / (3437.746771 x 0.964)`, the bearing is
//! `atan2(east, north)`, and
//!
//! - `lat = asin(sin lat0 cos c + cos lat0 sin c cos bearing)`;
//! - `lon = lon0 + atan2(sin bearing sin c cos lat0,
//!   cos c - sin lat0 sin lat)`, which is in the run.
//!
//! The origin itself is the centre. The file's longitude is the run's less
//! 360 where the run's is above 180.
//!
//! | Hex | east, nm | north, nm | lat | lon in the run | lon in the file |
//! | --- | --- | --- | --- | --- | --- |
//! | (0,0) | 0.000 | 0.000 | 4.0000 | 174.5000 | 174.5000 |
//! | (10,0) | 400.000 | 0.000 | 3.9709 | 181.4324 | -178.5676 |
//! | (0,-10) | -200.000 | 346.410 | 9.9781 | 170.9954 | 170.9954 |
//! | (18,-13) | 460.000 | 450.333 | 11.7213 | 182.5983 | -177.4017 |
//! | (-40,30) | -1000.000 | -1039.230 | -13.8741 | 156.9763 | 156.9763 |
//! | (45,-20) | 1400.000 | 692.820 | 15.2519 | 199.4508 | -160.5492 |
//! | (-46,3) | -1780.000 | -103.923 | 1.7252 | 143.7157 | 143.7157 |
//!
//! Latitude and longitude hold to four decimals as printed. Each of these
//! centres, put back through part 1, falls in its own hex. So does every hex
//! of part 3.
//!
//! ## Part 3: the hexes of the extent
//!
//! A hex is of the map when its centre's latitude is from -30 to 38 and its
//! longitude in the run from 142 to 207, edges included.
//!
//! - There are 10,395 such hexes, in rows -60 to 58.
//! - Row by row, the first and last q and the count:
//!
//! | r | first q | last q | hexes |
//! | --- | --- | --- | --- |
//! | -28 | -30 | 58 | 89 |
//! | -10 | -41 | 51 | 93 |
//! | 0 | -46 | 46 | 93 |
//! | 3 | -48 | 45 | 94 |
//! | 10 | -52 | 42 | 95 |
//! | 30 | -61 | 31 | 93 |
//! | 57 | -71 | 14 | 56 |
//!
//! - The edges bow. Along the west edge (142 E) the plane's east is least at
//!   2.1 S, -1882.04 nm, and at the north-west corner it is only -1559.20.
//!   Along the north edge (38 N) the plane's north is greatest at the two
//!   corners, 2090.98 nm, and along the south edge least at the two corners,
//!   -2034.03. A box taken from the north-west and south-east corners alone
//!   (west -1559.2, east 1721.7) with a hex to spare would miss 587 of the
//!   10,395. Hex (-46,3) of part 2 is one of them.
//! - No hex centre lies within 0.0005 degree of an edge, so the count does
//!   not turn on the last bits of the arithmetic.
//!
//! ## Part 4: the header's lines
//!
//! For this frame the tool writes exactly:
//!
//! ```text
//! # Projection: about a centre. Hex (0,0) is centred at 4 N, 174.5 E.
//! #   distance_nm = angle from the centre in radians * 3437.746771 * 0.964, on the bearing from the centre
//! #   east_nm = distance_nm * sin(bearing), north_nm = distance_nm * cos(bearing); a sphere of 60 nm to the degree.
//! # Extent: every hex whose centre lies from 38 N to 30 S and from 142 E to 153 W.
//! ```
//!
//! `sim-data` reads them back as: kind about a centre, centre 4 and 174.5,
//! scale 0.964, north 38, south -30, west 142, east 207 in the run. A header
//! with the first line and without the second or third is refused; so is one
//! with a line of each kind.
//!
//! ## Part 5: the two present maps
//!
//! For a frame of one factor nothing changes: the lines the tool writes are
//! those it writes today, and the hexes of the extent are the same hexes in
//! the same order. The test of the committed maps checks this for both.
