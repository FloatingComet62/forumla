dev loglevel="debug":
  RUST_LOG={{loglevel}} cargo run --example basic

fmt:
  cargo fmt

publish:
  cd forumla-macros
  cargo publish
  cd ..
  cargo publish
