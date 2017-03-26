

# Rust compiler flags
if DEBUG
RUSTC_FLAGS =
else
RUSTC_FLAGS = --release
endif
# RUST_LDFLAGS = -ldl -lpthread -lgcc_s -lc -lm -lrt -lutil

all-local: dvr_bin

dvr_bin:
	cargo rustc $(RUSTC_FLAGS) -- --crate-type=bin -o $(abs_builddir)/$@


