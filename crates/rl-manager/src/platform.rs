#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arch {
    X86_64,
    Aarch64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Linux,
    Macos,
    Windows,
    Android,
}

impl Arch {
    pub fn detect() -> Self {
        match std::env::consts::ARCH {
            "x86_64" | "amd64" => Arch::X86_64,
            "aarch64" | "arm64" => Arch::Aarch64,
            other => panic!("unsupported arch: {}", other),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Arch::X86_64 => "x86_64",
            Arch::Aarch64 => "aarch64",
        }
    }
}

impl Platform {
    pub fn detect() -> Self {
        if cfg!(target_os = "linux") {
            if is_termux() {
                Platform::Android
            } else {
                Platform::Linux
            }
        } else if cfg!(target_os = "macos") {
            Platform::Macos
        } else if cfg!(target_os = "windows") {
            Platform::Windows
        } else {
            panic!("unsupported platform")
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Platform::Linux => "linux",
            Platform::Macos => "macos",
            Platform::Windows => "windows",
            Platform::Android => "android",
        }
    }

    pub fn archive_ext(self) -> &'static str {
        match self {
            Platform::Windows => "zip",
            _ => "tar.gz",
        }
    }
}

fn is_termux() -> bool {
    std::env::var("TERMUX_VERSION").is_ok()
        || std::env::var("PREFIX").is_ok()
        || std::path::Path::new("/data/data/com.termux").exists()
}

pub fn default_install_dir() -> std::path::PathBuf {
    if cfg!(target_os = "windows") {
        dirs().join("rl-lang").join("bin")
    } else {
        dirs().join("bin")
    }
}

fn dirs() -> std::path::PathBuf {
    if cfg!(target_os = "windows") {
        std::env::var("LOCALAPPDATA")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
    } else {
        std::env::var("HOME")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .join(".local")
    }
}
