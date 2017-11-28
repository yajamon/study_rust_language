#!/bin/bash

readonly REPOROOT=$(realpath $(dirname $0)/..);

section=""
while [ -z $section ]; do
    echo -n "section name :" >&2
    read -r section _trush
done

cd $REPOROOT
cargo new study_$section --bin
git checkout -b $section

git add $REPOROOT/study_$section
