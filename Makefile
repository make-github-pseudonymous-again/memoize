.PHONY: all install install-shell build-rust build-rust-release check-rust test-rust clean

VERSION = 1.0.0
PN = memoize

PREFIX ?= /usr
BINDIR = $(PREFIX)/bin

SHSRC = src/sh

RUSTPACKAGE = src/rust/memoize
RUSTLIBPACKAGE = src/rust/memoize_lib
RUSTSRC = $(RUSTPACKAGE)/src/bin
RUSTBIN = _build/rust/bin
RUSTLIB = _build/rust/lib
RUSTSOURCES = $(shell find $(RUSTSRC) -type f)
RUSTBINARIES = $(basename $(subst $(RUSTSRC),$(RUSTBIN),$(RUSTSOURCES)))

all:
	echo $(RUSTBINARIES)

install: install-shell

install-shell:
	@echo -e '\033[1;32minstalling shell scripts...\033[0m'
	mkdir -p "$(DESTDIR)$(BINDIR)"
	install -Dm755 $(SHSRC)/* "$(DESTDIR)$(BINDIR)"

build-rust-release: $(RUSTSOURCES)
	cargo build \
		--release \
		--manifest-path $(RUSTPACKAGE)/Cargo.toml \
		--target-dir $(RUSTBIN)

build-rust: $(RUSTSOURCES)
	cargo build \
		--manifest-path $(RUSTPACKAGE)/Cargo.toml \
		--target-dir $(RUSTBIN)

check-rust: $(RUSTSOURCES)
	cargo check \
		--verbose \
		--manifest-path $(RUSTPACKAGE)/Cargo.toml \
		--target-dir $(RUSTBIN)

test-rust: $(RUST)
	cargo test \
		--verbose \
		--manifest-path $(RUSTLIBPACKAGE)/Cargo.toml \
		--target-dir $(RUSTLIB)

clean:
	rm -rf _build
