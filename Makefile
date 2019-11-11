VERSION = 1.0.0
PN = memoize

PREFIX ?= /usr
BINDIR = $(PREFIX)/bin

install: install-shell

install-shell:
	@echo -e '\033[1;32minstalling shell scripts...\033[0m'
	mkdir -p "$(DESTDIR)$(BINDIR)"
	install -Dm755 src/sh/* "$(DESTDIR)$(BINDIR)"
