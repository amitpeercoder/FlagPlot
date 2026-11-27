// The worker's door, the page's own (decision 0229, rulings 15, 16 and 18).
//
// `table/index.html` asks the game through one function, `reach(path, body)`. In the
// browser's build the build script turns one marked line of the page into the script
// tag of this file, and this file answers every path the bridge answers for a person
// against the engine, from the worker that holds the game and from static files that
// lie beside the page. It sets `window.flagplotDoor` and nothing else.
//
// This file is the page's, and plays: it is given a seat's answers and nothing else.
// It never names the module or the data the worker hands to it; it makes the worker
// and speaks to it in the seven words of the module's door, in the order sent. It is
// the only thing that makes a worker (ruling 24). Every address in it is relative to
// this file, so the folder plays from any host that serves it, or from a folder.
//
// A game lives as long as the page: reload the page and the game is begun again from
// its first evening (kept games are the next stage). What was chosen at the new game
// screen is kept for the tab, so that a reload begins the same game; `/new` forgets
// it and the screen comes back.

(function () {
  'use strict';

  var HERE = document.currentScript.src;
  function at(path) { return new URL(path, HERE).href; }
  var SIDES = ['allied', 'japanese'];
  var CHOSEN = 'flagplot-chosen';
  var COUNT = 'flagplot-games-begun';
  // What a player is told when the game's worker has stopped (ruling 27): one fixed
  // sentence, never the engine's own words.
  var STOPPED = 'The game has stopped on an internal error. Begin it again from the new game screen.';

  function getJson(path) {
    return fetch(at(path)).then(function (r) {
      if (!r.ok) throw new Error(path + ' is not there (' + r.status + ')');
      return r.json();
    });
  }

  // ---- what was chosen, kept for this tab ----
  var chosenHere = null;
  function chosen() {
    if (chosenHere) return chosenHere;
    try { chosenHere = JSON.parse(sessionStorage.getItem(CHOSEN) || 'null'); } catch (e) {}
    return chosenHere;
  }
  function choose(value) {
    chosenHere = value;
    try { if (value) sessionStorage.setItem(CHOSEN, JSON.stringify(value)); else sessionStorage.removeItem(CHOSEN); } catch (e) {}
  }
  // A number for each game begun, so that what the page keeps for one game is named by it.
  var counted = 0;
  function nextNumber() {
    try { counted = parseInt(localStorage.getItem(COUNT) || '0', 10) || 0; } catch (e) {}
    counted += 1;
    try { localStorage.setItem(COUNT, String(counted)); } catch (e) {}
    return counted;
  }

  // ---- the list of games the build offers ----
  var listed = null;
  function list() {
    if (!listed) listed = getJson('scenarios.json').then(function (a) { return a.scenarios; });
    return listed;
  }
  function entryOf(id) { return list().then(function (all) { return all.filter(function (e) { return e.id === id; })[0]; }); }
  // A scenario as the screen shows it: no file named.
  function offered(entry) {
    return {
      id: entry.id, name: entry.name, dates: entry.dates || '', days: entry.days, lines: entry.lines || [], sides: entry.sides || {},
      pairings: entry.pairings.map(function (p) { var q = {}; Object.keys(p).forEach(function (k) { if (k !== 'file') q[k] = p[k]; }); return q; })
    };
  }
  function stem(path) { return path.split('/').pop().replace(/\.[^.]*$/, ''); }

  // ---- the worker: one game, one seat, one worker ----
  var held = null;     // { seat, ready, config, entry, number, files } of the game in hand
  function Seat() {
    var self = this, worker = new Worker(at('worker.js'), { type: 'module' });
    self.worker = worker; self.waiting = {}; self.next = 0; self.stopped = false;
    function stop(words) {
      self.stopped = true;
      Object.keys(self.waiting).forEach(function (id) { self.waiting[id].reject(new Error(words)); });
      self.waiting = {};
    }
    worker.onmessage = function (m) {
      var w = self.waiting[m.data.id]; if (!w) return;
      delete self.waiting[m.data.id];
      if (m.data.error !== undefined) w.reject(new Error(m.data.error)); else w.resolve(m.data.ok);
    };
    worker.onerror = function () { stop(STOPPED); };
    worker.onmessageerror = function () { stop(STOPPED); };
    // One message at a time, answered in the order sent: the worker takes them so.
    self.ask = function (fn, args) {
      if (self.stopped) return Promise.reject(new Error(STOPPED));
      return new Promise(function (resolve, reject) {
        var id = ++self.next; self.waiting[id] = { resolve: resolve, reject: reject };
        worker.postMessage({ id: id, fn: fn, args: args || [] });
      });
    };
    self.end = function () { stop(STOPPED); worker.terminate(); };
  }
  function endGame() { if (held) { held.seat.end(); held = null; } }

  // Opens the game that was chosen, once; a game whose worker has failed is never used again.
  function ensure() {
    var c = chosen();
    if (!c) return Promise.reject(new Error('no game is in hand: choose one on the new game screen'));
    // The game in hand is the one chosen: while it stands it is used, and once its worker has
    // stopped it stays stopped until the player begins again (a new number), never begun anew
    // under him.
    if (held && held.number === c.number) {
      if (held.seat.stopped) return Promise.reject(new Error(STOPPED));
      return held.ready;
    }
    endGame();
    var game = held = { seat: new Seat(), number: c.number, entry: null, config: null, ready: null };
    game.ready = entryOf(c.scenario).then(function (entry) {
      game.entry = entry;
      var side = SIDES.filter(function (s) { return c[s] === 'person'; })[0];
      var file = entry.pairings.filter(function (p) { return p.allied === c.allied && p.japanese === c.japanese; })[0].file;
      var directives = (entry.directives || {})[side];
      return Promise.all([
        game.seat.ask('begin', [JSON.stringify({ scenario: file, seat: side })]),
        directives ? getJson('directives/' + directives.split('/').pop()) : []
      ]).then(function (a) {
        game.config = {
          seat: side, start: entry.start || '', islands: entry.islands || [], directives: a[1],
          home: { scenario: entry.name, kind: 'engine', agents: [], number: c.number }
        };
        // A rung of the ladder (decision 0237) gives the page its mission, the entry's, unchanged.
        if (entry.mission) game.config.mission = entry.mission;
        return game;
      });
    }).catch(function (e) { if (held === game) { game.seat.end(); held = null; } throw e; });
    return game.ready;
  }

  // ---- the chart: files made when the build was made, for the map of the game in hand ----
  function chart(part) {
    var c = chosen();
    if (!c) return Promise.resolve({ refused: 'no game is in hand: choose one on the new game screen' });
    return entryOf(c.scenario).then(function (entry) {
      return getJson('chart/' + stem(entry.map) + '.json');
    }).then(function (all) {
      return all[part] === undefined || all[part] === null ? (part === 'frame' ? null : { refused: 'this map has no ' + part }) : all[part];
    });
  }

  var NOT_HERE = 'This build plays a person against the engine. Watching a game and playing against an agent are at the table, with the bridge.';

  function reach(path, body) {
    var name = path.split('?')[0];
    switch (name) {
      case '/config':
        if (!chosen()) return Promise.resolve({ screen: true });
        return ensure().then(function (g) { return g.config; });
      case '/scenarios':
        return list().then(function (all) { return { scenarios: all.map(offered) }; });
      case '/begin':
        return list().then(function (all) {
          var entry = all.filter(function (e) { return e.id === (body || {}).scenario; })[0];
          if (!entry) return { refused: 'no such scenario on the list' };
          var pairing = entry.pairings.filter(function (p) { return SIDES.every(function (s) { return p[s] === body[s]; }); })[0];
          if (!pairing) return { refused: 'that game is not offered for this scenario' };
          endGame();
          choose({ scenario: entry.id, allied: pairing.allied, japanese: pairing.japanese, number: nextNumber() });
          return { ok: true };
        });
      case '/restart':
        if (!chosen()) return Promise.resolve({ refused: 'no game is in hand: choose one on the new game screen' });
        endGame();
        var c = chosen();
        choose({ scenario: c.scenario, allied: c.allied, japanese: c.japanese, number: nextNumber() });
        return Promise.resolve({ ok: true });
      case '/new':
        endGame(); choose(null);
        return Promise.resolve({ ok: true });
      case '/map': return chart('hexes');
      case '/frame': return chart('frame');
      case '/coast': return chart('coast');
      case '/names': return chart('names');
      case '/tools':
        return ensure().then(function (g) { return g.seat.ask('tools'); }).then(function (text) {
          return JSON.parse(text).tools.map(function (t) { return t.name; });
        });
      case '/call':
        if (!body || typeof body.tool !== 'string') return Promise.resolve({ refused: 'a call is {tool, arguments}' });
        if ('side' in body) return Promise.resolve({ refused: 'there is no watcher in this game' });
        return ensure().then(function (g) {
          return g.seat.ask('call', [body.tool, JSON.stringify(body.arguments || {})]);
        }).then(function (text) { return JSON.parse(text); })
          // A fault of the door's own use is words of the door's, which may be shown;
          // a worker that has stopped says its one sentence.
          .catch(function (e) { return { refused: e.message }; });
      default:
        return Promise.resolve({ refused: NOT_HERE });
    }
  }

  window.flagplotDoor = { reach: reach };
})();
