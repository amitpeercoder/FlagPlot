# The player's account

A Sonnet model, playing alone on Nov 12, 2026 with `sim-cli play` and nothing else, told to command the Allied side for the first five days of the fourteen. Its report as it gave it; only the game file's path is left out.

I resolved days 1 to 5 and read the day-6 briefing. I stopped there.

## Understanding

- The briefing heading says "Day 1 of 14", not 5, though I was told to stop after day 5. Mission: land the convoy at Henderson Field (base:1), keep it flying, win the carrier fight.
- Scoring: enemy ships sunk (carrier 10 and so on), ships crippled, enemy aircraft 0.05 each, tons unloaded at Henderson 0.005 each, and Henderson open at the end (10).
- I held TF16 (Enterprise) and TF11 (Saratoga), both at (7,-5), and Henderson air groups. I also held a supply convoy (TF5, 3 ships) at Espiritu Santo, plus several "Placeholder" task forces in the south that I left alone.
- I knew no enemy position at the start. The only enemy base the tools showed was Rabaul, 556 miles from Henderson. My dive bombers' strike radius is 200, so I could not strike Rabaul.

## Day 1

- TF5 ordered to load supply.
- Both carrier scout groups (VS-5, VS-3) searched; VS-5 got the full circle at 200 miles, VS-3 the northern half.
- Fighters on combat air patrol (carriers and Henderson).
- Dive and torpedo bombers on naval_strike so the staff launches them when a contact appears.
- Refused: my first VS-5 search range of 250 ("at most the aircraft's 200 nm"). The refusal named the maximum, so I fixed it at once.
- Result: no contacts. The Japanese bombed Henderson with 41 aircraft, and I took losses mostly to patrol and wear. Espiritu supply dropped from 960 to 0, which showed the load worked.

## Day 2

- Convoy to Henderson at full speed (measure showed arrival D3 23:20 at full, versus D3 10:00 at cruise measured with the same engine rule; the "ends" clock for cruise looked inconsistent, see below).
- I queued unload at the same time, to see if it was accepted at sea; it was accepted.
- Scouts rotated.
- Result: another raid on Henderson, 30 aircraft. My claims: 5 shot down.

## Day 3

- I issued no orders, because my fighters and Henderson's anti-aircraft fire were doing the work.
- Another raid of 21 aircraft, 3 hits.

## Day 4

- Convoy reached Henderson. I re-queued unload.
- The Saratoga scouts were at fatigue 44, so I rested them and put VS-5 on search.
- A contact appeared and my staff launched VB-6 and VB-3 from the carriers automatically. Japanese strike groups (air_group:111, 112, 114, 115) hit Enterprise with 15 and 14 aircraft.
- Enterprise took 8 hits: propulsion out and flight deck out. TF16 cannot move.
- I lost about 10 aircraft that day.
- I claimed 1 and 3 hits on a "carrier". The briefing says 0 sunk, and the contact was later re-classed as a "cruiser" (5 ships).
- The convoy unloaded 746.7 t.

## Day 5

- Contact:1 sat at (7,-10), 5 ships. Its distance was 200 miles, and the torpedo bombers' radius is 190, so only the dive bombers could reach it. I moved TF11 two hexes north to (7,-7) so all its groups are in range.
- Both carrier forces set to battle readiness.
- Convoy ordered back to Espiritu Santo.
- The "load fuel" order was REFUSED at the start of the day: "the task force is under way". I had queued it while the force was still at Henderson. The reason was clear, but I could not know that load only works when stopped. I did not fix it, as the game ended for me.
- Result: a quiet day. No contacts, no fights.

## State at the end of day 5 (day-6 briefing)

- Henderson holds 4444 t supply and fuel dropped from 400 to 135.68 t. I have not understood why fuel fell (maybe the aircraft burn it). It is still open.
- Convoy is underway, at (7,-3), heading back to Espiritu Santo, with no fuel to carry.
- Enterprise is crippled but afloat and cannot move. Saratoga's group is intact.
- I claim 42 enemy aircraft down (points 2.1) and 746.7 t unloaded (3.7 points). No enemy ship is claimed sunk. I believed I had held Henderson and landed supply, but had lost the first carrier exchange.

## Confusing or missing

1. The day count: "Day 1 of 14" against the instructions' five days. Day 1 begins in the night pulse; I took the briefing's "Day 6" as proof day 5 had resolved.
2. A mission is standing. I could not see a contact and then order a strike. The staff launched without my say, which is stated in the tool text and was fine, but I had no way to choose targets or hold strikes back. I could not hold Enterprise's strikers back to defend against the raid.
3. No way to see which enemy aircraft or carriers I faced. The contact was first "carrier", then "cruiser" with 5 ships, and I never knew what I was striking.
4. Cargo aboard is not shown in the briefing or list_forces. I deduced it from the Espiritu supply change and from ship:37 supply_t 480. The unload tonnage appeared in the OBJECTIVES.
5. The measure passage for the same route said arrival "D3 10:00" at cruise and "D3 23:20" at full, which is more or less plausible, but "leaves: D1 18:00" labelled "night" confused me.
6. I cannot tell whether a load or unload order at sea is a no-op. Unload was accepted at sea and then worked on arrival; load was refused.
7. Torpedo bomber radius (190) differs from the dive bombers' (200). I found it only by measuring air_reach with each group.
8. No tool shows enemy air strength or raids' origin, other than "air_group:1" and similar numbers in events.
9. The Henderson fuel fall of 264 t in one day was not explained anywhere.

## Refusals

- VS-5 search 250 nm: refused, and the reason told me the maximum (200).
- Load fuel on day 5: refused because the force was under way. The reason did not tell me what to do instead (it needs to be in port).

## Rules

I kept to them. I used only "target/debug/sim-cli play" commands, with grep and sed filters on its output, and a shell function to call it. I used one `mkdir -p` for the game file's folder. I read no repository file, did not look at the scenario, source, docs or the game file, ran nothing else, built nothing, used no web, and used no export options. Every issue_orders call carried a reason.
