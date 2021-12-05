#! /usr/bin/env bash

set -euo pipefail

pushd examples
npm install svg-term-cli@2.1.1
popd

for example in examples/*.rs; do
  example="$(basename "${example}")"
  example="${example%.rs}"

  cargo build --example "${example}" --release

  asciinema rec \
    --command="
      echo $ cargo run --example ${example}
      echo
      target/release/examples/${example}
      echo
      echo $ cat examples/${example}.rs
      echo
      bat --paging=never --style plain,numbers examples/${example}.rs
      echo
    " \
    --overwrite \
    "examples/${example}.cast"
  ./examples/node_modules/.bin/svg-term \
    --at 10000 \
    --no-cursor \
    --height "$(asciinema play "examples/${example}.cast" | wc -l)" \
    --out "examples/${example}.svg" \
    --window \
    < "examples/${example}.cast"
done
