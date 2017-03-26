
Rather than replacing whole libraries, we can do partial rewrites and exposed
C symbols from a .so.

This shows a rust library which exports a C compatible interface, and a header
that a C program can use.

Key to note is that we need to make sure that anything we malloc in Rust, must
be freed by rust: same for C.

To test this library:

autoreconf -fiv
./configure
make
make cargo_check
make check

To check that there are no leaks:

cp libehasx.so libehasx.so.0
LD_LIBRARY_PATH=./ valgrind .libs/test_ehasx



