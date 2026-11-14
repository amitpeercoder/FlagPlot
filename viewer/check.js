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
function checkSideViews(replay, umpireModel) {
  const notes = [];
  const airTotals = { raids: 0, bombers: 0, escorts: 0, launches: 0, nothing: 0 };
  const byLogId = new Map(replay.events.map(e => [e.id, e]));
  for (const side of Core.SIDES) {
    const label = side + ' view';
    const sv = replay.side_views[side];
    const enemyUnits = new Set();
    for (const [list, word] of [['ships', 'ship'], ['task_forces', 'task_force'], ['air_groups', 'air_group']]) {
      for (const u of replay.initial[list] || []) if (u.side !== side) enemyUnits.add(word + ':' + u.id);
    }
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
      // An event cut of nothing but the enemy's name keeps its own figures (decision 0065): a raid on the
      // side's own base (which has no air group), a bombardment of it (which has no task force), its own sighting and the alarm its force heard.
      const nameOnly = e.kind === 'sighting_roll' || e.kind === 'alarm_roll' || (e.kind === 'base_attacked' && !(e.data && 'air_group' in e.data)) || (e.kind === 'shore_bombardment' && !(e.data && 'task_force' in e.data));
      if (e.cut && !nameOnly) for (const k of ['chance', 'expected', 'hits', 'hits_on_aircraft']) assert(!(e.data && k in e.data), label + ': cut event ' + e.n + ' carries ' + k);
      // No event a side is given names an enemy ship, task force or air group, in its entities or its
      // text (decision 0065). The exception is the target the side wrote in its own order.
      if (!/^order_/.test(e.kind)) {
        const named = new Set([...(e.entities || []), ...(e.text.match(/(ship|task_force|air_group):\d+/g) || [])]);
        for (const unit of named) assert(!enemyUnits.has(unit), label + ': event ' + e.n + ' names ' + unit + ', which is the enemy\'s');
      }
    });
    assert(sv.events.length <= replay.events.length, label + ': more events than the log');

    // The page for a side is built from its events alone, so its own end state must come out of them.
    const sr = Core.sideReplay(replay, side);
    const model = Core.buildModel(sr);
    if (replay.steps) {
      assert(Array.isArray(sr.steps), label + ': the side page lost its steps');
      checkSideSteps(replay, side, sr, model, label);
      crossCheckRebuild(Core.buildModel(Core.sideReplay(Object.assign({}, replay, { steps: undefined }), side)), model, label);
    }
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
    const air = checkAirModel(replay, model, label);
    for (const k of Object.keys(airTotals)) airTotals[k] += air[k];
    notes.push(side + ' ' + sv.events.length + '/' + replay.events.length + ' events, ' + model.steps.length + ' steps');
  }
  // Every launch of the log is some side's own launch, so the sides' raids add up to the umpire's.
  const u = checkAirModel(replay, umpireModel, 'umpire');
  for (const k of Object.keys(airTotals)) assert(airTotals[k] === u[k], 'the two sides\' ' + k + ' (' + airTotals[k] + ') do not add up to the umpire\'s (' + u[k] + ')');
  // A strike that found nothing: the umpire's raid says where the force truly was, the sides' do not. In the full slice every one can be traced.
  if (/07_full_slice/.test(currentFile)) assert(u.nothing > 0 && u.untraced === 0, 'the full slice: ' + u.nothing + ' raids found nothing, ' + u.untraced + ' without a true hex');
  notes.push('air: ' + u.launches + ' launches in ' + u.raids + ' raids' + (u.nothing ? ', ' + u.nothing + ' found nothing' + (u.untraced ? ' (' + u.untraced + ' not traceable)' : '') : ''));
  return notes.join(', ');
}

// ---- The air layer: raids, launch hexes, reach, and what a side's page may hold ----
function eventHexes(events) {
  const out = new Set();
  const visit = v => {
    if (Array.isArray(v)) {
      if (v.length === 2 && Number.isInteger(v[0]) && Number.isInteger(v[1])) out.add(v[0] + ',' + v[1]);
      else v.forEach(visit);
    } else if (v && typeof v === 'object') Object.values(v).forEach(visit);
  };
  for (const e of events) visit(e.data);
  return out;
}

