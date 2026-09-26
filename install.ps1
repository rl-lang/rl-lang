#Requires -Version 5.1
param(
    [string]$Version,
    [string]$Prefix,
    [switch]$Force,
    [switch]$Uninstall,
    [switch]$Help
)
$ErrorActionPreference = "Stop"

$Repo = "rl-lang/rl-lang"
$Binary = "rlm"
$InstallDir = if ($Prefix) { $Prefix } elseif ($env:RL_INSTALL_DIR) { $env:RL_INSTALL_DIR } else { "$env:LOCALAPPDATA\rl-lang\bin" }

# --- Bootstrap note ---
# Thin bootstrapper: downloads and SHA256-verifies only `rlm`,
# the rl-lang toolchain manager. Then use `rlm` for everything else:
#   rlm install              # interactive picker (rl, rlc, rlt, rlrepl, rlsp, rldocs)
#   rlm install latest       # latest stable, no picker
#   rlm update               # update rlm itself
#   rlm uninstall            # remove installed binaries
#   irm https://raw.githubusercontent.com/rl-lang/rl-lang/main/install.ps1 | iex

# --- Output helpers ---

function Write-Info { param([string]$Text) Write-Host ("  :: " + $Text) -ForegroundColor DarkGray }
function Write-Ok   { param([string]$Text) Write-Host ("  [ OK ] " + $Text) -ForegroundColor Green }
function Write-Warn { param([string]$Text) Write-Host ("  [WARN] " + $Text) -ForegroundColor Yellow }
function Write-Err  { param([string]$Text) Write-Host ("  [FAIL] " + $Text) -ForegroundColor Red }

# --- Usage ---

function Write-Usage {
    $usage = @"

Usage: install.ps1 [OPTIONS] [VERSION]

Thin bootstrapper: installs only the rlm toolchain manager from
GitHub Releases (SHA256-verified). Afterwards use rlm itself:

  rlm install              # interactive binary picker
  rlm install latest       # latest stable, no picker
  rlm update               # update rlm itself
  rlm uninstall            # remove installed binaries

Arguments:
  VERSION    Version of rlm to install (default: interactive picker)
             Use "latest", "nightly", or a specific version like "v2.0.0"

Options:
  -Help              Show this help message
  -Prefix DIR        Install directory (default: %LOCALAPPDATA%\rl-lang\bin)
  -Force             Overwrite the existing rlm.exe without prompting
  -Uninstall         Remove rlm.exe (use 'rlm uninstall' for the rest)

Environment variables:
  RL_INSTALL_DIR     Same as -Prefix
  RL_VERSION         Same as VERSION argument

Examples:
  .\install.ps1                           # interactive install
  .\install.ps1 latest                    # install latest stable
  .\install.ps1 nightly                   # install nightly build
  .\install.ps1 v2.0.0                    # install specific version
  .\install.ps1 -Prefix C:\rl -Force v2.0.0
  .\install.ps1 -Uninstall               # remove rlm.exe
"@
    Write-Host $usage
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

# --- Install ---

function Install-Rlm {
    param($Arch, $Version, [switch]$ForceInstall)

    $exeName = "$Binary.exe"
    $exePath = Join-Path $InstallDir $exeName
    if ((Test-Path $exePath) -and -not $ForceInstall) {
        Write-Warn "$Binary already exists at $exePath. Use -Force to overwrite."
        return $true
    }

    Write-Info "Installing $Binary $Version (windows-$Arch)..."

    $asset = "$Binary-windows-$Arch.zip"
    $url = "https://github.com/$Repo/releases/download/$Version/$asset"

    $tmpDir = Join-Path ([IO.Path]::GetTempPath()) "rl-install-$(Get-Random)"
    New-Item -ItemType Directory -Path $tmpDir | Out-Null

    try {
        $zipPath = Join-Path $tmpDir $asset
        try {
            Invoke-WebRequest -Uri $url -OutFile $zipPath
        } catch {
            Write-Err "Failed to download $url"
            Write-Err "Check that rlm was published for version '$Version'."
            return $false
        }

        # SHA256 verification is mandatory: abort if the checksum file is
        # missing or the hash does not match.
        $shaUrl = "$url.sha256"
        $shaPath = Join-Path $tmpDir "$asset.sha256"
        try {
            Invoke-WebRequest -Uri $shaUrl -OutFile $shaPath
        } catch {
            Write-Err "Failed to download checksum file $shaUrl"
            Write-Err "Aborting installation: rlm cannot be verified."
            return $false
        }

        $expected = (Get-Content $shaPath -Raw).Trim().Split(" ")[0]
        $hash = (Get-FileHash -Path $zipPath -Algorithm SHA256).Hash
        if ($hash -ne $expected) {
            Write-Err "Checksum mismatch for $asset!"
            Write-Err "  Expected: $expected"
            Write-Err "  Got:      $hash"
            return $false
        }
        Write-Info "SHA256 verified: $asset"

        Expand-Archive -Path $zipPath -DestinationPath $tmpDir -Force

        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
        Copy-Item -Path (Join-Path $tmpDir $exeName) -Destination (Join-Path $InstallDir $exeName) -Force

        Write-Ok "Installed: $InstallDir\$exeName"
        return $true
    } finally {
        Remove-Item -Path $tmpDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

# --- Uninstall ---

function Uninstall-Rlm {
    Write-Host ""
    Write-Host "  Uninstalling rlm from $InstallDir..." -ForegroundColor White
    Write-Host "  (use 'rlm uninstall' to remove the rest of the toolchain)" -ForegroundColor DarkGray
    Write-Host ""

    $path = Join-Path $InstallDir "$Binary.exe"
    if (Test-Path $path) {
        Remove-Item -Path $path -Force
        Write-Ok "Removed: $path"
    } else {
        Write-Host "  No rlm.exe found in $InstallDir." -ForegroundColor DarkGray
    }
}

# --- Main ---

function Main {
    if ($Help) {
        Write-Usage
        return
    }

    if ($Uninstall) {
        Uninstall-Rlm
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
    Write-Host "  rlm bootstrapper"
    Write-Info "repo:    $Repo"
    Write-Info "arch:    $arch"
    Write-Info "version: $resolvedVersion"
    Write-Info "install: $InstallDir"
    Write-Info "----------------------------------------"
    Write-Host ""

    $params = @{
        Arch = $arch
        Version = $resolvedVersion
    }
    if ($Force) { $params.ForceInstall = $true }

    $ok = Install-Rlm @params

    Write-Host ""
    if ($ok) {
        Write-Host "  Summary: rlm installed." -ForegroundColor Green
    } else {
        Write-Host "  Summary: rlm installation failed." -ForegroundColor Yellow
        exit 1
    }

    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($userPath -notlike "*$InstallDir*") {
        if ([string]::IsNullOrWhiteSpace($userPath)) {
            $newPath = $InstallDir
        } else {
            $newPath = "$userPath;$InstallDir"
        }
        [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
        Write-Host "  Added $InstallDir to your user PATH. Restart your terminal for it to take effect."
    }

    Write-Host ""
    Write-Host "  Next: rlm install   # pick the rest of the toolchain (rl, rlc, rlt, rlrepl, rlsp, rldocs)"
}

Main
