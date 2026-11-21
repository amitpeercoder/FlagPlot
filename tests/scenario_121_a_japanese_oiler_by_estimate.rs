//! Scenario 121: a Japanese oiler by estimate. Toho Maru is derived with
//! one attribute estimated from like ships and one read as her sister's,
//! each marked, and sails with the Striking Force at the Coral Sea.
//!
//! Proves decision 0207. The statements are the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The figures of Toho Maru, worked from the rows
//!
//! A ton of the simulation is a long ton. A division is cut at the sixth
//! place, as the derivation cuts.
//!
//! 1. **Read from her own rows and her class's:** kind auxiliary, and the
//!    mark that she fuels others at sea; maximum speed 19 knots; cruising
//!    speed 16 knots; her own fuel 1,100 tons; endurance 15,000 nautical
//!    miles, so 0.073333 tons a mile at her cruising speed; no main
//!    battery and no torpedo tubes, by her kind.
//! 2. **Her cargo, as her sister's:** Toa Maru's 12,487 tons. Toho Maru
//!    has no cargo row of her own, and Toa Maru is the one ship of her
//!    class that has.
//! 3. **Her light displacement, by ratio.** Her deadweight is 13,431
//!    tons. The like classes, every class of the type oiler on file with
//!    a light displacement and a deadweight, are two:
//!    - the Cimarron class, 7,256 over 18,250, 0.397589;
//!    - the Patoka class, 5,375 over 12,600, 0.426587.
//!    Their mean is 0.412088. Times 13,431 it is 5,534.753928 tons.
//! 4. **Her toughness** is that over 25: 221.390157.
//! 5. **The least and the greatest** the like classes give are
//!    5,340.018 tons (the Cimarron class's ratio) and 5,729.489 tons (the
//!    Patoka class's), and the check prints them.
//! 6. **Her guns are not estimated.** No source gives them. Her
//!    anti-aircraft figure is 0 by the list's assumption that guns
//!    unknown means none, said as an assumption of the scenario; her
//!    radar is 0 by the same kind of assumption.
//!
//! # Statements: the marks
//!
//! 7. **The sources file says "estimated"** for her toughness, with the
//!    method, her deadweight row, each like class and its rows, and the
//!    least and the greatest; and for her cargo, with the method and Toa
//!    Maru's row. It prints no rule of a figure read beside either.
//! 8. **The class file games load names them:** its column of estimated
//!    figures has toughness and the cargo capacity for her class, and is
//!    empty for a class with none. No class file on file has the column.
//! 9. **The check counts them:** it prints, for her list, two attributes
//!    estimated, for Toho Maru.
//! 10. **A fact drives the estimate out.** With a row of any standing
//!     filed for Toho Maru's light displacement, or for her class's, her
//!     list's check fails and names the entry to take out.
//! 11. **An estimate does not rest on an estimate.** A second list that
//!     names Toho Maru's class as a like class for the light displacement
//!     fails its check.
//! 12. **Never evidence.** A measure of the validation that names a
//!     scenario which loads her class, and whose notes do not name the
//!     class as estimated, is refused by a test; with the note it passes.
//!     A scenario file that loads her class and does not say so in its
//!     header is refused by a test.
//! 13. **Neosho's cargo is marked too.** In decision 0206's Coral Sea
//!     list her cargo, the least of her sisters', is an entry by the
//!     second method and is marked estimated; its value, 15,408.3208
//!     tons, is what it was.
//!
//! # Statements: the Coral Sea with both oilers
//!
//! The new Coral Sea file is decision 0206's, with one force more.
//!
//! 14. **The Japanese fuelling group.** Toho Maru is a force of the
//!     Japanese side by herself, of the mission transport, with no home
//!     port, in the hex of the Striking Force, (4,-2), at the start. She
//!     has 1,100 tons of her own fuel and 12,487 tons of cargo fuel, full,
//!     as every ship in that file starts full. No order is given to her
//!     by the scenario. The file's header says her light displacement and
//!     her cargo are estimated, and that her load on 5 May is not known.
//! 15. **The staff sends a short force to her.** With a destroyer of the
//!     Striking Force below a quarter of her fuel and the Striking Force
//!     under a task of the Japanese staff's, the staff gives the Striking
//!     Force the order to fuel from Toho Maru's force on the first
//!     morning, as it does for the Allied side with Neosho.
//! 16. **A destroyer fuels.** With that destroyer 300 tons short of full,
//!     the Striking Force lying still and under the order to fuel from
//!     Toho Maru's force before the first night: in that night she takes
//!     300 tons, Toho Maru's cargo is 12,187 tons, and the order ends
//!     because every ship is full.
//!
//! # Properties
//!
//! - **No game on file moves.** Every scenario and recorded game on file
//!   gives the events and the daily hashes it gave before; no class file,
//!   ships file or sides file a scenario on file reads is changed but
//!   decision 0206's Coral Sea oilers' class file, which gains the mark
//!   for Neosho's cargo and no change of any figure.
//! - **Nothing is typed.** No figure of Toho Maru's class is written by
//!   hand: each is read or estimated by the derivation, and its check
//!   passes on her list.
//! - **The table's copies** are made from the new Coral Sea files, and its
//!   line for the Coral Sea says each side has an oiler.

// The architect's statement lists are list items in the doc comment above,
// which is left as written.
#![allow(clippy::doc_lazy_continuation)]
