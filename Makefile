CARGO = cargo
CONFIG_FILE = $(HOME)/.zym
DEFAULT_CONFIG_FILE = $(CURDIR)/.zym-default
XINITRC = $(HOME)/.xinitrc

debug:
	$(SHELL) script/check-zymconf.sh \
		$(CONFIG_FILE) $(DEFAULT_CONFIG_FILE) \
		$(XINITRC) "$(CURDIR)/target/debug/zym $(CONFIG_FILE) & xterm"
	$(CARGO) build
	startx

delete_config:
	rm -r $(CONFIG_FILE)