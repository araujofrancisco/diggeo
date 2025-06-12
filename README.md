# diggeo

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)

**diggeo** is a Rust project that allows you to query for domain IP addresses and geolocation information. This tool is designed for developers and network professionals who need to quickly resolve domain IPs and gather geolocation data.

## Features

- Query the IP address of a given domain.
- Retrieve geolocation data based on the resolved IP.
- Integrates with the [ipgeolocation.io](https://ipgeolocation.io) API to obtain geolocation information for IP addresses.
- Simple and fast CLI or library usage (details below).

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (if building from source)

### Installation

Clone the repository:

```bash
git clone https://github.com/araujofrancisco/diggeo.git
cd diggeo
```

Build the project:

```bash
cargo build --release
```

### Usage

You can use **diggeo** either by passing IPs directly, piping from a file, or resolving domains:

```bash
# Query geolocation for multiple IPs
diggeo 8.8.8.8 1.1.1.1

# Query geolocation for IPs listed in a file
cat ips.txt | diggeo

# Resolve a domain and get geolocation for its IP(s)
diggeo --dig example.com
```

## Configuration

Before using `diggeo`, you must create a configuration file located at `/etc/diggeo.conf`.  
This file should contain your API key in the following format:

```
api_key = your_api_key_here
```

Replace `your_api_key_here` with your actual API key.

## Project Structure

- `src/` - Main source code directory
- `Cargo.toml` - Project manifest
- `LICENSE` - GNU GPL v3.0 License
- `.gitignore`, `Cargo.lock`, `README.md` - Project files

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Author

- [araujofrancisco](https://github.com/araujofrancisco)

---

> _Note: Replace the "Usage" section with actual usage instructions as the project evolves._