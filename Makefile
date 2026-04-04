PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin
DATAROOTDIR = $(PREFIX)/share
APPSDIR = $(DATAROOTDIR)/applications

.PHONY: build install uninstall clean

build:
	cargo build --release

install: build
	install -d $(DESTDIR)$(BINDIR)
	install -m 755 target/release/notepad $(DESTDIR)$(BINDIR)/notepad
	install -d $(DESTDIR)$(APPSDIR)
	install -m 644 notepad.desktop $(DESTDIR)$(APPSDIR)/notepad.desktop

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/notepad
	rm -f $(DESTDIR)$(APPSDIR)/notepad.desktop

clean:
	cargo clean
