// The worker of the browser's build: the umpire's relay (decision 0229, rulings 16, 23 and 24).
//
// It holds the game's module, fetches the data the module plays on and hands that
// to the module as bytes; and it relays the seven functions of the module's door, by
// name, one message at a time, in the order they came. It reads no answer and no
// data: it has no means to (the build's check looks for the words that would, and
// for any message but these seven). It lies in `web/`, with the umpire's tools, and
// not in `table/`, which reads nothing of the umpire's.
//
// A page makes a worker for a game and makes another for the next; a worker whose
// module has ended is never used again and answers every later message with the one
// fixed sentence (ruling 27).

import init, * as door from './module/sim_web.js';

// The messages this worker takes: the seven functions of the module's door, and no others.
const MESSAGES = ['versions', 'begin', 'tools', 'call', 'game_file', 'resume', 'saved_game'];
// A message that carries the data the module plays on is given it here, not by the page.
const WITH_DATA = ['begin', 'resume'];
const ENDED = 'The game has stopped on an internal error. Begin it again from the new game screen.';

let data = null;
let ended = false;

// The module and its data, fetched once; every message waits for them, in the order sent.
const ready = (async () => {
  await init({ module_or_path: new URL('./module/sim_web_bg.wasm', import.meta.url) });
  const response = await fetch(new URL('./pack.json', import.meta.url));
  if (!response.ok) throw new Error('the game\'s data is not there');
  data = new Uint8Array(await response.arrayBuffer());
})();

self.onmessage = async (message) => {
  const { id, fn, args } = message.data;
  try {
    await ready;
    if (ended) throw new Error(ENDED);
    if (!MESSAGES.includes(fn)) throw new Error('not a message of the door');
    const given = WITH_DATA.includes(fn) ? [data, ...args] : args;
    const answer = door[fn](...given);
    self.postMessage({ id, ok: answer === undefined ? '' : answer });
  } catch (e) {
    // A trap is the module ending; a thrown string is the door's own words.
    if (e instanceof WebAssembly.RuntimeError) ended = true;
    self.postMessage({ id, error: ended ? ENDED : String(e && e.message !== undefined ? e.message : e) });
  }
};
