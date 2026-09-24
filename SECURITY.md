# Security Policy

## Supported Versions

| Version | Supported |
| ------- | --------- |
| 2.x     | Yes       |
| < 2.0   | No        |

Only the current major line receives security fixes. Nightly builds are
unsupported snapshots: if you hit something on nightly, check latest stable
first.

## Scope

In scope:

- The `rl` CLI and the bytecode VM (`rlc`)
- The C transpiler (`rlt`) and the C runtime in `crates/rl-cc/runtime`
  (memory safety matters here: it is handwritten C99)
- The standard library (`std::*`) and its native Rust implementations
- `rlrepl`, `rlsp`, `rldocs`, `rlm`, `rl pm`

Out of scope:

- Third-party crates and tools (report those upstream)
- The VS Code extensions and the tree-sitter grammar (separate repos)
- `rl-lang.github.io` site content (open a docs issue instead)

## Reporting a Vulnerability

Do not open a public issue for anything sensitive. Use
[private vulnerability reporting](https://github.com/rl-lang/rl-lang/security/advisories/new)
so details stay hidden until a fix ships.

Include:

1. What you ran (binary, version, OS) and the exact input or program
2. What you expected vs. what happened (crash output, ASan trace, PoC)
3. Whether it also happens on the latest stable release

Reports are acknowledged as soon as possible. Valid issues get a fix and a
CHANGELOG entry, with credit in the release notes unless you ask to stay
anonymous.
