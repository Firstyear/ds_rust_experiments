# Rust compiler flags
if DEBUG
if ASAN
RUSTC_FLAGS = -Z sanitizer=address
AM_CPPFLAGS = -g3 -DDEBUG -O0 -fsanitize=address -fno-omit-frame-pointer
else
RUSTC_FLAGS =
AM_CPPFLAGS = -g3 -DDEBUG -O0
endif
else
RUSTC_FLAGS = --release
AM_CPPFLAGS =
endif

lib_LTLIBRARIES = libplugin_r.la

# Build the rust plugin as a .so so we can dlopen it.

am_libplugin_r_la_OBJECTS = plugin_r.o
libplugin_r_la_SOURCES = ""
libplugin_r_la_LIBADD = libplugin_r.a

plugin_r.o:
	cargo rustc $(RUSTC_FLAGS) -- --emit obj=$(abs_builddir)/$@

libplugin_r.a:
	cargo rustc $(RUSTC_FLAGS) -- --crate-type=staticlib --emit link -o $(abs_builddir)/$@

