#!/usr/bin/env node
'use strict';
// Structural self-check for replay files, using the viewer's own pure functions.
//   node viewer/check.js viewer/replays/*.json
// It extracts the <script id="core"> and <script id="ui"> blocks from index.html,
// syntax-checks both, evaluates the core, then checks each replay and some synthetic edge cases.

const fs = require('fs');
const path = require('path');
const vm = require('vm');

let failures = 0;
function fail(msg) { failures++; console.log('  FAIL: ' + msg); }
function assert(cond, msg) { if (!cond) fail(msg); }

// ---- Extract and syntax-check the two script blocks ----
const html = fs.readFileSync(path.join(__dirname, 'index.html'), 'utf8');
function extract(id) {
  const m = new RegExp('<script id="' + id + '">([\\s\\S]*?)</script>').exec(html);
  if (!m) { console.log('FAIL: no <script id="' + id + '"> in index.html'); process.exit(1); }
  return m[1];
}
const coreText = extract('core');
const uiText = extract('ui');
for (const [name, text] of [['core', coreText], ['ui', uiText]]) {
  try { new vm.Script(text, { filename: 'index.html#' + name }); console.log('syntax ok: <script id="' + name + '"> (' + text.length + ' chars)'); }
  catch (e) { console.log('FAIL: syntax error in ' + name + ': ' + e.message); process.exit(1); }
}
// The core must not touch the DOM or window: evaluate it with nothing but a module object.
const sandbox = { module: { exports: {} }, Math, Number, String, Object, Array, Set, Map, JSON };
vm.createContext(sandbox);
vm.runInContext(coreText, sandbox, { filename: 'index.html#core' });
const Core = sandbox.module.exports;
assert(Core && typeof Core.buildModel === 'function', 'core did not export FlagPlotCore');

const EPS = 1e-6;
const close = (a, b) => Math.abs(a[0] - b[0]) < EPS && Math.abs(a[1] - b[1]) < EPS;

// ---- Geometry ----
function checkGeometry(replay, label) {
  const dirs = replay.directions;
  let edges = 0, oneSided = 0;
  const byKey = new Map(replay.map.hexes.map(h => [h.q + ',' + h.r, h]));
  for (const h of replay.map.hexes) {
    const c = Core.hexCenter(h.q, h.r);
    const corners = Core.hexCorners(h.q, h.r);
    for (let d = 0; d < 6; d++) {
      const [dq, dr] = dirs[d];
      const e = Core.edgeForDirection(h.q, h.r, d, dirs);
      const nc = Core.hexCenter(h.q + dq, h.r + dr);
      const ncorners = Core.hexCorners(h.q + dq, h.r + dr);
      for (const p of e) {
        if (!corners.some(k => close(k, p))) { fail(label + ': edge point of (' + h.q + ',' + h.r + ') d' + d + ' is not a corner of the hex'); return; }
        if (!ncorners.some(k => close(k, p))) { fail(label + ': edge point of (' + h.q + ',' + h.r + ') d' + d + ' is not a corner of the neighbour'); return; }
      }
      const mid = [(e[0][0] + e[1][0]) / 2, (e[0][1] + e[1][1]) / 2];
      if (!close(mid, [(c[0] + nc[0]) / 2, (c[1] + nc[1]) / 2])) { fail(label + ': edge midpoint of (' + h.q + ',' + h.r + ') d' + d + ' is not halfway between the centres'); return; }
      edges++;
    }
    // closed sides: a side is closed if either hex marks it; count one-sided marks for information
    for (const d of h.closed || []) {
      const nb = byKey.get((h.q + dirs[d][0]) + ',' + (h.r + dirs[d][1]));
      const opposite = (d + 3) % 6;
      if (nb && !(nb.closed || []).includes(opposite)) oneSided++;
    }
  }
  // The brief's two worked examples
  const e0 = Core.edgeForDirection(0, 0, 0, [[1, 0], [1, -1], [0, -1], [-1, 0], [-1, 1], [0, 1]]);
  assert(close(e0[0], [20, -11.547005]) && close(e0[1], [20, 11.547005]), 'direction 0 edge of (0,0) is not (20,-11.547)-(20,11.547)');
  const e1 = Core.edgeForDirection(0, 0, 1, [[1, 0], [1, -1], [0, -1], [-1, 0], [-1, 1], [0, 1]]);
  assert((close(e1[0], [0, -23.094011]) && close(e1[1], [20, -11.547005])) || (close(e1[1], [0, -23.094011]) && close(e1[0], [20, -11.547005])), 'direction 1 edge of (0,0) is not (0,-23.094)-(20,-11.547)');
  return { edges, oneSided };
}

