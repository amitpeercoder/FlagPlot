#!/usr/bin/env bash
# Make the files the proof of stage 4 (decision 0161, step 9) is read from and
# read them, one step at a time. It runs nothing unless told which step, and
# it writes only into the output folder, which must be outside the
# repository. The setup it follows is docs/validation/stage-4-proof-setup.md.
#
#   scripts/stage-4-proof/run.sh STEP --out DIR [options]
#
# STEP is one of:
#   check      `make_arrangements.py --check`: (a), (b) and (c) "as made".
#   build      sim-cli in release into DIR/target, and the harness built there
#              (the ignored tests of tests/proof_stage_4_harness.rs).
#   batch      (a), (b) and (c), seeds 1 to N, each twice: DIR/X.json and
#              DIR/X.again.json (the same start gives the same game, and
#              condition 12's games are read from the harness, not these).
#   exports    (a), one export a seed, seeds 1 to N: DIR/a/seedN.json.
#   sample     The sample seeds of (a), (b), (c), exported twice (stage 3's).
#   read       The readers, into DIR/read/: condition 3 (scripts/0159-proof/
#              hunger.py) on seeds 1 to N and on seeds 101 to N alone;
#              condition 13 (scripts/unfed/unfed.py --all) on the same two
#              sets; stage 3's same.py on each pair of batches. It prints
#              counts and never a verdict.
#   time       Condition 10: `sim-cli time`, release, each size in its own
#              process, a trial, a trial beside another (a seat's trial with
#              its second) and a window, three runs each, with the peak
#              memory of each process: DIR/time/. Run it with no other job of
#              the proof running. Needs --machine.
#   harness    The ignored tests of tests/proof_stage_4_harness.rs, each with
#              the seeds and games the setup gives it: conditions 4 and 5 on
#              seed 1; 7 and 8 on seeds 1 to 200 (W3 on 1 to 20); 9; 12 on
#              seeds 1 to 1,000 of (a), (b) and (c); the units of condition
#              13; and the six estimates of condition 11 with their key.
#              Into DIR/harness/ (the legibility files into
#              DIR/harness/legibility/, to be copied by the architect to
#              crates/sim-cli/tests/recorded_games/legibility_stage_4/).
#   all        check, build, batch, exports, sample, read, harness, in that
#              order. Not `time`, which wants the machine to itself.
#
# Condition 1 is `cargo test --workspace` in release (by hand, at the proof's
# commit); condition 2 is `sh scripts/regenerate.sh` and the baselines' tests.
# They are not steps here: they need no file made.
#
# Options:
#   --out DIR       The output folder, outside the repository (required).
#   --seeds N       Seeds 1 to N for batch and exports (default 1000).
#   --jobs J        Exports and the harness's games run J at a time
#                   (default 4). The files are the same whatever J is.
#   --machine TEXT  The machine named in the time report.
#   --seed S        The first seed of the harness (default 1). Anything else
#                   plays other seeds: use 2001 to show that it runs.
#
# Every sim-cli command runs from the top of the repository, as the earlier
# proofs' scripts run them. Nothing here is read by a test or a build step.
#
# Do not run this before the proof: the judged seeds are 1 to 1,000.
set -euo pipefail

usage() { sed -n '2,52p' "$0" | sed 's/^# \{0,1\}//'; exit 2; }

HERE="$(cd "$(dirname "$0")" && pwd -P)"
REPO="$(cd "$HERE/../.." && pwd -P)"
STAGE3="$REPO/scripts/stage-3-proof"
FOLDER="data/scenarios/stage-3-proof"

[ $# -ge 1 ] || usage
STEP="$1"; shift
case "$STEP" in
  check|build|batch|exports|sample|read|time|harness|all) ;;
  -h|--help|help) usage ;;
  *) echo "run.sh: unknown step \"$STEP\"" >&2; usage ;;
esac

OUT=""; SEEDS=1000; JOBS=4; MACHINE=""; SEED=1
while [ $# -gt 0 ]; do
  case "$1" in
    --out) OUT="$2"; shift 2 ;;
    --seeds) SEEDS="$2"; shift 2 ;;
    --jobs) JOBS="$2"; shift 2 ;;
    --machine) MACHINE="$2"; shift 2 ;;
    --seed) SEED="$2"; shift 2 ;;
    *) echo "run.sh: unknown argument \"$1\"" >&2; usage ;;
  esac
done
[ -n "$OUT" ] || { echo "run.sh: --out DIR is required" >&2; exit 2; }
for n in "$SEEDS" "$JOBS" "$SEED"; do
  case "$n" in ''|*[!0-9]*) echo "run.sh: a number is needed, not \"$n\"" >&2; exit 2 ;; esac
done

# The output folder, made absolute, must not be in the repository nor in the
# main checkout that holds a worktree.
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
stage3() { bash "$STAGE3/run.sh" "$@" --out "$OUT" --seeds "$SEEDS" --jobs "$JOBS"; }

do_check() {
  say "check: (a), (b) and (c) are as made"
  (cd "$REPO" && python3 scripts/stage-3-proof/make_arrangements.py --check)
}

