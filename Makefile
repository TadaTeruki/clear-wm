CARGO = cargo
PROJECT_NAME = clear_wm
CONFIG_FILE = $(HOME)/.$(PROJECT_NAME)_config
DEFAULT_CONFIG_FILE = $(CURDIR)/.wm_default_config
XINITRC = $(HOME)/.xinitrc

debug:
	$(SHELL) script/check_config_file.sh \
		$(CONFIG_FILE) $(DEFAULT_CONFIG_FILE) \
		$(XINITRC) "xcompmgr & $(CURDIR)/target/debug/$(PROJECT_NAME) $(CONFIG_FILE) & xterm" $(PROJECT_NAME)
	$(CARGO) build
	startx

delete_config:
	rm -r $(CONFIG_FILE)