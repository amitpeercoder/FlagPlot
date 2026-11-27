#!/bin/sh
# The proof that a scenario built on another is the scenario it replaces
# (decision 0234, "The proof that nothing is lost", steps C and D):
#
#   sh scripts/overlay-proof.sh REV [COUNT]
#
# REV is the commit the branch left `main` at, by default the one
# `git merge-base HEAD origin/main` gives. Every scenario file under
# data/scenarios that now states `built_on` is held against the same file as
# it was at REV, with this tree's own `sim-cli` on both:
#
#   1. `sim-cli resolve` of the overlay is byte for byte `sim-cli resolve`
#      of the file at REV (which has no `built_on`, so its table);
#   2. `sim-cli run` of the two prints the same text: the same hash and
#      count of events for every day.
#
# One line for each file: its path, then `resolve same|DIFFERS` and
# `run same|DIFFERS`; a file that was not there at REV is `NEW`, a
# difference. It ends with a count and exits 1 when anything differs, or when
# COUNT is given and the number of overlays is not COUNT (27 for step C, 31
# for step D). It writes nothing in the repository: REV's data is unpacked
# in a folder of its own and removed.
#
# SIM_CLI names the program to use (default `target/release/sim-cli`, built
# here if it is not there). Run from the top of the repository.

set -eu
LC_ALL=C
export LC_ALL

rev="${1:-}"
count="${2:-}"
if [ -z "$rev" ]; then
  rev=$(git merge-base HEAD origin/main)
fi

sim="${SIM_CLI:-}"
if [ -z "$sim" ]; then
  cargo build --release -p sim-cli >&2
  sim="${CARGO_TARGET_DIR:-target}/release/sim-cli"
fi

work=$(mktemp -d "${TMPDIR:-/tmp}/flagplot-overlay-proof.XXXXXX")
trap 'rm -rf "$work"' EXIT
git archive "$rev" data | tar -x -C "$work"

total=0
differing=0
for file in $(grep -rlE '^built_on[[:space:]]*=' data/scenarios --include='*.toml' | sort); do
  total=$((total + 1))
  old="$work/$file"
  if [ ! -f "$old" ]; then
    echo "$file NEW"
    differing=$((differing + 1))
    continue
  fi
  "$sim" resolve "$file" >"$work/new.resolved" 2>/dev/null || echo "resolve failed" >"$work/new.resolved"
  "$sim" resolve "$old" >"$work/old.resolved" 2>/dev/null || echo "resolve failed" >"$work/old.resolved"
  if cmp -s "$work/new.resolved" "$work/old.resolved"; then
    resolve=same
  else
    resolve=DIFFERS
  fi
  "$sim" run "$file" --events >"$work/new.run" 2>&1 || true
  "$sim" run "$old" --events >"$work/old.run" 2>&1 || true
  # The run prints the path it was given where it names an error; with the
  # two trees at different paths that is the only text allowed to differ.
  sed "s|$work/||g" "$work/old.run" >"$work/old.run.cut"
  if cmp -s "$work/new.run" "$work/old.run.cut"; then
    run=same
  else
    run=DIFFERS
  fi
  echo "$file resolve $resolve run $run"
  if [ "$resolve" != same ] || [ "$run" != same ]; then
    differing=$((differing + 1))
  fi
done

echo "$total overlays, $differing differing"
status=0
if [ "$differing" -ne 0 ]; then
  status=1
fi
if [ -n "$count" ] && [ "$total" -ne "$count" ]; then
  echo "expected $count overlays, found $total"
  status=1
fi
exit "$status"
