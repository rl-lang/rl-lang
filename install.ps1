#Requires -Version 5.1
param(
    [string]$Version,
    [string]$Prefix,
    [string]$Binaries,
    [switch]$Force,
    [switch]$Uninstall,
    [switch]$Help
)
$ErrorActionPreference = "Stop"

$Repo = "rl-lang/rl-lang"
$InstallDir = if ($Prefix) { $Prefix } elseif ($env:RL_INSTALL_DIR) { $env:RL_INSTALL_DIR } else { "$env:LOCALAPPDATA\rl-lang\bin" }

# --- Bootstrap note ---
# This script is a one-time bootstrapper. Once installed, use `rlm` to manage
# your rl-lang toolchain (install, update, uninstall).
#   irm https://raw.githubusercontent.com/rl-lang/rl-lang/main/install.ps1 | iex
# Or install rlm directly from GitHub Releases and use:
#   rlm install

# --- Binary definitions ---

$Binaries_ = @("rl", "rlc", "rlt", "rlrepl", "rlsp", "rldocs", "rlm")

# --- Output helpers ---

function Write-Info { param([string]$Text) Write-Host ("  :: " + $Text) -ForegroundColor DarkGray }
function Write-Ok   { param([string]$Text) Write-Host ("  [ OK ] " + $Text) -ForegroundColor Green }
function Write-Warn { param([string]$Text) Write-Host ("  [WARN] " + $Text) -ForegroundColor Yellow }
function Write-Err  { param([string]$Text) Write-Host ("  [FAIL] " + $Text) -ForegroundColor Red }

# --- Usage ---

function Write-Usage {
    $usage = @"

Usage: install.ps1 [OPTIONS] [VERSION]

Install prebuilt rl-lang binaries from GitHub Releases.

Arguments:
  VERSION    Version to install (default: interactive picker)
             Use "latest", "nightly", or a specific version like "v2.0.0"

Options:
  -Help              Show this help message
  -Prefix DIR        Install directory (default: %LOCALAPPDATA%\rl-lang\bin)
  -Force             Overwrite existing binaries without prompting
  -Binaries BINS     Comma-separated list of binaries to install
                     Use "all" to install all binaries
  -Uninstall         Remove installed binaries

Environment variables:
  RL_INSTALL_DIR     Same as -Prefix
  RL_VERSION         Same as VERSION argument
  RL_BINARIES        Same as -Binaries

Examples:
  .\install.ps1                           # interactive install
  .\install.ps1 latest                    # install latest stable
  .\install.ps1 nightly                   # install nightly build
  .\install.ps1 v2.0.0                    # install specific version
  .\install.ps1 -Binaries rl,rlc,rlm      # install specific binaries
  .\install.ps1 -Binaries all latest      # install all binaries
  .\install.ps1 -Prefix C:\rl -Force v2.0.0
  .\install.ps1 -Uninstall               # remove all installed binaries
"@
    Write-Host $usage
}

# --- Binary selection ---

function Print-Menu {
    Write-Host "  Select binaries to install:"
    Write-Host ""
    Write-Host "    1) rl        - core (run, check, new, dev, format, pm)" -ForegroundColor Cyan
    Write-Host "    2) rlc       - compiler (VM backend)" -ForegroundColor Cyan
    Write-Host "    3) rlt       - transpiler (to C99)" -ForegroundColor Cyan
    Write-Host "    4) rlrepl    - interactive TUI REPL" -ForegroundColor Cyan
    Write-Host "    5) rlsp      - LSP server" -ForegroundColor Cyan
    Write-Host "    6) rldocs    - documentation viewer" -ForegroundColor Cyan
    Write-Host "    7) rlm       - toolchain manager" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "  Enter number(s), comma-separated (e.g. 1,3,7), or 'all'." -ForegroundColor DarkGray
}

