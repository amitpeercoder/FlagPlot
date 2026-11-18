#!/bin/sh
# The commands of the proof of decision 0156 (orders inside a day), step 5,
# as they were run on Nov 18, 2026. Run from the top of the repository, with
# OUT a folder outside it and BEFORE a second worktree at the merge of pull
# request 425 (2eb55806), built in release:
#
#   git worktree add --detach BEFORE 2eb55806d0f46769883b1cd899314b32037f0374
#   (cd BEFORE && cargo build --release -p sim-cli)
#
#   sh scripts/orders-inside-proof/run.sh STEP --out OUT [--before BEFORE]
#
# STEP is one of: files, 1, 2, 3, 4, 5, 6, 8, read7. Condition 7's games are
# played by models through `sim-cli play` (condition-7/ holds the briefs and
# the wrapper each called); `read7` reads them once they are over. Nothing
# here is run by a test or a build step.
set -eu
STEP="${1:?a step}"; shift
OUT=""; BEFORE=""
while [ $# -gt 0 ]; do
  case "$1" in
    --out) OUT="$2"; shift 2 ;;
    --before) BEFORE="$2"; shift 2 ;;
    *) echo "unknown argument $1" >&2; exit 2 ;;
  esac
done
[ -n "$OUT" ] || { echo "--out is needed" >&2; exit 2; }
mkdir -p "$OUT"
BIN=target/release/sim-cli
cargo build --release -p sim-cli

case "$STEP" in
  files)
    python3 scripts/orders-inside-proof/make_arrangements.py --check ;;
  1)
    # (a) every generated file is what the engine gives.
    sh scripts/regenerate.sh --check
    # (b) the Eastern Solomons, 1,000 seeds, before decision 0156 and after.
    [ -n "$BEFORE" ] || { echo "--before is needed" >&2; exit 2; }
    (cd "$BEFORE" && target/release/sim-cli batch data/scenarios/05_eastern_solomons.toml \
        --runs 1000 --first-seed 1 --label proof --out "$OUT/before.json")
    $BIN batch data/scenarios/05_eastern_solomons.toml \
        --runs 1000 --first-seed 1 --label proof --out "$OUT/after.json"
    cmp "$OUT/before.json" "$OUT/after.json" && echo "byte for byte the same" ;;
  2)
    PROOF_SEEDS=1-1000 cargo test --release -p sim-agent --test orders_inside_proof \
        condition_2 -- --ignored --nocapture ;;
  3)
    cargo test -p scenario-tests --test scenario_78_orders_inside_a_day ;;
  4)
    $BIN run data/scenarios/78_orders_inside_a_day.toml --save "$OUT/s78.save.json"
    $BIN replay "$OUT/s78.save.json"
    # A copy of the data the save names, under OUT/tree, with the key changed;
    # the save names the scenario by the path relative to where replay runs.
    rm -rf "$OUT/tree"; mkdir -p "$OUT/tree/data/scenarios"
    cp -R data/map data/reference "$OUT/tree/data/"
    cp data/scenarios/78_orders_inside_a_day.toml "$OUT/tree/data/scenarios/"
    TOP=$(pwd)
    (cd "$OUT/tree" && sed -i '' 's/^orders_inside_a_day = true/orders_inside_a_day = false/' \
        data/scenarios/78_orders_inside_a_day.toml && \
        "$TOP/$BIN" replay "$OUT/s78.save.json" || true)
    (cd "$OUT/tree" && python3 - <<'EOF'
p = 'data/scenarios/78_orders_inside_a_day.toml'
t = open(p).read()
a = t.index('# Day 1, at 0600'); b = t.index('# Days 2 and 3')
open(p, 'w').write(t[:a] + t[b:])
EOF
    "$TOP/$BIN" replay "$OUT/s78.save.json" || true) ;;
  5)
    cargo test -p sim-agent --test leaks the_library_code_names_nothing_of_the_truth
    cargo test -p sim-agent --test leaks the_tools_of_a_seat_read_the_world_only_through_the_seats_view
    cargo test --release -p sim-agent --test orders_inside_proof condition_5 -- --nocapture ;;
  6)
    PROOF_OUT="$OUT/c6" cargo test --release -p sim-agent --test orders_inside_proof \
        condition_6 -- --ignored --nocapture ;;
  8)
    PROOF_REPEATS=30 cargo test --release -p sim-agent --test orders_inside_proof \
        condition_8 -- --ignored --nocapture ;;
  read7)
    python3 scripts/orders-inside-proof/read_games.py "$OUT/days" --out "$OUT/read/days.json"
    python3 scripts/orders-inside-proof/read_games.py "$OUT/pulse" --out "$OUT/read/pulse.json" ;;
  *)
    echo "unknown step $STEP" >&2; exit 2 ;;
esac
