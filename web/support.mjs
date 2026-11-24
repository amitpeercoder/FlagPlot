// What the games of the check in `games/` share (decision 0229, stage 3). A game
// of the check is a module with `scenario` (the path of a scenario file in the pack), `seat`
// (`allied` or `japanese`) and `steps`, a generator function: each `yield
// [tool, arguments]` is one call of the seat, and what it gives back is the
// answer, parsed (`{ok: ..}` or `{refused: ..}`), so that a game can go on
// from what it was told, as a person does. `cut`, where a game has one, says
// after which answer the check cuts the game to take it up in the other build.
//
// No game carries an expected value: every answer is compared with the native
// build's, which `check.mjs` asks.

/** The person's seat reads the briefing and ends each day to the last. */
export function* toTheEnd() {
  for (let day = 0; day < 100; day++) {
    yield ['get_briefing', {}];
    const answer = yield ['end_day', {}];
    if (answer.ok === undefined || answer.ok.game_over) break;
  }
  yield ['get_briefing', {}];
}

/** What a seat reads when a page loads (`table/check_screen.py`, PAGE_LOAD_CALLS). */
export function* theReadsOfAPageLoad() {
  yield ['list_forces', {}];
  yield ['list_bases', {}];
  yield ['list_contacts', {}];
  yield ['command_state', {}];
  yield ['get_briefing', { text: false }];
  yield ['queued', {}];
  yield ['events', { day: 1 }];
}
