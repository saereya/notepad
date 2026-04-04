PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin

.PHONY: build install uninstall clean

build:
	cargo build --release

install: build
	install -d $(DESTDIR)$(BINDIR)
	install -m 755 target/release/notepad $(DESTDIR)$(BINDIR)/notepad

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/notepad

clean:
	cargo clean
