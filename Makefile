.PHONY: build test clean check rpm deb compat-test

CARGO := cargo
VERSION := $(shell grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')

build:
	$(CARGO) build --release

test: build
	$(CARGO) test --release

clean:
	$(CARGO) clean
	rm -rf *.rpm *.deb

check:
	$(CARGO) check
	$(CARGO) clippy 2>/dev/null || true

rpm: build
	mkdir -p $$HOME/rpmbuild/SOURCES
	tar -czf /tmp/libconfig-rs-$(VERSION).tar.gz \
		--transform 's,^.,libconfig-rs-$(VERSION),' --exclude target -C . .
	cp /tmp/libconfig-rs-$(VERSION).tar.gz $$HOME/rpmbuild/SOURCES/
	rpmbuild -ba libconfig-rs.spec

deb: build
	dpkg-buildpackage -us -uc -b

# Compatibility testing against old libconfig versions
compat-test-1.3:
	PKG_CONFIG_PATH=/tmp/libconfig-install/1.3.2/lib/pkgconfig $(CARGO) build

compat-test-1.4:
	LD_LIBRARY_PATH=/tmp/libconfig-install/1.4.8/lib PKG_CONFIG_PATH=/tmp/libconfig-install/1.4.8/lib/pkgconfig $(CARGO) test

compat-test-1.5:
	LD_LIBRARY_PATH=/tmp/libconfig-install/1.5/lib PKG_CONFIG_PATH=/tmp/libconfig-install/1.5/lib/pkgconfig $(CARGO) test

compat-test: compat-test-1.3 compat-test-1.4 compat-test-1.5