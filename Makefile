VERSION = 1.0.0
PN = memoize

PREFIX ?= /usr
BINDIR = $(PREFIX)/bin

install: install-bin

install-bin:
	@echo -e '\033[1;32minstalling scripts...\033[0m'
	install -Dm755 bin/* "$(DESTDIR)$(BINDIR)"
