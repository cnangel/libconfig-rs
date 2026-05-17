Name:           libconfig-rs
Version:        0.1.0
Release:        1%{?dist}
Summary:        Safe Rust bindings for the libconfig C configuration library
License:        BSD
URL:            https://github.com/cnangel/libconfig-rs
Source0:        %{name}-%{version}.tar.gz

# find-debuginfo.sh cannot parse Rust DWARF to locate .rs sources.
# Disable only the debugsource sub-package; debuginfo from the .so is kept.
%global _debugsource_packages 0

BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  libconfig-devel
BuildRequires:  gcc

%description
Safe Rust wrapper around the libconfig C library providing compile-time
lifetime safety, full type safety, and all libconfig 1.1.x through 1.8.x
features including configuration options, formatting, and safe getters.

%prep
%setup -q

%build
cargo build --release

%install
# Install shared library for linking and debuginfo extraction
# cargo names cdylib as lib<crate>.so; rename to libconfig_rs to avoid
# confusion with the system libconfig.so
install -d -m 755 %{buildroot}%{_libdir}
install -m 755 target/release/liblibconfig.so %{buildroot}%{_libdir}/libconfig_rs.so

# Install source to cargo registry
install -d -m 755 %{buildroot}%{_datadir}/cargo/registry/%{name}-%{version}
tar --exclude=target -cf - . | tar -xf - -C %{buildroot}%{_datadir}/cargo/registry/%{name}-%{version}/

%files
%doc README.md
%{_libdir}/libconfig_rs.so
%{_datadir}/cargo/registry/%{name}-%{version}

%changelog
* Sun May 17 2026 Cnangel <cnangel@gmail.com> 0.1.0-1
- Initial Rust binding for libconfig with full API surface