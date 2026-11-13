# Legibility test: the umpire's log

Every event of the game in `legibility-game.md`, as the umpire sees it (`sim-cli export` of the saved game, the event texts), with the order acceptances and fuel burns left out. Event numbers here are the umpire's; the side's briefing numbers its own events. Never shown to the model.

```
#41 D1 night     task_force:3 steamed 204 nm at 17 kn from (9,-19) to (8,-13)  <- order:17, order:18
#56 D1 night     task_force:4 steamed 240 nm at 20 kn from (7,-18) to (7,-12)  <- order:19, order:20
#61 D1 morning   air_group:3 searched 270-000 out to 200 nm from ship:1 at (7,-5) with 16 aircraft  <- order:6, order:5
#62 D1 morning   air_group:7 searched 000-090 out to 200 nm from ship:11 at (7,-5) with 15 aircraft  <- order:12, order:11
#63 D1 morning   air_group:10 searched 300-060 out to 200 nm from base:1 with 11 aircraft  <- order:16, order:15
#64 D1 morning   air_group:17 searched 120-180 out to 330 nm from ship:21 at (8,-13) with 5 aircraft  <- order:31, order:32
#65 D1 morning   air_group:17 searched for task_force:1 at 302 nm, chance 0.316201: not sighted  <- #64
#66 D1 morning   air_group:17 searched for task_force:2 at 302 nm, chance 0.316201: not sighted  <- #64
#67 D1 morning   task_force:3 steamed 98 nm at 17 kn from (8,-13) to (8,-11)  <- order:17, order:18
#82 D1 morning   task_force:3 arrived at (8,-11) at D1 11:46  <- #67
#83 D1 morning   task_force:4 steamed 120 nm at 20 kn from (7,-12) to (7,-9)  <- order:19, order:20
#88 D1 morning   task_force:4 arrived at (7,-9) at D1 12:00  <- #83
#89 D1 afternoon air_group:3 searched 270-000 out to 200 nm from ship:1 at (7,-5) with 16 aircraft  <- order:6, order:5
#90 D1 afternoon air_group:3 searched for task_force:4 at 160 nm, chance 1: sighted  <- #89
#91 D1 afternoon Allied report: 4 ships, carrier, at (7,-9): new contact:1  <- #90
#92 D1 afternoon air_group:7 searched 000-090 out to 200 nm from ship:11 at (7,-5) with 15 aircraft  <- order:12, order:11
#93 D1 afternoon air_group:10 searched 300-060 out to 200 nm from base:1 with 11 aircraft  <- order:16, order:15
#94 D1 afternoon air_group:10 searched for task_force:4 at 200 nm, chance 0.525211: not sighted  <- #93
#95 D1 afternoon air_group:17 searched 120-180 out to 330 nm from ship:21 at (8,-11) with 5 aircraft  <- order:31, order:32
#96 D1 afternoon air_group:17 searched for task_force:1 at 223 nm, chance 0.428219: not sighted  <- #95
#97 D1 afternoon air_group:17 searched for task_force:2 at 223 nm, chance 0.428219: sighted  <- #95
#98 D1 afternoon Japanese report: 9 ships, transport, at (7,-5): new contact:1  <- #97
#99 D1 afternoon air_group:2 launched 17 aircraft at contact:1 at (7,-9), 160 nm  <- order:4, #91
#100 D1 afternoon air_group:4 launched 7 aircraft at contact:1 at (7,-9), 160 nm  <- order:7, #91
#101 D1 afternoon air_group:6 launched 15 aircraft at contact:1 at (7,-9), 160 nm  <- order:10, #91
#102 D1 afternoon air_group:8 launched 8 aircraft at contact:1 at (7,-9), 160 nm  <- order:13, #91
#103 D1 afternoon air_group:12 launched 18 aircraft at contact:1 at (7,-5), 223 nm  <- order:23, order:24, #98
#104 D1 afternoon air_group:15 launched 9 aircraft at contact:1 at (7,-5), 223 nm  <- order:28, order:29, #98
#105 D1 afternoon air_group:19 launched 6 aircraft at base:1 at (2,-4), 200 nm  <- order:35, order:36
#106 D1 afternoon air_group:18 patrol at the bombers of air_group:2: 2 shots x 0.16875 = 0.3375 expected, 1 hit  <- #99
#107 D1 afternoon air_group:2 lost 1 aircraft, 16 left  <- #106
#108 D1 afternoon air_group:18 patrol at the bombers of air_group:6: 1 shots x 0.16875 = 0.16875 expected, 0 hits  <- #101
#109 D1 afternoon air_group:11 sent 4 escorts with air_group:12  <- order:21, order:22, #103
#110 D1 afternoon air_group:11 escorts at the patrol of air_group:1: 4 shots x 0.16875 = 0.675 expected, 1 hit  <- #103, #109
#111 D1 afternoon air_group:1 patrol at the escorts of air_group:11: 4 shots x 0.3 = 1.2 expected, 2 hits  <- #103, #109
#112 D1 afternoon air_group:1 patrol at the bombers of air_group:12: 11 shots x 0.3 = 3.3 expected, 4 hits  <- #103
#113 D1 afternoon air_group:1 lost 1 aircraft, 26 left  <- #110
#114 D1 afternoon air_group:11 lost 2 aircraft, 21 left  <- #111
#115 D1 afternoon air_group:12 lost 4 aircraft, 14 left  <- #112
#116 D1 afternoon air_group:14 sent 6 escorts with air_group:15  <- order:26, order:27, #104
#117 D1 afternoon air_group:14 escorts at the patrol of air_group:1: 6 shots x 0.16875 = 1.0125 expected, 1 hit  <- #104, #116
#118 D1 afternoon air_group:1 patrol at the escorts of air_group:14: 6 shots x 0.3 = 1.8 expected, 1 hit  <- #104, #116
#119 D1 afternoon air_group:1 patrol at the bombers of air_group:15: 6 shots x 0.3 = 1.8 expected, 2 hits  <- #104
#120 D1 afternoon air_group:1 lost 1 aircraft, 25 left  <- #117
#121 D1 afternoon air_group:14 lost 1 aircraft, 18 left  <- #118
#122 D1 afternoon air_group:15 lost 2 aircraft, 7 left  <- #119
#123 D1 afternoon air_group:18 sent 15 escorts with air_group:19  <- order:33, order:34, #105
#124 D1 afternoon air_group:18 escorts at the patrol of air_group:9: 15 shots x 0.16875 = 2.53125 expected, 2 hits  <- #105, #123
#125 D1 afternoon air_group:9 patrol at the escorts of air_group:18: 9 shots x 0.3 = 2.7 expected, 2 hits  <- #105, #123
#126 D1 afternoon air_group:9 patrol at the bombers of air_group:19: 1 shots x 0.3 = 0.3 expected, 0 hits  <- #105
#127 D1 afternoon air_group:9 lost 2 aircraft, 11 left  <- #124
#128 D1 afternoon air_group:18 lost 2 aircraft, 21 left  <- #125
#129 D1 afternoon air_group:2 bombs at ship:33: 16 shots x 0.2 = 3.2 expected, 5 hits  <- #99, #107
#130 D1 afternoon air_group:4 torpedoes at ship:33: 7 shots x 0.15 = 1.05 expected, 1 hit  <- #100
#131 D1 afternoon air_group:6 bombs at ship:33: 15 shots x 0.2 = 3 expected, 2 hits  <- #101
#132 D1 afternoon air_group:8 torpedoes at ship:33: 8 shots x 0.15 = 1.2 expected, 2 hits  <- #102
#133 D1 afternoon air_group:12 bombs at ship:1: 14 shots x 0.2 = 2.8 expected, 5 hits  <- #103, #115
#134 D1 afternoon air_group:15 bombs at ship:1: 7 shots x 0.2 = 1.4 expected, 0 hits  <- #104, #122
#135 D1 afternoon air_group:19 bombs at base:1: 6 shots x 0.2 = 1.2 expected, 0 hits: 0 on aircraft, 0 on supply, 0 on nothing  <- #105
#136 D1 afternoon air_group:18 patrol at the bombers of air_group:2: 3 shots x 0.16875 = 0.50625 expected, 0 hits after the attack  <- #99, #129, #130
#137 D1 afternoon task_force:4 guns at the raid of air_group:2: 26 shots x 0.028125 = 0.73125 expected, 0 hits after the attack  <- #99, #129, #130
#138 D1 afternoon air_group:18 patrol at the bombers of air_group:6: 2 shots x 0.16875 = 0.3375 expected, 1 hit after the attack  <- #101, #131, #132
#139 D1 afternoon air_group:6 lost 1 aircraft, 14 left  <- #138
#140 D1 afternoon task_force:4 guns at the raid of air_group:6: 26 shots x 0.028125 = 0.73125 expected, 0 hits after the attack  <- #101, #131, #132
#141 D1 afternoon air_group:1 patrol at the bombers of air_group:12: 18 shots x 0.3 = 5.4 expected, 7 hits after the attack  <- #103, #133
#142 D1 afternoon air_group:12 lost 7 aircraft, 7 left  <- #141
#143 D1 afternoon task_force:1 guns at the raid of air_group:12: 123 shots x 0.05 = 6.15 expected, 5 hits after the attack  <- #103, #133
#144 D1 afternoon air_group:12 lost 5 aircraft, 2 left  <- #143
#145 D1 afternoon air_group:1 patrol at the bombers of air_group:15: 9 shots x 0.3 = 2.7 expected, 4 hits after the attack  <- #104, #134
#146 D1 afternoon air_group:15 lost 4 aircraft, 3 left  <- #145
#147 D1 afternoon task_force:1 guns at the raid of air_group:15: 123 shots x 0.05 = 6.15 expected, 4 hits after the attack  <- #104, #134
#148 D1 afternoon air_group:15 lost 3 aircraft, 0 left  <- #147
#149 D1 afternoon air_group:9 patrol at the bombers of air_group:19: 3 shots x 0.3 = 0.9 expected, 0 hits after the attack  <- #105, #135
#150 D1 afternoon ship:33 took 4 hits on flotation, 71.320752 points, now 71.320752  <- #129
#151 D1 afternoon ship:33 took 1 hit on fire, 17.830188 points, now 17.830188  <- #129
#152 D1 afternoon ship:33 propulsion hit, working -> degraded  <- #129
#153 D1 afternoon ship:33 flight deck hit, working -> degraded  <- #129
#154 D1 afternoon ship:33 flight deck hit, degraded -> out  <- #129
#155 D1 afternoon ship:33 flight deck hit, no change, still out  <- #129
#156 D1 afternoon ship:33 propulsion hit, degraded -> out  <- #129
#157 D1 afternoon Allied report: 5 ships, carrier, at (7,-9): refreshed contact:1  <- #129
#158 D1 afternoon ship:33 took 1 hit on flotation, 17.26415 points, now 88.584902  <- #130
#159 D1 afternoon ship:33 fire control hit, no change, still working  <- #130
#160 D1 afternoon Allied report: 4 ships, carrier, at (6,-8): refreshed contact:1  <- #130
#161 D1 afternoon ship:33 took 2 hits on flotation, 35.660376 points, now 100  <- #131
#162 D1 afternoon ship:33 of task_force:4 sank  <- #161
#163 D1 afternoon air_group:18 lost with ship:33: 0 aircraft  <- #162
#164 D1 afternoon air_group:19 lost with ship:33: 0 aircraft  <- #162
#165 D1 afternoon ship:33 flight deck hit, no change, still out  <- #131
#166 D1 afternoon ship:33 flight deck hit, no change, still out  <- #131
#167 D1 afternoon Allied report: 3 ships, destroyer, at (7,-9): refreshed contact:1  <- #131
#168 D1 afternoon ship:33 took 2 hits on flotation, 34.5283 points, now 100  <- #132
#169 D1 afternoon ship:33 propulsion hit, no change, still out  <- #132
#170 D1 afternoon ship:33 propulsion hit, no change, still out  <- #132
#171 D1 afternoon Allied report: 2 ships, battleship, at (6,-8): refreshed contact:1  <- #132
#172 D1 afternoon ship:1 took 3 hits on flotation, 15.782826 points, now 15.782826  <- #133
#173 D1 afternoon ship:1 took 2 hits on fire, 10.521884 points, now 10.521884  <- #133
#174 D1 afternoon ship:1 flight deck hit, working -> degraded  <- #133
#175 D1 afternoon ship:1 main battery hit, no change, still working  <- #133
#176 D1 afternoon Japanese report: 9 ships, carrier, at (7,-4): refreshed contact:1  <- #133
#177 D1 afternoon Japanese report: 10 ships, carrier, at (7,-5): refreshed contact:1  <- #134
#178 D1 afternoon Allied claim at (7,-9): 8 hits on a carrier, 0 sunk  <- #129, #130
#179 D1 afternoon Japanese claim over (7,-9): 4 aircraft shot down  <- #106, #136, #137
#180 D1 afternoon Allied claim at (7,-9): 6 hits on a carrier, 1 sunk  <- #131, #132
#181 D1 afternoon Japanese claim over (7,-9): 3 aircraft shot down  <- #108, #138, #140
#182 D1 afternoon Japanese claim at (7,-5): 6 hits on a transport, 0 sunk, 2 aircraft shot down  <- #110, #133
#183 D1 afternoon Allied claim over (7,-5): 44 aircraft shot down  <- #111, #112, #141, #143
#184 D1 afternoon Japanese claim at (7,-5): 1 hit on a transport, 0 sunk, 5 aircraft shot down  <- #117, #134
#185 D1 afternoon Allied claim over (7,-5): 25 aircraft shot down  <- #118, #119, #145, #147
#186 D1 afternoon Japanese claim at (2,-4): 14 aircraft shot down, 1 aircraft destroyed on the ground  <- #124, #135
#187 D1 afternoon Allied claim over (2,-4): 9 aircraft shot down  <- #125, #126, #149
#188 D1 afternoon air_group:19 base strike sortie of 200 nm, 25 beyond its radius of 175: 6 aircraft x 0.285714 = 1.714284 expected out of fuel, 3 ditched, 3 ready left  <- #105
#189 D1 afternoon air_group:18 had no deck: 21 aircraft came down in the sea at (7,-9), beside ships of its own side (chance 0.95), 0 ready left  <- #162, order:33, order:34, #123
#190 D1 afternoon air_group:19 had no deck: 3 aircraft came down in the sea at (7,-9), beside ships of its own side (chance 0.95), 0 ready left  <- #162, #105
#191 D1 -         base:1 at the end of the day: 0 t of supply, 22 aircraft ready: not open
```
