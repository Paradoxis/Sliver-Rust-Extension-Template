TARGETS := x86_64-unknown-linux-gnu x86_64-pc-windows-gnu

all: release

debug:
	$(foreach target,$(TARGETS),cargo build --lib --target $(target);)
	cp target/x86_64-pc-windows-gnu/debug/hello.dll hello.dll
	cp target/x86_64-unknown-linux-gnu/debug/libhello.so hello.so

release:
	$(foreach target,$(TARGETS),cargo build --release --lib --target $(target);)
	cp target/x86_64-pc-windows-gnu/release/hello.dll hello.dll
	cp target/x86_64-unknown-linux-gnu/release/libhello.so hello.so
