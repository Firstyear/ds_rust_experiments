
We use ASAN in DS: So we need to be able to build Rust with it.

RUSTFLAGS="-Z sanitizer=address" cargo run --target x86_64-unknown-linux-gnu --verbose

https://github.com/japaric/rust-san#rust-san

To test, run "make" in this folder.

