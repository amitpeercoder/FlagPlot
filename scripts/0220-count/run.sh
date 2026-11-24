#!/bin/sh
# The count of decision 0220 on the games for play: every scenario with
# `for_play = true` that gives both sides orders (staff against staff), live
# figures, seeds 1 to 100. From the top of the repository, with OUT outside it:
#   cargo build --release -p sim-cli
#   scripts/0220-count/run.sh OUT
#   python3 scripts/0220-count/count.py sum OUT/*.jsonl
# Each run is exported to a file that is read and removed.
set -eu
OUT=$1
mkdir -p "$OUT"
SIM=target/release/sim-cli
FILES="data/scenarios/coral-sea/01a_coral_sea.toml
data/scenarios/coral-sea/02a_coral_sea_oilers.toml
data/scenarios/coral-sea/03a_coral_sea_both_oilers.toml
data/scenarios/five-days/01a_eastern_solomons.toml
data/scenarios/five-days/02a_eastern_solomons_oilers.toml
data/scenarios/midway/01a_midway.toml
data/scenarios/table-boats/01a_midway_submarines.toml
data/scenarios/table-boats/01b_midway_submarines.toml
data/scenarios/table-boats/01c_midway_submarines.toml
data/scenarios/table-boats/02a_eastern_solomons_oilers_submarines.toml
data/scenarios/table-boats/02b_eastern_solomons_oilers_submarines.toml
data/scenarios/table-boats/02c_eastern_solomons_oilers_submarines.toml
data/scenarios/table-boats/03a_coral_sea_both_oilers_submarines.toml
data/scenarios/table-boats/03b_coral_sea_both_oilers_submarines.toml
data/scenarios/table-boats/03c_coral_sea_both_oilers_submarines.toml"
for f in $FILES; do
  name=$(basename "$f" .toml)
  seq 1 100 | OUT="$OUT" SIM="$SIM" F="$f" NAME="$name" xargs -P 8 -I{} sh -c '
    tmp="$OUT/$NAME.{}.json"
    "$SIM" export "$F" --seed {} --out "$tmp" >/dev/null
    python3 scripts/0220-count/count.py pairs "$tmp" --label "$NAME" --seed {} >"$OUT/$NAME.{}.part"
    rm "$tmp"'
  cat "$OUT/$name".*.part >"$OUT/$name.jsonl"
  rm "$OUT/$name".*.part
done
