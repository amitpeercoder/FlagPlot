#!/bin/bash
# The measurement of decision 0231 ("How it is judged", 3): exports of seeds 1 to 100 of
# one arrangement, read by unfed.py. Usage: run.sh SCENARIO.toml OUTDIR [JOBS]
# The scenario is the proof's arrangement (data/scenarios/stage-3-proof/07a_slice.toml)
# or the same with the table's ground figures. Needs a release sim-cli at
# $SIM_CLI (default target/release/sim-cli).
set -e
S=${SIM_CLI:-target/release/sim-cli}
F=$1; D=$2; J=${3:-4}
mkdir -p "$D"
seq 1 100 | xargs -P "$J" -I{} sh -c "$S export $F --seed {} --out $D/seed{}.json >/dev/null 2>$D/err{}.txt || echo FAIL {}"
python3 -I "$(dirname "$0")/unfed.py" "$D" --all --rows "$D/rows.jsonl"