/** Checks the air layer of one model (the umpire's or a side's) and returns its totals. */
function checkAirModel(replay, model, label) {
  const side = model.replay.view_side;
  const tot = { raids: 0, bombers: 0, escorts: 0, launches: 0, nothing: 0, untraced: 0 };
  const types = model.aircraftTypes;
  const known = side ? eventHexes(model.events) : null; // hexes a side's own events name
  if (side) {
    for (const b of model.bases.values()) if (b.side === side) known.add(b.q + ',' + b.r);
    for (const st of model.states) for (const f of st.forces.values()) for (const h of f.trail) known.add(h[0] + ',' + h[1]);
  }
  model.steps.forEach((s, si) => {
    const st = model.states[si], air = st.air;
    const inStep = new Set(s.events.map(ei => model.events[ei].id));
    const raidOfEvent = new Map();
    const raidIds = new Set();
    for (const raid of air.raids) {
      assert(!raidIds.has(raid.id), label + ': raid id ' + raid.id + ' is used twice');
      raidIds.add(raid.id);
      tot.raids++; tot.bombers += raid.bombers; tot.escorts += raid.escorts;
      assert(raid.step === si, label + ': raid ' + raid.id + ' is not in step ' + si);
      for (const id of raid.events) {
        assert(model.eventById.has(id), label + ': raid ' + raid.id + ' names event ' + id + ', which is not in the model');
        assert(inStep.has(id), label + ': raid ' + raid.id + ' holds event ' + id + ', which is not in its step');
        assert(!raidOfEvent.has(id), label + ': event ' + id + ' is in raids ' + raidOfEvent.get(id) + ' and ' + raid.id);
        raidOfEvent.set(id, raid.id);
      }
      assert(raid.events.includes(raid.keyEvent), label + ': raid ' + raid.id + ' key event is not one of its events');
      if (raid.outcome === 'nothing') tot.nothing++;
      if (side) {
        assert(raid.trueHex === undefined && raid.trueNote === undefined, label + ': a side raid holds a true hex');
        assert(raid.side === side, label + ': raid ' + raid.id + ' is the other side\'s');
        assert(raid.launch && raid.target, label + ': raid ' + raid.id + ' has no launch or target hex');
        assert(raid.target && known.has(raid.target.join(',')), label + ': raid ' + raid.id + ' target ' + raid.target + ' is in none of the side\'s own events');
        assert(raid.launch && known.has(raid.launch.join(',')), label + ': raid ' + raid.id + ' launch ' + raid.launch + ' is where none of the side\'s own groups was');
      } else if (raid.outcome === 'nothing') { if (raid.trueHex === null) tot.untraced++; assert(raid.trueHex !== undefined && (raid.trueHex !== null || raid.trueNote === 'not traceable'), label + ': a found-nothing raid has neither a true hex nor a note'); }
      else assert(raid.trueHex === null && raid.trueNote === null, label + ': raid ' + raid.id + ' found something but has a true hex');
    }
    for (const t of air.tallies) {
      for (const id of t.events) assert(raidOfEvent.has(id), label + ': tally ' + t.key + ' holds event ' + id + ' that is in no raid');
      assert(t.events.length === air.raids.filter(r => t.raids.includes(r.id)).reduce((n, r) => n + r.events.length, 0), label + ': tally ' + t.key + ' does not hold exactly its raids\' events');
      if (side) assert(t.patrolEngaged === null || t.patrolEngaged === 0, label + ': a side tally knows the patrol that engaged');
    }
    if (side) {
      assert(air.misses.length === 0, label + ': a side page has a miss mark');
      for (const m of air.sighted) { assert(m.trueHex === null, label + ': a side sighting mark has a true hex'); assert(known.has(m.q + ',' + m.r), label + ': sighted mark at ' + m.q + ',' + m.r + ' is in none of the side\'s own events'); }
    }
    // Launches: the hex they were launched from is the one the group was at, so the engine's distance comes out.
    const raidOfLaunch = new Map();
    for (const raid of air.raids) for (const id of raid.launchEvents) raidOfLaunch.set(id, raid);
    for (const ei of s.events) {
      const ev = model.events[ei], d = ev.data || {};
      if (ev.kind === 'strike_launched' || ev.kind === 'base_strike_launched') {
        tot.launches++;
        const raid = raidOfLaunch.get(ev.id);
        if (!raid) { fail(label + ': launch ' + ev.id + ' is in no raid'); continue; }
        if (!raid.launch) { fail(label + ': launch ' + ev.id + ' has no launch hex'); continue; }
        const dist = Core.airDistanceNm(raid.launch[0], raid.launch[1], d.hex[0], d.hex[1]);
        assert(dist === d.distance_nm, label + ': launch ' + ev.id + ' from (' + raid.launch + ') to (' + d.hex + ') is ' + dist + ' nm by the page, ' + d.distance_nm + ' by the engine');
        const g = model.groupTable.get(d.air_group), t = g && types.get(g.typeId);
        if (t) {
          const limit = st.reach.get(d.air_group) === 'extended' ? t.extended_reach_nm : t.strike_radius_nm;
          assert(d.distance_nm <= limit, label + ': launch ' + ev.id + ' is ' + d.distance_nm + ' nm for a group with a reach of ' + limit);
        }
      } else if (ev.kind === 'search_flown') {
        const g = model.groupTable.get(d.air_group), t = g && types.get(g.typeId);
        if (t) assert(d.range_nm <= t.search_range_nm, label + ': search ' + ev.id + ' range ' + d.range_nm + ' is over its type\'s ' + t.search_range_nm);
      }
    }
  });
  return tot;
}


// ---- Replay version 4: the state at each step is in the file ----
const STEP_KINDS = ['task_forces', 'ships', 'air_groups', 'bases', 'land_units'];
const SORTED = x => JSON.stringify(x, (k, v) => (v && typeof v === 'object' && !Array.isArray(v)) ? Object.fromEntries(Object.entries(v).sort(([a], [b]) => a < b ? -1 : 1)) : v);

/** Applies the entries of `steps` in order, on its own (no code of the page), and returns each step's records as JSON text by kind and key. */
function applySteps(steps) {
  const cur = { task_forces: new Map(), ships: new Map(), air_groups: new Map(), bases: new Map(), land_units: new Map(), contacts: { allied: new Map(), japanese: new Map() }, ground_estimates: { allied: new Map(), japanese: new Map() } };
  const out = [];
  for (const e of steps) {
    const ch = e.changed || {}, rm = e.removed || {};
    for (const k of STEP_KINDS) { for (const r of ch[k] || []) cur[k].set(r.id, SORTED(r)); for (const id of rm[k] || []) cur[k].delete(id); }
    for (const side of Core.SIDES) {
      for (const r of (ch.contacts && ch.contacts[side]) || []) cur.contacts[side].set(r.id, SORTED(r));
      for (const id of (rm.contacts && rm.contacts[side]) || []) cur.contacts[side].delete(id);
      for (const r of (ch.ground_estimates && ch.ground_estimates[side]) || []) cur.ground_estimates[side].set(r.hex.join(','), SORTED(r));
      for (const h of (rm.ground_estimates && rm.ground_estimates[side]) || []) cur.ground_estimates[side].delete(h.join(','));
    }
    out.push({ task_forces: new Map(cur.task_forces), ships: new Map(cur.ships), air_groups: new Map(cur.air_groups), bases: new Map(cur.bases), land_units: new Map(cur.land_units) });
  }
  return { last: cur, all: out };
}

function sameJson(a, b) { return SORTED(a) === SORTED(b); }

