// Statement 6: Eastern Solomons, the Allied seat: orders to a task force and
// to an air group on the first evening (a course, a speed, a mission, a
// search arc), then the day ended to the last. The calls are those
// `table/check_screen.py` makes; no expected value is written here.
import { theReadsOfAPageLoad, toTheEnd } from '../support.mjs';

export default {
  scenario: 'data/scenarios/second-proof/05c_eastern_solomons.toml',
  seat: 'allied',
  *steps() {
    yield* theReadsOfAPageLoad();
    yield ['issue_orders', { orders: [
      { unit: 'task_force:1', order: { kind: 'set_route', waypoints: [[6, -4], [5, -3]] } },
      { unit: 'task_force:1', order: { kind: 'set_speed', speed: 'full' } },
      { unit: 'task_force:1', order: { kind: 'set_mission', mission: 'carrier' } },
      { unit: 'air_group:2', order: { kind: 'set_air_mission', mission: 'search' } },
      { unit: 'air_group:2', order: { kind: 'set_search_arc', from_deg: 0, to_deg: 90, range_nm: '200' } },
      { unit: 'air_group:1', order: { kind: 'set_air_mission', mission: 'combat_air_patrol' } },
    ], reason: 'a check of the browser build' }];
    yield ['queued', {}];
    yield* toTheEnd();
  },
};
