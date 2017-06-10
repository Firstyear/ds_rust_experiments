
RUSTC_FLAGS = @asan_rust_defs@ @debug_rust_defs@
AM_CPPFLAGS = @asan_c_defs@ @debug_c_defs@

RUST_LDFLAGS = -ldl -lpthread -lgcc_s -lc -lm -lrt -lutil

bin_PROGRAMS = controller

lib_LTLIBRARIES = 	liblibrary.la \
					libplugin_c.la \
					libplugin_r.la

liblibrary_la_SOURCES = ""
liblibrary_la_LIBADD = liblibrary.ra

libplugin_c_la_SOURCES = plugin_c/plugin.c
libplugin_c_la_LDFLAGS = $(RUST_LDFLAGS)

libplugin_r_la_SOURCES = ""
libplugin_r_la_LIBADD = libplugin_r.ra

%.ra:
	cd $(shell echo $@ | sed 's/\.ra$$//' | sed 's/^lib//') && \
	cargo rustc $(RUSTC_FLAGS) -- --emit obj=$(abs_builddir)/$@ && \
	cargo rustc $(RUSTC_FLAGS) -- --crate-type=staticlib --emit link -o $(abs_builddir)/$@

controller:
	cd $@ && cargo rustc $(RUSTC_FLAGS)
	cd $@ && cargo install --root=$(abs_builddir) --bin $@
	# mv /bin/$@ ./