// ---- A side's view (decision 0047): what the engine let each side see, and a page built from nothing else ----
function checkSideViews(replay) {
  const notes = [];
  const byLogId = new Map(replay.events.map(e => [e.id, e]));
  for (const side of Core.SIDES) {
    const label = side + ' view';
    const sv = replay.side_views[side];
    let last = 0;
    sv.events.forEach((e, i) => {
      assert(e.n === i + 1, label + ': event ' + (i + 1) + ' is numbered ' + e.n + ', not 1, 2, 3 in order');
      const real = byLogId.get(e.log_id);
      if (!real) { fail(label + ': event ' + e.n + ' points at log event ' + e.log_id + ', which is not in the log'); return; }
      assert(e.log_id > last, label + ': event ' + e.n + ' is out of log order');
      last = e.log_id;
      assert(real.day === e.day && (real.pulse || null) === (e.pulse || null) && real.kind === e.kind, label + ': event ' + e.n + ' does not match log event ' + e.log_id + ' in day, pulse or kind');
      assert(new RegExp('^#' + e.n + ' D' + e.day + ' ').test(e.text), label + ': event ' + e.n + ' text does not start with its own number: ' + e.text.slice(0, 20));
      for (const c of e.causes || []) {
        if (c.event !== undefined) assert(Number.isInteger(c.event) && c.event >= 1 && c.event < e.n, label + ': event ' + e.n + ' cites event ' + c.event + ', which is not an earlier event of this side');
        else assert(Number.isInteger(c.order), label + ': event ' + e.n + ' has a cause that is neither an order nor an event');
      }
      if (e.cut) for (const k of ['chance', 'expected', 'hits', 'hits_on_aircraft']) assert(!(e.data && k in e.data), label + ': cut event ' + e.n + ' carries ' + k);
    });
    assert(sv.events.length <= replay.events.length, label + ': more events than the log');

    // The page for a side is built from its events alone, so its own end state must come out of them.
    const sr = Core.sideReplay(replay, side);
    const model = Core.buildModel(sr);
    Core.compareWithFinal(model).forEach(p => fail(label + ': rebuild vs final: ' + p));
    let enemy = 0;
    for (const st of model.states) {
      for (const f of st.forces.values()) if (f.side !== side) enemy++;
      for (const other of Object.keys(st.contacts)) if (other !== side && st.contacts[other].size) enemy++;
    }
    assert(enemy === 0, label + ': ' + enemy + ' enemy force(s) or enemy-held contacts in the side\'s page');
    for (const b of model.bases.values()) if (b.side !== side) assert(Object.keys(b).every(k => ['id', 'name', 'side', 'q', 'r'].includes(k)), label + ': enemy base ' + b.id + ' carries more than id, name, side and hex');
    assert(model.initShips.size === (replay.initial.ships || []).filter(s => s.side === side).length, label + ': ships are not exactly its own');
    assert(model.days.size === 0, label + ': carries state hashes, which are the umpire\'s');
    assert(model.replay.orders.every(o => o.side === side), label + ': carries the other side\'s orders');
    const day0 = sv.events.map((e, i) => e.day === 0 ? i : -1).filter(i => i >= 0);
    assert(JSON.stringify(model.steps[0].events) === JSON.stringify(day0), label + ': the Start step does not hold exactly the day-0 events');
    const seen = new Set();
    model.steps.forEach(s => s.events.forEach(ei => seen.add(ei)));
    assert(seen.size === sv.events.length, label + ': not every side event is in a step');
    notes.push(side + ' ' + sv.events.length + '/' + replay.events.length + ' events, ' + model.steps.length + ' steps');
  }
  return notes.join(', ');
}

