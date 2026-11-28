// Statement 18: rung 3, the Allied seat: the reads of a page's load, then the
// order to take base 2 with task force 1, land unit 1 and base 1 as its forces
// and the risk press, previewed and then signed, leave to launch given, then
// `run` until twelve days are resolved, then the supply order for base 2 at a
// floor of 2 days and an interval of 5, with ship 4 and base 1 as its forces,
// previewed and then signed, then `run` to the end with whatever stops it
// answered by `run` again, then `events` of each day from 1 to 21 and
// `list_bases` as the verdict asks them.
import { theReadsOfAPageLoad } from '../support.mjs';

export default {
  scenario: 'data/scenarios/ladder/140_men_then_food.toml',
  seat: 'allied',
  *steps() {
    yield* theReadsOfAPageLoad();
    const take = yield ['preview', {
      command: 'set_intent', intent_kind: 'take_place', base: 2,
      limits: [
        { limit: 'forces', task_forces: [1], ships: [], land_units: [1], source: 1 },
        { limit: 'risk', risk: 'press' },
      ],
    }];
    yield ['approve', { preview: take.ok?.preview }];
    yield ['set_leave', { leave: 'launch', given: true }];
    let resolved = 0;
    for (let i = 0; i < 100 && resolved < 12; i++) {
      const answer = yield ['run', { days: 12 - resolved }];
      if (answer.ok === undefined || answer.ok.game_over) break;
      resolved = answer.ok.days_resolved.last;
    }
    const supply = yield ['preview', {
      command: 'set_intent', intent_kind: 'keep_supplied', base: 2, floor_days: 2, interval_days: 5,
      limits: [{ limit: 'forces', ships: [4], task_forces: [], source: 1 }],
    }];
    yield ['approve', { preview: supply.ok?.preview }];
    for (let i = 0; i < 100; i++) {
      const answer = yield ['run', { days: 21 }];
      if (answer.ok === undefined || answer.ok.game_over) break;
    }
    for (let day = 1; day <= 21; day++) yield ['events', { day }];
    yield ['list_bases', {}];
  },
};
