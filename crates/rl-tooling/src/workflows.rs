use std::fs;
use std::path::Path;

const CHECK_YML: &str = r#"name: RL Check

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
  workflow_dispatch:

jobs:
  rl-check:
    name: Check ${{ matrix.file }}
    runs-on: ubuntu-latest
    strategy:
      fail-fast: false
      matrix:
        file:
          - src/main.rl
    steps:
      - uses: actions/checkout@v4

      - name: Install dependencies
        run: rl pm install

      - name: Run RL check
        uses: rl-lang/rl-check@main
        with:
          version: {rl_version}
          file: ${{ matrix.file }}
"#;

const PACKAGE_YML: &str = r#"name: Release

on:
  workflow_dispatch:

jobs:
  package:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest]
        file:
          - { path: src/main.rl, name: program }
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4

      - name: Install dependencies
        run: rl pm install

      - name: Package
        uses: rl-lang/rl-package@main
        with:
          version: {rl_version}
          file: ${{ matrix.file.path }}
          output: ${{ matrix.file.name }}

  release:
    needs: package
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
      - name: Download all artifacts
        uses: actions/download-artifact@v4
        with:
          path: artifacts

      - name: Create GitHub Release
        uses: softprops/action-gh-release@v2
        with:
          tag_name: release-${{ github.run_number }}
          name: Release ${{ github.run_number }}
          files: artifacts/**/*
"#;

const TEST_YML: &str = r#"name: RL Test

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
  workflow_dispatch:

jobs:
  rl-test:
    name: Test ${{ matrix.file }}
    runs-on: ubuntu-latest
    strategy:
      fail-fast: false
      matrix:
        file:
          - tests.rl
    steps:
      - uses: actions/checkout@v4

      - name: Run RL tests
        uses: rl-lang/rl-test@main
        with:
          version: {rl_version}
          file: ${{ matrix.file }}
"#;

const TRANSPILE_YML: &str = r#"name: RL Transpile

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
  workflow_dispatch:

jobs:
  rl-transpile:
    name: Transpile ${{ matrix.file }}
    runs-on: ubuntu-latest
    strategy:
      fail-fast: false
      matrix:
        file:
          - src/main.rl
    steps:
      - uses: actions/checkout@v4

      - name: Transpile to C
        uses: rl-lang/rl-transpile@main
        with:
          version: {rl_version}
          file: ${{ matrix.file }}
          compile: true
"#;

const FORMAT_YML: &str = r#"name: RL Format

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
  workflow_dispatch:

jobs:
  rl-format:
    name: Format check
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Check formatting
        uses: rl-lang/rl-format@main
        with:
          version: {rl_version}
          folder: src
"#;

/// Generates GitHub Actions workflow files based on the provided flags.
///
/// - check     -> workflows/check.yml
/// - package   -> workflows/release.yml
/// - test      -> workflows/test.yml
/// - transpile -> workflows/transpile.yml
/// - format    -> workflows/format.yml
///
/// `rl_version` is threaded into every generated `version:` input
/// (`latest` when omitted). Generated workflows install prebuilt RL
/// binaries through the `rl-lang/rl-*` actions - no Rust toolchain.
///
/// Exits with code 1 if a target file already exists or any IO error occurs.
pub fn generate(
    check: bool,
    package: bool,
    test: bool,
    transpile: bool,
    format: bool,
    rl_version: Option<String>,
) {
    let dir = Path::new("workflows");
    try_generate(dir, check, package, test, transpile, format, rl_version).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1)
    });
}

/// Inner fallible implementation of [`generate`].
///
/// Returns an error if writing the directory or file fails.
pub fn try_generate(
    dir: &Path,
    check: bool,
    package: bool,
    test: bool,
    transpile: bool,
    format: bool,
    rl_version: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(dir).map_err(|e| format!("error: could not create workflows: {}", e))?;
    let version = rl_version.unwrap_or_else(|| "latest".to_string());

    if check {
        write_file(
            dir.join("check.yml").as_path(),
            &CHECK_YML.replace("{rl_version}", &version),
        )?;
    }

    if package {
        write_file(
            dir.join("release.yml").as_path(),
            &PACKAGE_YML.replace("{rl_version}", &version),
        )?;
    }

    if test {
        write_file(
            dir.join("test.yml").as_path(),
            &TEST_YML.replace("{rl_version}", &version),
        )?;
    }

    if transpile {
        write_file(
            dir.join("transpile.yml").as_path(),
            &TRANSPILE_YML.replace("{rl_version}", &version),
        )?;
    }

    if format {
        write_file(
            dir.join("format.yml").as_path(),
            &FORMAT_YML.replace("{rl_version}", &version),
        )?;
    }

    Ok(())
}

fn write_file(path: &Path, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    if path.exists() {
        return Err(format!("error: '{}' already exists", path.display()).into());
    }

    fs::write(path, content)
        .map_err(|e| format!("error: failed to write '{}': {}", path.display(), e))?;

    println!("created '{}'", path.display());
    Ok(())
}
