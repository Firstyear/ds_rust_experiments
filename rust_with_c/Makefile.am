lib_LTLIBRARIES = librwc.la

# Rust compiler flags
if DEBUG
RUSTC_FLAGS =
else
RUSTC_FLAGS = --release
endif

# Hello Rust!
am_librwc_la_OBJECTS = rwc.o
librwc_la_SOURCES = ""
librwc_la_LIBADD = librwc.a

rwc.o:
	cargo rustc $(RUSTC_FLAGS) -- --emit obj=$(abs_builddir)/$@

librwc.a:
	cargo rustc $(RUSTC_FLAGS) -- --crate-type=staticlib --emit link -o $(abs_builddir)/$@

cargo_check:
	cargo test

clean-local:
	cargo clean
	rm librwc.{a,d,so}


