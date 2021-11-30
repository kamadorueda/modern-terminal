t:
  clear
  cargo test

r ESCAPE:
  @echo -e '{{ESCAPE}}' | sed 's|\\u{1b}|\x1b|g'