/** The steps of a version 4 replay: they end exactly at `final`, cover every viewer step, and a side's page holds none of the enemy's. */
function checkSteps(replay, model) {
  const steps = replay.steps;
  const before = failures;
  assert(steps[0].day === 0 && steps[0].phase === 'start', 'the first entry of steps is not day 0 "start"');
  // Every day has every phase, in order, and the days run 1..n
  const want = [];
  for (let d = 1; d <= replay.scenario.days; d++) for (const p of ['orders', 'night', 'morning', 'afternoon', 'end']) want.push(d + ':' + p);
  assert(JSON.stringify(steps.slice(1).map(e => e.day + ':' + e.phase)) === JSON.stringify(want), 'steps do not hold every phase of every day: ' + steps.length + ' entries for ' + replay.scenario.days + ' days');
  // The first entry carries every record of the start state: nothing is "removed", and it equals initial plus the day-0 events' effect on contacts
  assert(!steps[0].removed || Object.keys(steps[0].removed).length === 0, 'the first entry removes something');
  // Applied from the first, the last step is exactly final's records, every kind and field
  const { last, all } = applySteps(steps);
  const final = replay.final;
  const sets = [];
  for (const k of STEP_KINDS) sets.push([k, [...last[k].values()], (final[k] || []).map(SORTED)]);
  for (const side of Core.SIDES) {
    sets.push(['contacts.' + side, [...last.contacts[side].values()], ((final.contacts || {})[side] || []).map(SORTED)]);
    sets.push(['ground_estimates.' + side, [...last.ground_estimates[side].values()], ((final.ground_estimates || {})[side] || []).map(SORTED)]);
  }
  for (const [name, have, wanted] of sets) {
    assert(have.length === wanted.length, 'the last step has ' + have.length + ' ' + name + ', final ' + wanted.length);
    const byKey = new Set(wanted);
    for (const h of have) if (!byKey.has(h)) { fail('the last step\'s ' + name + ' record differs from final\'s: ' + h.slice(0, 120)); break; }
  }
  // A step that changed nothing is just its day and phase; no entry lists a record that equals the one before it
  steps.forEach((e, i) => {
    if (i === 0) return;
    for (const k of STEP_KINDS) for (const r of (e.changed || {})[k] || []) assert(all[i - 1][k].get(r.id) !== SORTED(r), 'step ' + i + ' lists ' + k + ' ' + r.id + ' unchanged');
    for (const part of ['changed', 'removed']) if (e[part] !== undefined) assert(Object.keys(e[part]).length > 0 && Object.values(e[part]).every(v => Array.isArray(v) ? v.length > 0 : Object.values(v).some(l => l.length > 0)), 'step ' + i + ' has an empty ' + part + ' or an empty list in it');
  });
  // Every viewer step has its state, and it is the one at the end of the viewer step's own (day, phase) for the umpire
  assert(model.states.length === model.steps.length, 'the model has ' + model.states.length + ' states for ' + model.steps.length + ' steps');
  if (!replay.view_side) for (const s of model.steps) assert(s.stateDay === s.day && s.statePhase === s.phase, 'step ' + s.label + ' takes its state from another step');
  return failures === before;
}

/** The state of each step in a model, as comparable text, for the parts both ways of building a model make. */
function stateText(model, st) {
  const mapOf = m => [...m.entries()].sort((a, b) => a[0] < b[0] ? -1 : a[0] > b[0] ? 1 : 0);
  const forces = [...st.forces.values()].sort((a, b) => a.id - b.id).map(f => [f.id, f.side, f.q, f.r, [...f.ships].sort((a, b) => a - b), f.trail, f.detachedFrom, f.readiness, f.alerted]);
  const contacts = Core.SIDES.map(side => mapOf(st.contacts[side] || new Map()));
  const estimates = Core.SIDES.map(side => mapOf((st.estimates && st.estimates[side]) || new Map()));
  // The rebuild does not set the march a falling back gives a unit (decisions 0104, 0105: toward the hex it fell back to, no day marched
  // yet); the records hold it. It is left out of the comparison until the unit has marched a day.
  const land = [...st.landUnits.values()].sort((a, b) => a.id - b.id).map(u => [u.id, u.side, u.men_ashore, u.hex, u.posture, u.fatigue, u.fieldworks, u.fed_share, u.march && u.march.days > 0 ? [u.march.to, u.march.days, u.march.of] : null]);
  const baseSide = mapOf(new Map([...st.baseSide].filter(([id]) => model.bases.has(id))));
  return {
    forces, contacts, estimates, land, baseSide, formed: mapOf(new Map([...st.formedBases].map(([k, b]) => [k, b.side]))), // by hex and holder: the rebuild gives a hex the id of an original base captured there, in place of the formed base's sunk: [...st.sunk].sort((a, b) => a - b), shipForce: mapOf(st.shipForce),
    reach: mapOf(st.reach), mission: mapOf(st.mission), groupBase: mapOf(st.groupBase),
    moves: mapOf(st.moves), fights: st.fights, searches: st.searches, air: st.air,
  };
}

/** The model built from a version 4 replay's steps against the one the rebuild from the events makes of the same file, at every step. */
function crossCheckRebuild(oldModel, model, label) {
  if (oldModel.states.length !== model.states.length) { fail(label + ': the rebuild has ' + oldModel.states.length + ' steps, the steps\' model ' + model.states.length); return; }
  const problems = [];
  model.states.forEach((st, si) => {
    const a = stateText(oldModel, oldModel.states[si]), b = stateText(model, st);
    for (const k of Object.keys(a)) if (SORTED(a[k]) !== SORTED(b[k])) problems.push('step ' + si + ' (' + model.steps[si].label + '): ' + k + ' differs: rebuild ' + SORTED(a[k]).slice(0, 200) + ' / steps ' + SORTED(b[k]).slice(0, 200));
  });
  assert(JSON.stringify([...oldModel.eventForces]) === JSON.stringify([...model.eventForces]), label + ': eventForces differ');
  assert(oldModel.hasAir === model.hasAir, label + ': hasAir differs');
  problems.slice(0, 5).forEach(p => fail(label + ': ' + p));
  if (problems.length > 5) fail(label + ': ... and ' + (problems.length - 5) + ' more differences from the rebuild');
}

