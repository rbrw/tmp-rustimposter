
libdir := "$(CURDIR)"/target/debug
libmax := $(CURDIR)/target/debug/libmax.so

all: max

check: max
	LD_LIBRARY_PATH="$(libdir)" ./max | grep -E '^max: 11\.0+$$'

.PHONY: $(libmax)
$(libmax):
	cargo build

max: $(libmax)

#lib%.so: %.rs
#	rustc --crate-type=dylib -C prefer-dynamic $<

clean:
	rm -f max
	cargo clean
