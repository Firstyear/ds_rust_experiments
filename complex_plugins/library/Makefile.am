
RUSTC_FLAGS = @asan_rust_defs@ @debug_rust_defs@
AM_CPPFLAGS = @asan_c_defs@ @debug_r_defs@

lib_LTLIBRARIES = liblibrary.la

# Build the library as a .so so that plugin.c can link it.

# am_liblibrary_la_OBJECTS = library.o
liblibrary_la_SOURCES = ""
liblibrary_la_LIBADD = liblibrary.a

library.o:
	cargo rustc $(RUSTC_FLAGS) -- --emit obj=$(abs_builddir)/$@

liblibrary.a: library.o
	cargo rustc $(RUSTC_FLAGS) -- --crate-type=staticlib --emit link -o $(abs_builddir)/$@


