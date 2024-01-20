TARGETS := x86_64-unknown-linux-gnu x86_64-pc-windows-gnu

all: release

debug:
	$(foreach target,$(TARGETS),cargo build --lib --target $(target);)

release:
	$(foreach target,$(TARGETS),cargo build --release --lib --target $(target);)
