# This Makefile wraps Rust's cargo build tool.
# It's mostly here for the `install` command.
#
# This Makefile is self-documenting (via the 'help' target)
# see: http://marmelab.com/blog/2016/02/29/auto-documented-makefile.html

HACKS=exuberantbovines exuberantplasma

CARGO ?= cargo
CARGO_OPTS ?=
PREFIX ?= /usr
PANDOC ?= pandoc
PANDOCFLAGS ?=
MANSECTION = 6
MANPAGE.md = $(PANDOC) --standalone $(PANDOCFLAGS) --to man
MANPAGES = $(foreach HACK, $(HACKS), doc/$(HACK).$(MANSECTION))
MANPAGEZ = $(foreach HACK, $(HACKS), doc/$(HACK).$(MANSECTION).gz)

INSTALL_BIN=$(PREFIX)/lib/xscreensaver
INSTALL_CONFIG=$(PREFIX)/share/xscreensaver/config
INSTALL_MAN=$(PREFIX)/share/man/man$(MANSECTION)

all: build doc ## Alias for build

build:    ## Compiles the default target
	@$(CARGO) $(CARGO_OPTS) build

build-debug:     ## Compiles for 'debug' target
	@$(CARGO) $(CARGO_OPTS) build --debug

build-release:   ## Compiles for 'release' target
	@$(CARGO) $(CARGO_OPTS) build --release

install: build-release doc-release  ## Installs everything!
	@echo "Installing under $(PREFIX)..."
	sudo mkdir -p $(PREFIX)/lib/xscreensaver
	sudo mkdir -p $(PREFIX)/share/xscreensaver/config
	@for PROG in $(HACKS); do \
		sudo install -m 0755 target/release/$$PROG $(INSTALL_BIN); \
		sudo install -m 0644 configs/$$PROG.xml $(INSTALL_CONFIG); \
		sudo install -m 0644 doc/$$PROG.$(MANSECTION).gz $(INSTALL_MAN); \
	done
	@echo "Updating mandb (this takes a few seconds)..."
	@sudo mandb -q
	@echo ""
	@echo "IMPORTANT: For the hacks to show up you'll need to add the following lines to ~/.xscreensaver:"
	@echo "";
	@for PROG in $(HACKS); do \
		echo "\tGL:               $$PROG --root             \\\\n\\\\ "; \
	done
	@echo "";
	@echo "(don't worry about duplicate lines, xscreensaver will clean that up)"

clean:   ## Deletes intermediate files (including compiled deps)
	@$(CARGO) $(CARGO_OPTS) clean
	@rm -f $(MANPAGES) $(MANPAGEZ)

check: build test   ## Rebuilds and runs tests

test:    ## Runs tests (if they exist)
	@$(CARGO) $(CARGO_OPTS) test

bench:   ## Run benchmarks (if they exist)
	@$(CARGO) $(CARGO_OPTS) bench

docs: doc

doc: $(MANPAGEZ) ## Builds documentation (if there is any) for the default target
	@$(CARGO) $(CARGO_OPTS) doc --no-deps

doc-release: $(MANPAGEZ) ## Builds documentation (if there is any) for release
	@$(CARGO) $(CARGO_OPTS) doc --no-deps --release

$(MANPAGES): %.$(MANSECTION): %.$(MANSECTION).md
	$(MANPAGE.md) $< -o $@

$(MANPAGEZ): %.$(MANSECTION).gz: %.$(MANSECTION)
	@gzip --keep --force $<

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-25s\033[0m %s\n", $$1, $$2}'

.PHONY: all build build-debug build-release clean install check test bench doc doc-release help
.DEFAULT_GOAL := all
