Name:           libconfig-rs
Version:        0.1.0
Release:        1%{?dist}
Summary:        Safe Rust bindings for the libconfig C configuration library
License:        BSD
URL:            https://github.com/cnangel/libconfig-rs
Source0:        %{name}-%{version}.tar.gz

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
# This is a Rust library crate — no binaries to install
# RPM packaging for -devel crate only
install -d -m 755 %{buildroot}%{_datadir}/cargo/registry/%{name}-%{version}
cp -r . %{buildroot}%{_datadir}/cargo/registry/%{name}-%{version}/

%files
%doc README.md
%{_datadir}/cargo/registry/%{name}-%{version}

%changelog
* Sat May 16 2026 Cnangel <cnangel@gmail.com> 0.1.0-1
- Initial Rust binding for libconfig with full API surface