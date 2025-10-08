# yu - a package manager wrapper for all unix-like systems

`yu` allows users to use the same command to install, uninstall, and upgrade packages on all unix-like systems.

![CI/CD](https://github.com/Young-TW/yu/actions/workflows/rust.yml/badge.svg)

## Supported package managers

### Native

- [x] `apt`
- [x] `dnf`
- [x] `pacman`
- [x] `yum`
- [x] `homebrew`
- [x] `zypper`
- [x] `portage`
- [x] `apk`

### Sandbox / Snapshot

- [ ] `flatpak`
- [ ] `snap`
- [ ] `nix`
- [ ] `guix`

### Domain-Specific

- [ ] `conda` (Anaconda)
  - [ ] `pixi` (Pixi)
- [ ] `spack` (Scientific computing)

### Language-Specific

- [ ] `cargo` (Rust)
- [ ] `npm` (Node.js)
  - [ ] `pnpm` (Node.js)
- [ ] `pip` (Python)
  - [ ] `pipx` (Python)
  - [ ] `uv` (Python)
- [ ] `conan` (C/C++)
- [ ] `vcpkg` (C/C++)

## Installation

### Using cargo

If you have `cargo` installed, you can install `yu` using the following command:

```bash
cargo install yu-pkg
```

### Build from source manually

#### Build

Please make sure you have installed `rust` and `cargo`.

```bash
git clone https://github.com/Young-TW/yu.git
cd yu
cargo build --release
```

#### Install

```bash
cargo install --path .
```

please make sure the `~/.cargo/bin` is in your `PATH`.

Another way to install `yu` is just copy the `yu` script to your system.

```bash
sudo cp target/release/yu /usr/local/bin/yu
```

## Usage

install package:

```bash
yu install <package>
```

uninstall package:

```bash
yu uninstall <package>
```

reinstall package:

```bash
yu reinstall <package>
```

update package list:

```bash
yu update
```

upgrade all packages:

```bash
yu
```

or

```bash
yu upgrade
```

package name is the same as the package name in the package manager of the system.

auto remove unused packages:

```bash
yu autoremove
```

search package:

```bash
yu search <package>
```

### Flags

- `-h`, `--help`: Show help message.
- `-V`, `--version`: Show version.
- `-v`, `--verbose`: Show verbose output.
- `-s`, `--silent`: Show no output(only error output).

## Stats

![Alt](https://repobeats.axiom.co/api/embed/789e96f4a8ee75e44d631fb61bb7fa1aa348f476.svg "Repobeats analytics image")
