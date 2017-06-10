
RUSTC_FLAGS = @asan_rust_defs@ @debug_rust_defs@
AM_CPPFLAGS = @asan_c_defs@ @debug_r_defs@

# lib_LTLIBRARIES = libcontroller.la
bin_PROGRAMS = controller

controller:
	cargo rustc $(RUSTC_FLAGS)
	cargo install --root=$(abs_builddir) --bin $@
	mv bin/$@ ./