/** A side's page of a version 4 replay holds nothing of the enemy: in its steps, its model and its states. */
function checkSideSteps(replay, side, sr, model, label) {
  const enemy = Core.SIDES.find(x => x !== side);
  const enemyIds = {};
  for (const k of STEP_KINDS) enemyIds[k] = new Set([...(replay.initial[k] || []), ...(replay.final[k] || []), ...replay.steps.flatMap(e => (e.changed || {})[k] || [])].filter(r => r.side === enemy).map(r => r.id));
  assert(sr.steps.length === replay.steps.length, label + ': the side has ' + sr.steps.length + ' entries of steps, the umpire ' + replay.steps.length);
  sr.steps.forEach((e, i) => {
    const u = replay.steps[i];
    assert(u.day === e.day && u.phase === e.phase, label + ': step ' + i + ' is not the umpire\'s step');
    for (const k of STEP_KINDS) {
      for (const r of (e.changed || {})[k] || []) {
        assert(r.side === side, label + ': step ' + i + ' holds ' + k + ' ' + r.id + ' of ' + r.side);
        assert(((u.changed || {})[k] || []).some(x => x.id === r.id && SORTED(x) === SORTED(r)), label + ': step ' + i + ' ' + k + ' ' + r.id + ' is not the umpire\'s record');
      }
    }
    for (const k of ['contacts', 'ground_estimates']) for (const part of [e.changed, e.removed]) assert(!part || !part[k] || (Object.keys(part[k]).length === 1 && part[k][side] !== undefined), label + ': step ' + i + ' holds the ' + enemy + '\'s ' + k);
  });
  for (const st of model.states) {
    for (const f of st.forces.values()) assert(f.side === side, label + ': an enemy force in a state');
    for (const u of st.landUnits.values()) assert(u.side === side, label + ': an enemy land unit in a state');
    for (const other of Core.SIDES) if (other !== side) assert(st.contacts[other].size === 0 && st.estimates[other].size === 0, label + ': the ' + other + '\'s contacts or estimates in a state');
    for (const [id] of st.shipForce) assert(!enemyIds.ships.has(id), label + ': the enemy ship ' + id + ' is in a force');
  }
  // What the side holds at the end is exactly its own final records.
  const { last } = applySteps(sr.steps);
  for (const k of STEP_KINDS) {
    const wanted = (replay.final[k] || []).filter(r => r.side === side).map(SORTED).sort(), have = [...last[k].values()].sort();
    assert(JSON.stringify(wanted) === JSON.stringify(have), label + ': the side\'s last ' + k + ' are not its own final ' + k);
  }
}

