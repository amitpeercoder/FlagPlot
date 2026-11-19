#!/usr/bin/env bash
# Make the files the proof of decision 0159 is read from, and read them, one
# step at a time. It runs nothing unless told which step, and it writes only
# into the output folder, which must be outside the repository.
#
#   scripts/0159-proof/run.sh STEP --out DIR [options]
#
# STEP is one of:
#   build      Builds sim-cli in release into DIR/target: stage 3's
#              `run.sh build`.
#   batch      Arrangements (a), (b) and (c), seeds 1 to N, each made twice:
#              DIR/X.json and DIR/X.again.json (stage 3's `run.sh batch`).
#              Condition 8's margins and the check that the same start
#              gives the same game.
#   exports    Arrangement (a), one export a seed, seeds 1 to N: DIR/a/seedN.json
#              (stage 3's `run.sh exports a`). Conditions 3, 4 and 8.
#   sample     The sample seeds of each arrangement exported twice:
#              DIR/X.sample/ and DIR/X.sample.again/ (stage 3's `run.sh
#              sample`).
#   orders     The six plan orders of condition 6, each as a seat's `order`
#              tool answers it: DIR/orders/*.json and DIR/orders/check.txt,
#              by the ignored helper tests/proof_0159_orders.rs, built into
#              DIR/target.
#   read       Runs the readers on what is there, each into DIR/read/:
#              unused.py (condition 2; needs the build), hunger.py (3),
#              short_lifts.py (4), stage 3's conditions.py on (a) with its
#              batch (8), its margin.py on (a) beside (b) and beside (c) (8),
#              and its same.py on each arrangement's two batches and sample
#              exports. A file not made yet is passed over with a note. It
#              prints counts and never a verdict.
#   all        build, batch, exports, sample, orders and read, in that order.
#
# Options:
#   --out DIR       The output folder, outside the repository (required).
#   --seeds N       Seeds 1 to N for batch and exports (default 1000).
#   --sample "S..." The sample seeds, in quotes (stage 3's default "1 2 3 500
#                   1000", less any above N).
#   --jobs J        Exports run J at a time (default 1): stage 3's --jobs.
#
# Every sim-cli command runs from the top of the repository, as stage 3's
# run.sh runs it. Nothing here is read by a test or a build step.
#
# Disk and time, measured on Nov 19, 2026 on an Apple M3 Pro (11 cores):
# docs/validation/0159-proof-setup.md has the measurements and the estimate
# for 1,000 seeds.
set -euo pipefail

usage() { sed -n '2,42p' "$0" | sed 's/^# \{0,1\}//'; exit 2; }

HERE="$(cd "$(dirname "$0")" && pwd -P)"
REPO="$(cd "$HERE/../.." && pwd -P)"
STAGE3="$REPO/scripts/stage-3-proof"

[ $# -ge 1 ] || usage
STEP="$1"; shift
case "$STEP" in
  build|batch|exports|sample|orders|read|all) ;;
  -h|--help|help) usage ;;
  *) echo "run.sh: unknown step \"$STEP\"" >&2; usage ;;
esac

OUT=""; PASS=()
while [ $# -gt 0 ]; do
  case "$1" in
    --out) OUT="$2"; shift 2 ;;
    --seeds|--sample|--jobs) PASS+=("$1" "$2"); shift 2 ;;
    *) echo "run.sh: unknown argument \"$1\"" >&2; usage ;;
  esac
done
[ -n "$OUT" ] || { echo "run.sh: --out DIR is required" >&2; exit 2; }

