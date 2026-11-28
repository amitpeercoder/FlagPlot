// The check of the browser's build (decision 0229, stage 3): the module, run
// in node on the very files the build ships, against the native `door`
// program built from the same commit in the same profile, over its line
// protocol. Run from anywhere after `sh scripts/web/build.sh`:
//
//     node web/check.mjs
//
// It reads `Cargo.toml` and the shipped files and the games of the
// check in `web/games/`; it reads no Markdown (ruling 47). The expected value of every
// comparison is what the native build gives: no hash is pinned and no number
// is written by hand (ruling 29). Every answer is compared as the string the
// door gave, byte for byte; nothing a door gives is parsed to be compared (an
// answer in a game of the check is parsed only so that the game can go on
// from what it was told).
//
// # The statements this check holds
//
// Written by the architect in decision 0229, copied word for word:
//
// 6. Eastern Solomons, the Allied seat: orders to a task force and to an air
// group on the first evening (a course, a speed, a mission, a search arc),
// then the day ended to the last. Every answer and the saved game are the
// native `door`'s, byte for byte as ruling 29 compares them.
// 7. The 39 days, the Allied seat: an order to the staff to hold Lunga and one
// to control Guadalcanal, each previewed and then signed, a force held and
// released, a limit set and a standing interrupt switched off, then `run` to
// the end with whatever stops it answered by `run` again. Every answer and
// the saved game are the native build's.
// 8. Midway, the Japanese seat: a patrol ordered for the submarines and a
// direct order to one boat, then every day ended. Every answer and the saved
// game are the native build's.
// 9. Midway, the Allied seat, whose file lets a seat stop inside a day:
// `advance` a pulse at a time, an order to a task force given at 0600 and one
// at 1200, an order that acts only at dusk given at 0600, and a dry run of an
// order to the staff at the smallest size. Every answer and the saved game
// are the native build's.
// 10. Each of the twelve offered files, the person's seat ending each day and
// giving nothing: every answer and the saved game are the native build's.
// 11. A game file of statement 7 cut at day 20 and continued natively, and one
// cut natively and continued in the module, each end in the saved game of
// statement 7.
// 12. The module exports the seven functions of ruling 17, each a function,
// and no other name but the binding's own: `memory`, names beginning
// `__wbindgen_`, `__wbg_` or `__externref_`, and the globals
// `__abort_handler` and `__instance_terminated`.
// 13. The module refuses a pack whose versions are not its own, and a game
// file whose pin is not the pack's.
// 13a. For the Eastern Solomons game of statement 6, the saved game `sim-cli
// play --export-save` writes from the door's game file is the door's, byte
// for byte.
//
// And, from stage 4 (the page on the worker's door), written by the architect in
// the same record and copied word for word:
//
// 14. With the bridge, the page behaves as before: `table/check_screen.py` and
// `table/check_watch.py` pass unchanged but for the vendored files' addresses.
// 15. `worker.js` passes the scan of ruling 24; `index.html` and `door.js` name
// neither the module nor the pack; the site's page is `table/index.html` but
// for the one marked line.
// 16. The folder served from one's own machine with the network off plays
// Midway from the new game screen to "How it ended".
//
// And, from decision 0237 (rung 1 on the table), written by the architect in
// docs/decisions/as-built/0237-the-first-three-rungs.md and copied word for word:
//
// 17. Rung 1, the Allied seat: the reads of a page's load, then the supply
// order for base 2 at a floor of 2 days and an interval of 5, with ship 1 and
// base 1 as its forces, previewed and then signed, leave to launch given, then
// `run` to the end with whatever stops it answered by `run` again, then
// `events` of each day from 1 to 14 as the verdict asks them. Every answer and
// the saved game are the native door's, byte for byte.
//
// And, from decision 0237 (rung 3 on the table), written by the architect in
// the same record and copied word for word:
//
// 18. Rung 3, the Allied seat: the reads of a page's load, then the order to
// take base 2 with task force 1, land unit 1 and base 1 as its forces and the
// risk press, previewed and then signed, leave to launch given, then `run`
// until twelve days are resolved, then the supply order for base 2 at a floor
// of 2 days and an interval of 5, with ship 4 and base 1 as its forces,
// previewed and then signed, then `run` to the end with whatever stops it
// answered by `run` again, then `events` of each day from 1 to 21 and
// `list_bases` as the verdict asks them. Every answer and the saved game are
// the native door's, byte for byte.
//
// Statement 15 is held here, on the files of the site the build assembled
// (`target/web/site/`). Statements 14 and 16 are not held by CI: 14 is the
// developer's run of the two scripts, which CI does not run, and 16 is held by
// hand in the browsers, both said in the record's "As built" (ruling 48).
//
// Also held here, from the rulings: the `web` profile has `overflow-checks =
// true` written out (ruling 30); the stack is 8 MB and lies first in memory
// (ruling 7); the games of the check in `web/games/` are those of the
// games the list offers a person against the engine (ruling 29).

