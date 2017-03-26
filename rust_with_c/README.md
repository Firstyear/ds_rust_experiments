This library shows that we can compile rust that internally contains C.

We do this through autotools to complete the build, and you can check
for the relevant symbols.

autoreconf -fiv
./configure
make
make cargo_check

In theory, we don't actually need autotools for this, but we do it
to prove the point.

cargo test

Is just as valid.

readelf -Ws librwc.so | grep -i hello

To assert the symbols really did work.

