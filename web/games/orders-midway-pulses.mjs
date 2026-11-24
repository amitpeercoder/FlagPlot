// Statement 9: Midway, the Allied seat, whose file lets a seat stop inside a
// day: `advance` a pulse at a time, an order to a task force given at 0600 and
// one at 1200, an order that acts only at dusk given at 0600, and a dry run of
// an order to the staff at the smallest size.
import { theReadsOfAPageLoad, toTheEnd } from '../support.mjs';

export default {
  scenario: 'data/scenarios/table-boats/01b_midway_submarines.toml',
  seat: 'allied',
  *steps() {
    yield* theReadsOfAPageLoad();
    yield ['advance', { pulses: 1 }]; // dusk to 0600
    yield ['issue_orders', { orders: [{ unit: 'task_force:1', order: { kind: 'set_speed', speed: 'full' } }] }];
    // An order that acts only at dusk, given at 0600.
    yield ['issue_orders', { orders: [{ unit: 'task_force:1', order: { kind: 'set_mission', mission: 'surface_combat' } }] }];
    yield ['queued', {}];
    yield ['advance', { pulses: 1 }]; // 0600 to 1200
    yield ['issue_orders', { orders: [{ unit: 'task_force:1', order: { kind: 'set_route', waypoints: [[5, 2]] } }] }];
    const preview = yield ['preview', { command: 'set_intent', intent_kind: 'patrol', hex: [5, 0], distance: 2 }];
    yield ['trial', { preview: preview.ok?.preview, size: 'small' }];
    yield ['advance', { pulses: 1 }]; // 1200 to dusk of day 2
    yield* toTheEnd();
  },
};
