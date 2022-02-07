BUILDDIR = build

CARGO ?= cargo
RUSTC ?= rustc
RUST_DIRS := -L $(BUILDDIR) -L target/debug -L target/debug/deps

VALGRIND ?= valgrind

RUSTC_CMD := $(RUSTC) --out-dir $(BUILDDIR) $(RUST_DIRS) -O $(RUSTFLAGS)
VALGRIND_CMD := $(VALGRIND) -q --log-file=/dev/null

LIB_ALL_SRC := $(shell find src -type f -name '*.rs')
LIB         := $(BUILDDIR)/libvgrs.dummy

.PHONY: all
all: $(LIB)

$(BUILDDIR):
	mkdir -p $@

$(LIB): $(LIB_ALL_SRC) | $(BUILDDIR)
	$(CARGO) build
	touch $(LIB)

TEST_TOOLS = valgrind memcheck

.PHONY: check
check: 
	cd test && cargo build
	$(VALGRIND_CMD) --tool=none test/target/debug/vgrs_valgrind_test
	$(VALGRIND_CMD) --tool=memcheck test/target/debug/vgrs_memcheck_test

.PHONY: clean
clean:
	$(CARGO) clean
	rm -fr $(BUILDDIR)
