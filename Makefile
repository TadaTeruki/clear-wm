CARGO = cargo

all:
	$(CARGO) run

debug: ~/.xinitrc
	$(CARGO) build
	echo "$(CURDIR)/target/debug/zym & xterm" > ~/.xinitrc
	startx