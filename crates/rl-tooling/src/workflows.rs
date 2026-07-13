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

      - name: Run RL check
        uses: rl-lang/rl-check@main
        with:
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
      - uses: rl-lang/rl-package@main
        with:
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

/// Generates GitHub Actions workflow files based on the provided flags.
///
/// - check   -> workflows/check.yml
/// - package -> workflows/release.yml
///
/// Exits with code 1 if a target file already exists or any IO error occurs.
pub fn generate(check: bool, package: bool) {
    let dir = Path::new("workflows");
    if let Err(e) = fs::create_dir_all(dir) {
        eprintln!("error: could not create workflows: {}", e);
        std::process::exit(1);
    }

    if check {
        write_file(dir.join("check.yml").as_path(), CHECK_YML);
    }
    if package {
        write_file(dir.join("release.yml").as_path(), PACKAGE_YML);
    }
}

fn write_file(path: &Path, content: &str) {
    if path.exists() {
        eprintln!("error: '{}' already exists", path.display());
        std::process::exit(1);
    }
    if let Err(e) = fs::write(path, content) {
        eprintln!("error: failed to write '{}': {}", path.display(), e);
        std::process::exit(1);
    }
    println!("created '{}'", path.display());
}
