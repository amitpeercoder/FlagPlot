#!/bin/sh
# Says whether a commit needs the full checks: prints "true" unless every
# file it changes against its first parent is a Markdown document.
#
# CI checks out one commit with its parents. For a pull request that is the
# merge of the branch into its base, so the first parent is the base and the
# difference is the whole pull request. For a push to main it is the merge
# commit, and the first parent is main as it was.
#
# Anything unexpected prints "true": no parent, a failed diff, no files.
# The full checks are the safe answer.

commit="${1:-HEAD}"

if ! files=$(git diff --name-only "$commit^1" "$commit" 2>/dev/null); then
    echo true
    exit 0
fi
if [ -z "$files" ]; then
    echo true
    exit 0
fi

# One name a line. A name that does not end in .md needs the checks.
if printf '%s\n' "$files" | grep -qv '\.md$'; then
    echo true
else
    echo false
fi
