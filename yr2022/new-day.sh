#!/usr/bin/env sh

dir="./src/bin/day$1"

mkdir "$dir"
cp "./src/bin/template.rs" "$dir/main.rs"
touch "$dir/input.txt"
