#!/bin/sh
# Stands in for `cargo` (through the CARGO variable the tests read) in the
# jobs that run the tests from an archive built elsewhere, so that the
# scenario tests that shell out to cargo to run the command-line program
# (scenarios 36, 40, 41, 42, 45 and 104) do not build it again in every job.
#
#   cargo build [--quiet] -p sim-cli [--release]
#       builds nothing: puts the prebuilt program where a build would have
#       (target/debug, or target/release with --release).
#   cargo run [--quiet] -p sim-cli [--release] -- ARGUMENTS
#       runs the prebuilt program with ARGUMENTS.
#   anything else
#       goes to the real cargo.
#
# SIM_CLI_PREBUILT is the program the archive holds (the `checked` profile:
# the same state, events and hashes as the dev profile, by the rules of
# docs/engineering/determinism.md, and faster). REAL_CARGO is the cargo to
# fall back on. The script fails, and builds nothing, when the program is not
# there.

set -eu

real=${REAL_CARGO:-cargo}
prebuilt=${SIM_CLI_PREBUILT:-}

command=${1:-}
case "$command" in
build | run) ;;
*) exec "$real" "$@" ;;
esac
shift

# Only the forms the tests use: flags, `-p sim-cli`, flags, then (for run)
# `--` and the program's arguments.
release=no
package=
while [ $# -gt 0 ]; do
    case "$1" in
    --quiet | -q) ;;
    --release) release=yes ;;
    -p) shift; package=${1:-} ;;
    --) shift; break ;;
    *) package="?$1" ;;
    esac
    shift
done

if [ "$package" != sim-cli ] || [ -z "$prebuilt" ] || [ ! -x "$prebuilt" ]; then
    echo "cargo-prebuilt.sh: asked for something it cannot answer from the archive: $command, package '$package', program '$prebuilt'" >&2
    exit 97
fi

if [ "$command" = run ]; then
    exec "$prebuilt" "$@"
fi

if [ "$release" = yes ]; then dir=release; else dir=debug; fi
target=${CARGO_TARGET_DIR:-target}/$dir
mkdir -p "$target"
# Tests run at the same time, one process each, and each asks for the build:
# copy to a name of its own and rename, so the program is never half there.
cp "$prebuilt" "$target/sim-cli.$$"
mv -f "$target/sim-cli.$$" "$target/sim-cli"