function Select-Binaries {
    if ($Binaries) {
        if ($Binaries.Trim().ToLower() -eq "all") { return $Binaries_ }
        return $Binaries -split "," | ForEach-Object { $_.Trim() }
    }

    if ($env:RL_BINARIES) {
        if ($env:RL_BINARIES.Trim().ToLower() -eq "all") { return $Binaries_ }
        return $env:RL_BINARIES -split "," | ForEach-Object { $_.Trim() }
    }

    if (-not [Environment]::UserInteractive) {
        Write-Err "No interactive terminal detected and RL_BINARIES is not set."
        Write-Err "Non-interactive use requires: `$env:RL_BINARIES = 'rl,rlc,rlm'; .\install.ps1 [version]"
        exit 1
    }

    Print-Menu
    $choices = Read-Host "  Enter number(s), comma-separated (e.g. 1,3,7), or 'all'"

    if ($choices.Trim().ToLower() -eq "all") {
        return $Binaries_
    }

    $selected = @()
    $labels = @("rl", "rlc", "rlt", "rlrepl", "rlsp", "rldocs", "rlm")
    foreach ($part in ($choices -split ",")) {
        $trimmed = $part.Trim()
        if (-not $trimmed) { continue }

        $index = 0
        if (-not [int]::TryParse($trimmed, [ref]$index) -or $index -lt 1 -or $index -gt $labels.Count) {
            Write-Err "Invalid selection: $trimmed"
            exit 1
        }

        $selected += $labels[$index - 1]
    }

    return $selected
}

# --- Platform detection ---

function Get-Arch {
    $arch = $env:PROCESSOR_ARCHITECTURE
    switch ($arch) {
        "AMD64" { return "x86_64" }
        "ARM64" { return "aarch64" }
        default {
            Write-Err "Unsupported arch: $arch"
            exit 1
        }
    }
}

# --- Version resolution ---

function Test-Release {
    param([string]$Tag)
    try {
        $null = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/tags/$Tag"
        return $true
    } catch {
        return $false
    }
}

function Get-Version {
    param([string]$Requested)

    switch -Regex ($Requested) {
        "^latest$" {
            try {
                $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
                return $release.tag_name
            } catch {
                Write-Err "Could not resolve the latest release from GitHub."
                exit 1
            }
        }
        "^nightly$" {
            if (-not (Test-Release "nightly")) {
                Write-Err "Release 'nightly' not found on GitHub."
                exit 1
            }
            return "nightly"
        }
        "^v[0-9]" {
            if (-not (Test-Release $Requested)) {
                Write-Err "Release '$Requested' not found on GitHub."
                Write-Err "Check the tag (e.g. 'v1.0.0') or use 'latest'/'nightly'."
                exit 1
            }
            return $Requested
        }
        "^[0-9]" {
            $normalized = "v$Requested"
            if (-not (Test-Release $normalized)) {
                Write-Err "Release '$normalized' not found on GitHub."
                Write-Err "Check the tag (e.g. 'v1.0.0') or use 'latest'/'nightly'."
                exit 1
            }
            return $normalized
        }
        default {
            Write-Err "Unknown version '$Requested'."
            Write-Err "Use 'latest', 'nightly', or a specific version like 'v1.0.0'."
            exit 1
        }
    }
}

function Select-VersionPicker {
    Write-Host ""
    Write-Host "  Select a version to install:"
    Write-Host ""
    Write-Host "    1) latest   - newest stable release" -ForegroundColor Cyan
    Write-Host "    2) nightly  - latest build from the dev branch" -ForegroundColor Cyan
    Write-Host "    3) custom   - pin a specific version (e.g. v2.0.0)" -ForegroundColor Cyan
    Write-Host ""
    $choice = Read-Host "  Choose [1]"
    if ([string]::IsNullOrWhiteSpace($choice)) { $choice = "1" }

    switch ($choice.Trim()) {
        "2" { return "nightly" }
        "3" {
            $custom = Read-Host "  Enter version (e.g. v2.0.0)"
            if ([string]::IsNullOrWhiteSpace($custom)) { return "latest" }
            return $custom.Trim()
        }
        default { return "latest" }
    }
}

# --- Checksum verification ---

function Test-Checksum {
    param([string]$FilePath)

    $shaPath = "$FilePath.sha256"
    if (-not (Test-Path $shaPath)) {
        Write-Warn "No checksum file found for $(Split-Path $FilePath -Leaf). Skipping verification."
        return $true
    }

    $expected = (Get-Content $shaPath -Raw).Trim().Split(" ")[0]
    $hash = (Get-FileHash -Path $FilePath -Algorithm SHA256).Hash.ToLower()

    if ($hash -eq $expected) {
        return $true
    } else {
        Write-Err "Checksum mismatch for $(Split-Path $FilePath -Leaf)!"
        Write-Err "  Expected: $expected"
        Write-Err "  Got:      $hash"
        return $false
    }
}

# --- Install ---

