
RUSTC_FLAGS = @asan_rust_defs@ @debug_rust_defs@
AM_CPPFLAGS = @asan_c_defs@ @debug_r_defs@

lib_LTLIBRARIES = libplugin_r.la

# Build the rust plugin as a .so so we can dlopen it.

# am_libplugin_r_la_OBJECTS = plugin_r.o
libplugin_r_la_SOURCES = ""
libplugin_r_la_LIBADD = libplugin_r.a

plugin_r.o:
	cargo rustc $(RUSTC_FLAGS) -- --emit obj=$(abs_builddir)/$@

libplugin_r.a: plugin_r.o
	cargo rustc $(RUSTC_FLAGS) -- --crate-type=staticlib --emit link -o $(abs_builddir)/$@