// ---- One replay ----
function checkReplay(file) {
  console.log(path.basename(file));
  let replay;
  try { replay = JSON.parse(fs.readFileSync(file, 'utf8')); } catch (e) { fail('not JSON: ' + e.message); return; }
  const bad = Core.validateReplay(replay);
  if (bad) { fail('validateReplay: ' + bad); return; }
  const before = failures;
  const model = Core.buildModel(replay);
  const g = checkGeometry(replay, path.basename(file)) || { edges: 0, oneSided: 0 };

  // Steps: Start first; every event in exactly one step; steps ordered
  // Day-0 events (contacts a scenario starts a side with) are the Start step's; every other step has days >= 1.
  const day0 = replay.events.map((e, i) => e.day === 0 ? i : -1).filter(i => i >= 0);
  assert(model.steps[0].label === 'Start' && model.steps[0].index === 0, 'step 0 is not the Start step');
  assert(JSON.stringify(model.steps[0].events) === JSON.stringify(day0), 'the Start step does not hold exactly the day-0 events');
  assert(model.steps.slice(1).every(s => s.day >= 1), 'a step after Start has day 0');
  for (const d of replay.days) {
    for (const ei of day0) assert(ei < d.first_event || ei >= d.first_event + d.event_count, 'day ' + d.day + ' range includes day-0 event index ' + ei);
  }
  const count = new Map();
  model.steps.forEach((s, i) => {
    assert(s.index === i, 'step index mismatch at ' + i);
    if (i > 0) assert(s.events.length > 0, 'step "' + s.label + '" has no events');
    s.events.forEach(ei => count.set(ei, (count.get(ei) || 0) + 1));
    if (i > 1) {
      const p = model.steps[i - 1];
      assert(p.day < s.day || (p.day === s.day && Core.PHASES.indexOf(p.phase) < Core.PHASES.indexOf(s.phase)), 'steps out of order at ' + s.label);
    }
  });
  assert(count.size === replay.events.length && [...count.values()].every(n => n === 1), 'not every event is in exactly one step (' + count.size + ' of ' + replay.events.length + ')');
  // Orders-phase events are the null-pulse events before the first pulsed event of the day, etc.
  for (const s of model.steps) {
    if (s.phase === 'orders') s.events.forEach(ei => assert(replay.events[ei].pulse === null, 'orders step holds a pulsed event'));
    else if (s.phase === 'end') s.events.forEach(ei => assert(replay.events[ei].pulse === null, 'end step holds a pulsed event'));
    else if (s.phase !== 'start') s.events.forEach(ei => assert(replay.events[ei].pulse === s.phase, 'phase ' + s.phase + ' holds an event with pulse ' + replay.events[ei].pulse));
  }

  // Causes resolve to an order or an earlier event
  let causeCount = 0;
  for (const ev of replay.events) {
    for (const c of ev.causes) {
      causeCount++;
      if (c.order !== undefined) assert(model.orderById.has(c.order), 'event ' + ev.id + ' cites missing order ' + c.order);
      else {
        assert(model.eventById.has(c.event), 'event ' + ev.id + ' cites missing event ' + c.event);
        assert(c.event < ev.id, 'event ' + ev.id + ' cites event ' + c.event + ', which is not earlier');
      }
    }
    // the cause suffix in the text agrees with causes
    const parts = Core.splitText(ev.text);
    assert(parts.body.length > 0 && !/^#\d/.test(parts.body), 'event ' + ev.id + ': text prefix not stripped: ' + parts.body.slice(0, 30));
  }
  // led-to is the reverse of causes
  let reversed = 0;
  for (const list of model.ledTo.values()) reversed += list.length;
  assert(reversed === causeCount, 'led-to index has ' + reversed + ' links but causes have ' + causeCount);
  // why trees terminate and stay inside their limits
  for (const ev of replay.events) { const t = Core.whyTree(model, ev.id, { maxDepth: 8, maxNodes: 60 }); assert(t.count <= 60, 'why tree for ' + ev.id + ' is too big'); }

  // Day hashes
  for (const d of replay.days) assert(/^[0-9a-f]{16}$/.test(d.hash), 'day ' + d.day + ' hash is not 16 hex chars');
  for (const s of model.steps) if (s.phase !== 'start') assert(model.days.has(s.day), 'no day record (hash) for day ' + s.day);

  // Rebuild equals final
  const problems = Core.compareWithFinal(model);
  problems.forEach(p => fail('rebuild vs final: ' + p));

  // Every state is sane
  model.states.forEach((st, i) => {
    for (const f of st.forces.values()) assert(Number.isFinite(f.q) && Number.isFinite(f.r), 'step ' + i + ': force ' + f.id + ' has a bad position');
    for (const fgt of st.fights) assert(model.hexByKey.has(fgt.q + ',' + fgt.r), 'step ' + i + ': fight hex off the map');
  });

  // Labels
  if (model.hasLatLon) {
    for (const p of model.placements) {
      assert(model.hexByKey.has(p.q + ',' + p.r), 'label ' + p.label.place + ' placed off the map');
      // independent recomputation of the nearest hex
      let best = null, bd = Infinity;
      for (const h of replay.map.hexes) {
        if (typeof h.lat !== 'number') continue;
        const dx = (p.label.lon - h.lon) * Math.cos(12 * Math.PI / 180), dy = p.label.lat - h.lat;
        const d = dx * dx + dy * dy;
        if (d < bd) { bd = d; best = h; }
      }
      assert(best.q === p.q && best.r === p.r, 'label ' + p.label.place + ' not in the nearest hex');
    }
    assert(model.placements.length === replay.labels.length, 'not every label was placed');
  } else assert(model.placements.length === 0, 'labels placed on a map without lat/lon');

  const sideNote = replay.side_views ? checkSideViews(replay) : 'no side views (version ' + replay.version + ')';

  const s = Core.summarize(model);
  const unclosed = new Set(replay.events.map(e => e.kind));
  console.log('  steps ' + s.steps + ', events ' + s.events + ', forces ' + s.forces + ', contacts at end ' + s.contacts + ', labels placed ' + s.labels + '/' + replay.labels.length +
    ', hexes ' + replay.map.hexes.length + ', edges checked ' + g.edges + ', one-sided closed marks ' + g.oneSided + ', event kinds ' + unclosed.size + ', ' + sideNote +
    (failures === before ? '  OK' : '  FAILED'));
}

// ---- Synthetic edge cases ----
function synthetic() {
  console.log('synthetic cases');
  const dirs = [[1, 0], [1, -1], [0, -1], [-1, 0], [-1, 1], [0, 1]];
  const hex = (q, r, extra) => Object.assign({ q, r, kind: 'deep_sea', land_percent: 0, closed: [], port_size: 0, airfield_size: 0, anchorage: false, terrain: null }, extra || {});
  const base = () => ({
    format: 'flagplot-replay', version: 1, scenario: { name: 'Synthetic', placeholder: true, seed: 1, days: 1, path: 'x' },
    map: { hex_nm: 40, real: false, hexes: [hex(0, 0)] }, directions: dirs, labels: [], bases: [],
    initial: { task_forces: [], ships: [], air_groups: [], bases: [] }, orders: [], events: [], days: [],
    final: { task_forces: [], ships: [], air_groups: [], bases: [], contacts: { allied: [], japanese: [] }, claims: { allied: [], japanese: [] } },
  });
  const run = (name, replay, expectSteps) => {
    const msg = Core.validateReplay(replay);
    if (msg) { fail(name + ': rejected: ' + msg); return; }
    try {
      const m = Core.buildModel(replay);
      assert(m.steps.length === expectSteps, name + ': expected ' + expectSteps + ' steps, got ' + m.steps.length);
      assert(Core.compareWithFinal(m).length === 0, name + ': rebuild vs final differ: ' + Core.compareWithFinal(m).join('; '));
      Core.summarize(m);
      console.log('  ok: ' + name + ' (' + m.steps.length + ' step(s))');
    } catch (e) { fail(name + ': threw ' + e.stack); }
  };
  run('empty events', base(), 1);
  const noFinal = base(); delete noFinal.final; delete noFinal.initial; delete noFinal.labels; delete noFinal.bases; delete noFinal.orders; delete noFinal.days;
  run('only required keys (no initial, final, labels, bases, orders, days)', noFinal, 1);
  const single = base();
  single.initial.task_forces = [{ id: 1, name: 'Solo', side: 'allied', position: { q: 0, r: 0 }, ships: [1] }];
  single.initial.ships = [{ id: 1, name: 'S', side: 'allied' }];
  single.final.task_forces = [{ id: 1, name: 'Solo', side: 'allied', position: { q: 0, r: 0 }, ships: [1] }];
  single.final.ships = [{ id: 1, location: { task_force: 1 } }];
  single.events = [{ id: 1, day: 1, pulse: null, kind: 'order_accepted', text: '#1 D1 -         order:1 accepted for task_force:1  <- order:1', causes: [{ order: 1 }], entities: ['task_force:1'], data: { order: 1, target: 'task_force:1' } }];
  single.orders = [{ id: 1, side: 'allied', day: 1, issuer: 'player', target: 'task_force:1', text: 'hold' }];
  run('single hex, one force, orders only', single, 2);
  // A failed alarm roll leaves the force as it was; a successful one alerts it. alerted is compared for every force.
  const alarm = (failedOnly) => {
    const r = base();
    const tf = (alerted) => ({ id: 1, name: 'Solo', side: 'allied', position: { q: 0, r: 0 }, ships: [1], readiness: 'battle', alerted });
    r.initial.task_forces = [tf(false)];
    r.initial.ships = [{ id: 1, name: 'S', side: 'allied' }];
    r.final.task_forces = [tf(!failedOnly)];
    r.final.ships = [{ id: 1, location: { task_force: 1 } }];
    const roll = (id, alarmed) => ({ id, day: 1, pulse: 'night', kind: 'alarm_roll', text: '#' + id + ' D1 night     task_force:1 heard the fight of task_force:2 and task_force:3, chance 0.5: ' + (alarmed ? 'alarmed' : 'not alarmed'), causes: [], entities: ['task_force:1', 'task_force:2', 'task_force:3'], data: { task_force: 1, by_a: 2, by_b: 3, chance: 0.5, alarmed } });
    r.events = failedOnly ? [roll(1, false)] : [roll(1, false), roll(2, true)];
    return r;
  };
  run('a failed alarm roll alerts nobody', alarm(true), 2);
  run('a successful alarm roll alerts the force', alarm(false), 2);
  const nolatlon = base();
  nolatlon.labels = [{ place: 'nowhere', feature: 'x', lat: -5, lon: 150 }];
  run('labels but no lat/lon on the map', nolatlon, 1);
  assert(Core.placeLabels(nolatlon.map.hexes, nolatlon.labels).length === 0, 'placeLabels placed on a map without lat/lon');
  run('zero task forces with events', Object.assign(base(), { events: [{ id: 1, day: 1, pulse: 'night', kind: 'claim_received', text: '#1 D1 night     Allied claims 0 enemy ships sunk at (0,0)', causes: [], entities: [], data: { side: 'allied', hex: [0, 0], claimed_sunk: 0 } }] }), 2);
  // Step rules
  const ev = (id, day, pulse) => ({ id, day, pulse, kind: 'x', text: '#' + id + ' D' + day + ' ' + (pulse || '-') + ' t', causes: [], entities: [], data: {} });
  const steps = Core.buildSteps([ev(1, 1, null), ev(2, 1, null), ev(3, 1, 'night'), ev(4, 1, 'afternoon'), ev(5, 1, null), ev(6, 2, null), ev(7, 3, 'morning'), ev(8, 3, null)]);
  const labels = steps.map(s => s.label);
  const want = ['Start', 'Day 1, orders', 'Day 1, night (18:00 to 06:00)', 'Day 1, afternoon (12:00 to 18:00)', 'Day 1, end of day', 'Day 2, orders', 'Day 3, morning (06:00 to 12:00)', 'Day 3, end of day'];
  assert(JSON.stringify(labels) === JSON.stringify(want), 'step labels: ' + JSON.stringify(labels));
  console.log('  ok: step phases and labels');
  const withStart = Core.buildSteps([ev(1, 0, null), ev(2, 0, null), ev(3, 1, null), ev(4, 1, 'night')]);
  assert(JSON.stringify(withStart.map(s => s.label)) === JSON.stringify(['Start', 'Day 1, orders', 'Day 1, night (18:00 to 06:00)']) && JSON.stringify(withStart[0].events) === '[0,1]', 'day-0 events are not the Start step: ' + JSON.stringify(withStart));
  console.log('  ok: day-0 events form the Start step');
  // Geometry spot checks
  assert(Core.arcWidth(60, 120) === 60 && Core.arcWidth(300, 60) === 120 && Core.arcWidth(90, 90) === 360 && Core.arcWidth(0, 0) === 360, 'arcWidth');
  const p = Core.bearingPoint(0, 0, 90, 10); assert(close(p, [10, 0]), 'bearing 90 is east');
  const n = Core.bearingPoint(0, 0, 0, 10); assert(close(n, [0, -10]), 'bearing 0 is north (y decreases)');
  for (const q of [-3, 0, 5]) for (const r of [-2, 0, 4]) { const c = Core.hexCenter(q, r); const h = Core.pointToHex(c[0], c[1]); assert(h.q === q && h.r === r, 'pointToHex round trip'); }
  console.log('  ok: bearings and pointToHex');
  // Bad files
  const reject = (name, obj, needle) => {
    const msg = Core.validateReplay(obj);
    if (!msg) fail('bad file accepted: ' + name);
    else if (needle && !msg.includes(needle)) fail('bad file ' + name + ' rejected with an unhelpful message: ' + msg);
    else console.log('  ok: rejects ' + name + ': ' + msg);
  };
  reject('wrong format', Object.assign(base(), { format: 'something-else' }), 'format');
  reject('version 3', Object.assign(base(), { version: 3 }), '3');
  reject('version 2 without side views', Object.assign(base(), { version: 2 }), 'side_views');
  reject('version 2 with a side event without its number', Object.assign(base(), { version: 2, side_views: { allied: { events: [{ log_id: 1, day: 1, kind: 'x' }] }, japanese: { events: [] } } }), 'allied');
  const ok2 = Object.assign(base(), { version: 2, side_views: { allied: { events: [] }, japanese: { events: [] } } });
  if (Core.validateReplay(ok2)) fail('a version 2 file with empty side views was rejected: ' + Core.validateReplay(ok2));
  else { const m = Core.buildModel(Core.sideReplay(ok2, 'allied')); assert(m.steps.length === 1, 'empty side view should have only the Start step'); console.log('  ok: version 2 with empty side views, and a side page built from it'); }
  reject('missing version', Object.assign(base(), { version: undefined }), 'version');
  reject('array', [1, 2], 'object');
  reject('null', null, 'object');
  reject('no events list', Object.assign(base(), { events: undefined }), 'events');
  reject('duplicate event ids', Object.assign(base(), { events: [ev(1, 1, null), ev(1, 1, null)] }), 'share');
}

// ---- The measuring tool against the engine's own answers ----
// viewer/measure_fixture.json is written by the engine (crates/sim-cli/src/export/measure_fixture.rs).
function checkMeasuring() {
  console.log('measuring tool vs the engine (viewer/measure_fixture.json)');
  let fx;
  try { fx = JSON.parse(fs.readFileSync(path.join(__dirname, 'measure_fixture.json'), 'utf8')); }
  catch (e) { fail('cannot read viewer/measure_fixture.json: ' + e.message); return; }
  assert(fx.format === 'flagplot-measure-fixture' && fx.version === 1, 'unknown fixture format');
  assert(fx.hex_nm === Core.HEX_NM, 'the page says a hex is ' + Core.HEX_NM + ' nm, the engine says ' + fx.hex_nm);
  const before = failures;
  for (const c of fx.air) {
    const got = Core.airDistanceNm(c.from[0], c.from[1], c.to[0], c.to[1]);
    if (got !== c.nm) { fail('air distance ' + JSON.stringify(c.from) + ' to ' + JSON.stringify(c.to) + ': page ' + got + ', engine ' + c.nm); if (failures - before > 10) break; }
  }
  const sea = Core.makeSeaMap(fx.hexes, fx.directions);
  let routes = 0, none = 0;
  for (const c of fx.sea) {
    const label = 'sea route ' + JSON.stringify(c.from) + ' to ' + JSON.stringify(c.to);
    const got = Core.measureHexes(sea, c.from, c.to);
    if (c.path === null) {
      none++;
      assert(got.sea === null, label + ': page found a route, engine found none');
    } else {
      routes++;
      assert(got.sea !== null && JSON.stringify(got.sea.path) === JSON.stringify(c.path), label + ': page path differs from the engine path');
      assert(got.sea !== null && got.sea.steps === c.steps && got.sea.nm === c.nm, label + ': page ' + (got.sea && got.sea.nm) + ' nm, engine ' + c.nm);
    }
    assert(got.air === Core.airDistanceNm(c.from[0], c.from[1], c.to[0], c.to[1]), label + ': air distance inconsistent');
  }
  // isqrt against exact big-integer arithmetic
  for (const n of [0, 1, 2, 3, 4, 15, 16, 17, 99999999, 4503599627370495, 2 ** 52 + 1]) {
    const s = BigInt(Core.isqrt(n)), N = BigInt(n);
    assert(s * s <= N && (s + 1n) * (s + 1n) > N, 'isqrt(' + n + ') is wrong');
  }
  console.log('  air cases ' + fx.air.length + ', sea cases ' + fx.sea.length + ' (' + routes + ' with a route, ' + none + ' with none)' + (failures === before ? '  OK' : '  FAILED'));
}

// ---- Main ----
const files = process.argv.slice(2);
if (!files.length) console.log('(no replay files given: only the script and synthetic checks run)');
for (const f of files) checkReplay(f);
synthetic();
checkMeasuring();
console.log(failures ? '\n' + failures + ' check(s) FAILED' : '\nAll checks passed.');
process.exit(failures ? 1 : 0);
