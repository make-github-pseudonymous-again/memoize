.PHONY: all install install-shell install-dash install-rust check-shell build-dash build-rust build-rust-release check-rust test test-cli test-cli-shell test-cli-rust test-rust-sources test-rust-sources-bin test-rust-sources-lib clean

VERSION = 1.0.0
PN = memoize

PREFIX ?= /usr
BINDIR = $(PREFIX)/bin

SHSRC = src/sh
SHSOURCES = $(shell find $(SHSRC) -type f)

RUSTBINPACKAGE = src/rust/memoize
RUSTLIBPACKAGE = src/rust/memoize_lib
RUSTSRC = $(RUSTBINPACKAGE)/src/bin
RUSTBIN = _build/rust/bin
RUSTLIB = _build/rust/lib
RUSTSOURCES = $(shell find $(RUSTSRC) -type f)
RUSTBINARIES_DEBUG = $(basename $(subst $(RUSTSRC),$(RUSTBIN)/debug,$(RUSTSOURCES)))
RUSTBINARIES_RELEASE = $(basename $(subst $(RUSTSRC),$(RUSTBIN)/release,$(RUSTSOURCES)))

DASHBUILD = _build/dash

all:
	@echo $(RUSTBINARIES_DEBUG)

install: install-shell

install-shell:
	@echo -e '\033[1;32minstalling shell scripts...\033[0m'
	mkdir -p "$(DESTDIR)$(BINDIR)"
	install -Dm755 $(SHSRC)/* "$(DESTDIR)$(BINDIR)"

install-dash: build-dash
	@echo -e '\033[1;32minstalling dash scripts...\033[0m'
	mkdir -p "$(DESTDIR)$(BINDIR)"
	install -Dm755 $(DASHBUILD)/* "$(DESTDIR)$(BINDIR)"

install-rust: build-rust-release
	@echo -e '\033[1;32minstalling rust binaries...\033[0m'
	mkdir -p "$(DESTDIR)$(BINDIR)"
	install -Dm755 $(shell find $(RUSTBIN)/release -maxdepth 1 -perm -111 -type f) "$(DESTDIR)$(BINDIR)"

check-shell:
	shellcheck $(SHSRC)/*
	shellcheck tests/run tests/setup tests/teardown tests/sandbox

build-dash: $(SHSOURCES)
	mkdir -p $(DASHBUILD)
	cp $(SHSRC)/* $(DASHBUILD)
	sed 's:^#!/usr/bin/env sh$$:#!/usr/bin/env dash:g' -i $(DASHBUILD)/*

build-rust-release: $(RUSTSOURCES)
	cargo build \
		--release \
		--manifest-path $(RUSTBINPACKAGE)/Cargo.toml \
		--target-dir $(RUSTBIN)

build-rust: $(RUSTSOURCES)
	cargo build \
		--manifest-path $(RUSTBINPACKAGE)/Cargo.toml \
		--target-dir $(RUSTBIN)

check-rust: $(RUSTSOURCES)
	cargo check \
		--verbose \
		--manifest-path $(RUSTBINPACKAGE)/Cargo.toml \
		--target-dir $(RUSTBIN)

test: test-rust-sources test-cli

test-cli: test-cli-shell test-cli-rust

test-cli-shell:
	env PATH="$(abspath $(SHSRC)):$(PATH)" sh tests/run tests

test-cli-rust: build-rust
	env PATH="$(abspath $(RUSTBIN))/debug:$(PATH)" sh tests/run tests

test-rust-sources: test-rust-sources-lib test-rust-sources-bin

test-rust-sources-bin:
	cargo test \
		--verbose \
		--manifest-path $(RUSTBINPACKAGE)/Cargo.toml \
		--target-dir $(RUSTBIN)

test-rust-sources-lib:
	cargo test \
		--verbose \
		--manifest-path $(RUSTLIBPACKAGE)/Cargo.toml \
		--target-dir $(RUSTLIB)

clean:
	rm -rf _build
