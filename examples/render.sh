#! /usr/bin/env bash

set -euo pipefail

pushd examples
npm install svg-term-cli@2.1.1
popd

for example in examples/*.rs; do
  example="$(basename "${example}")"
  example="${example%.rs}"

  cargo build --example "${example}" --release

  gnome-terminal --wait --zoom=0.5 -- "${SHELL}" -c "
    resize -s 20 80
    asciinema rec \\
      --command=target/release/examples/${example} \\
      --overwrite examples/${example}.cast
  "
  ./examples/node_modules/.bin/svg-term \
    --at 10000 \
    --no-cursor \
    --height "$(asciinema play "examples/${example}.cast" | wc -l)" \
    --out "examples/${example}.svg" \
    < "examples/${example}.cast"
done
