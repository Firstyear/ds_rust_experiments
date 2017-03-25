# .PHONY: libehasx.a ehasx.o

lib_LTLIBRARIES = libehasx.la

# Hello Rust!
am_libehasx_la_OBJECTS = ehasx.o
libehasx_la_SOURCES = ""
libehasx_la_LIBADD = libehasx.a

ehasx.o:
	cargo rustc -- --emit obj=$(abs_builddir)/$@

libehasx.a:
	cargo rustc -- --crate-type=staticlib --emit link -o $(abs_builddir)/$@

check_PROGRAMS = test_ehasx
TESTS = test_ehasx

test_ehasx_SOURCES = test.c
test_ehasx_LDADD = libehasx.la


clean-local:
	cargo clean
	rm libehasx.{a,d,so}


