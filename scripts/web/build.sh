#!/bin/sh
# Makes the browser's build of the game (decision 0229, stage 3): the module,
# `target/web/module/` (`sim_web.js` and `sim_web_bg.wasm`), and the pack,
# `target/web/pack.json`, which holds the data of every game the build offers,
# and (stage 4) the site, `target/web/site/`, that holds them with the page.
# It also builds, in the same `web` profile, the native `door` and `sim-cli`
# that `web/check.mjs` compares the module with (ruling 29).
#
# Run from anywhere; it works from the top of the repository. Needs the
# pinned toolchain with the target `wasm32-unknown-unknown` (rust-toolchain.toml
# names it), `wasm-bindgen-cli` at the version `crates/sim-web/Cargo.toml` pins,
# and node. Stops if the tool and the crate are not the same version.

set -eu
cd "$(dirname "$0")/../.."

out=target/web

pinned=$(sed -n 's/^wasm-bindgen = "=\([0-9][0-9.]*\)".*/\1/p' crates/sim-web/Cargo.toml)
if [ -z "$pinned" ]; then
    echo "crates/sim-web/Cargo.toml does not pin wasm-bindgen to one exact version (=x.y.z)." >&2
    exit 1
fi
if ! command -v wasm-bindgen >/dev/null 2>&1; then
    echo "wasm-bindgen is not installed: cargo install wasm-bindgen-cli --version $pinned --locked" >&2
    exit 1
fi
have=$(wasm-bindgen --version | sed 's/^wasm-bindgen //')
if [ "$have" != "$pinned" ]; then
    echo "wasm-bindgen is $have and the crate is pinned to $pinned: cargo install wasm-bindgen-cli --version $pinned --locked --force" >&2
    exit 1
fi

# The module. The stack is set to a native program's 8 MB, and not left to the
# linker's default of 1 MB (ruling 7); the target puts the stack first in
# memory, so that running out of it is a trap and never a write over the game's
# data (`web/check.mjs` reads both from the module).
RUSTFLAGS="-C link-arg=-zstack-size=8388608" \
    cargo build --locked --profile web --target wasm32-unknown-unknown -p sim-web --lib
rm -rf "$out/module"
mkdir -p "$out/module"
wasm-bindgen --target web --out-dir "$out/module" --out-name sim_web \
    target/wasm32-unknown-unknown/web/sim_web.wasm

# What the module is compared with: the same door, built natively in the same
# profile, and the program that makes the pack.
cargo build --locked --profile web -p sim-web --bin door
cargo build --locked --profile web -p sim-cli

# The pack: the files every offered game's load reads, made by `sim-data` from
# the repository's own paths (rulings 9 and 10).
# shellcheck disable=SC2046
"$out/sim-cli" pack $(node web/offered.mjs) --out "$out/pack.json"

# The site (decision 0229, stage 4): a folder of static files that is the game,
# opened from any static host or from a folder served on one's own machine, and
# fetching nothing from anywhere else. The page is `table/index.html` with its one
# marked line made the script tag of `web/door.js`; the list of games, the chart of
# each map and the directives are made by the bridge's own readers (rulings 14, 16).
site="$out/site"
rm -rf "$site"
mkdir -p "$site/module"
cp "$out/module/sim_web.js" "$out/module/sim_web_bg.wasm" "$site/module/"
cp "$out/pack.json" "$site/pack.json"
cp web/door.js web/worker.js "$site/"
cp -R table/vendor "$site/vendor"
python3 -I scripts/web/site.py "$site"

ls -l "$out/module/sim_web_bg.wasm" "$out/pack.json"
echo "the site is $site: serve it with, for example, python3 -m http.server -d $site PORT"