import { spawn, spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { pathToFileURL } from 'node:url';

import { offered, ROOT } from './offered.mjs';

const OUT = `${ROOT}target/web/`;
const FUNCTIONS = ['versions', 'begin', 'tools', 'call', 'game_file', 'resume', 'saved_game'];
const STARTED = Date.now();

let failures = 0;
function fail(words) {
  failures += 1;
  console.log(`FAIL ${words}`);
}
function ok(words) {
  console.log(`ok   ${words}`);
}
function check(holds, words) {
  if (holds) ok(words);
  else fail(words);
}
function stop(words) {
  fail(words);
  console.log(`\n${failures} check(s) failed after ${seconds()} s.`);
  process.exit(1);
}
function seconds() {
  return ((Date.now() - STARTED) / 1000).toFixed(1);
}

// ---------------------------------------------------------------------------
// The manifest: overflow checks on in the `web` profile (ruling 30).
// ---------------------------------------------------------------------------

function profileWeb() {
  const text = readFileSync(`${ROOT}Cargo.toml`, 'utf8');
  const start = text.indexOf('[profile.web]');
  if (start < 0) return null;
  const rest = text.slice(start + '[profile.web]'.length);
  const end = rest.search(/^\[/m);
  return (end < 0 ? rest : rest.slice(0, end))
    .split('\n')
    .map((l) => l.replace(/#.*/, '').trim())
    .filter(Boolean);
}
{
  const lines = profileWeb();
  check(lines !== null, 'Cargo.toml has the profile `web`');
  check(
    lines?.includes('inherits = "release"') && lines.includes('overflow-checks = true'),
    'the `web` profile inherits release and writes overflow-checks = true out again',
  );
}

// ---------------------------------------------------------------------------
// The module's own facts: exports (statement 12), stack (ruling 7).
// ---------------------------------------------------------------------------

const wasmBytes = readFileSync(`${OUT}module/sim_web_bg.wasm`);
const compiled = new WebAssembly.Module(wasmBytes);

{
  // The allowed set (statement 12, as amended by the rulings on pull request
  // 733): the seven, each a function; `memory`, the one memory; names the
  // binding makes by prefix; and the two globals, by exact name and kind.
  const exported = WebAssembly.Module.exports(compiled);
  const kinds = Object.fromEntries(exported.map((e) => [e.name, e.kind]));
  const wrong = FUNCTIONS.filter((f) => kinds[f] !== 'function');
  const memories = exported.filter((e) => e.kind === 'memory');
  const allowed = (e) =>
    FUNCTIONS.includes(e.name) ||
    (e.name === 'memory' && e.kind === 'memory') ||
    (/^(__wbindgen_|__wbg_|__externref_)/.test(e.name) && ['function', 'table', 'global'].includes(e.kind)) ||
    (['__abort_handler', '__instance_terminated'].includes(e.name) && e.kind === 'global');
  const extra = exported.filter((e) => !allowed(e)).map((e) => `${e.name} (${e.kind})`);
  check(wrong.length === 0, `the module exports each of the seven functions of the door, as a function${wrong.length ? ` (not so: ${wrong})` : ''}`);
  check(memories.length === 1 && memories[0].name === 'memory', 'the module exports exactly one memory, named `memory`');
  check(extra.length === 0, `the module exports no name but the binding's own${extra.length ? ` (also ${extra})` : ''}`);
  const glue = await import(`${pathToFileURL(`${OUT}module/sim_web.js`)}?exports`);
  const glueNames = Object.keys(glue).filter((n) => n !== 'default' && n !== 'initSync');
  const glueExtra = glueNames.filter((n) => !FUNCTIONS.includes(n));
  const glueMissing = FUNCTIONS.filter((n) => !glueNames.includes(n));
  check(
    glueExtra.length === 0 && glueMissing.length === 0,
    `the glue exports the seven functions and nothing else but init${glueExtra.length ? ` (also ${glueExtra})` : ''}`,
  );
}

{
  // The stack pointer's first value is the stack's size, and no data lies
  // below it: the stack is first in memory, so that running out of it is a
  // trap and never a write over the game's data.
  const b = wasmBytes;
  let p = 8;
  const leb = () => {
    let r = 0, s = 0, x;
    do { x = b[p++]; r += (x & 0x7f) * 2 ** s; s += 7; } while (x & 0x80);
    return r;
  };
  const sleb = () => {
    let r = 0, s = 0, x;
    do { x = b[p++]; r |= (x & 0x7f) << s; s += 7; } while (x & 0x80);
    if (x & 0x40) r |= -(1 << s);
    return r;
  };
  let stack = null;
  let lowest = Infinity;
  while (p < b.length) {
    const id = b[p++];
    const end = leb() + p;
    if (id === 6) {
      leb(); // count; the first global is the stack pointer
      p += 2; // type, mutability
      if (b[p++] === 0x41) stack = sleb();
    } else if (id === 11) {
      const n = leb();
      for (let i = 0; i < n; i++) {
        const flag = leb();
        if (flag === 0) {
          p++; // i32.const
          lowest = Math.min(lowest, sleb());
          p++; // end
        } else if (flag !== 1) {
          fail(`a data segment of kind ${flag} is not read by this check`);
        }
        const size = leb();
        p += size;
      }
    }
    p = end;
  }
  check(stack === 8388608, `the stack is 8 MB (the stack pointer starts at ${stack})`);
  check(lowest >= stack, `the stack is first in memory: no data below ${stack} (the lowest data is at ${lowest})`);
}

// ---------------------------------------------------------------------------
// The site (statement 15; rulings 14, 16, 23, 24 and 40), on the shipped files.
// ---------------------------------------------------------------------------

{
  const SITE = `${OUT}site/`;
  const read = (name) => readFileSync(`${SITE}${name}`, 'utf8');
  const stripped = (text) => text.replace(/\/\*[\s\S]*?\*\//g, '').replace(/^\s*\/\/.*$/gm, '').replace(/\s\/\/ .*$/gm, '');
  const shipped = ['index.html', 'door.js', 'worker.js', 'pack.json', 'scenarios.json', 'module/sim_web.js', 'module/sim_web_bg.wasm', 'vendor/vendor.toml', 'vendor/three.min.js', 'vendor/fonts.css'];
  const missing = shipped.filter((f) => !existsSync(`${SITE}${f}`));
  check(missing.length === 0, `the site holds the page, the door, the worker, the module, the pack, the list and the vendored files${missing.length ? ` (missing ${missing})` : ''}`);
  if (missing.length) stop('the site is not assembled: run scripts/web/build.sh');

  // The scan of ruling 24 on `worker.js`.
  const worker = stripped(read('worker.js'));
  const listed = /const MESSAGES = \[([^\]]*)\]/.exec(worker);
  const names = listed ? [...listed[1].matchAll(/'([^']*)'/g)].map((m) => m[1]) : [];
  check(JSON.stringify([...names].sort()) === JSON.stringify([...FUNCTIONS].sort()), "worker.js lists the seven messages of the door and no others (`const MESSAGES`)");
  const words = [...worker.matchAll(/'([a-z_]+)'/g)].map((m) => m[1]).filter((w) => !FUNCTIONS.includes(w));
  check(words.length === 0, `worker.js names no message in its source but those seven${words.length ? ` (also ${words})` : ''}`);
  const means = ['JSON.parse', 'TextDecoder', '.text(', '.json(', '.blob(', '.formData(', '.getReader('].filter((w) => worker.includes(w));
  check(means.length === 0, `worker.js has no means to read an answer or the pack: none of JSON.parse, TextDecoder, .text(, .json(${means.length ? ` (found ${means})` : ''}`);
  check(worker.includes('.arrayBuffer()') && (worker.match(/\bfetch\(/g) || []).length === 1, 'worker.js takes the pack only as arrayBuffer(): one fetch, read so');
  check(!/new (Shared)?Worker\(/.test(worker), 'worker.js makes no worker');

  // The page and the door name neither the module nor the pack; the door alone makes a worker.
  const page = read('index.html');
  const doorJs = read('door.js');
  const names2 = ['sim_web', 'pack.json'];
  const named = ['index.html', 'door.js'].flatMap((f) => names2.filter((n) => read(f).includes(n)).map((n) => `${f}: ${n}`));
  check(named.length === 0, `index.html and door.js name neither the module nor the pack${named.length ? ` (${named})` : ''}`);
  check(!/new (Shared)?Worker\(/.test(page) && (stripped(doorJs).match(/new Worker\(/g) || []).length === 1, 'door.js is the only thing that makes a worker, and makes it once');

  // The site's page is the table's, but for the one marked line.
  const table = readFileSync(`${ROOT}table/index.html`, 'utf8').split('\n');
  const made = page.split('\n');
  const differing = table.map((l, i) => [l, made[i]]).filter(([a, b]) => a !== b);
  check(
    table.length === made.length && differing.length === 1 && differing[0][0] === '<!--@door-->' && differing[0][1] === '<script src="door.js"></script>',
    "the site's page is table/index.html but for the one marked line, which is the script tag of door.js",
  );

  // Nothing is loaded from another site (ruling 40).
  const elsewhere = ['index.html', 'door.js', 'worker.js', 'vendor/fonts.css'].flatMap((f) => ['cdnjs.cloudflare.com', 'fonts.googleapis.com', 'fonts.gstatic.com'].filter((h) => read(f).includes(h)).map((h) => `${f}: ${h}`));
  check(elsewhere.length === 0, `the page, the door, the worker and the fonts' stylesheet name no other site's address${elsewhere.length ? ` (${elsewhere})` : ''}`);

  // The vendored files are those vendor.toml names, by SHA-256, and no others (ruling 40).
  const toml = readFileSync(`${ROOT}table/vendor/vendor.toml`, 'utf8').split('\n');
  const pairs = [];
  let path = null;
  for (const line of toml) {
    const m = /^(path|licence_file|sha256|licence_sha256) = "(.*)"$/.exec(line);
    if (!m) continue;
    if (m[1] === 'path' || m[1] === 'licence_file') path = m[2];
    else pairs.push([path, m[2]]);
  }
  const hash = (file) => createHash('sha256').update(readFileSync(`vendor/${file}`.replace(/^/, SITE))).digest('hex');
  const wrong = pairs.filter(([f, h]) => !existsSync(`${SITE}vendor/${f}`) || hash(f) !== h).map(([f]) => f);
  check(pairs.length > 0 && wrong.length === 0, `each of the ${pairs.length} vendored files is what vendor.toml says, by SHA-256${wrong.length ? ` (not so: ${wrong})` : ''}`);
  const walk = (dir) => readdirSync(dir).flatMap((n) => (statSync(`${dir}/${n}`).isDirectory() ? walk(`${dir}/${n}`).map((f) => `${n}/${f}`) : [n]));
  const unlisted = walk(`${SITE}vendor`).filter((f) => f !== 'vendor.toml' && !pairs.some(([p]) => p === f));
  check(unlisted.length === 0, `no vendored file is left out of vendor.toml${unlisted.length ? ` (${unlisted})` : ''}`);

  // The list holds the offered games and each game's chart and directives are there.
  const list = JSON.parse(read('scenarios.json')).scenarios;
  const files = [...new Set(list.flatMap((e) => e.pairings.map((p) => p.file)))];
  check(JSON.stringify([...files].sort()) === JSON.stringify([...offered()].sort()), "the site's list offers the scenario files the check's games are held to");
  const absent = list.flatMap((e) => [
    `chart/${e.map.split('/').pop().replace(/\.[^.]*$/, '')}.json`,
    ...Object.values(e.directives || {}).map((d) => `directives/${d.split('/').pop()}`),
  ]).filter((f) => !existsSync(`${SITE}${f}`));
  check(absent.length === 0, `the chart of each map and the directives of each game the list offers are in the site${absent.length ? ` (missing ${absent})` : ''}`);
}

// ---------------------------------------------------------------------------
// The two owed by the rulings on pull request 734, run on the shipped files with stubs
// for what a browser gives (the module, the worker, the page's storage).
// ---------------------------------------------------------------------------

{
  const SITE = `${OUT}site/`;
  const ENDED = 'The game has stopped on an internal error. Begin it again from the new game screen.';

  // `web/worker.js`: "a throw from the call `door[fn](...)` ends the module unless it is a plain
  // `Error` (its prototype is `Error.prototype`): every `RuntimeError`, `RangeError` and anything
  // that is not an `Error` sets `ended`, and the answer is the fixed sentence. Errors thrown before
  // the call (the module or the data not there, a message that is not one of the seven) stay as they are."
  const dir = mkdtempSync(`${tmpdir()}/flagplot-worker-`);
  mkdirSync(`${dir}/module`);
  writeFileSync(`${dir}/worker.js`, readFileSync(`${SITE}worker.js`));
  writeFileSync(
    `${dir}/module/sim_web.js`,
    `export default async function init() {}
export const versions = () => '{}';
export const begin = () => {};
export const tools = () => '{"tools":[]}';
export const game_file = () => '';
export const resume = () => {};
export const saved_game = () => '';
export function call(tool) {
  if (tool === 'plain') throw new Error('the words of the door');
  if (tool === 'range') throw new RangeError('Maximum call stack size exceeded');
  if (tool === 'trap') throw new WebAssembly.RuntimeError('unreachable');
  if (tool === 'string') throw 'not an error';
  return '{"ok":1}';
}
`,
  );
  const sent = [];
  globalThis.self = { postMessage: (m) => sent.push(m) };
  globalThis.fetch = async () => ({ ok: true, arrayBuffer: async () => new ArrayBuffer(2) });
  await import(pathToFileURL(`${dir}/worker.js`));
  const ask = async (id, fn, args = []) => {
    await globalThis.self.onmessage({ data: { id, fn, args } });
    return sent.find((m) => m.id === id);
  };
  const first = await ask(1, 'call', ['plain', '{}']);
  check(first.error === 'the words of the door', 'worker.js: a plain Error from the call is the door\'s own words');
  const after = await ask(2, 'call', ['any', '{}']);
  check(after.ok === '{"ok":1}', 'worker.js: a plain Error does not end the module');
  const before = await ask(3, 'nothing', []);
  check(before.error === 'not a message of the door', 'worker.js: a message that is not one of the seven stays as it was, and does not end the module');
  check((await ask(4, 'call', ['any', '{}'])).ok === '{"ok":1}', 'worker.js: the module is still used after that');
  const range = await ask(5, 'call', ['range', '{}']);
  check(range.error === ENDED, 'worker.js: a RangeError from the call is the fixed sentence');
  const later = await ask(6, 'call', ['any', '{}']);
  check(later.error === ENDED && later.ok === undefined, 'worker.js: every later message is the fixed sentence, and the module is never used again');
  for (const [tool, name] of [['trap', 'a RuntimeError'], ['string', 'a thrown thing that is not an Error']]) {
    // A worker for each: the module is ended once, for good.
    const again = mkdtempSync(`${tmpdir()}/flagplot-worker-`);
    mkdirSync(`${again}/module`);
    writeFileSync(`${again}/worker.js`, readFileSync(`${dir}/worker.js`));
    writeFileSync(`${again}/module/sim_web.js`, readFileSync(`${dir}/module/sim_web.js`));
    globalThis.self = { postMessage: (m) => sent.push(m) };
    await import(`${pathToFileURL(`${again}/worker.js`)}`);
    sent.length = 0;
    await globalThis.self.onmessage({ data: { id: 1, fn: 'call', args: [tool, '{}'] } });
    check(sent[0].error === ENDED, `worker.js: ${name} from the call is the fixed sentence`);
  }

  // `web/door.js`, `ensure`: "where the game in hand is the one chosen and its seat has stopped,
  // reject with the fixed sentence and make no worker. A new worker is made only for a game with a
  // new number (`/begin`, `/restart`) or when no game is held (the page just loaded)."
  const made = [];
  class StubWorker {
    constructor() {
      made.push(this);
      this.terminated = false;
    }
    postMessage({ id, fn }) {
      const ok = fn === 'tools' ? '{"tools":[{"name":"get_briefing"}]}' : '{"ok":1}';
      queueMicrotask(() => this.onmessage({ data: { id, ok } }));
    }
    terminate() {
      this.terminated = true;
    }
  }
  const store = () => {
    const m = new Map();
    return { getItem: (k) => (m.has(k) ? m.get(k) : null), setItem: (k, v) => m.set(k, String(v)), removeItem: (k) => m.delete(k) };
  };
  globalThis.window = globalThis;
  globalThis.document = { currentScript: { src: 'http://site.test/door.js' } };
  globalThis.Worker = StubWorker;
  globalThis.sessionStorage = store();
  globalThis.localStorage = store();
  globalThis.fetch = async (url) => {
    const path = String(url).replace('http://site.test/', '');
    const file = `${SITE}${path}`;
    return existsSync(file) ? { ok: true, status: 200, json: async () => JSON.parse(readFileSync(file, 'utf8')) } : { ok: false, status: 404 };
  };
  await import(`${pathToFileURL(`${SITE}door.js`)}?check`);
  const reach = globalThis.flagplotDoor.reach;
  const pairing = JSON.parse(readFileSync(`${SITE}scenarios.json`, 'utf8')).scenarios[0].pairings[0];
  const begun = await reach('/begin', { scenario: JSON.parse(readFileSync(`${SITE}scenarios.json`, 'utf8')).scenarios[0].id, allied: pairing.allied, japanese: pairing.japanese });
  check(begun.ok === true && made.length === 0, 'door.js: begin records the choice and makes no worker');
  const config = await reach('/config');
  check(config.home?.number === 1 && made.length === 1, 'door.js: the game is opened, in one worker, when the page asks for it');
  check((await reach('/call', { tool: 'get_briefing', arguments: {} })).ok === 1 && made.length === 1, 'door.js: a call is answered by the same worker');
  made[0].onerror();
  const stopped = await reach('/call', { tool: 'get_briefing', arguments: {} });
  check(stopped.refused === ENDED && made.length === 1, 'door.js: a game whose seat has stopped is refused with the fixed sentence, and no worker is made');
  check(
    (await reach('/tools').then(() => 'answered', (e) => e.message)) === ENDED && (await reach('/config').then(() => 'answered', (e) => e.message)) === ENDED && made.length === 1,
    'door.js: the same for every other path that needs the game, and the game is not begun again',
  );
  await reach('/restart', {});
  const again = await reach('/config');
  check(again.home?.number === 2 && made.length === 2 && made[0].terminated, 'door.js: a new game number makes a new worker');
}

// ---------------------------------------------------------------------------
// The two doors.
// ---------------------------------------------------------------------------

const packPath = `${OUT}pack.json`;
const packBytes = readFileSync(packPath);

/** The native `door` held open: one request a line, one answer a line. */
function nativeDoor(path) {
  const child = spawn(`${OUT}door`, [], { stdio: ['pipe', 'pipe', 'inherit'], cwd: ROOT });
  const waiting = [];
  let buffer = '';
  child.stdout.setEncoding('utf8');
  child.stdout.on('data', (chunk) => {
    buffer += chunk;
    let at;
    while ((at = buffer.indexOf('\n')) >= 0) {
      const line = buffer.slice(0, at);
      buffer = buffer.slice(at + 1);
      waiting.shift()?.(JSON.parse(line));
    }
  });
  child.stdin.on('error', () => {});
  return {
    kind: 'native',
    start(fn, args) {
      return new Promise((resolve) => {
        waiting.push(resolve);
        const list = fn === 'begin' || fn === 'resume' ? [path, ...args] : args;
        child.stdin.write(`${JSON.stringify({ function: fn, args: list })}\n`);
      });
    },
    close() {
      child.stdin.end();
    },
  };
}

let instances = 0;
/** The module, a fresh instance of it: one door for the life of the instance. */
async function moduleDoor(bytes) {
  const glue = await import(`${pathToFileURL(`${OUT}module/sim_web.js`)}?door=${instances++}`);
  glue.initSync({ module: compiled });
  return {
    kind: 'module',
    start(fn, args) {
      try {
        const given = fn === 'begin' || fn === 'resume' ? [bytes, ...args] : args;
        const answer = glue[fn](...given);
        return Promise.resolve({ ok: answer === undefined ? '' : answer });
      } catch (e) {
        // A trap is the module ending (a panic): that is a failure, not an answer.
        if (e instanceof WebAssembly.RuntimeError) throw e;
        return Promise.resolve({ error: String(e.message ?? e) });
      }
    },
    close() {},
  };
}

function same(a, b) {
  return a.ok === b.ok && a.error === b.error;
}

function difference(a, b) {
  const x = a.ok ?? `error: ${a.error}`;
  const y = b.ok ?? `error: ${b.error}`;
  let i = 0;
  while (i < x.length && i < y.length && x[i] === y[i]) i++;
  const show = (s) => JSON.stringify(s.slice(Math.max(0, i - 60), i + 120));
  return `they differ at character ${i} (native ${x.length} long, module ${y.length} long): native ${show(x)}, module ${show(y)}`;
}

/** The same request to the native door and the module; they must answer alike. */
async function both(native, mod, fn, args, label) {
  const pn = native.start(fn, args);
  const pm = mod.start(fn, args);
  const [rn, rm] = await Promise.all([pn, pm]);
  if (!same(rn, rm)) stop(`${label}: ${fn} ${JSON.stringify(args).slice(0, 200)}: ${difference(rn, rm)}`);
  return rn;
}

const parse = (answer) => (answer.ok === undefined ? { error: answer.error } : JSON.parse(answer.ok));

// ---------------------------------------------------------------------------
// A game, played through both doors in step; and played through one.
// ---------------------------------------------------------------------------

const request = (game) => JSON.stringify({ scenario: game.scenario, seat: game.seat });

/** The game played through the native door and the module, call for call. */
async function straight(game, label) {
  const native = nativeDoor(packPath);
  const mod = await moduleDoor(packBytes);
  await both(native, mod, 'versions', [], label);
  await both(native, mod, 'begin', [request(game)], label);
  await both(native, mod, 'tools', [], label);
  const calls = [];
  const steps = game.steps();
  let answer;
  let next = steps.next();
  while (!next.done) {
    const [tool, args] = next.value;
    const text = JSON.stringify(args);
    answer = await both(native, mod, 'call', [tool, text], label);
    calls.push({ tool, args, text, answer });
    next = steps.next(parse(answer));
  }
  const file = await both(native, mod, 'game_file', [], label);
  const saved = await both(native, mod, 'saved_game', [], label);
  if (saved.ok === undefined) stop(`${label}: the game is not over at its end: ${saved.error}`);
  native.close();
  mod.close();
  return { calls, file: file.ok, saved: saved.ok };
}

/** Plays the game through `door` up to and including the first answer `game.cut` names; returns where it stood. */
async function playToTheCut(game, door, straightRun, label) {
  const steps = game.steps();
  let next = steps.next();
  let i = 0;
  for (;;) {
    if (next.done) stop(`${label}: no answer of the game is one to cut at`);
    const [tool, args] = next.value;
    const want = straightRun.calls[i];
    if (want.tool !== tool || want.text !== JSON.stringify(args)) stop(`${label}: call ${i + 1} is not the call of the straight game`);
    const answer = await door.start('call', [tool, want.text]);
    if (!same(answer, want.answer)) stop(`${label}: call ${i + 1} ${tool}: ${difference(want.answer, answer)}`);
    i += 1;
    const parsed = parse(answer);
    const cut = game.cut(tool, parsed);
    next = steps.next(parsed);
    if (cut) return { steps, next, i };
  }
}

/** Finishes a game, begun again from a game file, through `door`, every answer the straight game's. */
async function finish(door, state, straightRun, label) {
  let { steps, next, i } = state;
  while (!next.done) {
    const [tool, args] = next.value;
    const want = straightRun.calls[i];
    if (!want || want.tool !== tool || want.text !== JSON.stringify(args)) stop(`${label}: call ${i + 1} is not the call of the straight game`);
    const answer = await door.start('call', [tool, want.text]);
    if (!same(answer, want.answer)) stop(`${label}: call ${i + 1} ${tool}: ${difference(want.answer, answer)}`);
    i += 1;
    next = steps.next(parse(answer));
  }
  if (i !== straightRun.calls.length) stop(`${label}: the game made ${i} calls and the straight game ${straightRun.calls.length}`);
  const saved = await door.start('saved_game', []);
  if (saved.ok !== straightRun.saved) stop(`${label}: the saved game is not the straight game's: ${difference({ ok: straightRun.saved }, saved)}`);
}

// ---------------------------------------------------------------------------
// The games of the check.
// ---------------------------------------------------------------------------

const files = readdirSync(`${ROOT}web/games`).filter((f) => f.endsWith('.mjs')).sort();
const games = {};
for (const f of files) games[f.replace(/\.mjs$/, '')] = (await import(`${ROOT}web/games/${f}`)).default;

{
  // The files are those of the games the list offers a person against the engine.
  const list = offered();
  const names = list.map((p) => p.split('/').pop().replace(/\.toml$/, ''));
  const plain = Object.keys(games).filter((n) => !n.startsWith('orders-'));
  check(
    JSON.stringify([...plain].sort()) === JSON.stringify([...names].sort()),
    `web/games holds one game of the check for each of the ${names.length} scenario files the list offers, and no other plain one`,
  );
  const scenarios = JSON.parse(readFileSync(`${ROOT}table/scenarios.json`, 'utf8')).scenarios;
  const seatOf = (file) => {
    for (const s of scenarios) {
      for (const p of s.pairings) {
        if (p.file === file && p.allied === 'person' && p.japanese === 'engine') return 'allied';
        if (p.file === file && p.japanese === 'person' && p.allied === 'engine') return 'japanese';
      }
    }
    return null;
  };
  check(
    plain.every((n) => games[n].scenario.endsWith(`/${n}.toml`) && list.includes(games[n].scenario) && games[n].seat === seatOf(games[n].scenario)),
    "each plain file names its scenario and the person's seat as the list gives them",
  );
  check(
    Object.keys(games).filter((n) => n.startsWith('orders-')).every((n) => list.includes(games[n].scenario) && games[n].seat === seatOf(games[n].scenario)),
    'each game of orders is on an offered scenario, from the seat the list gives a person',
  );
  const pack = JSON.parse(packBytes.toString('utf8'));
  const paths = Object.keys(pack.files);
  check(
    list.every((s) => paths.includes(s)),
    'the pack holds every offered scenario file',
  );
}

// ---------------------------------------------------------------------------
// Statements 13, 6 to 10 and 13a, 11.
// ---------------------------------------------------------------------------

{
  // 13. A pack whose versions are not the module's own, and a game file whose
  // pin is not the pack's, are refused, in the native door's words.
  const game = games['orders-eastern-solomons'];
  const packText = packBytes.toString('utf8');
  const dir = mkdtempSync(`${tmpdir()}/flagplot-web-`);
  const refused = async (label, bytes, fn, args) => {
    const path = `${dir}/${label.replace(/\W+/g, '-')}.json`;
    writeFileSync(path, bytes);
    const native = nativeDoor(path);
    const mod = await moduleDoor(bytes);
    const answer = await both(native, mod, fn, args, label);
    const after = await both(native, mod, 'tools', [], label);
    native.close();
    check(answer.error !== undefined, `${label}: refused (${answer.error?.slice(0, 90)})`);
    check(after.error?.includes('no game has begun'), `${label}: the door has no game after`);
    return answer.error;
  };
  const withVersion = (key) => {
    const value = JSON.parse(packText);
    value.versions[key] = typeof value.versions[key] === 'number' ? value.versions[key] + 1 : `${value.versions[key]}-changed`;
    return Buffer.from(JSON.stringify(value));
  };
  // The game file to resume is the first game's own, from a door that has begun it.
  const native = nativeDoor(packPath);
  await native.start('begin', [request(game)]);
  const gameFile = (await native.start('game_file', [])).ok;
  native.close();
  for (const key of ['rules', 'state', 'doctrine']) {
    const bytes = withVersion(key);
    await refused(`a pack of another ${key} version, begin`, bytes, 'begin', [request(game)]);
    await refused(`a pack of another ${key} version, resume`, bytes, 'resume', [gameFile]);
  }
  // One byte of one data file's text changed in the pack after the file was begun.
  const value = JSON.parse(packText);
  const data = Object.keys(value.files).find((p) => p.startsWith(game.scenario.replace(/[^/]*$/, '')) && p.endsWith('search.toml'));
  value.files[data] += ' ';
  await refused('a game file whose pin is not the pack\'s', Buffer.from(JSON.stringify(value)), 'resume', [gameFile]);
}

const runs = {};
const ordered = [
  ...Object.keys(games).filter((n) => n.startsWith('orders-')),
  ...Object.keys(games).filter((n) => !n.startsWith('orders-')),
];
for (const name of ordered) {
  const began = Date.now();
  runs[name] = await straight(games[name], name);
  const statement = {
    'orders-eastern-solomons': '6',
    'orders-guadalcanal-staff': '7',
    'orders-midway-submarines': '8',
    'orders-midway-pulses': '9',
    'orders-ladder-one-ship': '17',
    'orders-ladder-men-then-food': '18',
  }[name] ?? '10';
  ok(`statement ${statement}: ${name}: ${runs[name].calls.length} calls and the saved game (${runs[name].saved.length} characters) are the native door's, byte for byte (${((Date.now() - began) / 1000).toFixed(1)} s)`);
}

{
  // 13a. `sim-cli play --export-save` of the door's game file is the door's saved game.
  const run = runs['orders-eastern-solomons'];
  const dir = mkdtempSync(`${tmpdir()}/flagplot-web-`);
  writeFileSync(`${dir}/game.jsonl`, run.file);
  const done = spawnSync(`${OUT}sim-cli`, ['play', '--game', `${dir}/game.jsonl`, '--export-save', `${dir}/save.json`], { cwd: ROOT, encoding: 'utf8' });
  if (done.status !== 0) stop(`13a: sim-cli play --export-save failed: ${done.stdout}${done.stderr}`);
  const made = readFileSync(`${dir}/save.json`, 'utf8');
  check(made === run.saved, `statement 13a: the saved game \`sim-cli play --export-save\` writes from the door's game file is the door's, byte for byte (${made.length} characters)`);
}

{
  // 11. A game cut at day 20, taken up in the other build.
  const game = games['orders-guadalcanal-staff'];
  const run = runs['orders-guadalcanal-staff'];
  for (const [first, second] of [['module', 'native'], ['native', 'module']]) {
    const label = `statement 11: cut in the ${first} and continued in the ${second}`;
    const began = Date.now();
    const make = (kind) => (kind === 'native' ? nativeDoor(packPath) : moduleDoor(packBytes));
    const a = await make(first);
    const b0 = await a.start('begin', [request(game)]);
    if (b0.error !== undefined) stop(`${label}: begin: ${b0.error}`);
    const at = await playToTheCut(game, a, run, label);
    const file = await a.start('game_file', []);
    a.close();
    const b = await make(second);
    const resumed = await b.start('resume', [file.ok]);
    if (resumed.error !== undefined) stop(`${label}: resume: ${resumed.error}`);
    await finish(b, at, run, label);
    b.close();
    ok(`${label}: cut after call ${at.i} of ${run.calls.length}, every later answer and the saved game are the straight game's (${((Date.now() - began) / 1000).toFixed(1)} s)`);
  }
}

console.log(`\n${failures === 0 ? 'Every check held' : `${failures} check(s) failed`} after ${seconds()} s.`);
process.exit(failures === 0 ? 0 : 1);
