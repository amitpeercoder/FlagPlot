#!/usr/bin/env bash
# Make the files the proof of stage 3 is read from (decision 0153), one step
# at a time. It runs nothing unless told which step, and it writes only into
# the output folder, which must be outside the repository.
#
#   scripts/stage-3-proof/run.sh STEP [ARRANGEMENT...] --out DIR [options]
#
# STEP is one of:
#   build      Builds sim-cli in release into DIR/target (CARGO_TARGET_DIR),
#              so that nothing is written in the repository.
#   loading    The loading check of decision 0153: arrangement (a), seed 1,
#              breakdowns on (export and run record them; only batch turns
#              them off). Writes DIR/loading/: the export, the run with every
#              event, the saved game and the replay of it.
#   batch      For each arrangement, the batch of seeds 1 to N and the same
#              batch again: DIR/X.json and DIR/X.again.json (condition 1,
#              condition 5, and condition 2's check).
#   exports    For each arrangement, one export a seed, seeds 1 to N:
#              DIR/X/seedN.json (conditions 2 to 4 read every seed of (a);
#              pick.py reads seeds 1 to 10 of (a)).
#   sample     For each arrangement, the exports of the sample seeds made
#              twice: DIR/X.sample/ and DIR/X.sample.again/ (condition 1).
#   time       sim-cli time on arrangement (a) (condition 7): DIR/time.txt.
#   read       Runs the reading scripts on what is there, and writes their
#              output to DIR/read/. A file not made yet is passed over with
#              a note. It prints counts and never a verdict.
#   all        build, loading, batch, exports, sample, time and read, in
#              that order, for the arrangements named.
#
# ARRANGEMENT is a, b or c (decision 0153's table); with none, all three.
# Options:
#   --out DIR       The output folder, outside the repository (required).
#   --seeds N       Seeds 1 to N for batch and exports (default 1000).
#   --sample "S..." The sample seeds, in quotes (default "1 2 3 500 1000",
#                   less any above N).
#   --jobs J        Exports run J at a time (default 1). The files are the
#                   same whatever J is: each seed is its own game.
#   --machine TEXT  The machine named in the time report (required by time).
#
# sim-cli is DIR/target/release/sim-cli once `build` has run, or the path in
# the environment variable SIM_CLI. Every sim-cli command runs from the top of
# the repository, so that both runs of a batch name their inputs by the same
# relative paths.
#
# Disk and time, measured on Nov 18, 2026 on an Apple M3 Pro (11 cores), main
# at 4dec6155 (before decision 0152): an export of arrangement (a) is about
# 12.7 MB and 0.3 s; a game in a batch about 0.2 s. docs/validation/
# stage-3-proof-setup.md has the measurements and the estimate for the run.
set -euo pipefail

usage() { sed -n '2,45p' "$0" | sed 's/^# \{0,1\}//'; exit 2; }

HERE="$(cd "$(dirname "$0")" && pwd -P)"
REPO="$(cd "$HERE/../.." && pwd -P)"
FOLDER="data/scenarios/stage-3-proof"

[ $# -ge 1 ] || usage
STEP="$1"; shift
case "$STEP" in
  build|loading|batch|exports|sample|time|read|all) ;;
  -h|--help|help) usage ;;
  *) echo "run.sh: unknown step \"$STEP\"" >&2; usage ;;
esac

OUT=""; SEEDS=1000; SAMPLE=""; JOBS=1; MACHINE=""; ARRS=""
while [ $# -gt 0 ]; do
  case "$1" in
    --out) OUT="$2"; shift 2 ;;
    --seeds) SEEDS="$2"; shift 2 ;;
    --sample) SAMPLE="$2"; shift 2 ;;
    --jobs) JOBS="$2"; shift 2 ;;
    --machine) MACHINE="$2"; shift 2 ;;
    a|b|c) ARRS="$ARRS $1"; shift ;;
    *) echo "run.sh: unknown argument \"$1\"" >&2; usage ;;
  esac
