

RUSTC_FLAGS = @asan_rust_defs@ @debug_rust_defs@ -L $(abs_builddir)/.rlibs/
AM_CPPFLAGS = @asan_c_defs@ @debug_c_defs@

RUST_LDFLAGS = -ldl -lpthread -lgcc_s -lc -lm -lrt -lutil

%.rs:
	echo "---------------------------------------------"
	echo $@ $<
	rustc $(RUSTC_FLAGS) --crate-type dylib --emit obj,link -o $(abs_builddir)/ $<

rlibs:
	-mkdir -p $(abs_builddir)/.rlibs/

# liblibc.rlib: rlibs
# 	rustc $(RUSTC_FLAGS) --crate-type rlib --cfg 'feature="libc"' -o $(abs_builddir)/.rlibs/$@ external/libc/src/lib.rs

libdylib.rlib: rlibs
	rustc $(RUSTC_FLAGS) --crate-type rlib --cfg 'feature="libc"' -o $(abs_builddir)/.rlibs/$@ external/dylib/src/lib.rs


lib_LTLIBRARIES = 	liblibrary.la \
					libplugin_c.la \
					libplugin_r.la

bin_PROGRAMS = controller_bin

liblibrary_la_SOURCES = library/src/lib.rs

libplugin_r_la_SOURCES = plugin_r/src/lib.rs

libplugin_c_la_SOURCES = plugin_c/plugin.c
libplugin_c_la_LDFLAGS = $(RUST_LDFLAGS)


controller_bin_SOURCES = controller/src/main.rs
# controller_bin_DEPENDENCIES = 	liblibc.rlib \
# 								libdylib.rlib

controller_bin_DEPENDENCIES = 	libdylib.rlib



