Name:     angkorfetch
Version:  1.0.0
Release:  1%{?dist}
Summary:  A fast, cross-platform system fetch tool

License:  MIT
URL:      https://github.com/AMRSKH/angkorfetch
Source0:  %{url}/archive/v%{version}.tar.gz

BuildRequires: cargo
Requires:      gcc-c++

%description
AngkorFetch is a fast, cross-platform system-info ("fetch") tool for
Windows, Linux, and macOS, written in Rust. It displays system
information (OS, CPU, GPU, memory, disk, network, battery, etc.)
in a colorful terminal output.

%prep
%setup -q

%build
cargo build --release --locked

%install
install -Dm755 target/release/%{name} %{buildroot}%{_bindir}/%{name}
install -Dm644 README.md %{buildroot}%{_docdir}/%{name}/README.md

%files
%{_bindir}/%{name}
%{_docdir}/%{name}/README.md
%license LICENSE

%changelog
* Mon Jul 27 2026 AMRSKH <inforithseyhacambo@gmail.com> - 1.0.0-1
- Initial RPM package
