#!/bin/sh
# Prints the integration test targets of one shard of the tests, as the
# arguments `cargo test` takes: `--test NAME`, one word a line.
#
#   sh .github/scripts/test-shard.sh K N
#
# K counts from 0 and N is the number of shards. The names are every target
# of kind "test" in the workspace, as `cargo metadata` lists them, sorted and
# without repeats; shard K takes the names at places K, K + N, K + 2N and so
# on. So each name is in exactly one shard, a test file added later is in one
# with no change here, and the shards together are every integration test
# that `cargo test --workspace` runs. A name two packages share (`seat`, for
# one) selects the target of that name in both, in the same shard.
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

names=$(cargo metadata --format-version 1 --no-deps --locked |
    jq -r '.packages[].targets[] | select(.kind == ["test"]) | .name' |
    LC_ALL=C sort -u)

if [ -z "$names" ]; then
    echo "test-shard.sh: cargo metadata listed no test targets" >&2
    exit 1
fi

printf '%s\n' "$names" |
    awk -v k="$k" -v n="$n" '(NR - 1) % n == k { print "--test"; print $0 }'
