# GOAT Packaging Audit

## Overview
This audit evaluates the current GOAT repository to determine what needs to be included in a public alpha release artifact, what should be excluded, and how the installation process should behave.

## Included Artifacts (The Release Archive)
A successful build should produce a `tar.gz` archive containing:
1. `goat` (Compiled Rust release binary)
2. `apps/dashboard/out/` (Static exported Next.js dashboard - must be bundled or served)
3. `goat.toml` (Default configuration template)
4. `install.sh` (Helper script for users)
5. `uninstall.sh` (Helper script for users)
6. `README.md` and `LICENSE`

## Excluded Paths (Do Not Package)
- `src/`, `Cargo.toml`, `Cargo.lock`, `target/`
- `apps/dashboard/src/`, `apps/dashboard/node_modules/`, `apps/dashboard/.next/` (only the `out` dir is needed if serving statically, though GOAT might need to wrap or serve it)
- `.git/`, `.github/`, `.env`
- Any local `~/.local/share/goat` data or SQLite databases.

## Installation Strategy (`install.sh`)
- **Binary**: Move `goat` to `~/.local/bin/goat`.
- **Config**: Copy `goat.toml` to `~/.config/goat/goat.toml` (only if it doesn't already exist).
- **Dashboard**: The static dashboard assets should be copied to `~/.local/share/goat/dashboard/` so the GOAT daemon can serve them statically on port 3000.
- **Path**: Ensure `~/.local/bin` is in the user's `$PATH`.

## Uninstallation Strategy (`uninstall.sh`)
- **Binary**: Remove `~/.local/bin/goat`.
- **Config**: Remove `~/.config/goat/` (prompt user first).
- **Data**: Remove `~/.local/share/goat/` (prompt user first, as this deletes the Brain DB and projects).

## Build Script (`build-release.sh`)
1. Ensure Rust and Node.js/npm are installed.
2. Run `cargo build --release`.
3. Run `npm install` and `npm run build` in `apps/dashboard`.
4. Collect the artifacts into a staging directory `dist/`.

## Packaging Script (`package-alpha.sh`)
1. Call `build-release.sh`.
2. Generate SHA-256 checksums.
3. Compress `dist/` into `goat-<version>-<target>.tar.gz`.

## GitHub Actions (`release-alpha.yml`)
- Trigger on `v*` tag.
- Matrix build for Linux (and macOS/Windows eventually).
- Upload the `tar.gz` and checksums to the GitHub Release draft.
