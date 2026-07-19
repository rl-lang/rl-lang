# Versioning & Releases

rl-lang follows [SemVer 2.0](https://semver.org/): `MAJOR.MINOR.PATCH[-PRERELEASE]`, tagged as `vX.Y.Z` (e.g. `v0.3.0`, `v0.3.0-rc.1`).

## When to use what

| Segment | Format | When to use it | Example tag |
|---|---|---|---|
| **Major** | `X.0.0` | Breaking changes -- syntax changes, removed/renamed stdlib functions, incompatible `rl.toml` format, anything that breaks existing `.rl` scripts or crates depending on the workspace | `v1.0.0`, `v2.0.0` |
| **Minor** | `x.Y.0` | New features that don't break existing code -- new stdlib module, new CLI subcommand, new language feature that's additive | `v0.3.0`, `v0.4.0` |
| **Patch** | `x.y.Z` | Bug fixes, performance improvements, doc fixes -- no new features, nothing breaks | `v0.2.1`, `v0.2.2` |
| **Alpha** | `x.y.z-alpha` / `-alpha.N` | Earliest testing -- feature incomplete, actively changing, expect breakage; for maintainers/close contributors only | `v0.3.0-alpha`, `v0.3.0-alpha.2` |
| **Beta** | `x.y.z-beta` / `-beta.N` | Feature-complete but unstable -- hunting for bugs; safe for wider early testers | `v0.3.0-beta`, `v0.3.0-beta.2` |
| **RC** | `x.y.z-rc.N` | "This is what we intend to ship" -- no known bugs, waiting to see if anything surfaces | `v0.3.0-rc.1` |
| **Dev / nightly** | `x.y.z-dev` / `-nightly.DATE` | Automated/continuous builds off `main`, not a deliberate release -- most volatile | `v0.3.0-dev`, `v0.3.0-nightly.2026.07.19` |

**Ordering:** a pre-release always sorts before the version it leads to: `0.3.0-alpha < 0.3.0-alpha.1 < 0.3.0-beta < 0.3.0-rc.1 < 0.3.0`. `v0.3.0-rc.1` means "release candidate for the upcoming 0.3.0," not "something after 0.3.0."

## Cutting a release

```bash
git tag v0.3.0-alpha.1   # start testing a new feature
git push origin v0.3.0-alpha.1

# ...iterate...
git tag v0.3.0-beta.1
git push origin v0.3.0-beta.1

# ...stabilizes...
git tag v0.3.0-rc.1
git push origin v0.3.0-rc.1

# ...no issues found...
git tag v0.3.0
git push origin v0.3.0
```

## What happens automatically

Pushing a `v*` tag runs [`.github/workflows/release.yml`](.github/workflows/release.yml):

1. **`release`** -- builds the `rl` binary (default + `debug` feature) for Linux and Windows, uploads them as artifacts.
2. **`publish`** -- creates the GitHub Release from those artifacts and auto-generates release notes. `prerelease: auto` reads the tag's SemVer pre-release part and ticks GitHub's "pre-release" flag automatically for `-alpha`/`-beta`/`-rc`/etc. tags.
3. **`announce`** -- classifies the release (major/minor/patch bump, or alpha/beta/rc/dev pre-release) by diffing the new tag against the previous one, then posts an embed to the `#announcement` Discord channel via the `DISCORD_ANNOUNCEMENT_WEBHOOK` repo secret.

You don't need to do anything beyond pushing the tag -- classification, the GitHub Release, and the Discord announcement are all handled by the workflow.
