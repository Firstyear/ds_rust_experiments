This shows how we can use configure options to swap between a
debug and a release build for rust.

We use the same --enable-debug flag as normal with ./configure,
and it will enable the release flag.

We also use debug symbols as rpm will strip them for us

To test:

autoreconf -fiv
./configure --enable-debug
make
make check

./configure
make
make check


