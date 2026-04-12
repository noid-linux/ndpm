# NDPM

Noid Linux's Package Manger - A user-friendly package manager wrapper
for XBPS

## Features

- **Simplified commands**: No more remembering XBPS flags - just
  `install`, `remove`, `search`
- **Safety first**: Prevents accidental root usage that could break
  your system

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

## Why NDPM?

XBPS is powerful but can be verbose. NDPM provides a cleaner interface
while maintaining all the functionality you need for daily package
management on Void Linux systems.

## Requirements

- Void Linux (or compatible XBPS-based system)
- `sudo` access for system package operations

## License

ndpm is released under the MIT License. For more details, see
the [LICENSE](https://github.com/noid-linux/ndpm/blob/main/LICENSE) file.
