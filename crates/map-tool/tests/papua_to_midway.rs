//! The map from Papua to Midway (decision 0242, step 2): the committed map,
//! its marks and its file of located positions, checked against the
//! architect's figures.
//!
//! # The hand calculation
//!
//! Written by the architect on Nov 28, 2026, before any code. Nobody else
//! changes a number here; a mismatch goes back to the architect.
//!
//! ## The map
//!
//! `data/map/papua_to_midway.csv` is made from `data/map/frames/papua_to_midway.toml`
//! and the anchors file `data/map/papua_to_midway_anchors.csv`. Its header's
//! projection and extent lines are those of part 4 of
//! `tests/about_a_centre.rs`, and its hexes are the 10,395 of that file's
//! part 3, in rows -60 to 58, each centre where the projection puts it.
//!
//! ## The marks
//!
//! Two land marks, each a place with a base whose hex the sampling finds no
//! land in. By part 1 of `tests/about_a_centre.rs`:
//!
//! - Midway, row 5100 (28.232, -177.370), is in hex (31,-41).
//! - Wake, row 5107 (19.299, 166.624), is in hex (2,-26).
//!
//! Each of the two hexes is coast with land percentage 0, has one body of
//! water, and is named in the header's "Marked land" line. Neither is counted
//! among the hexes that have a land point and 0 percent.
//!
//! ## The counts by kind (the tool's own, not worked by hand)
//!
//! The sampling's result cannot be worked by hand. The architect's own run
//! of the tool on this frame without marks or anchors gave 344 land, 363
//! coast and 9,688 deep sea. With the two marks that is 344 land, 365 coast
//! and 9,686 deep sea; the anchors change which water a hex stands for and
//! never its kind. The test holds the map to those three numbers. If the
//! tool gives others, that is reported, not mended here.
//!
//! ## The file of located positions
//!
//! `data/map/papua_to_midway_located.csv` has one line for every row of
//! `data/oob/place_positions.csv` whose position is inside the extent, in the
//! order of the rows' ids: `row,place,lat,lon,q,r`, with `lat` and `lon` as
//! the row gives them and (q, r) the hex whose centre is nearest by part 1 of
//! `tests/about_a_centre.rs`. A row outside the extent has no line. On
//! Nov 28, 2026 the file of positions has 113 rows and every one is inside.
//!
//! Worked by the architect apart from the tool, each row's hex:
//!
//! 113 rows. The least margin between a row's nearest centre and its next nearest is 0.11 nm.
//! 1 rabaul (-39,14); 2 rabaul (-39,14); 3 rabaul (-39,13)
//! 4 tulagi (-32,22); 5 tulagi (-32,22); 6 lunga-point (-32,22)
//! 7 lunga-point (-32,22); 8 henderson-field (-32,22); 9 henderson-field (-32,22)
//! 10 savo-island (-32,22); 11 savo-island (-32,22); 12 espiritu-santo (-27,33)
//! 13 espiritu-santo (-27,33); 14 espiritu-santo (-27,33); 15 espiritu-santo (-27,32)
//! 16 noumea (-33,44); 17 noumea (-33,44); 18 kavieng (-40,11)
//! 19 kavieng (-40,11); 20 buka (-37,16); 21 buka (-37,16)
//! 22 buin (-36,18); 23 buin (-36,18); 24 shortland-harbor (-36,18)
//! 25 shortland-harbor (-36,18); 26 rekata-bay (-32,19); 27 rekata-bay (-32,19)
//! 28 gizo (-35,20); 29 gizo (-35,20); 30 munda (-35,21)
//! 31 munda (-35,21); 32 cape-esperance (-32,22); 33 cape-esperance (-32,22)
//! 34 ndeni-graciosa-bay (-25,25); 35 ndeni-graciosa-bay (-25,25); 36 efate (-27,36)
//! 37 efate (-27,36); 38 efate (-27,36); 39 port-moresby (-51,23)
//! 40 port-moresby (-51,23); 41 milne-bay (-47,24); 42 milne-bay (-47,24)
//! 43 milne-bay (-46,24); 44 lae (-49,18); 45 buna (-48,21)
//! 46 truk (-29,-7); 47 truk (-30,-6); 48 henderson-field (-32,22)
//! 49 rabaul-vunakanau (-39,14); 50 rabaul-lakunai (-39,14); 51 tassafaronga (-32,22)
//! 52 taivu-point (-31,22); 53 tasimboko (-32,23); 54 koli-point (-32,23)
//! 55 tenaru-river (-32,22); 56 ilu-river (-32,22); 57 bloody-ridge (-32,22)
//! 58 matanikau-river (-32,22); 59 point-cruz (-32,22); 60 kokumbona (-32,22)
//! 61 aola-bay (-32,23); 62 kamimbo (-32,22); 63 haleta (-32,22)
//! 64 halavo (-32,22); 65 metapona-river (-32,23); 66 kokumbona (-32,22)
//! 67 kukum (-32,22); 68 gavutu (-32,22); 69 tetere (-32,23)
//! 70 koli-point (-32,22); 71 tulagi (-32,22); 72 taivu-point (-31,22)
//! 73 kamimbo (-32,22); 74 gavutu (-32,22); 75 tanambogo (-32,22)
//! 76 kukum (-32,22); 77 halavo (-32,22); 78 haleta (-32,22)
//! 79 tetere (-32,23); 80 metapona-river (-32,23); 81 maravovo (-32,22)
//! 82 maravovo (-32,22); 83 visale (-32,22); 84 koilotumaria (-32,23)
//! 85 gurabusu (-32,23); 86 kokumbona (-32,22); 87 tassafaronga (-32,22)
//! 88 taivu-point (-31,22); 89 tasimboko (-32,23); 90 red-beach (-32,22)
//! 91 tenaru-river (-32,22); 92 lunga-point (-32,22); 93 point-cruz (-32,22)
//! 94 aola-bay (-32,23); 95 deboyne (-44,25); 96 misima (-44,25)
//! 97 rossel-island (-42,26); 98 townsville (-59,40); 99 point-zed (-41,30)
//! 5100 midway (31,-41); 5101 midway (31,-41); 5102 midway-sand-island (31,-41)
//! 5103 midway-sand-island (31,-41); 5104 midway-eastern-island (31,-41); 5105 midway-eastern-island (31,-41)
//! 5106 kure-atoll (30,-41); 5107 wake (2,-26); 5108 saipan (-30,-20)
//! 5109 guam (-33,-18); 5110 french-frigate-shoals (43,-34); 5111 laysan (37,-37)
//! 5112 lisianski (34,-37); 5113 point-luck (39,-47)
//!
//! Two of these are land hexes on the map (row 44, Lae town; row 98,
//! Townsville city). That is decision 0242's rule 9 and is not mended here:
//! the file says where a position falls, not where a port is.
