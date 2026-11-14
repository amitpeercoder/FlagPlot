#!/bin/sh
# Compares every scenario's run with the previous build (decision 0108,
# item 5): what a rule build used to do by hand, by keeping an old binary.
#
#   sh scripts/compare-scenarios.sh [--against REV] [--live-only] [--jobs N]
#
# REV is the previous build: by default the commit the branch starts from,
# `git merge-base HEAD origin/main`. Every scenario is run with `run --events`
# by this tree's release build, on two sets of figures:
#
#   live    each file in data/scenarios, on the figures it names, against
#           the run baselines committed at REV
#           (crates/sim-cli/tests/run_baseline; the soak, which has no events
#           baseline, without --events);
#   frozen  each file in data/scenarios/step2-frozen, and each file in
#           data/scenarios without a frozen copy, with
#           --reference data/reference/step1 as the hand-calculation tests run
#           them, against the same runs by REV's own build of REV's own files.
#           REV is checked out and built once, in a cache folder outside the
#           repository, and kept for the next comparison.
#
# The day's hash is cut from every line `day N hash H ...` on both sides
# before they are compared, so a change to what is hashed alone shows
# nothing. Each scenario whose output then differs is named with the first
# line that differs, from REV and from this tree; a scenario only one side
# has is named too. It ends with a count, and exits 1 when anything differs
# and 0 when nothing does. It writes nothing in the repository.
#
# --live-only skips the frozen figures, so REV is not built.
# --jobs N runs N scenarios at a time (default 8).
# FLAGPLOT_COMPARE_CACHE names the cache folder (default
# ${TMPDIR:-/tmp}/flagplot-compare); delete it to free the space.
#
# Run from the top of the repository.

set -eu
LC_ALL=C
export LC_ALL

against=""
live_only=0
jobs=8
while [ $# -gt 0 ]; do
    case "$1" in
        --against) against="$2"; shift 2 ;;
        --live-only) live_only=1; shift ;;
        --jobs) jobs="$2"; shift 2 ;;
        *) echo "usage: sh scripts/compare-scenarios.sh [--against REV] [--live-only] [--jobs N]" >&2; exit 2 ;;
    esac
done

top=$(pwd)
if [ ! -d "$top/data/scenarios" ] || [ ! -f "$top/Cargo.toml" ]; then
    echo "compare-scenarios: run from the top of the repository" >&2
    exit 2
fi
if [ -z "$against" ]; then
    against=$(git merge-base HEAD origin/main)
fi
rev=$(git rev-parse --verify "$against^{commit}")
short=$(git rev-parse --short "$rev")
baselines=crates/sim-cli/tests/run_baseline
frozen_dir=data/scenarios/step2-frozen
frozen_ref=data/reference/step1
tmp_root="${TMPDIR:-/tmp}"
cache="${FLAGPLOT_COMPARE_CACHE:-${tmp_root%/}/flagplot-compare}"

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
mkdir -p "$work/new" "$work/old"

echo "compare-scenarios: this tree against $short ($(git log -1 --format=%s "$rev"))"
echo "compare-scenarios: building this tree"
cargo build --release --quiet -p sim-cli
new_bin="$top/target/release/sim-cli"

# One line a job: TAG FILE [ARGS...]. The output goes to DIR/TAG.
live_jobs() {
    for f in data/scenarios/*.toml; do
        name=$(basename "$f" .toml)
        if [ "$name" = 08_soak ]; then
            echo "live-$name $f"
        else
            echo "live-$name $f --events"
        fi
    done
}
frozen_jobs() { # run in the tree whose files are read
    for f in data/scenarios/*.toml; do
        name=$(basename "$f" .toml)
        [ -f "$frozen_dir/$name.toml" ] && continue
        echo "frozen-$name $f --events --reference $frozen_ref"
    done
    for f in "$frozen_dir"/*.toml; do
        [ -f "$f" ] || continue
        echo "frozen-copy-$(basename "$f" .toml) $f --events --reference $frozen_ref"
    done
}
# run_jobs BIN TREE OUTDIR < jobs
run_jobs() {
    (cd "$2" && xargs -P "$jobs" -L 1 sh -c '
        bin="$1"; out="$2"; tag="$3"; shift 3
        if "$bin" run "$@" > "$out/$tag" 2> "$out/$tag.err"; then :; else
            { echo "run failed:"; cat "$out/$tag.err"; } > "$out/$tag"
        fi
        rm -f "$out/$tag.err"
    ' sh "$1" "$3")
}
strip_hashes() {
    sed -E 's/^(day +[0-9]+ +hash )[0-9a-f]{16}/\1-/' "$1"
}

echo "compare-scenarios: running the live figures"
live_jobs | run_jobs "$new_bin" "$top" "$work/new"
for f in $(git ls-tree --name-only "$rev" "$baselines/"); do
    base=$(basename "$f")
    case "$base" in
        *.events.txt) name=${base%.events.txt} ;;
        08_soak.txt) name=08_soak ;;
        *) continue ;;
    esac
    git show "$rev:$f" > "$work/old/live-$name"
done

if [ "$live_only" -eq 0 ]; then
    tree="$cache/$rev"
    if [ ! -x "$tree/target/release/sim-cli" ]; then
        echo "compare-scenarios: building $short in $tree"
        mkdir -p "$cache"
        if [ ! -d "$tree" ]; then
            git worktree add --detach --quiet "$tree" "$rev"
        fi
        (cd "$tree" && cargo build --release --quiet -p sim-cli)
    else
        echo "compare-scenarios: reusing the build of $short in $tree"
    fi
    echo "compare-scenarios: running the frozen figures, on both builds"
    (cd "$top" && frozen_jobs) | run_jobs "$new_bin" "$top" "$work/new"
    (cd "$tree" && frozen_jobs) | run_jobs "$tree/target/release/sim-cli" "$tree" "$work/old"
fi

# The comparison.
compared=0
differ=0
for tag in $( (ls "$work/new"; ls "$work/old") | sort -u); do
    compared=$((compared + 1))
    if [ ! -f "$work/old/$tag" ]; then
        differ=$((differ + 1)); echo "NEW       $tag: not in $short"; continue
    fi
    if [ ! -f "$work/new/$tag" ]; then
        differ=$((differ + 1)); echo "GONE      $tag: in $short, not in this tree"; continue
    fi
    strip_hashes "$work/old/$tag" > "$work/a"
    strip_hashes "$work/new/$tag" > "$work/b"
    if ! cmp -s "$work/a" "$work/b"; then
        differ=$((differ + 1))
        awk -v tag="$tag" -v short="$short" '
            FILENAME == ARGV[1] { a[FNR] = $0; na = FNR; next }
            { b[FNR] = $0; nb = FNR }
            END {
                n = na > nb ? na : nb
                for (i = 1; i <= n; i++) {
                    if (!(i in a) || !(i in b) || a[i] != b[i]) {
                        print "DIFFERS   " tag ": first at line " i
                        print "  " short ": " ((i in a) ? a[i] : "(ends)")
                        print "  this tree: " ((i in b) ? b[i] : "(ends)")
                        exit
                    }
                }
            }
        ' "$work/a" "$work/b"
    fi
done
same=$((compared - differ))
echo "compare-scenarios: $compared runs compared, $same the same with the hashes cut, $differ differ"
[ "$differ" -eq 0 ]
