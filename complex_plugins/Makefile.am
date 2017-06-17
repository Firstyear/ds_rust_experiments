

RUSTC_FLAGS = @asan_rust_defs@ @debug_rust_defs@ -L $(abs_builddir)/.rlibs/
AM_CPPFLAGS = @asan_c_defs@ @debug_c_defs@

RUST_LDFLAGS = -ldl -lpthread -lgcc_s -lc -lm -lrt -lutil

rlibs:
	-mkdir -p $(abs_builddir)/.rlibs/

.rs.a :
	rustc $(RUSTC_FLAGS) --crate-type staticlib --emit link -o $@ $<

.rs.o :
	rustc $(RUSTC_FLAGS) --crate-type cdylib --emit obj -o $@ $<

libdylib.rlib: rlibs
	rustc $(RUSTC_FLAGS) --crate-type rlib --crate-name dylib --cfg 'feature="libc"' -o $(abs_builddir)/.rlibs/$@ external/dylib/src/lib.rs

controller_bin$(EXEEXT): libdylib.rlib
	rustc $(RUSTC_FLAGS) --crate-type bin -o $(abs_builddir)/controller_bin controller/src/main.rs

lib_LTLIBRARIES = 	liblibrary.la \
					libplugin_c.la \
					libplugin_r.la

bin_PROGRAMS = controller_bin

liblibrary_la_LIBADD = library/src/lib.a
am_liblibrary_la_OBJECTS = library/src/lib.o
liblibrary_la_SOURCES = ""

libplugin_r_la_LIBADD = plugin_r/src/lib.a
am_libplugin_r_la_OBJECTS = plugin_r/src/lib.o
libplugin_r_la_SOURCES = ""

libplugin_c_la_SOURCES = plugin_c/plugin.c
libplugin_c_la_LDFLAGS = $(RUST_LDFLAGS)

clean-local:
	rm $(abs_builddir)/.rlibs/*.rlib; true
	rm plugin_r/src/lib.{o,a}; true
	rm library/src/lib.{o,a}; true



