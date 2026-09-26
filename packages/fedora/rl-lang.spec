Name:           rl-lang
Version:        2.3.0
Release:        1%{?dist}
Summary:        Programming language with first-class VM and C transpiler

License:        MIT OR Apache-2.0
URL:            https://github.com/rl-lang/rl-lang
Source0:        %{url}/archive/v%{version}/%{name}-%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  rustc
BuildRequires:  pkg-config

%description
rl-lang is a modern programming language featuring a bytecode VM,
optional cranelift JIT, and C99 transpilation. It includes a REPL,
language server, and standard library.

%prep
%autosetup -n %{name}-%{version}

%build
cargo build --release --all-features

%install
install -Dm755 target/release/rl %{buildroot}%{_bindir}/rl
install -Dm755 target/release/rlc %{buildroot}%{_bindir}/rlc
install -Dm755 target/release/rld %{buildroot}%{_bindir}/rld || true
install -Dm644 man/rl.1 %{buildroot}%{_mandir}/man1/rl.1
install -Dm644 man/rl.info %{buildroot}%{_infodir}/rl.info

%files
%license LICENSE-MIT LICENSE-APACHE
%doc README.md CHANGELOG.md
%{_bindir}/rl
%{_bindir}/rlc
%{_bindir}/rld
%{_mandir}/man1/rl.1
%{_infodir}/rl.info

%changelog
* Mon Sep 09 2026 rl-lang maintainers <https://github.com/rl-lang/rl-lang> - 2.3.0-1
- Initial RPM package