// ---- One replay ----
let currentFile = '';
function checkReplay(file) {
  currentFile = path.basename(file);
  console.log(path.basename(file));
  let replay;
  try { replay = JSON.parse(fs.readFileSync(file, 'utf8')); } catch (e) { fail('not JSON: ' + e.message); return; }
  const bad = Core.validateReplay(replay);
  if (bad) { fail('validateReplay: ' + bad); return; }
  const before = failures;
  const model = Core.buildModel(replay);
  if (replay.steps) {
    checkSteps(replay, model);
    crossCheckRebuild(Core.buildModel(Object.assign({}, replay, { steps: undefined, version: 3 })), model, 'umpire');
  }
  const g = checkGeometry(replay, path.basename(file)) || { edges: 0, oneSided: 0 };

  // Each air group at a base ends at the base the final state gives: a rebase (decision 0091) is followed.
  const lastState = model.states[model.states.length - 1];
  for (const fg of (replay.final && replay.final.air_groups) || []) {
    if (fg.base == null || !lastState.groupBase || !lastState.groupBase.has(fg.id)) continue;
    assert(lastState.groupBase.get(fg.id) === fg.base, 'air_group:' + fg.id + ' is rebuilt at base:' + lastState.groupBase.get(fg.id) + ' but ends at base:' + fg.base);
  }

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

  const sideNote = replay.side_views ? checkSideViews(replay, model) : 'no side views (version ' + replay.version + ')';

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
  reject('version 5', Object.assign(base(), { version: 5 }), '1, 2, 3 and 4');
  reject('version 2 without side views', Object.assign(base(), { version: 2 }), 'side_views');
  reject('version 3 without side views', Object.assign(base(), { version: 3, aircraft_types: [] }), 'side_views');
  reject('version 2 with a side event without its number', Object.assign(base(), { version: 2, side_views: { allied: { events: [{ log_id: 1, day: 1, kind: 'x' }] }, japanese: { events: [] } } }), 'allied');
  const ok2 = Object.assign(base(), { version: 2, side_views: { allied: { events: [] }, japanese: { events: [] } } });
  if (Core.validateReplay(ok2)) fail('a version 2 file with empty side views was rejected: ' + Core.validateReplay(ok2));
  else { const m = Core.buildModel(Core.sideReplay(ok2, 'allied')); assert(m.steps.length === 1, 'empty side view should have only the Start step'); console.log('  ok: version 2 with empty side views, and a side page built from it'); }
  // Version 3: aircraft_types, each with an id, a name and three figures, the extended reach not below the radius
  const typ = (extra) => Object.assign({ id: 2000, name: 'Test Fighter', strike_radius_nm: 100, extended_reach_nm: 130, search_range_nm: 150 }, extra || {});
  const v3 = (types) => Object.assign(base(), { version: 3, side_views: { allied: { events: [] }, japanese: { events: [] } }, aircraft_types: types });
  const ok3 = v3([typ(), typ({ id: 2001, name: 'Other', extended_reach_nm: 100 })]);
  if (Core.validateReplay(ok3)) fail('a version 3 file with valid aircraft_types was rejected: ' + Core.validateReplay(ok3));
  else console.log('  ok: version 3 with valid aircraft_types');
  if (Core.validateReplay(v3([]))) fail('a version 3 file with no aircraft types was rejected'); else console.log('  ok: version 3 with an empty aircraft_types list');
  reject('version 3 without aircraft_types', Object.assign(v3([]), { aircraft_types: undefined }), 'aircraft_types');
  reject('version 3 with an aircraft type without a name', v3([typ({ name: undefined })]), 'aircraft_types');
  reject('version 3 with extended reach below the radius', v3([typ({ extended_reach_nm: 99 })]), 'aircraft_types');
  reject('version 3 with a non-number figure', v3([typ({ search_range_nm: '150' })]), 'aircraft_types');
  reject('version 3 with a negative radius', v3([typ({ strike_radius_nm: -1, extended_reach_nm: 0 })]), 'aircraft_types');
  const side3 = Core.sideReplay(ok3, 'allied');
  assert(Array.isArray(side3.aircraft_types) && side3.aircraft_types.length === 0, 'a side page carries aircraft types of groups it does not have');
  // A version 3 file opens as it always did: its state is rebuilt from its events.
  {
    const r3 = Object.assign(base(), { version: 3, side_views: { allied: { events: [] }, japanese: { events: [] } }, aircraft_types: [] });
    r3.initial.task_forces = [{ id: 1, name: 'Solo', side: 'allied', position: { q: 0, r: 0 }, ships: [1] }];
    r3.initial.ships = [{ id: 1, name: 'S', side: 'allied' }];
    r3.final.task_forces = [{ id: 1, name: 'Solo', side: 'allied', position: { q: 0, r: 0 }, ships: [1] }];
    r3.final.ships = [{ id: 1, location: { task_force: 1 } }];
    r3.events = [{ id: 1, day: 1, pulse: null, kind: 'order_accepted', text: '#1 D1 -         order:1 accepted for task_force:1  <- order:1', causes: [{ order: 1 }], entities: ['task_force:1'], data: { order: 1, target: 'task_force:1' } }];
    r3.orders = [{ id: 1, side: 'allied', day: 1, issuer: 'player', target: 'task_force:1', text: 'hold' }];
    assert(r3.steps === undefined && Core.validateReplay(r3) === null, 'a version 3 file without steps was rejected');
    run('version 3 file, rebuilt from its events', r3, 2);
  }
  synthetic4(base, hex, ev, reject, dirs);
  const v1 = base();
  if (Core.validateReplay(v1)) fail('a version 1 file was rejected'); else console.log('  ok: version 1 still opens');
  const m1 = Core.buildModel(v1);
  assert(m1.aircraftTypes.size === 0 && !m1.hasAir, 'version 1: aircraft types or air activity out of nothing');
  reject('missing version', Object.assign(base(), { version: undefined }), 'version');
  reject('array', [1, 2], 'object');
  reject('null', null, 'object');
  reject('no events list', Object.assign(base(), { events: undefined }), 'events');
  reject('duplicate event ids', Object.assign(base(), { events: [ev(1, 1, null), ev(1, 1, null)] }), 'share');
}


