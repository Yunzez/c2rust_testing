#!/bin/bash
S=/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad
while read -r -u 3 e flag; do
  "$S/plan_build.sh" "$e" "$flag" </dev/null
done 3< "$S/plan_entries.txt"
echo ALLDONE
