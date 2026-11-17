#!/bin/sh
# Prints the integration test targets of one shard of the tests, as the
# arguments `cargo test` takes: `--test NAME`, one word a line.
#
#   sh .github/scripts/test-shard.sh K N
#
# K counts from 0 and N is the number of shards. The names are every target
# of kind "test" in the workspace, as `cargo metadata` lists them, without
# repeats, in one order: first the slow ones named below, slowest first, then
# the rest sorted. Shard K takes the names at places K, K + N, K + 2N and so
# on. So each name is in exactly one shard, a test file added later is in one
# with no change here, and the shards together are every integration test
# that `cargo test --workspace` runs. A name two packages share (`seat`, for
# one) selects the target of that name in both, in the same shard.
#
# The slow list only spreads the longest runs over the shards. It decides
# nothing else: a name in it that no longer exists is passed over, and a slow
# test missing from it still runs, in whichever shard its place gives.
#
# The unit tests of the libraries and programs (`--lib --bins`) and the
# documentation tests (`--doc`) are not here: CI runs them in a shard of
# their own.

set -eu

if [ $# -ne 2 ]; then
    echo "usage: sh .github/scripts/test-shard.sh K N" >&2
    exit 2
fi
k=$1
n=$2

# Slowest first, by the seconds each took to run in CI in November 2026.
slow="scenario_45_soak_ashore
mcp
export
scenario_42_interrupts_and_runs
scenario_41_hold_change_release
scenario_36_one_convoy
leaks"

names=$(cargo metadata --format-version 1 --no-deps --locked |
    jq -r '.packages[].targets[] | select(.kind == ["test"]) | .name' |
    LC_ALL=C sort -u)

if [ -z "$names" ]; then
    echo "test-shard.sh: cargo metadata listed no test targets" >&2
    exit 1
fi

# The slow names that exist, in the list's order, then every other name.
{
    printf '%s\n' "$slow" | while IFS= read -r name; do
        if printf '%s\n' "$names" | grep -qxF "$name"; then
            printf '%s\n' "$name"
        fi
    done
    printf '%s\n' "$names" | grep -vxF "$slow" || true
} | awk -v k="$k" -v n="$n" '(NR - 1) % n == k { print "--test"; print $0 }'
