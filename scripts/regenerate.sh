#!/bin/sh
# Remakes every generated file in one command and says in plain words what
# changed (decision 0108, item 4): what each rule build used to do in eight
# steps, some by hand.
#
#   sh scripts/regenerate.sh [--check] [--jobs N]
#
# It builds the release binary once, then remakes, in this order:
#
#   1 run baselines     crates/sim-cli/tests/run_baseline: `run` and
#                       `run --events` of every file in data/scenarios (the
#                       soak has no events file); a baseline whose scenario is
#                       gone is removed
#   2 agent goldens     crates/sim-agent/tests/golden/*.txt and
#                       viewer/measure_fixture.json (FLAGPILOT_UPDATE_GOLDEN=1)
#   3 recorded games    transcript.jsonl and save.json of each game under
#                       crates/sim-cli/tests/recorded_games (regame.py)
#   4 pinned hashes     tests/golden/pinned_hashes.txt, written by the tests
#                       that pin them (FLAGPILOT_UPDATE_GOLDEN=1)
#   5 batch fixture     crates/sim-cli/tests/batch_before, the old-format
#                       batch file, written by its test (FLAGPILOT_UPDATE_GOLDEN=1)
#   6 submarine tables  data/reference/submarine_classes_1942.csv and
#                       submarine_torpedoes_1942.csv with their sources, from
#                       data/derivation/submarines_1942.toml (oob-derive,
#                       decision 0218, stage 1); their frozen copies in
#                       data/reference/step1 are not remade (they are copies
#                       as first derived)
#
# It then checks what stays by hand and says so: the version strings in
# documents (listed, not edited), NOT_IN_THE_FULL_SLICE in
# tests/scenario_07_determinism.rs (the test that needs a line is run and its
# message printed), and the [[test]] entries of tests/Cargo.toml (a test file
# with no entry is listed).
#
# Last it says, for each group, how many files changed and which, from
# `git status --porcelain` (so against the last commit, and counting anything
# already changed there); for the run baselines which scenarios changed beyond
# their hash lines; for the recorded games whether a transcript changed or
# only a save.
#
# Exits 0 when every step ran and the by-hand checks passed, whatever changed;
# otherwise 1, naming each step that failed.
#
# --check remakes nothing: it runs the tests that compare the generated files
# and says which group is stale. It exits 1 when one is.
# --jobs N runs N baseline runs at a time (default 8).
#
# Run from the top of the repository.

set -eu
LC_ALL=C
export LC_ALL

check=0
jobs=8
while [ $# -gt 0 ]; do
    case "$1" in
        --check) check=1; shift ;;
        --jobs) jobs="$2"; shift 2 ;;
        *) echo "usage: sh scripts/regenerate.sh [--check] [--jobs N]" >&2; exit 2 ;;
    esac
done

top=$(pwd)
if [ ! -d "$top/data/scenarios" ] || [ ! -f "$top/Cargo.toml" ]; then
    echo "regenerate: run from the top of the repository" >&2
    exit 2
fi

baselines=crates/sim-cli/tests/run_baseline
goldens=crates/sim-agent/tests/golden
measure=viewer/measure_fixture.json
games=crates/sim-cli/tests/recorded_games
pinned=tests/golden/pinned_hashes.txt
batch_before=crates/sim-cli/tests/batch_before
bin="$top/target/release/sim-cli"