done
[ -n "$ARRS" ] || ARRS="a b c"
[ -n "$OUT" ] || { echo "run.sh: --out DIR is required" >&2; exit 2; }
case "$SEEDS" in ''|*[!0-9]*) echo "run.sh: --seeds takes a whole number" >&2; exit 2 ;; esac
[ "$SEEDS" -ge 1 ] || { echo "run.sh: --seeds is at least 1" >&2; exit 2; }
case "$JOBS" in ''|*[!0-9]*) echo "run.sh: --jobs takes a whole number" >&2; exit 2 ;; esac

# The output folder, made absolute, must not be in the repository.
OUT="$(python3 -I -c 'import os,sys; print(os.path.realpath(os.path.abspath(sys.argv[1])))' "$OUT")"
case "$OUT/" in
  "$REPO"/*) echo "run.sh: $OUT is inside the repository ($REPO); give a folder outside it" >&2; exit 2 ;;
esac
# A worktree lies inside the main checkout: refuse that too.
MAIN="$(cd "$REPO" && git rev-parse --path-format=absolute --git-common-dir 2>/dev/null || true)"
if [ -n "$MAIN" ]; then
  MAIN="$(cd "$(dirname "$MAIN")" && pwd -P)"
  case "$OUT/" in
    "$MAIN"/*) echo "run.sh: $OUT is inside the repository's checkout ($MAIN); give a folder outside it" >&2; exit 2 ;;
  esac
fi
mkdir -p "$OUT"

if [ -z "$SAMPLE" ]; then
  for s in 1 2 3 500 1000; do [ "$s" -le "$SEEDS" ] && SAMPLE="$SAMPLE $s"; done
fi

SIM="${SIM_CLI:-$OUT/target/release/sim-cli}"
scenario() { echo "$FOLDER/07$1_slice.toml"; }
need_sim() {
  [ -x "$SIM" ] || { echo "run.sh: no sim-cli at $SIM: run the build step, or set SIM_CLI" >&2; exit 2; }
}
say() { echo "== $*"; }
elapsed() { python3 -I -c 'import time; print(f"{time.time():.1f}")'; }

do_build() {
  say "build: sim-cli, release, into $OUT/target"
  (cd "$REPO" && CARGO_TARGET_DIR="$OUT/target" cargo build --release -p sim-cli)
}

do_loading() {
  need_sim
  local s; s="$(scenario a)"
  mkdir -p "$OUT/loading"
  say "loading check: $s, seed 1, breakdowns on"
  (cd "$REPO" && "$SIM" export "$s" --seed 1 --out "$OUT/loading/a.seed1.json")
  (cd "$REPO" && "$SIM" run "$s" --seed 1 --events --save "$OUT/loading/a.seed1.save.json" > "$OUT/loading/a.seed1.run.txt")
  (cd "$REPO" && "$SIM" replay "$OUT/loading/a.seed1.save.json" > "$OUT/loading/a.seed1.replay.txt")
  echo "wrote $OUT/loading/: a.seed1.json (export), a.seed1.run.txt (run --events), a.seed1.save.json, a.seed1.replay.txt"
}

do_batch() {
  need_sim
  local x s t0
  for x in $ARRS; do
    s="$(scenario "$x")"
    for copy in "" ".again"; do
      say "batch ($x)$copy: $s, seeds 1 to $SEEDS"
      t0="$(elapsed)"
      (cd "$REPO" && "$SIM" batch "$s" --runs "$SEEDS" --first-seed 1 --out "$OUT/$x$copy.json") > "$OUT/$x$copy.batch.txt"
      echo "wrote $OUT/$x$copy.json in $(python3 -I -c "import sys; print(round($(elapsed)-$t0,1))") s"
    done
  done
}

# Export the seeds read from standard input of arrangement $1 into folder $2.
export_seeds() {
  local x="$1" dir="$2" s; s="$(scenario "$x")"
  mkdir -p "$dir"
  (cd "$REPO" && xargs -P "$JOBS" -I{} "$SIM" export "$s" --seed {} --out "$dir/seed{}.json" > /dev/null)
}

do_exports() {
  need_sim
  local x t0
  for x in $ARRS; do
    say "exports ($x): seeds 1 to $SEEDS into $OUT/$x/"
    t0="$(elapsed)"
    seq 1 "$SEEDS" | export_seeds "$x" "$OUT/$x"
    echo "wrote $SEEDS exports in $(python3 -I -c "print(round($(elapsed)-$t0,1))") s; $(du -sh "$OUT/$x" | cut -f1) on disk"
  done
}

do_sample() {
  need_sim
  local x
  for x in $ARRS; do
    for copy in "" ".again"; do
      say "sample ($x)$copy: seeds$SAMPLE into $OUT/$x.sample$copy/"
      rm -rf "$OUT/$x.sample$copy"
      printf '%s\n' $SAMPLE | export_seeds "$x" "$OUT/$x.sample$copy"
    done
  done
}

do_time() {
  need_sim
  [ -n "$MACHINE" ] || { echo "run.sh: time needs --machine TEXT" >&2; exit 2; }
  say "time: $(scenario a) on \"$MACHINE\""
  (cd "$REPO" && "$SIM" time --machine "$MACHINE" --slice "$(scenario a)") > "$OUT/time.txt"
  echo "wrote $OUT/time.txt"
}

do_read() {
  local x py="python3"
  mkdir -p "$OUT/read"
  say "read: the scripts of $HERE, output in $OUT/read/"
  for x in $ARRS; do
    if [ -d "$OUT/$x" ]; then
      local batch=()
      [ -f "$OUT/$x.json" ] && batch=(--batch "$OUT/$x.json")
      $py "$HERE/conditions.py" "$OUT/$x" "${batch[@]+"${batch[@]}"}" --list --jsonl "$OUT/read/$x.conditions.jsonl" \
        | tee "$OUT/read/$x.conditions.txt"
    else
      echo "(no exports of ($x) in $OUT/$x: conditions.py passed over)"
    fi
    if [ -f "$OUT/$x.json" ] && [ -f "$OUT/$x.again.json" ]; then
      local ex=()
      [ -d "$OUT/$x.sample" ] && [ -d "$OUT/$x.sample.again" ] && ex=(--exports "$OUT/$x.sample" "$OUT/$x.sample.again")
      $py "$HERE/same.py" --batch "$OUT/$x.json" "$OUT/$x.again.json" "${ex[@]+"${ex[@]}"}" | tee "$OUT/read/$x.same.txt"
    else
      echo "(no two batches of ($x): same.py passed over)"
    fi
  done
  for pair in "b Allied" "c Japanese"; do
    set -- $pair
    if [ -f "$OUT/a.json" ] && [ -f "$OUT/$1.json" ]; then
      echo "-- condition 5, the $2 half: (a) beside ($1)"
      $py "$HERE/margin.py" "$OUT/a.json" "$OUT/$1.json" | tee "$OUT/read/margin.a-$1.txt"
    else
      echo "(no batch of (a) and ($1): margin.py for the $2 half passed over)"
    fi
  done
  if [ -d "$OUT/a" ]; then
    $py "$HERE/pick.py" "$OUT/a" --out "$OUT/read/legibility.json" | tee "$OUT/read/pick.txt"
  else
    echo "(no exports of (a): pick.py passed over)"
  fi
}

for step in build loading batch exports sample time read; do
  if [ "$STEP" = "$step" ] || { [ "$STEP" = all ] && { [ "$step" != time ] || [ -n "$MACHINE" ]; }; }; then
    "do_$step"
  fi
done
if [ "$STEP" = all ] && [ -z "$MACHINE" ]; then
  echo "(time passed over: give --machine TEXT)"
fi
