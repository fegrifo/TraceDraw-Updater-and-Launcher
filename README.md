# TraceDraw Launcher

A small Rust launcher and updater for [TraceDraw](https://github.com/tracedraw/tracedraw) on Linux. It downloads the official Linux release, keeps the editor and CLI together in a per-user folder, and can register TraceDraw as a desktop application for supported drawing formats.

> This is an independent community project. It is not affiliated with or endorsed by the TraceDraw project or Corel.

## Features

- Checks the latest stable TraceDraw release on GitHub.
- Downloads the official Linux archive for `x86_64` or `aarch64` and extracts only `tracedraw` and `tracedraw-cli`.
- Installs those executables under `~/.local/opt/tracedraw/` by default. No administrator privileges are needed.
- Creates a user-level `.desktop` entry and registers file associations for `.cdr`, `.tdraw`, `.cmx`, `.svg`, `.svgz`, `.ai`, `.eps`, `.dxf`, and `.plt`.
- Opens the installed editor directly.

TraceDraw itself supports additional formats. The launcher currently avoids claiming common formats such as PDF, PNG, and JPEG as defaults.

## Status and supported platforms

This is an early project. The current desktop integration targets Linux only, and has been developed on Fedora Workstation. The downloader supports Linux `x86_64` and `aarch64` packages published by TraceDraw. Windows and macOS support are not implemented yet.

The launcher interface text is currently in Brazilian Portuguese; the repository documentation is in English. UI localization has not been implemented yet.

The launcher downloads the latest stable release from the official TraceDraw GitHub repository. It does not currently verify a published checksum or digital signature for the downloaded archive. The user is relying on HTTPS and the upstream GitHub release source.

## Build from source

You need stable Rust (including Cargo), a C compiler, and the Linux development libraries used by `eframe` and `rfd`.

### Fedora

```bash
sudo dnf install gcc gcc-c++ make pkgconf-pkg-config clang \
  libxkbcommon-devel libxcb-devel gtk3-devel atk fontconfig-devel
```

Install Rust using [rustup](https://www.rust-lang.org/tools/install):

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Restart the terminal, then build and run:

```bash
cargo run --release
```

On other Linux distributions, install the equivalents of the compiler, `pkg-config`, XCB, xkbcommon, GTK 3, ATK, and fontconfig development packages.

## Installation and file associations

The default application folder is `~/.local/opt/tracedraw/`. The launcher creates the executables there and writes its desktop integration under `~/.local/share/`. The first successful download also attempts to refresh the local MIME and desktop databases and set the listed drawing formats to open with TraceDraw. These actions apply only to the current user and do not use `sudo`.

The desktop entry points directly to the TraceDraw editor, so opening a supported file from the file manager starts the editor with that file. The launcher itself is used to download or update TraceDraw.

## Contributing

Bug reports, Linux distribution notes, design feedback, and pull requests are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request.

## License

This project is dual-licensed under MIT or Apache-2.0, at your option. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE). Confirm the copyright name in `LICENSE-MIT` before publishing. Do not redistribute TraceDraw binaries as part of this project; the launcher downloads them from the upstream release page.

## Acknowledgements

TraceDraw is the work of its own maintainers and contributors. This launcher downloads their published Linux packages; it does not include or modify TraceDraw source code.