# The output folder, made absolute, must not be in the repository, nor in the
# main checkout that holds a worktree. Stage 3's run.sh checks the same for
# the steps it runs.
OUT="$(python3 -I -c 'import os,sys; print(os.path.realpath(os.path.abspath(sys.argv[1])))' "$OUT")"
case "$OUT/" in
  "$REPO"/*) echo "run.sh: $OUT is inside the repository ($REPO); give a folder outside it" >&2; exit 2 ;;
esac
MAIN="$(cd "$REPO" && git rev-parse --path-format=absolute --git-common-dir 2>/dev/null || true)"
if [ -n "$MAIN" ]; then
  MAIN="$(cd "$(dirname "$MAIN")" && pwd -P)"
  case "$OUT/" in
    "$MAIN"/*) echo "run.sh: $OUT is inside the repository's checkout ($MAIN); give a folder outside it" >&2; exit 2 ;;
  esac
fi
mkdir -p "$OUT"

SIM="${SIM_CLI:-$OUT/target/release/sim-cli}"
say() { echo "== $*"; }
stage3() { bash "$STAGE3/run.sh" "$@" --out "$OUT" "${PASS[@]+"${PASS[@]}"}"; }

do_build() { stage3 build; }
do_batch() { stage3 batch a b c; }
do_exports() { stage3 exports a; }
do_sample() { stage3 sample a b c; }

do_orders() {
  say "orders: the six plan orders of condition 6 into $OUT/orders/"
  rm -rf "$OUT/orders"
  (cd "$REPO" && CARGO_TARGET_DIR="$OUT/target" FLAGPILOT_0159_ORDERS_OUT="$OUT/orders" \
    cargo test --release -p scenario-tests --test proof_0159_orders -- --ignored)
  echo "wrote $OUT/orders/ ($(ls "$OUT/orders" | grep -c '\.json$') orders and check.txt)"
}

do_read() {
  local py="python3" x
  mkdir -p "$OUT/read"
  say "read: output in $OUT/read/"
  if [ -x "$SIM" ]; then
    $py "$HERE/unused.py" --sim "$SIM" --list | tee "$OUT/read/unused.txt"
  else
    echo "(no sim-cli at $SIM: unused.py passed over; run the build step)"
  fi
  if [ -d "$OUT/a" ]; then
    $py "$HERE/hunger.py" "$OUT/a" --list --jsonl "$OUT/read/hunger.jsonl" | tee "$OUT/read/hunger.txt"
    $py "$HERE/short_lifts.py" "$OUT/a" --list --jsonl "$OUT/read/short_lifts.jsonl" | tee "$OUT/read/short_lifts.txt"
    local batch=()
    [ -f "$OUT/a.json" ] && batch=(--batch "$OUT/a.json")
    $py "$STAGE3/conditions.py" "$OUT/a" "${batch[@]+"${batch[@]}"}" --list --jsonl "$OUT/read/a.conditions.jsonl" \
      | tee "$OUT/read/a.conditions.txt"
  else
    echo "(no exports of (a) in $OUT/a: hunger.py, short_lifts.py and conditions.py passed over)"
  fi
  for pair in "b Allied" "c Japanese"; do
    set -- $pair
    if [ -f "$OUT/a.json" ] && [ -f "$OUT/$1.json" ]; then
      echo "-- condition 8, the $2 margin: (a) beside ($1)"
      $py "$STAGE3/margin.py" "$OUT/a.json" "$OUT/$1.json" | tee "$OUT/read/margin.a-$1.txt"
    else
      echo "(no batch of (a) and ($1): margin.py for the $2 side passed over)"
    fi
  done
  for x in a b c; do
    if [ -f "$OUT/$x.json" ] && [ -f "$OUT/$x.again.json" ]; then
      local ex=()
      [ -d "$OUT/$x.sample" ] && [ -d "$OUT/$x.sample.again" ] && ex=(--exports "$OUT/$x.sample" "$OUT/$x.sample.again")
      $py "$STAGE3/same.py" --batch "$OUT/$x.json" "$OUT/$x.again.json" "${ex[@]+"${ex[@]}"}" | tee "$OUT/read/$x.same.txt"
    else
      echo "(no two batches of ($x): same.py passed over)"
    fi
  done
}

for step in build batch exports sample orders read; do
  if [ "$STEP" = "$step" ] || [ "$STEP" = all ]; then
    "do_$step"
  fi
done