// ---- A hand-built version 4 replay: the state of each step is the file's ----
function synthetic4(base, hex, ev, reject) {
  console.log('synthetic version 4');
  const before = failures;
  const tf = (id, side, q, r, ships) => ({ id, name: 'TF' + id, side, position: { q, r }, ships, readiness: 'normal', alerted: false, mission: 'idle', status: 'idle' });
  const ship = (id, side, location) => ({ id, name: 'S' + id, side, location, fuel: 10 });
  const contact = (id, age) => ({ id, hex: [1, 0], ship_count: 1, kind: 'cruiser', age_pulses: age });
  const events = [
    { id: 1, day: 1, pulse: 'night', kind: 'task_force_steamed', text: '#1 D1 night     task_force:1 steamed', causes: [], entities: ['task_force:1'], data: { task_force: 1, from: [0, 0], to: [1, 0], hexes_entered: [[1, 0]] } },
    { id: 2, day: 1, pulse: null, kind: 'claim_received', text: '#2 D1 -         claim', causes: [], entities: [], data: { side: 'allied', hex: [1, 0], claimed_sunk: 0 } },
  ];
  const entries = [
    { day: 0, phase: 'start', changed: { task_forces: [tf(1, 'allied', 0, 0, [1]), tf(2, 'japanese', 1, 0, [2])], ships: [ship(1, 'allied', { task_force: 1 }), ship(2, 'japanese', { task_force: 2 })], contacts: { allied: [contact(1, 0)], japanese: [] } } },
    { day: 1, phase: 'orders' },
    { day: 1, phase: 'night', changed: { task_forces: [tf(1, 'allied', 1, 0, [1])] } },
    { day: 1, phase: 'morning', changed: { contacts: { allied: [contact(1, 1)], japanese: [] } } },
    { day: 1, phase: 'afternoon' },
    { day: 1, phase: 'end', changed: { ships: [ship(2, 'japanese', 'sunk')], task_forces: [tf(2, 'japanese', 1, 0, [])] }, removed: { contacts: { allied: [1], japanese: [] } } },
  ];
  const mk = () => Object.assign(base(), {
    version: 4, side_views: { allied: { events: [] }, japanese: { events: [] } }, aircraft_types: [],
    map: { hex_nm: 40, real: false, hexes: [hex(0, 0), hex(1, 0)] }, scenario: { name: 'Synthetic 4', placeholder: true, seed: 1, days: 1, path: 'x' },
    events: events.map(e => Object.assign({}, e)), steps: JSON.parse(JSON.stringify(entries)),
    initial: { task_forces: [tf(1, 'allied', 0, 0, [1]), tf(2, 'japanese', 1, 0, [2])], ships: [{ id: 1, name: 'S1', side: 'allied' }, { id: 2, name: 'S2', side: 'japanese' }], air_groups: [], bases: [] },
    final: { task_forces: [tf(1, 'allied', 1, 0, [1]), tf(2, 'japanese', 1, 0, [])], ships: [ship(1, 'allied', { task_force: 1 }), ship(2, 'japanese', 'sunk')], air_groups: [], bases: [], contacts: { allied: [], japanese: [] }, claims: { allied: [], japanese: [] } },
  });
  const r4 = mk();
  const bad = Core.validateReplay(r4);
  if (bad) { fail('a valid version 4 file was rejected: ' + bad); return; }
  const m = Core.buildModel(r4);
  assert(m.steps.length === 3 && m.states.length === 3, 'version 4: expected 3 steps (Start, night, end), got ' + m.steps.length);
  const f = (si, id) => m.states[si].forces.get(id);
  assert(f(0, 1).q === 0 && f(1, 1).q === 1 && f(2, 1).q === 1, 'version 4: force 1 is not where the steps put it at each step');
  assert(JSON.stringify(f(1, 1).trail) === '[[0,0],[1,0]]' && JSON.stringify(f(0, 1).trail) === '[[0,0]]', 'version 4: the trail is not the hexes steamed through: ' + JSON.stringify(f(1, 1).trail));
  assert(m.states[1].moves.get(1).length === 1 && m.states[0].moves.size === 0, 'version 4: the step\'s move is not drawn from its events');
  assert(m.states[1].contacts.allied.size === 1 && m.states[1].contacts.allied.get(1).age_pulses === 0, 'version 4: contact at the night step');
  assert(m.states[2].contacts.allied.size === 0, 'version 4: a removed contact is still there at the end');
  assert(!m.states[1].sunk.has(2) && m.states[2].sunk.has(2), 'version 4: ship 2 sunk at the wrong step');
  assert(m.states[2].shipForce.get(1) === 1 && f(2, 2).ships.length === 0, 'version 4: ships of a force');
  assert(Core.compareWithFinal(m).length === 0, 'version 4: end state differs from final: ' + Core.compareWithFinal(m).join('; '));
  assert(checkSteps(Object.assign({}, r4, { scenario: Object.assign({}, r4.scenario, { days: 1 }) }), m), 'version 4: checkSteps found a fault in the hand-built file');
  // The allied page: its own forces and contacts, nothing of the enemy's
  const sr = Core.sideReplay(Object.assign({}, r4, { side_views: { allied: { events: events.map((e, i) => ({ n: i + 1, log_id: e.id, day: e.day, pulse: e.pulse, kind: e.kind, text: e.text, entities: e.entities, data: e.data, causes: [] })) }, japanese: { events: [] } } }), 'allied');
  const sm = Core.buildModel(sr);
  assert(sm.states.every(st => [...st.forces.keys()].join() === '1' && st.contacts.japanese.size === 0 && !st.shipForce.has(2)), 'version 4: the allied page holds the enemy\'s force, ship or contacts');
  assert(sr.steps.every(e => ((e.changed || {}).task_forces || []).every(t => t.side === 'allied') && ((e.changed || {}).ships || []).every(t => t.side === 'allied')), 'version 4: the allied steps hold an enemy record');
  assert(sm.states[sm.states.length - 1].contacts.allied.size === 0 && sm.states[1].forces.get(1).q === 1, 'version 4: the allied page does not follow its own steps');
  checkSideSteps(r4, 'allied', sr, sm, 'version 4 synthetic allied');
  // A viewer step with no entry is an error the page reports
  const gap = mk(); gap.steps = gap.steps.filter(e => e.phase !== 'end');
  let threw = '';
  try { Core.buildModel(gap); } catch (e) { threw = e.message; }
  assert(/no state for Day 1, end of day/.test(threw), 'version 4: a step with no entry did not give its error: ' + threw);
  // Bad files
  const noSteps = mk(); delete noSteps.steps;
  reject('version 4 without steps', noSteps, 'steps');
  const late = mk(); late.steps[0] = { day: 1, phase: 'orders' };
  reject('steps that do not begin with start', late, 'start');
  const order = mk(); order.steps[3] = { day: 1, phase: 'orders' };
  reject('steps out of order', order, 'not after');
  const noId = mk(); noId.steps[0].changed.ships[0] = { name: 'x' };
  reject('a changed record with no id', noId, 'id');
  const badKind = mk(); badKind.steps[2].changed.task_forces = 'x';
  reject('a changed list that is not a list', badKind, 'not a list');
  const badPhase = mk(); badPhase.steps[1].phase = 'dawn';
  reject('an unknown phase', badPhase, 'phase');
  if (failures === before) console.log('  ok: version 4 file opens from its steps, the allied page is cut, a missing step is an error');
}

