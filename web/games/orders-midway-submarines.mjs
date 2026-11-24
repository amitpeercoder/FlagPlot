// Statement 8: Midway, the Japanese seat: a patrol ordered for the submarines
// and a direct order to one boat, then every day ended.
import { theReadsOfAPageLoad, toTheEnd } from '../support.mjs';

export default {
  scenario: 'data/scenarios/table-boats/01c_midway_submarines.toml',
  seat: 'japanese',
  *steps() {
    yield* theReadsOfAPageLoad();
    const preview = yield ['preview', { command: 'set_intent', intent_kind: 'patrol', hex: [5, 0], distance: 2 }];
    yield ['approve', { preview: preview.ok?.preview }];
    yield ['issue_orders', { orders: [{ unit: 'submarine:1', order: { kind: 'station', hex: [7, 0] } }] }];
    yield ['queued', {}];
    yield* toTheEnd();
  },
};
