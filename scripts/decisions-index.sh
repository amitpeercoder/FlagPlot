#!/bin/sh
# Makes the table of docs/decisions/README.md from the records (decision
# 0108, item 9). The table is never edited by hand.
#
#   sh scripts/decisions-index.sh          writes the table
#   sh scripts/decisions-index.sh --check  says whether the
#                                                  committed table is what
#                                                  the records give
#
# Every record begins with the same header, each part a paragraph of its
# own:
#
#   # NNNN: Title
#
#   Status: the record's own words on its standing, never rewritten here
#
#   Summary: the one line the index shows for it
#
#   Standing: the short standing the index shows (Confirmed, Settled, ...)
#
# One row is made for each record, in the order of the file names, with the
# number before the title's first ": ", the summary and the standing. Only
# the lines between the two markers in README.md are written (the table,
# with an empty line before and after it); the prose
# around them is kept as it is. A record without the header is an error
# that names it, and nothing is written.
#
# Run from the top of the repository. POSIX sh and awk only, so that CI runs
# it on every commit, documents alone included.

set -eu
LC_ALL=C
export LC_ALL

dir=docs/decisions
index="$dir/README.md"
start='<!-- The table is generated from the records by scripts/decisions-index.sh: do not edit it by hand. -->'
end='<!-- End of the generated table. -->'

mode=write
case "${1:-}" in
    "") ;;
    --check) mode=check ;;
    *) echo "usage: sh scripts/decisions-index.sh [--check]" >&2; exit 2 ;;
esac

if [ ! -f "$index" ]; then
    echo "decisions-index: $index not found; run from the top of the repository" >&2
    exit 2
fi

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

# The rows, from each record's header.
{
    echo '| # | Decision | Status |'
    echo '| --- | --- | --- |'
} > "$tmp/table"
problems=0
for record in "$dir"/[0-9][0-9][0-9][0-9]-*.md; do
    if ! awk -v file="$record" '
        NR == 1 {
            if (substr($0, 1, 2) != "# " || index($0, ": ") == 0) {
                print file ": line 1 is not \"# NNNN: Title\"" > "/dev/stderr"; bad = 1; exit
            }
            id = substr($0, 3, index($0, ": ") - 3)
        }
        NR == 3 && substr($0, 1, 8) != "Status: " {
            print file ": line 3 is not \"Status: ...\"" > "/dev/stderr"; bad = 1; exit
        }
        NR == 5 {
            if (substr($0, 1, 9) != "Summary: ") {
                print file ": line 5 is not \"Summary: ...\"" > "/dev/stderr"; bad = 1; exit
            }
            summary = substr($0, 10)
        }
        NR == 7 {
            if (substr($0, 1, 10) != "Standing: ") {
                print file ": line 7 is not \"Standing: ...\"" > "/dev/stderr"; bad = 1; exit
            }
            standing = substr($0, 11)
        }
        (NR == 2 || NR == 4 || NR == 6 || NR == 8) && $0 != "" {
            print file ": line " NR " is not empty" > "/dev/stderr"; bad = 1; exit
        }
        NR == 8 { exit }
        END {
            if (bad) exit 1
            if (NR < 8) { print file ": the header is not complete" > "/dev/stderr"; exit 1 }
            if (index(summary, "|") || index(standing, "|")) {
                print file ": a summary or standing holds a \"|\"" > "/dev/stderr"; exit 1
            }
            print "| " id " | " summary " | " standing " |"
        }
    ' "$record" >> "$tmp/table"; then
        problems=$((problems + 1))
    fi
done
if [ "$problems" -ne 0 ]; then
    echo "decisions-index: $problems record(s) without the header; nothing written" >&2
    exit 1
fi

# The README with the lines between the markers replaced.
if ! awk -v start="$start" -v end="$end" -v table="$tmp/table" '
    $0 == start { print; print ""; while ((getline row < table) > 0) print row; print ""; skipping = 1; seen_start++; next }
    $0 == end { skipping = 0; seen_end++ }
    !skipping { print }
    END { if (seen_start != 1 || seen_end != 1) exit 1 }
' "$index" > "$tmp/README.md"; then
    echo "decisions-index: $index must hold each marker line once:" >&2
    echo "  $start" >&2
    echo "  $end" >&2
    exit 1
fi

if [ "$mode" = check ]; then
    if cmp -s "$index" "$tmp/README.md"; then
        echo "decisions-index: $index is what the records give"
        exit 0
    fi
    echo "decisions-index: $index is not what the records give. Edit the record's" >&2
    echo "Summary or Standing line, not the table, and run sh scripts/decisions-index.sh" >&2
    diff "$index" "$tmp/README.md" >&2 || true
    exit 1
fi

if cmp -s "$index" "$tmp/README.md"; then
    echo "decisions-index: $index unchanged"
else
    cp "$tmp/README.md" "$index"
    echo "decisions-index: wrote $index"
fi
