ifdef GIT_ROOT
else
GIT_ROOT := $(shell git rev-parse --show-toplevel 2>/dev/null)
endif

MK_DIR := $(GIT_ROOT)/.make

include ekgf-make.mk

#
# Install all the tools needed to run this project.
#
.PHONY: install
install: \
	os-tools-install \
	brew-install \
	zip-install \
	rustup-install \
	cargo-install \
	oxigraph-install

.PHONY: build-without-rdfox-dylib
build-without-rdfox-dylib: cargo-check
	$(CARGO_BIN) build --release --no-default-features --workspace

.PHONY: build-with-rdfox-dylib
build-with-rdfox-dylib: cargo-check
	$(CARGO_BIN) build --release --no-default-features --features rdfox-dylib,fs --workspace

.PHONY: build-all
build-all: build-without-rdfox-dylib build-with-rdfox-dylib

.PHONY: test-without-rdfox-dylib
test-without-rdfox-dylib: cargo-check
	$(CARGO_BIN) test --release --no-default-features --workspace

.PHONY: test-with-rdfox-dylib
test-with-rdfox-dylib: cargo-check
	$(CARGO_BIN) test --release --no-default-features --features rdfox-dylib,fs --workspace

.PHONY: test-all
test-all: test-without-rdfox-dylib test-with-rdfox-dylib

#
# Just build the workspace
#
.PHONY: build
build: cargo-build-workspace

.PHONY: build-release
build-release: cargo-build-workspace-release

