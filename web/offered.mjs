// The scenario files the build offers a person to play against the engine
// (decision 0229, rulings 29 and 36): every pairing of `table/scenarios.json`
// in which one side is a person and the other the engine, in the order the
// list gives them, each file once. Read by `scripts/web/build.sh`, which packs
// them, and by `web/check.mjs`, which holds the game files to them.
//
// Run directly, it prints the files one to a line.

import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

export const ROOT = fileURLToPath(new URL('..', import.meta.url));

export function offered() {
  const list = JSON.parse(readFileSync(`${ROOT}table/scenarios.json`, 'utf8'));
  const files = [];
  for (const scenario of list.scenarios) {
    for (const pairing of scenario.pairings) {
      const sides = [pairing.allied, pairing.japanese].sort();
      if (sides[0] === 'engine' && sides[1] === 'person' && !files.includes(pairing.file)) {
        files.push(pairing.file);
      }
    }
  }
  return files;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  for (const file of offered()) console.log(file);
}
