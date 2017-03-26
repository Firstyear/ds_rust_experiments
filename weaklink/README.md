This shows how we can use weak C symbols to allow rust overrides
of some functions. This will allow us to do quick testing and
assertion that rust types have the same behaviours, without sacrificing
our C implementations.

To test this:

autoreconf -fiv
./configure --enable-rust
make
./test_link