do_build() {
  say "build: sim-cli and the harness, release, into $OUT/target"
  (cd "$REPO" && CARGO_TARGET_DIR="$OUT/target" cargo build --release -p sim-cli)
  (cd "$REPO" && CARGO_TARGET_DIR="$OUT/target" cargo test --release -p scenario-tests \
    --test proof_stage_4_harness --no-run)
}

do_batch() { stage3 batch a b c; }
do_exports() { stage3 exports a; }
do_sample() { stage3 sample a b c; }

# A folder of links to the exports of seeds 101 to N, which nobody has read
# (the setup, amendment 6).
subset_101() {
  rm -rf "$OUT/a101"; mkdir -p "$OUT/a101"
  local s
  for s in $(seq 101 "$SEEDS"); do
    [ -f "$OUT/a/seed$s.json" ] && ln -s "$OUT/a/seed$s.json" "$OUT/a101/seed$s.json"
  done
}

do_read() {
  mkdir -p "$OUT/read"
  say "read: output in $OUT/read/"
  if [ ! -d "$OUT/a" ]; then
    echo "(no exports of (a) in $OUT/a: hunger.py and unfed.py passed over)"
  else
    subset_101
    python3 "$REPO/scripts/0159-proof/hunger.py" "$OUT/a" --list --jsonl "$OUT/read/hunger.jsonl" | tee "$OUT/read/hunger.txt"
    python3 "$REPO/scripts/0159-proof/hunger.py" "$OUT/a101" --list --jsonl "$OUT/read/hunger.101.jsonl" | tee "$OUT/read/hunger.101.txt"
    python3 -I "$REPO/scripts/unfed/unfed.py" "$OUT/a" --all --rows "$OUT/read/unfed.jsonl" | tee "$OUT/read/unfed.txt"
    python3 -I "$REPO/scripts/unfed/unfed.py" "$OUT/a101" --all --rows "$OUT/read/unfed.101.jsonl" | tee "$OUT/read/unfed.101.txt"
  fi
  local x
  for x in a b c; do
    if [ -f "$OUT/$x.json" ] && [ -f "$OUT/$x.again.json" ]; then
      local ex=()
      [ -d "$OUT/$x.sample" ] && [ -d "$OUT/$x.sample.again" ] && ex=(--exports "$OUT/$x.sample" "$OUT/$x.sample.again")
      python3 "$STAGE3/same.py" --batch "$OUT/$x.json" "$OUT/$x.again.json" "${ex[@]+"${ex[@]}"}" | tee "$OUT/read/$x.same.txt"
    else
      echo "(no two batches of ($x): same.py passed over)"
    fi
  done
}

do_time() {
  [ -x "$SIM" ] || { echo "run.sh: no sim-cli at $SIM: run the build step" >&2; exit 2; }
  [ -n "$MACHINE" ] || { echo "run.sh: time needs --machine TEXT" >&2; exit 2; }
  mkdir -p "$OUT/time"
  say "time: each size in its own process, three runs, in $OUT/time/"
  local size kind run flag
  for size in small standard full; do
    for kind in trial seat-trial window; do
      case "$kind" in
        trial) flag="--only-trial" ;;
        seat-trial) flag="--only-seat-trial" ;;
        window) flag="--only-window" ;;
      esac
      for run in 1 2 3; do
        # `time -l` gives the peak memory of the process on macOS.
        (cd "$REPO" && /usr/bin/time -l "$SIM" time --machine "$MACHINE" "$flag" --trial-size "$size") \
          > "$OUT/time/$kind.$size.$run.txt" 2> "$OUT/time/$kind.$size.$run.mem.txt"
        echo "$kind $size run $run: $(grep -h 'maximum resident' "$OUT/time/$kind.$size.$run.mem.txt" | tr -s ' ')"
      done
    done
  done
}

# One ignored test of the harness, with its seeds and games, into DIR/harness.
harness_test() {
  local games="$1" test="$2"
  say "harness: $test (first seed $SEED, $games game(s) where it plays any)"
  (cd "$REPO" && CARGO_TARGET_DIR="$OUT/target" \
    FLAGPILOT_STAGE4_OUT="$OUT/harness" FLAGPILOT_STAGE4_SEED="$SEED" \
    FLAGPILOT_STAGE4_GAMES="$games" FLAGPILOT_STAGE4_JOBS="$JOBS" \
    cargo test --release -p scenario-tests --test proof_stage_4_harness -- --ignored --nocapture "$test")
}

do_harness() {
  mkdir -p "$OUT/harness"
  harness_test 1 condition_4_no_peek
  harness_test 1 condition_5_the_same_twice
  harness_test 200 condition_7_spreads
  harness_test 200 condition_8_wrong_pictures
  harness_test 1 condition_9_preparation
  harness_test "$SEEDS" condition_12_beside_the_games
  harness_test 1 condition_13_japanese_ashore_at_the_start
  harness_test 1 legibility_estimates
  echo "wrote $OUT/harness/: condition_4.txt, _5, _7, _8, _9, _12, condition_13_units.txt, legibility/"
}

for step in check build batch exports sample read harness; do
  if [ "$STEP" = "$step" ] || [ "$STEP" = all ]; then
    "do_$step"
  fi
done
if [ "$STEP" = time ]; then do_time; fi
