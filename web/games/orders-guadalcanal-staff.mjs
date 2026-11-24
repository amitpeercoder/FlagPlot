// Statement 7: the 39 days, the Allied seat: an order to the staff to hold
// Lunga and one to control Guadalcanal, each previewed and then signed, a
// force held and released, a limit set and a standing interrupt switched off,
// then `run` to the end with whatever stops it answered by `run` again.
// `cut` names the answer after which the check cuts the game at about day 20
// to take it up in the other build (statement 11).
import { theReadsOfAPageLoad } from '../support.mjs';

const days = (answer) => answer.ok?.days_resolved?.last ?? 0;

export default {
  scenario: 'data/scenarios/table-rear-bases/07b_slice_submarines.toml',
  seat: 'allied',
  cut: (tool, answer) => tool === 'run' && days(answer) >= 20,
  *steps() {
    yield* theReadsOfAPageLoad();
    for (const intent of [
      { command: 'set_intent', intent_kind: 'hold', base: 1 },
      { command: 'set_intent', intent_kind: 'control_island', island: 1 },
    ]) {
      const preview = yield ['preview', intent];
      yield ['approve', { preview: preview.ok?.preview }];
    }
    yield ['hold', { task_force: 1 }];
    yield ['set_limit', { limit: 'priority', intent: 1, rank: 1 }];
    yield ['set_interrupt', { standing: 'cannot_meet', on: false }];
    yield ['queued', {}];
    let released = false;
    for (let i = 0; i < 100; i++) {
      const answer = yield ['run', { days: 5, reason: 'a check of the browser build' }];
      if (answer.ok === undefined || answer.ok.game_over) break;
      if (!released) {
        released = true;
        yield ['release', { task_force: 1 }];
      }
      yield ['command_state', {}];
    }
    yield ['get_briefing', {}];
  },
};
