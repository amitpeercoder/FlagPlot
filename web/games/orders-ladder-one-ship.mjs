// Statement 17: rung 1, the Allied seat: the reads of a page's load, then the
// supply order for base 2 at a floor of 2 days and an interval of 5, with ship 1
// and base 1 as its forces, previewed and then signed, leave to launch given,
// then `run` to the end with whatever stops it answered by `run` again, then
// `events` of each day from 1 to 14 as the verdict asks them.
import { theReadsOfAPageLoad } from '../support.mjs';

export default {
  scenario: 'data/scenarios/ladder/138_one_ship.toml',
  seat: 'allied',
  *steps() {
    yield* theReadsOfAPageLoad();
    const preview = yield ['preview', {
      command: 'set_intent', intent_kind: 'keep_supplied', base: 2, floor_days: 2, interval_days: 5,
      limits: [{ limit: 'forces', ships: [1], task_forces: [], source: 1 }],
    }];
    yield ['approve', { preview: preview.ok?.preview }];
    yield ['set_leave', { leave: 'launch', given: true }];
    for (let i = 0; i < 100; i++) {
      const answer = yield ['run', { days: 14 }];
      if (answer.ok === undefined || answer.ok.game_over) break;
    }
    for (let day = 1; day <= 14; day++) yield ['events', { day }];
  },
};
