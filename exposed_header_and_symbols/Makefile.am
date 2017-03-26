# .PHONY: libehasx.a ehasx.o

lib_LTLIBRARIES = libehasx.la

# Rust compiler flags
if DEBUG
RUSTC_FLAGS =
else
RUSTC_FLAGS = --release
endif
RUST_LDFLAGS = -ldl -lpthread -lgcc_s -lc -lm -lrt -lutil

# Hello Rust!
am_libehasx_la_OBJECTS = ehasx.o
libehasx_la_SOURCES = ""
libehasx_la_LIBADD = libehasx.a

ehasx.o:
	cargo rustc $(RUSTC_FLAGS) -- --emit obj=$(abs_builddir)/$@

libehasx.a:
	cargo rustc $(RUSTC_FLAGS) -- --crate-type=staticlib --emit link=$(abs_builddir)/$@

cargo_check:
	cargo test

check_PROGRAMS = test_ehasx
TESTS = test_ehasx

test_ehasx_SOURCES = test.c
test_ehasx_LDADD = libehasx.la
test_ehasx_LDFLAGS = $(RUST_LDFLAGS)


clean-local:
	cargo clean
	rm libehasx.{a,d,so}