failed=""
fail() { failed="$failed
  $1"; echo "regenerate: STEP FAILED: $1" >&2; }
say() { echo "regenerate: $*"; }
# quiet CMD...: runs the command, shows its output only when it fails.
quiet() {
    log=$(mktemp)
    if "$@" > "$log" 2>&1; then rm -f "$log"; return 0; fi
    cat "$log"; rm -f "$log"; return 1
}

# ---------------------------------------------------------------- --check
if [ "$check" -eq 1 ]; then
    stale=""
    probe() { # probe GROUP CMD...
        group="$1"; shift
        say "checking $group"
        if "$@" > "${TMPDIR:-/tmp}/regenerate-check.out" 2>&1; then :; else
            stale="$stale
  $group"
            tail -n 15 "${TMPDIR:-/tmp}/regenerate-check.out"
        fi
    }
    probe "1 run baselines" cargo test --release --quiet -p sim-cli --test export run_output_matches_the_golden_files
    probe "2 agent goldens" cargo test --release --quiet -p sim-agent --test golden
    probe "2 viewer measure fixture" cargo test --release --quiet -p sim-cli measure_fixture
    probe "3 recorded games" cargo test --release --quiet -p sim-cli --test mcp every_recorded_game_replays_to_its_transcript_and_save
    probe "4 pinned hashes" cargo test --release --quiet -p scenario-tests --test scenario_10_objectives --test scenario_17a_target_choice
    probe "5 batch fixture" cargo test --release --quiet -p sim-cli --test batch the_keys_a_version_1_reader_used_keep_their_values
    probe "6 submarine tables" cargo test --release --quiet -p oob-derive --test submarines the_submarine_list_derives_exactly_the_files_in_data_reference
    rm -f "${TMPDIR:-/tmp}/regenerate-check.out"
    if [ -n "$stale" ]; then
        say "stale, or failing for another reason (read the output above):$stale"
        exit 1
    fi
    say "every generated file is what the engine gives"
    exit 0
fi

# ----------------------------------------------------------------- remake
say "building the release binary"
cargo build --release --quiet -p sim-cli || { echo "regenerate: the build failed" >&2; exit 1; }

# 1 run baselines
say "1 run baselines"
list=$(mktemp)
trap 'rm -f "$list"' EXIT
for f in data/scenarios/*.toml; do
    name=$(basename "$f" .toml)
    echo "$name $f" >> "$list"
done
if xargs -P "$jobs" -L 1 sh -c '
    name="$1"; file="$2"; base="'"$baselines"'"
    "'"$bin"'" run "$file" > "$base/$name.txt" || exit 255
    if [ "$name" != 08_soak ] && [ "$name" != 45_soak_ashore ]; then
        "'"$bin"'" run "$file" --events > "$base/$name.events.txt" || exit 255
    fi
' sh < "$list"; then :; else fail "1 run baselines"; fi
for f in "$baselines"/*.txt; do
    base=$(basename "$f" .txt)
    name=${base%.events}
    if [ ! -f "data/scenarios/$name.toml" ]; then
        rm -f "$f"
        say "removed $f: no scenario $name"
    fi
done

# 2 agent goldens and the viewer's measure fixture
say "2 agent goldens"
quiet env FLAGPILOT_UPDATE_GOLDEN=1 cargo test --release --quiet -p sim-agent --test golden \
    || fail "2 agent goldens"
say "2 viewer measure fixture"
quiet env FLAGPILOT_UPDATE_GOLDEN=1 cargo test --release --quiet -p sim-cli measure_fixture \
    || fail "2 viewer measure fixture"

# 3 recorded games
say "3 recorded games"
quiet python3 "$games/regame.py" || fail "3 recorded games"

# 4 pinned hashes (after 1: the scenario 10 test reads the baseline)
say "4 pinned hashes"
quiet env FLAGPILOT_UPDATE_GOLDEN=1 cargo test --release --quiet -p scenario-tests \
    --test scenario_10_objectives --test scenario_17a_target_choice \
    || fail "4 pinned hashes"

# 5 the old-format batch fixture
say "5 batch fixture"
quiet env FLAGPILOT_UPDATE_GOLDEN=1 cargo test --release --quiet -p sim-cli --test batch \
    the_keys_a_version_1_reader_used_keep_their_values \
    || fail "5 batch fixture"

# 6 the submarine tables (decision 0218, stage 1)
say "6 submarine tables"
quiet cargo run --release --quiet -p oob-derive -- data/derivation/submarines_1942.toml \
    || fail "6 submarine tables"

# ------------------------------------------------------- what stays by hand
current=$(sed -n 's/.*STATE_VERSION: &\[u8\] = b"\(flagplot-state-v[0-9]*\)";.*/\1/p' crates/sim-core/src/hash.rs)
say "version reminder: the state version is $current"
reminder=$(git grep -n -o -E 'flagplot-state-v[0-9]+' -- . ':!docs/decisions' ':!docs/validation' \
    | awk -F: -v cur="$current" '$NF != cur && $NF != "flagplot-state-v0" { print $1 }' | sort -u || true)
if [ -n "$reminder" ]; then
    echo "These files outside docs/decisions and docs/validation name another state version; each may be right (a record of an older engine) or may need the new one:"
    echo "$reminder" | sed 's/^/  /'
else
    echo "No file outside docs/decisions and docs/validation names another state version."
fi

say "7a NOT_IN_THE_FULL_SLICE"
out=$(mktemp)
if cargo test --release --quiet -p scenario-tests --test scenario_07_determinism \
    the_full_slice_log_has_every_kind_of_event_the_engine_can_emit > "$out" 2>&1; then
    echo "Every kind of event the engine can emit is in the full slice or in the list."
else
    cat "$out"
    echo "NOT_IN_THE_FULL_SLICE in tests/scenario_07_determinism.rs needs a line (or the slice a kind): whether a kind is expected there is a judgement, with a reason written beside it. Do it by hand."
    fail "7a NOT_IN_THE_FULL_SLICE needs a line (by hand)"
fi
rm -f "$out"

say "7b tests/Cargo.toml"
missing=""
for f in tests/*.rs; do
    stem=$(basename "$f" .rs)
    grep -q "^name = \"$stem\"\$" tests/Cargo.toml || missing="$missing $stem"
done
if [ -n "$missing" ]; then
    echo "These test files have no [[test]] entry in tests/Cargo.toml, so cargo does not run them (a hand calculation written before its code stays so until its stage is built; add the entry when it is):"
    for stem in $missing; do echo "  tests/$stem.rs"; done
else
    echo "Every test file has an entry in tests/Cargo.toml."
fi

# ------------------------------------------------------------ what changed
echo
say "what changed (against the last commit)"
group() { # group TITLE PATH...
    title="$1"; shift
    changes=$(git status --porcelain --untracked-files=all -- "$@")
    if [ -z "$changes" ]; then
        echo "$title: no file changed"
    else
        n=$(echo "$changes" | wc -l | tr -d ' ')
        echo "$title: $n file(s) changed"
        echo "$changes" | sed 's/^/  /'
    fi
}
group "1 run baselines" "$baselines"
changes=$(git status --porcelain --untracked-files=all -- "$baselines" | awk '{ print $1, $2 }')
if [ -n "$changes" ]; then
    beyond=""
    cut_hashes() { sed -E 's/^(day +[0-9]+ +hash )[0-9a-f]{16}/\1-/'; }
    echo "$changes" | while read -r code path; do
        case "$code" in
            M*)
                if [ "$(git show "HEAD:$path" | cut_hashes)" != "$(cut_hashes < "$path")" ]; then
                    echo "  changed beyond its hash lines: $path"
                fi ;;
            D*) echo "  removed: $path" ;;
            *) echo "  new: $path" ;;
        esac
    done > "${TMPDIR:-/tmp}/regenerate-beyond.out"
    if [ -s "${TMPDIR:-/tmp}/regenerate-beyond.out" ]; then
        echo "Scenarios whose run changed beyond the hash lines:"
        cat "${TMPDIR:-/tmp}/regenerate-beyond.out"
    else
        echo "No run baseline changed beyond its hash lines."
    fi
    rm -f "${TMPDIR:-/tmp}/regenerate-beyond.out"
fi
group "2 agent goldens and the viewer's measure fixture" "$goldens" "$measure"
group "3 recorded games" "$games"
transcripts=$(git status --porcelain --untracked-files=all -- "$games" | grep 'transcript.jsonl' || true)
saves=$(git status --porcelain --untracked-files=all -- "$games" | grep 'save.json' || true)
if [ -n "$transcripts" ]; then
    echo "A transcript changed: an answer a player is shown differs."
elif [ -n "$saves" ]; then
    echo "Only saves changed, no transcript."
fi
if [ -n "$transcripts$saves" ]; then
    echo "Write a \"Regenerated after ...\" paragraph by hand in $games/README.md."
fi
group "4 pinned hashes" "$pinned"
group "5 batch fixture" "$batch_before"
group "6 submarine tables" data/reference/submarine_classes_1942.csv \
    data/reference/submarine_classes_1942.sources.csv \
    data/reference/submarine_torpedoes_1942.csv \
    data/reference/submarine_torpedoes_1942.sources.csv

echo
if [ -n "$failed" ]; then
    echo "regenerate: failed:$failed" >&2
    exit 1
fi
say "done: every generated file is remade"
