use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static TOOLCHAIN_MANAGER: ConceptEntry = ConceptEntry {
    name: "rlm (toolchain manager)",
    summary: "installing, updating, and managing rl-lang binaries with `rlm` - the standalone toolchain manager that replaces install scripts",
    category: ConceptCategory::Tooling,
    prerequisites: &["tooling"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("overview"),
            description: "`rlm` is the rl-lang toolchain manager. It downloads and installs prebuilt binaries (rl, rlc, rlt, rlrepl, rlsp, rldocs, rlm) from GitHub Releases. Use it to install, update, or uninstall the rl-lang toolchain.",
            examples: &[],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("rlm install"),
            description: "`rlm install` starts an interactive TUI that lets you pick a version and which binaries to install. Use `--no-tui` for non-interactive mode, `--variant` to pick binaries directly, and `--prefix` to set the install directory.",
            examples: &[
                "rlm install",
                "rlm install --no-tui",
                "rlm install --no-tui --variant rl,rlc,rlm",
                "rlm install --prefix /usr/local/bin",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("rlm update"),
            description: "`rlm update` self-updates rlm to the latest version from GitHub Releases.",
            examples: &["rlm update"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("rlm list"),
            description: "`rlm list` shows all installed rl-lang binaries and their versions.",
            examples: &["rlm list"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("rlm uninstall"),
            description: "`rlm uninstall` removes all installed rl-lang binaries from the install directory.",
            examples: &["rlm uninstall"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Note,
            title: Some("install scripts vs rlm"),
            description: "the install.sh and install.ps1 scripts are bootstrappers for first-time installation. after that, use `rlm` to manage your toolchain - it can self-update and provides a better experience with the TUI picker.",
            examples: &[],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("platform support"),
            description: "rlm supports Linux, macOS, Windows, and Android (aarch64). cross-compilation targets (e.g. aarch64 on x86_64) are downloaded as prebuilt binaries, not compiled locally.",
            examples: &[],
            expected_output: &[],
        },
    ],
    pitfalls: &[
        "use `rlm` after first installation - install scripts are bootstrappers only",
        "rlm supports Linux, macOS, Windows, and Android (aarch64)",
        "`rlm install --no-tui` is required for CI/CD or non-interactive environments",
    ],
    related: &["tooling", "package manager"],
    related_stdlib: &[],
    since: Some("v2.2.0"),
};
