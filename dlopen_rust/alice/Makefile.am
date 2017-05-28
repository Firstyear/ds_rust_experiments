
lib_LTLIBRARIES = libalice.la

# Rust compiler flags
if DEBUG
RUSTC_FLAGS =
else
RUSTC_FLAGS = --release
endif
# RUST_LDFLAGS = -ldl -lpthread -lgcc_s -lc -lm -lrt -lutil

am_alice_la_OBJECTS = alice.o
libalice_la_SOURCES = ""
libalice_la_LIBADD = libalice.a

alice.o:
	cargo rustc $(RUSTC_FLAGS) -- --emit obj=$(abs_builddir)/$@

libalice.a:
	cargo rustc $(RUSTC_FLAGS) -- --crate-type=staticlib --emit link -o $(abs_builddir)/$@

clean-local:
	cargo clean
	rm libalice.{a,d,so}




