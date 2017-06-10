
CARGO_FLAGS = @debug_cargo_defs@
RUSTC_FLAGS = @asan_rust_defs@ @debug_rust_defs@
AM_CPPFLAGS = @asan_c_defs@ @debug_c_defs@

RUST_LDFLAGS = -ldl -lpthread -lgcc_s -lc -lm -lrt -lutil

bin_PROGRAMS = controller_bin

lib_LTLIBRARIES = 	liblibrary.la \
					libplugin_c.la \
					libplugin_r.la

liblibrary_la_SOURCES = ""
liblibrary_la_LIBADD = liblibrary.ra

libplugin_c_la_SOURCES = plugin_c/plugin.c
libplugin_c_la_LDFLAGS = $(RUST_LDFLAGS)

libplugin_r_la_SOURCES = ""
libplugin_r_la_LIBADD = libplugin_r.ra

%.ro:
	cd $(shell echo $@ | sed 's/\.ra$$//' | sed 's/^lib//') && \
	cargo rustc $(CARGO_FLAGS) -- $(RUSTC_FLAGS) --emit obj=$(abs_builddir)/$@

%.ra: %.ro
	cd $(shell echo $@ | sed 's/\.ra$$//' | sed 's/^lib//') && \
	cargo rustc $(CARGO_FLAGS) -- $(RUSTC_FLAGS) --crate-type=staticlib --emit link -o $(abs_builddir)/$@

controller_bin:
	cd controller && cargo rustc $(CARGO_FLAGS) -- $(RUSTC_FLAGS)
	cd controller && cargo install --force --root=$(abs_builddir) --bin controller
	mv $(abs_builddir)/bin/controller ./controller_bin

clean-local:
	rm *.ra; true
	rm *.ro; true
	rm *.so; true
	cd plugin_r && cargo clean ; true
	cd library && cargo clean ; true
	cd controller && cargo clean ; true



