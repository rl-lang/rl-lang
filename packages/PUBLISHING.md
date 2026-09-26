# Publishing Packages

Step-by-step instructions for making rl-lang available through each package manager.

## Debian / Ubuntu (PPA)

1. Install tools: `sudo apt install build-essential devscripts debhelper`
2. Copy `packages/debian/*` into `debian/` at the repo root.
3. Update `debian/changelog` with the new version.
4. Build: `dpkg-buildpackage -us -uc`
5. Upload to your PPA: `dput ppa:your/ppa ../*.changes`
6. Users install: `sudo apt install rl-lang`

**Or** submit to the official Debian NEW queue if you have upload rights.

## Fedora (COPR)

1. Install tools: `sudo dnf install rpm-build`
2. Update `packages/fedora/rl-lang.spec` with the new version and source URL.
3. Build: `rpmbuild -ba packages/fedora/rl-lang.spec`
4. Create a COPR repo at `copr.fedorainfracloud.org` and upload the SRPM.
5. Users enable the repo and install: `sudo dnf install rl-lang`

## Arch Linux (AUR)

1. Copy `packages/arch/PKGBUILD` to a working directory.
2. Update `pkgver` and reset `sha256sums` to `SKIP` or compute real hashes.
3. Run `makepkg -si` to test locally.
4. Push to the AUR:
   ```
   git clone ssh://aur@aur.archlinux.org/rl-lang.git
   cp PKGBUILD rl-lang/
   cd rl-lang
   makepkg -si  # test
   git add -A && git commit -m "v2.3.0"
   git push
   ```
5. Users install: `yay -S rl-lang` or `paru -S rl-lang`

## Gentoo

1. Set up an overlay (e.g. with `layman` or `eselect repository`).
2. Copy `packages/gentoo/rl-lang-9999.ebuild` into your overlay's `dev-lang/rl-lang/`.
3. Manifest: `cd /var/db/repos/your-overlay/dev-lang/rl-lang && ebuild rl-lang-9999.ebuild manifest`
4. Test: `emerge --pretend =rl-lang-9999`
5. Push your overlay to GitHub.

For a versioned release ebuild, copy the 9999 ebuild to `rl-lang-2.3.0.ebuild`, remove the git source, and use a tarball URL instead.

## Nix

**For nixpkgs:**
1. Fork `NixOS/nixpkgs`.
2. Add `pkgs/by-name/rl/rl-lang/package.nix` using `packages/nix/rl-lang.nix` as base.
3. Update `hash` and `cargoHash` with actual values.
4. Run `nix-build -A rl-lang` to test.
5. Submit a PR to nixpkgs.

**For a standalone flake:**
1. Copy `packages/nix/rl-lang.nix` to your repo root as `default.nix` or `flake.nix`.
2. Users install: `nix run github:rl-lang/rl-lang`

## macOS (Homebrew)

1. Create a tap repo: `brew tap-new youruser/rl-lang`.
2. Copy `packages/macos/rl-lang.rb` into the tap formula directory.
3. Update the `url` and `sha256`.
4. Test: `brew install --build-from-source rl-lang.rb`
5. Push the tap repo. Users install:
   ```
   brew tap youruser/rl-lang
   brew install rl-lang
   ```

**Or** submit to homebrew-core if the project meets notability requirements.

## Windows (WinGet)

1. Clone `microsoft/winget-pkgs`.
2. Update the three YAML files in `packages/winget/` with the correct version and SHA256 hashes.
3. Place them in the correct directory structure:
   ```
   manifests/r/rl-lang/rl/2.3.0/
     rl-lang.rl.yaml              (version)
     rl-lang.rl.locale.en-US.yaml (locale)
     rl-lang.rl.installer.yaml    (installer)
   ```
4. Submit a PR to `microsoft/winget-pkgs`.
5. Users install: `winget install rl-lang.rl`

## Snap

1. Install snapcraft: `sudo snap install snapcraft --classic`
2. Copy `packages/snap/snapcraft.yaml` to the repo root as `snapcraft.yaml`.
3. Update `source-tag` and `version`.
4. Build and test: `snapcraft` then `sudo snap install rl_*.snap --classic`
5. Publish: `snapcraft login` then `snapcraft publish rl_*.snap`
6. Users install: `sudo snap install rl --classic`

## Flatpak

1. Install tools: `sudo flatpak install flathub org.freedesktop.Sdk.Extension.rust-stable//24.08`
2. Generate cargo sources:
   ```
   wget https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py
   python3 flatpak-cargo-generator.py Cargo.lock -o cargo-sources.json
   ```
3. Copy `packages/flatpak/io.github.rl-lang.rl.yml` to the repo root.
4. Update `sha256` for the source archive.
5. Build: `flatpak-builder --force-clean build-dir io.github.rl-lang.rl.yml`
6. Test: `flatpak-builder --run build-dir io.github.rl-lang.rl.yml rl`
7. Bundle: `flatpak-builder --repo=repo --force-clean build-dir io.github.rl-lang.rl.yml`
8. Submit to Flathub: fork `flathub/io.github.rl-lang.rl`, add the manifest, submit a PR.
9. Users install: `flatpak install flathub io.github.rl-lang.rl`

---

## Version bumps

When releasing a new version, update these files in `packages/`:

- `debian/changelog`
- `fedora/rl-lang.spec` (Version, Source0)
- `arch/PKGBUILD` (pkgver)
- `gentoo/rl-lang-*.ebuild` (copy 9999 to new versioned ebuild)
- `nix/rl-lang.nix` (version, hash, cargoHash)
- `macos/rl-lang.rb` (url, sha256)
- `winget/*.yaml` (PackageVersion, InstallerSha256)
- `snap/snapcraft.yaml` (version, source-tag)
- `flatpak/io.github.rl-lang.rl.yml` (version in URL, sha256)
