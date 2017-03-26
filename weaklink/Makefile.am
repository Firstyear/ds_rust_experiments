
bin_PROGRAMS = test_link

test_link_SOURCES = test_link.c

if RUST_ENABLE

test_link_LDADD = libweaklink.la
test_link_LDFLAGS = $(RUST_LDFLAGS)

lib_LTLIBRARIES = libweaklink.la

# Rust compiler flags
if DEBUG
RUSTC_FLAGS =
else
RUSTC_FLAGS = --release
endif
RUST_LDFLAGS = -ldl -lpthread -lgcc_s -lc -lm -lrt -lutil

# Hello Rust!
am_libweaklink_la_OBJECTS = weaklink.o
libweaklink_la_SOURCES = ""
libweaklink_la_LIBADD = libweaklink.a

weaklink.o:
	cargo rustc $(RUSTC_FLAGS) -- --emit obj=$(abs_builddir)/$@

libweaklink.a:
	cargo rustc $(RUSTC_FLAGS) -- --crate-type=staticlib --emit link -o $(abs_builddir)/$@

cargo_check:
	cargo test

clean-local:
	cargo clean
	rm libweaklink.{a,d,so}

else

RUST_LDFLAGS =

endif




