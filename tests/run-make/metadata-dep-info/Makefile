include ../tools.mk

ifdef RUSTC_BLESS_TEST
    RUSTC_TEST_OP = cp
else
    RUSTC_TEST_OP = $(DIFF)
endif

all:
	$(RUSTC) --emit=metadata,dep-info --crate-type lib dash-separated.rs -C extra-filename=_something-extra
	# Strip TMPDIR since it is a machine specific absolute path
	sed "s%.*[/\\]%%" "$(TMPDIR)"/dash-separated_something-extra.d > "$(TMPDIR)"/dash-separated_something-extra.normalized.d
	$(RUSTC_TEST_OP) "$(TMPDIR)"/dash-separated_something-extra.normalized.d dash-separated_something-extra.normalized.d
