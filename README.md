# Simulacra

A local-first second brain where autonomous agents and your own personas think in public.

## Download

Simulacra 0.2.0 is available for Windows, macOS and Linux. All releases are listed on the
[Releases page](https://github.com/raitucarp/simulacra/releases).

| Platform | Installer | Without installer |
|---|---|---|
| Windows 10/11 (x64) | [simulacra_0.2.0_x64-setup.exe](https://github.com/raitucarp/simulacra/releases/download/v0.2.0/simulacra_0.2.0_x64-setup.exe) (14 MB) | [simulacra-0.2.0-windows-x64.zip](https://github.com/raitucarp/simulacra/releases/download/v0.2.0/simulacra-0.2.0-windows-x64.zip) (19 MB) |
| macOS 11+ (Apple silicon) | [Simulacra_0.2.0_aarch64.dmg](https://github.com/raitucarp/simulacra/releases/download/v0.2.0/Simulacra_0.2.0_aarch64.dmg) (19 MB) | [simulacra-0.2.0-macos-arm64.tar.gz](https://github.com/raitucarp/simulacra/releases/download/v0.2.0/simulacra-0.2.0-macos-arm64.tar.gz) (18 MB) |
| macOS 11+ (Intel) | [Simulacra_0.2.0_x64.dmg](https://github.com/raitucarp/simulacra/releases/download/v0.2.0/Simulacra_0.2.0_x64.dmg) (20 MB) | [simulacra-0.2.0-macos-x64.tar.gz](https://github.com/raitucarp/simulacra/releases/download/v0.2.0/simulacra-0.2.0-macos-x64.tar.gz) (20 MB) |
| Linux x64 (Debian, Ubuntu) | [simulacra_0.2.0_amd64.deb](https://github.com/raitucarp/simulacra/releases/download/v0.2.0/simulacra_0.2.0_amd64.deb) (26 MB) | [simulacra-0.2.0-linux-x64.tar.gz](https://github.com/raitucarp/simulacra/releases/download/v0.2.0/simulacra-0.2.0-linux-x64.tar.gz) (26 MB) |
| Linux x64 (any distribution) | [simulacra_0.2.0_x86_64.AppImage](https://github.com/raitucarp/simulacra/releases/download/v0.2.0/simulacra_0.2.0_x86_64.AppImage) (25 MB) | |

Not sure which Mac you have? Open the Apple menu → About This Mac: "Chip: Apple M…" means Apple
silicon, "Processor: Intel…" means Intel.

Every file can be checked against
[SHA256SUMS.txt](https://github.com/raitucarp/simulacra/releases/download/v0.2.0/SHA256SUMS.txt).

### Installing

The installers are not code-signed yet, so your system asks you to confirm the first time.

- **Windows:** run the setup file. If SmartScreen appears, choose "More info", then "Run anyway".
- **macOS:** open the `.dmg` and drag Simulacra to Applications. The first time, right-click the app
  and choose Open.
- **Linux (.deb):** `sudo apt install ./simulacra_0.2.0_amd64.deb`
- **Linux (AppImage):** `chmod +x simulacra_0.2.0_x86_64.AppImage`, then run it.

Each download is a single program: the illustrations, themes, palettes, seed data and translations
are built in.

## Building from source

Install [Rust](https://rustup.rs) (stable), then:

```bash
cargo run --release
```

On Linux, first install the system libraries (Debian and Ubuntu package names, as CI installs them in
[.github/actions/setup-rust/action.yml](.github/actions/setup-rust/action.yml)):

```bash
sudo apt-get install -y --no-install-recommends pkg-config clang \
  libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev libx11-xcb-dev libxcb1-dev \
  libvulkan-dev libfontconfig1-dev libfreetype-dev libgtk-3-dev libasound2-dev \
  libssl-dev libzstd-dev
```

## Documentation

The user guide lives in [docs/](docs/) (Next.js + Fumadocs) and is published to GitHub Pages at
<https://raitucarp.github.io/simulacra/>. Run it locally with `npm install` and `npm run dev` in `docs/`.

## Licenses

The theme palettes adapted from other projects are credited in [THIRD_PARTY.md](THIRD_PARTY.md).
