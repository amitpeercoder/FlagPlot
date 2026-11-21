//! Scenario 120: real oilers. The American fleet oiler of 1942 is a class
//! derived from fact rows, and the oilers the record names are in the
//! Coral Sea and in the five days of the Eastern Solomons.
//!
//! Proves decision 0206. The statements are the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The figures of the class, worked from the rows
//!
//! The Navy's factor is 0.1516 long tons to a barrel of fuel oil
//! (ship_class_facts 14311). A ton of the simulation is a long ton.
//!
//! 1. **Neosho and Platte** (the class as her sisters have it):
//!    - kind auxiliary, and the mark that she fuels others at sea;
//!    - toughness: light displacement 7,256 tons over 25, 290.24;
//!    - maximum speed 18 knots; cruising speed 16.5 knots;
//!    - her own fuel: 15,000 barrels times 0.1516, 2,274 tons;
//!    - endurance 12,000 nautical miles, so 0.1895 tons a mile at her
//!      cruising speed;
//!    - cargo: 101,638 barrels times 0.1516, 15,408.3208 tons;
//!    - no main battery and no torpedo tubes.
//! 2. **Cimarron,** a class of her own: the same but for her own fuel,
//!    14,850 barrels, 2,251.26 tons, and her cargo, 103,233 barrels,
//!    15,650.1228 tons.
//! 3. **Their anti-aircraft figure** is what the list's rule gives each
//!    fit: Neosho, three 3-inch/50 and eight 20 mm with one 5-inch/51;
//!    Cimarron, four 5-inch/38 and eight 20 mm; Platte, the lesser of her
//!    two lines of 1942. The build reports the three figures and the rows
//!    each rests on, and the architect checks them: no figure is set here.
//! 4. **Sims** comes out with every figure her sisters Anderson, Hammann,
//!    Russell and Morris have in the Coral Sea's own class file, and with
//!    no mark.
//! 5. **Every figure names its rows.** The sources file of the new class
//!    file gives, for each figure of each class, the rows it was read
//!    from and the exception or rule that read them.
//! 6. **The lists on file are as they were.** Each derivation list on
//!    file passes its check, and the class files, ships files and sides
//!    files that scenarios on file read are byte for byte what they were.
//!
//! # Statements: the Coral Sea with her oiler
//!
//! The new Coral Sea file is the Coral Sea's own with one force more.
//!
//! 7. **The fuelling group.** Neosho and Sims are one force of the Allied
//!    side, of the mission transport, with no home port, in the hex of
//!    Task Force 17, (-3,5), at the start. Neosho has 2,274 tons of her
//!    own fuel and 15,408.3208 tons of cargo fuel. Sims is full, as her
//!    sisters are, with 453 tons. No order is given to the force by the
//!    scenario.
//! 8. **A destroyer fuels.** With Anderson set to 153 tons, 300 short of
//!    her 453, Task Force 17 lying still, and Task Force 17 given the
//!    order to fuel from Neosho's force before the first night: in that
//!    night Anderson is alongside for one hour and then takes 300 tons on
//!    one line at 250 tons an hour. She has 453 tons, Neosho's cargo is
//!    15,108.3208 tons, and the order ends because every ship is full.
//! 9. **The staff sends a short force to her.** A quarter of Anderson's
//!    453 tons is 113.25. With Anderson at 100 tons and Task Force 17
//!    under a task of the staff's, the staff gives Task Force 17 the
//!    order to fuel from Neosho's force on the first morning, and its
//!    reason names Anderson and Neosho's force. With Anderson at 120 tons
//!    it does not.
//! 10. **She is no lift.** With an order of the Allied commander's to
//!     take a place and no forces named, Neosho's force is not counted
//!     among the forces that could carry the men, and the staff's reason
//!     does not name it.
//! 11. **Sunk, she scores as an auxiliary,** by the points the Coral
//!     Sea's file already gives that kind.
//!
//! # Statements: the five days with their oilers
//!
//! 12. **The fuelling group.** Platte and Cimarron are one force of the
//!     Allied side, of the mission transport, with no home port, stopped
//!     in the hex of Efate, (7,10), at the start. Platte has 2,274 tons
//!     of her own fuel and 15,408.3208 of cargo; Cimarron 2,251.26 and
//!     15,650.1228. No order is given to the force by the scenario.
//! 13. **The meeting place is reachable.** From (7,10) to the meeting
//!     place of 25 August, hex (5,2), is a passage by sea on the map, and
//!     the force has the fuel for it many times over.
//!
//! # Properties
//!
//! - **No game on file moves.** Every scenario and recorded game on file
//!   gives the events and the daily hashes it gave before.
//! - **Nothing is invented.** No figure of an oiler's class is written by
//!   hand: each is derived, and the derivation's check passes on the new
//!   lists.
//! - **The table's new files** differ from the files they stand beside in
//!   the ships added, the class file and the sides file named, and
//!   nothing else.

// The architect's statement lists are list items in the doc comment above,
// which is left as written.
#![allow(clippy::doc_lazy_continuation)]