function Install-One {
    param($Binary, $Arch, $Version, [switch]$ForceInstall)

    $exePath = Join-Path $InstallDir "$Binary.exe"
    if ((Test-Path $exePath) -and -not $ForceInstall) {
        Write-Warn "$Binary already exists at $exePath. Use -Force to overwrite."
        return $true
    }

    Write-Info "Installing $Binary $Version (windows-$Arch)..."

    $asset = "$Binary-windows-$Arch.zip"
    $url = "https://github.com/$Repo/releases/download/$Version/$asset"

    $tmpDir = Join-Path $env:TEMP "rl-install-$(Get-Random)"
    New-Item -ItemType Directory -Path $tmpDir | Out-Null

    try {
        $zipPath = Join-Path $tmpDir $asset
        try {
            Invoke-WebRequest -Uri $url -OutFile $zipPath
        } catch {
            Write-Err "Failed to download $url"
            Write-Err "Check that this binary/version combination was published."
            return $false
        }

        # Download and verify checksum
        $shaUrl = "$url.sha256"
        $shaPath = Join-Path $tmpDir "$asset.sha256"
        try {
            Invoke-WebRequest -Uri $shaUrl -OutFile $shaPath -ErrorAction SilentlyContinue
        } catch {
            # Checksum file may not exist for older releases
        }

        if (Test-Path $shaPath) {
            $expected = (Get-Content $shaPath -Raw).Trim().Split(" ")[0]
            $hash = (Get-FileHash -Path $zipPath -Algorithm SHA256).Hash.ToLower()
            if ($hash -ne $expected) {
                Write-Err "Checksum mismatch for $asset!"
                Write-Err "  Expected: $expected"
                Write-Err "  Got:      $hash"
                return $false
            }
        } else {
            Write-Warn "No checksum file found. Skipping verification."
        }

        Expand-Archive -Path $zipPath -DestinationPath $tmpDir -Force

        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
        $exeName = "$Binary.exe"
        Copy-Item -Path (Join-Path $tmpDir $exeName) -Destination (Join-Path $InstallDir $exeName) -Force

        Write-Ok "Installed: $InstallDir\$exeName"
        return $true
    } finally {
        Remove-Item -Path $tmpDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

# --- Uninstall ---

function Uninstall-All {
    $removed = 0
    Write-Host ""
    Write-Host "  Uninstalling rl-lang binaries from $InstallDir..." -ForegroundColor White
    Write-Host ""

    foreach ($name in $Binaries_) {
        $path = Join-Path $InstallDir "$name.exe"
        if (Test-Path $path) {
            Remove-Item -Path $path -Force
            Write-Ok "Removed: $path"
            $removed++
        }
    }

    Write-Host ""
    if ($removed -gt 0) {
        Write-Host "  Removed $removed binary(ies)." -ForegroundColor Green
    } else {
        Write-Host "  No rl-lang binaries found in $InstallDir." -ForegroundColor DarkGray
    }
}

# --- Main ---

function Main {
    if ($Help) {
        Write-Usage
        return
    }

    if ($Uninstall) {
        Uninstall-All
        return
    }

    $arch = Get-Arch

    $requested = $null
    if (-not [string]::IsNullOrWhiteSpace($Version)) {
        $requested = $Version.Trim()
    } elseif ($env:RL_VERSION) {
        $requested = $env:RL_VERSION.Trim()
    } elseif ([Environment]::UserInteractive) {
        $requested = Select-VersionPicker
    } else {
        $requested = "latest"
    }

    $resolvedVersion = Get-Version $requested

    Write-Host ""
    Write-Host "  rl-lang installer"
    Write-Info "repo:    $Repo"
    Write-Info "arch:    $arch"
    Write-Info "version: $resolvedVersion"
    Write-Info "install: $InstallDir"
    Write-Info "----------------------------------------"
    Write-Host ""

    $selected = Select-Binaries
    $installed = 0
    $total = 0

    foreach ($bin in $selected) {
        $total++
        $params = @{
            Binary = $bin
            Arch = $arch
            Version = $resolvedVersion
        }
        if ($Force) { $params.ForceInstall = $true }

        if (Install-One @params) {
            $installed++
        }
    }

    Write-Host ""
    $failed = $total - $installed
    if ($failed -gt 0) {
        Write-Host ("  Summary: {0}/{1} installed, some failed." -f $installed, $total) -ForegroundColor Yellow
    } else {
        Write-Host ("  Summary: {0}/{1} installed." -f $installed, $total) -ForegroundColor Green
    }

    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($userPath -notlike "*$InstallDir*") {
        [Environment]::SetEnvironmentVariable("Path", "$userPath;$InstallDir", "User")
        Write-Host "  Added $InstallDir to your user PATH. Restart your terminal for it to take effect."
    }

    if ($failed -gt 0) {
        exit 1
    }
}

Main
