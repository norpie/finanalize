#!/usr/bin/env bash

GIT_ROOT=$(git rev-parse --show-toplevel)
PROMPT_IDS=$(find $GIT_ROOT/prompts -mindepth 1 -type d)

rm $GIT_ROOT/db/import_prompts.surql

for PROMPT_ID in $PROMPT_IDS; do
    HBS_FILES=$(find $PROMPT_ID -type f -name "*.hbs")
    PROMPT_ID=$(basename $PROMPT_ID)
    HBS_FILE=$(echo $HBS_FILES | cut -d ' ' -f 1)
    # Read file content, replace newlines with `\n`, and escape `'` with `\'`
    CONTENT=$(awk '{printf "%s\\n", $0}' $HBS_FILE | sed "s/'/\\\'/g")
    IFS=''
    read -r -d '' STATEMENT <<EOM
UPSERT prompt:$PROMPT_ID SET template = '$CONTENT';
EOM
    echo $STATEMENT >>$GIT_ROOT/db/import_prompts.surql
done
