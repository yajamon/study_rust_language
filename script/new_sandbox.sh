#!/bin/bash

readonly REPOROOT=$(realpath $(dirname $BASH_SOURCE)/..);

section=""
while [ -z $section ]; do
    echo -n "section name :" >&2
    read -r section _trush
done

cd $REPOROOT
cargo new sandbox_$section --bin
git checkout -b sandbox/$section

git add $REPOROOT/sandbox_$section
