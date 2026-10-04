.POSIX:
.PHONY: all build release test lint fmt install uninstall clean help

PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
DESTDIR ?=
CARGO ?= cargo
TARGET ?= x86_64-unknown-linux-musl

all: build

build:
	$(CARGO) build

release:
	$(CARGO) build --release

test:
	$(CARGO) test

lint:
	$(CARGO) fmt --check
	$(CARGO) clippy --all-targets -- -D warnings

fmt:
	$(CARGO) fmt

wasm:
	wasm-pack build --target web --out-dir site/pkg --release -- --no-default-features --features wasm

install: release
	install -d "$(DESTDIR)$(BINDIR)"
	install -m 0755 target/release/ayvu "$(DESTDIR)$(BINDIR)/ayvu" 2>/dev/null || \
	install -m 0755 target/$(TARGET)/release/ayvu "$(DESTDIR)$(BINDIR)/ayvu"

uninstall:
	rm -f "$(DESTDIR)$(BINDIR)/ayvu"

clean:
	$(CARGO) clean

help:
	@echo "Ayvu build targets:"
	@echo "  all       Build debug binary (default)"
	@echo "  release   Build optimized release binary"
	@echo "  test      Run test suite"
	@echo "  lint      Run formatting check and clippy lints"
	@echo "  fmt       Auto-format codebase"
	@echo "  install   Install binary to (DESTDIR)(BINDIR)"
	@echo "  uninstall Remove installed binary"
	@echo "  clean     Remove cargo build target artifacts"
