//! A scenario built on another (decision 0234): the architect's statements,
//! written on Nov 27, 2026, before any code. No number here is a figure of
//! the game. The rule is decision 0234's "The format as planned", cited
//! below by its item, and by its rulebook section once step B has moved it.
//!
//! This file holds no test until step B, which also names it as the
//! package names its test files that day. The statements L are of the loader and are tested in
//! `sim-data` on invented files held in a `Pack`; the statements R are of
//! the repository's files and are tested here.
//!
//! The loader
//!
//! L1.  A file with no `built_on` loads as before: the same scenario, the
//!      same `data_files`, and for a faulty file the same error text, line
//!      and column included (item 1).
//! L2.  A plain key the overlay states takes the place of the value below; a
//!      plain key it does not state is the one below. `ship_classes` stated
//!      as one name over a list below gives one name. `[side.allied]` stated
//!      over `[side.allied]` and `[side.japanese]` below gives a side table
//!      with no Japanese part (item 4).
//! L3.  In a list by id, a row whose id is below changes the keys it states
//!      and no other, and keeps its place: over ships 1, 2, 3 an overlay's
//!      row for ship 2 stating `fuel_t` gives ships 1, 2, 3 in that order,
//!      ship 2 with the new `fuel_t` and every other key as below (item 5).
//! L4.  A row with a new id comes after the rows below, in the overlay's
//!      order: over ships 1, 2 an overlay's rows 9 then 4 give 1, 2, 9, 4
//!      (item 5).
//! L5.  A row of a list by id with no `id`, and two rows of one id in the
//!      overlay, are each refused with an error that names the overlay, the
//!      list and, for the second, the id (item 5).
//! L6.  In any other list the overlay's rows come after the rows below: over
//!      commands A, B an overlay's C, D give A, B, C, D (item 6).
//! L7.  An overlay in another folder reads the files named below where the
//!      file below reads them, and names them with the folder pair struck
//!      out: an overlay in `x/` built on `../y/s.toml`, whose `map` is
//!      `../m/a.toml`, reads `../m/a.toml` from `x/`, and its `data_files`
//!      are, name for name, those of a copy of `s.toml` written out in full
//!      in `x/` (item 8). A file key the overlay states itself is used as
//!      written.
//! L8.  A chain of three loads as the three laid in order, lowest first
//!      (item 2).
//! L9.  A file built on itself, a chain of two that name each other, a
//!      chain of nine files, and a `built_on` that names no file are each
//!      refused; the error names the file asked for and the file that could
//!      not be read or was read twice (item 2).
//! L10. `built_on` given as a list, or as anything but one string, is
//!      refused (item 2).
//! L11. A key no scenario has, stated in an overlay or in the file below,
//!      is refused, and so is a float in either (item 3).
//! L12. Loaded with a reference root, an overlay reads the file it is built
//!      on where it names it, and its reference files from the root, as a
//!      file written out in full does (item 9).
//! L13. `data_files` of an overlay are those of the same scenario written
//!      out in full, then the files it is built on, nearest first. The pack
//!      of an overlay holds the overlay and each of them, and the overlay
//!      loads from that pack alone (item 11).
//! L14. `resolve` of a file with no `built_on` parses back to that file's
//!      table; `resolve` of an overlay, written to a file in the overlay's
//!      folder, loads to a scenario with the same hash at day 0, the same
//!      script and the same commands (item 12).
//! L15. `estimated_classes` of an overlay that states no `ship_classes`
//!      gives the classes of the same scenario written out in full. The
//!      check of decision 0207's rule 12 passes where the mark's words are
//!      in the header of the overlay or of any file it is built on, and
//!      fails where they are in none.
//! L16. Text that states `built_on`, given to `from_text` with a path, loads
//!      as a file of that text at that path. The resolved text of an
//!      overlay states no `built_on`, and given to `from_text` with the
//!      overlay's path loads to a scenario with the same hash at day 0,
//!      the same script and the same commands as the overlay (items 12
//!      and 13).
//!
//! The repository
//!
//! R1.  Every scenario file under `data/scenarios`, in every folder, that
//!      states `built_on` names a file under `data/scenarios` that is there,
//!      and that file loads alone (items 1 and 10).
//! R2.  No file of `step2-frozen`, `orders-inside-proof`, `second-proof`,
//!      `stage-2b-proof`, `stage-3-proof`, `stage-4-proof` or `staff`, and
//!      no scenario a recorded game under
//!      `crates/sim-cli/tests/recorded_games` names, states `built_on`
//!      (item 14).
//! R3.  Each converted file states `built_on`, the file decision 0234's
//!      table "Which files convert" gives it, and beside it only the keys,
//!      lists and ids that table gives its row. From step C for the 27
//!      files of `coral-sea`, `five-days` and `midway`; from step D for the
//!      31 of `table-hunger`, `table-boats` and `table-rear-bases`. The
//!      table is copied into this test as data when the step is built; a
//!      test reads no Markdown.
//! R4.  The three folders of the table's games hold the converted files and
//!      nothing else: `table-hunger` 12, `table-boats` 16,
//!      `table-rear-bases` 3 (from step D).
//! R5.  The words "Do not edit by hand" are in eight scenario files and no
//!      other: the four of `orders-inside-proof`, the three of
//!      `stage-3-proof` and `staff/07_full_slice.toml` (from step D).
