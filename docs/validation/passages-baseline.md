# Passages: validation of `baseline`

Passages: data/validation/passages.toml (hash dbe457cf8ac7eab6), 6 passages
Map: data/map/south_pacific.csv (hash ba6cb3aea5cc45f5)
Place positions: data/oob/place_positions.csv (hash d0a753225c40c296)
Label: `baseline`

No passage is judged. Each is one calculation, worked from the map and the rules' charge for a sea route (decision 0058), and reported beside the record. A place is at the centre of the hex its position falls in.

| Passage | From | To | Straight line (nm) | Charged (nm) | Short by (nm) | Miles counted | Speed | Calculated (min) | Record (min) | Calculated less record (min) | |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| japanese_column_up_the_slot_after_savo_island | row 10, savo-island, (2,-5) | position -7.5371, 158.2126, (1,-8) | 144 | 144 | - | 144 | 30 knots | 288 | 270 to 280 | +8 | reported |
| japanese_6th_cruiser_division_shortlands_to_savo_island | row 24, shortland-harbor, (-2,-8) | row 10, savo-island, (2,-5) | 243 | 243 | 10 | 233 | 24 knots; 30 knots from minute 240 | 514 | 573 | -59 | reported |
| mcfarland_espiritu_santo_to_lunga_point | row 12, espiritu-santo, (7,6) | row 6, lunga-point, (2,-4) | 529 | 629 | - | 629 | 17 knots | 2220 | 2775 | -555 | reported |
| minesweepers_towing_espiritu_santo_to_tulagi | row 12, espiritu-santo, (7,6) | row 4, tulagi, (3,-5) | 538 | 538 | - | 538 | 18 knots | 1794 | 2303 | -509 | reported |
| japanese_2nd_destroyer_squadron_noon_27_to_noon_28_october | position -6.2417, 165.8833, (14,-10) | position -1.3917, 161.7833, (12,-18) | 367 | 367 | - | 367 | 18 knots; 14 knots from minute 580; 18 knots from minute 1260 | 1375 | 1440 | -65 | reported |
| japanese_4th_destroyer_squadron_tassafaronga_to_the_shortlands | row 51, tassafaronga, (2,-5) | row 24, shortland-harbor, (-2,-8) | 243 | 243 | - | 243 | 28 knots | 521 | 650 | -129 | reported |

### japanese_column_up_the_slot_after_savo_island

- Counts: Minutes to steam from Savo Island to the point 135 miles from it on bearing 315, at 30 knots
- Rows: 236, 238, 245, 248

### japanese_6th_cruiser_division_shortlands_to_savo_island

- Counts: Minutes to steam from the Shortland Islands to the point 10 miles short of Savo Island, at 24 knots for four hours and 30 knots after
- Rows: 11

### mcfarland_espiritu_santo_to_lunga_point

- Counts: Minutes to steam from Espiritu Santo to Lunga Point at 17 knots
- Rows: 70

### minesweepers_towing_espiritu_santo_to_tulagi

- Counts: Minutes to steam from Espiritu Santo to Tulagi at 18 knots
- Rows: 71

### japanese_2nd_destroyer_squadron_noon_27_to_noon_28_october

- Counts: Minutes to steam from 6-14.5 S 165-53 E to 1-23.5 S 161-47 E, at 18 knots, 14 knots from 21:40 while refuelling, and 18 knots again from 09:00
- Rows: 100

### japanese_4th_destroyer_squadron_tassafaronga_to_the_shortlands

- Counts: Minutes to steam from Tassafaronga to Shortland Harbour at 28 knots
- Rows: 137

No passage is judged: these are reported to show the size of the difference between the record and the rules, which hold a force to its ordered speed on a straight course.
