# NDPM

Noid Linux's Package Manger - A user-friendly package manager wrapper for XBPS with built-in AppImage support.

## Features

- **Unified interface**: Manage both XBPS packages and AppImages through one tool
- **Simplified commands**: No more remembering XBPS flags - just `install`, `remove`, `search`
- **Safety first**: Prevents accidental root usage that could break your system
- **AppImage integration**: Download, install, and manage AppImages with automatic symlink creation

## Installation

```bash
cargo install ndpm
```

## Usage

### Package Management

```bash
# Install packages
ndpm install firefox neovim

# Update package database
ndpm update

# Upgrade system
ndpm upgrade -y

# Remove packages
ndpm remove unwanted-package

# Search for packages
ndpm search rust
```

### AppImage Management

```bash
# Install AppImage from URL
ndpm appimage install --from https://example.com/app.AppImage myapp

# List installed AppImages
ndpm ai list

# Remove AppImage
ndpm a remove myapp
```

## Why NDPM?

XBPS is powerful but can be verbose. NDPM provides a cleaner interface while maintaining all the functionality you need for daily package management on Void Linux systems.

## Requirements

- Void Linux (or compatible XBPS-based system)
- Rust toolchain for building
- `sudo` access for system package operations

## License

ndpm is released under the MIT License. For more details, see the [LICENSE](https://github.com/noid-linux/ndpm/blob/main/LICENSE) file.