// ---- A hand-built version 3 replay: one strike that found nothing, asserted exactly ----
function syntheticAir() {
  console.log('synthetic air layer');
  const dirs = [[1, 0], [1, -1], [0, -1], [-1, 0], [-1, 1], [0, 1]];
  const hex = (q, r) => ({ q, r, kind: 'deep_sea', land_percent: 0, closed: [], port_size: 0, airfield_size: 0, anchorage: false, terrain: null });
  const grp = (id, name, side, ship, mission, extra) => Object.assign({ id, name, side, ship, aircraft_type: 'Test Type', aircraft_type_id: side === 'allied' ? 2000 : 2001, ready: 10, mission, arc: null }, extra || {});
  const tf = (id, side, q, r, ship) => ({ id, name: 'TF' + id, side, position: { q, r }, ships: [ship], readiness: 'normal', alerted: false, mission: 'carrier', status: 'idle' });
  const types = [{ id: 2000, name: 'Allied Type', strike_radius_nm: 100, extended_reach_nm: 130, search_range_nm: 150 }, { id: 2001, name: 'Japanese Type', strike_radius_nm: 90, extended_reach_nm: 90, search_range_nm: 90 }];
  const raw = [
    // [pulse, kind, data, causes, entities]
    ['night', 'order_accepted', { order: 1, target: 'air_group:10' }, [{ order: 1 }], ['air_group:10']],
    ['morning', 'search_attempt', { air_group: 11, task_force: 2, distance_nm: 80, chance: 0.4, sighted: false }, [], ['air_group:11', 'task_force:2']],
    ['morning', 'search_attempt', { air_group: 11, task_force: 2, distance_nm: 60, chance: 0.5, sighted: true }, [], ['air_group:11', 'task_force:2']],
    ['morning', 'report_received', { side: 'allied', contact: 1, hex: [3, 0], ship_count: 1, kind: 'cruiser', created: true }, [{ event: 3 }], ['contact:1']],
    ['afternoon', 'strike_launched', { air_group: 10, contact: 1, hex: [3, 0], aircraft: 12, distance_nm: 120 }, [{ event: 4 }], ['air_group:10', 'contact:1']],
    ['afternoon', 'escorts_sent', { air_group: 12, with_air_group: 10, aircraft: 4 }, [{ event: 5 }], ['air_group:12', 'air_group:10']],
    ['afternoon', 'strike_found_nothing', { air_group: 10, hex: [3, 0] }, [{ event: 5 }], ['air_group:10']],
    ['afternoon', 'contact_not_there', { side: 'allied', contact: 1, hex: [3, 0] }, [{ event: 7 }], ['contact:1']],
    ['afternoon', 'aircraft_ditched', { air_group: 10, sortie: 'strike', distance_nm: 120, radius_nm: 100, aircraft: 12, chance: 0.5, expected: 6, ditched: 2, pilots_lost: 0, ready: 10 }, [], ['air_group:10']],
  ];
  const events = raw.map((r, i) => ({ id: i + 1, day: 1, pulse: r[0] === 'night' ? null : r[0], kind: r[1], text: '#' + (i + 1) + ' D1 ' + (r[0] || '-') + ' ' + r[1], causes: r[3], entities: r[4], data: r[2] }));
  const replay = {
    format: 'flagplot-replay', version: 3, scenario: { name: 'Synthetic air', placeholder: true, seed: 1, days: 1, path: 'x' },
    map: { hex_nm: 40, real: false, hexes: [hex(0, 0), hex(2, 0), hex(3, 0)] }, directions: dirs, labels: [], bases: [],
    aircraft_types: types,
    initial: {
      task_forces: [tf(1, 'allied', 0, 0, 1), tf(2, 'japanese', 2, 0, 2)], ships: [{ id: 1, name: 'A', side: 'allied' }, { id: 2, name: 'J', side: 'japanese' }],
      air_groups: [grp(10, 'VB', 'allied', 1, 'naval_strike'), grp(11, 'VS', 'allied', 1, 'search'), grp(12, 'VF', 'allied', 1, 'combat_air_patrol'), grp(20, 'J-VB', 'japanese', 2, 'rest')], bases: [],
    },
    orders: [{ id: 1, side: 'allied', day: 1, issuer: 'player', target: 'air_group:10', text: 'reach extended' }], events, days: [],
    final: {
      task_forces: [tf(1, 'allied', 0, 0, 1), tf(2, 'japanese', 2, 0, 2)], ships: [{ id: 1, location: { task_force: 1 } }, { id: 2, location: { task_force: 2 } }],
      air_groups: [grp(10, 'VB', 'allied', 1, 'naval_strike', { reach: 'extended' }), grp(11, 'VS', 'allied', 1, 'search'), grp(12, 'VF', 'allied', 1, 'combat_air_patrol'), grp(20, 'J-VB', 'japanese', 2, 'rest')], bases: [],
      contacts: { allied: [], japanese: [] }, claims: { allied: [], japanese: [] },
    },
  };
  // What the engine gives the allied side of these events: the attempt cut to its group, only when it sighted; no miss.
  // [log id, the side's number, data if cut, causes in the side's numbers, cut]
  const mine = [[3, 2, { air_group: 11 }, [], true], [4, 3, null, [{ event: 2 }]], [5, 4, null, [{ event: 3 }]], [6, 5, null, [{ event: 4 }]], [7, 6, null, [{ event: 4 }]], [8, 7, null, [{ event: 6 }]], [9, 8, null, []]];
  const sideEvents = [[1, 1], ...mine.map(m => [m[0], m[1]])].map(([logId, n]) => {
    const e = events[logId - 1], m = mine.find(x => x[0] === logId);
    return { n, log_id: logId, day: 1, pulse: e.pulse, kind: e.kind, text: '#' + n + ' D1 ' + (e.pulse || '-') + ' ' + e.kind, cut: !!(m && m[4]), causes: m && m[3] ? m[3] : [{ order: 1 }], entities: e.entities.filter(x => !/task_force:2/.test(x)), data: m && m[2] ? m[2] : e.data, hidden_causes: false };
  });
  replay.side_views = { allied: { events: sideEvents }, japanese: { events: [] } };
  const bad = Core.validateReplay(replay);
  if (bad) { fail('synthetic air replay rejected: ' + bad); return; }
  const before = failures;
  const um = Core.buildModel(replay);
  const stepIx = label => um.steps.findIndex(s => s.label.startsWith(label));
  const morning = um.states[stepIx('Day 1, morning')].air, afternoon = um.states[stepIx('Day 1, afternoon')].air;
  assert(um.hasAir, 'umpire: no air activity found');
  // Reach follows the accepted order
  assert(um.states[0].reach.get(10) === 'normal' && um.states[um.states.length - 1].reach.get(10) === 'extended' && um.states[um.states.length - 1].reach.get(11) === 'normal', 'umpire: reach of group 10 not tracked from the accepted order');
  // The morning: a miss where the force truly was, and a sighting where the report put it, with the truth beside it
  assert(morning.misses.length === 1 && morning.misses[0].q === 2 && morning.misses[0].r === 0 && morning.misses[0].side === 'allied' && morning.misses[0].event === 2, 'umpire: the miss mark is wrong: ' + JSON.stringify(morning.misses));
  assert(morning.sighted.length === 1 && morning.sighted[0].q === 3 && morning.sighted[0].r === 0 && JSON.stringify(morning.sighted[0].trueHex) === '[2,0]' && morning.sighted[0].event === 4 && morning.sighted[0].attempt === 3, 'umpire: the sighted mark is wrong: ' + JSON.stringify(morning.sighted));
  // The afternoon: one raid from the carrier's hex to the hex it was told of, which found nothing, with the force truly at (2, 0)
  assert(afternoon.raids.length === 1, 'umpire: expected one raid, got ' + afternoon.raids.length);
  const raid = afternoon.raids[0];
  assert(raid.bombers === 12 && raid.escorts === 4 && JSON.stringify(raid.launch) === '[0,0]' && JSON.stringify(raid.target) === '[3,0]' && raid.kind === 'ship' && raid.side === 'allied', 'umpire: raid figures wrong: ' + JSON.stringify(raid));
  assert(raid.outcome === 'nothing' && JSON.stringify(raid.trueHex) === '[2,0]' && raid.trueNote === null, 'umpire: the raid should have found nothing with the force truly at (2,0): ' + JSON.stringify(raid));
  assert(JSON.stringify(raid.strikeGroups) === '[10]' && JSON.stringify(raid.escortGroups) === '[12]', 'umpire: raid groups wrong');
  assert(JSON.stringify(raid.events) === '[5,6,7,8,9]' && raid.keyEvent === 7, 'umpire: raid events or key event wrong: ' + JSON.stringify(raid.events) + ' ' + raid.keyEvent);
  assert(afternoon.tallies.length === 1, 'umpire: expected one tally');
  const t = afternoon.tallies[0];
  assert(t.key === '3,0' && t.raids.length === 1 && t.bombers === 12 && t.escorts === 4 && t.ditched === 2 && t.hits === null && t.sunk === 0 && t.patrolEngaged === 0 && t.gunShots === 0 && t.landed === 0 && JSON.stringify(t.events) === '[5,6,7,8,9]' && t.keyEvent === 7, 'umpire: tally wrong: ' + JSON.stringify(t));
  assert(JSON.stringify(Core.airReach(um, um.states[stepIx('Day 1, afternoon')], 10).type) === JSON.stringify(types[0]) && Core.airReach(um, um.states[stepIx('Day 1, afternoon')], 10).reach === 'extended', 'umpire: airReach is wrong');
  // The allied page: the same raid from its own events, with no truth in it
  const sm = Core.buildModel(Core.sideReplay(replay, 'allied'));
  const sAfternoon = sm.states[sm.steps.findIndex(s => s.label.startsWith('Day 1, afternoon'))].air, sMorning = sm.states[sm.steps.findIndex(s => s.label.startsWith('Day 1, morning'))].air;
  assert(sAfternoon.raids.length === 1 && sAfternoon.raids[0].outcome === 'nothing' && sAfternoon.raids[0].trueHex === undefined && sAfternoon.raids[0].trueNote === undefined, 'allied: the raid should found nothing, with no true hex');
  assert(sAfternoon.raids[0].bombers === 12 && sAfternoon.raids[0].escorts === 4 && JSON.stringify(sAfternoon.raids[0].launch) === '[0,0]' && JSON.stringify(sAfternoon.raids[0].target) === '[3,0]', 'allied: raid figures wrong');
  assert(sAfternoon.tallies.length === 1 && sAfternoon.tallies[0].ditched === 2 && sAfternoon.tallies[0].events.length === 5, 'allied: tally wrong: ' + JSON.stringify(sAfternoon.tallies));
  assert(sMorning.misses.length === 0 && sMorning.sighted.length === 1 && sMorning.sighted[0].trueHex === null && sMorning.sighted[0].q === 3, 'allied: a miss or a true hex on the side page: ' + JSON.stringify(sMorning));
  assert(sm.aircraftTypes.size === 1 && sm.aircraftTypes.has(2000), 'allied: aircraft types are not exactly its own');
  const japan = Core.buildModel(Core.sideReplay(replay, 'japanese'));
  assert(japan.states.every(st => st.air.raids.length === 0 && st.air.sighted.length === 0 && st.air.misses.length === 0) && japan.aircraftTypes.size === 1 && japan.aircraftTypes.has(2001), 'japanese: the enemy\'s raid is on its page, or it carries the enemy\'s aircraft types');
  checkAirModel(replay, um, 'synthetic umpire'); checkAirModel(replay, sm, 'synthetic allied');
  if (failures === before) console.log('  ok: one strike that found nothing: raid, tally, true hex, and the allied page without it');
}

// ---- The measuring tool against the engine's own answers ----
// viewer/measure_fixture.json is written by the engine (crates/sim-cli/src/export/measure_fixture.rs).
function checkMeasuring() {
  console.log('measuring tool vs the engine (viewer/measure_fixture.json)');
  let fx;
  try { fx = JSON.parse(fs.readFileSync(path.join(__dirname, 'measure_fixture.json'), 'utf8')); }
  catch (e) { fail('cannot read viewer/measure_fixture.json: ' + e.message); return; }
  assert(fx.format === 'flagplot-measure-fixture' && fx.version === 2, 'unknown fixture format');
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
      assert(got.sea !== null && JSON.stringify(got.sea.charges) === JSON.stringify(c.charges), label + ': page step charges ' + (got.sea && JSON.stringify(got.sea.charges)) + ', engine ' + JSON.stringify(c.charges));
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
syntheticAir();
checkMeasuring();
console.log(failures ? '\n' + failures + ' check(s) FAILED' : '\nAll checks passed.');
process.exit(failures ? 1 : 0);
