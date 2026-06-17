# Contributing to rl-lang

Thanks for your interest in contributing!

## Getting started

```bash
git clone https://github.com/MohamedGonem/rl-lang
cd rl-lang
cargo build
```

## Before submitting a PR

```bash
cargo test --all-features   # make sure all tests pass
cargo clippy -- -D warnings # no lint warnings
```

- PRs must be up to date with the `dev` branch - always branch off `dev` and rebase before submitting

## What to work on

Check the [issues](https://github.com/MohamedGonem/rl-lang/issues) page for open bugs and feature requests, or the [roadmap](https://github.com/MohamedGonem/rl-lang/wiki/Roadmap) for planned work.

## Guidelines

- Keep PRs focused — one fix or feature per PR
- Add tests for new behavior where possible
- Follow the existing code style
- Update docs if you change language behavior or add stdlib functions

## Adding a stdlib function

1. Add the implementation under `src/interpreter/stdlib/<module>/`
2. Register it in the module's `mod.rs`
3. Add a doc entry under `src/docs/entries/stdlib/<module>.rs`

## Questions

Open an issue or reach out via GitHub.
