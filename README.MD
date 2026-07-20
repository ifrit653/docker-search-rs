# docker-search-rs

A `docker search`-like CLI that also shows image size per tag and platform.

## Features

- Search Docker Hub for images
- Shows star count and official status
- Lists recent tags with size per platform (amd64, arm64, etc.)

## Installation

Requires [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo install --path .
```

Or download a prebuilt binary from [Releases](../../releases).

## Usage

```bash
docker-search-rs <QUERY> [OPTIONS]
```

### Options

| Flag            | Description                                     |
| --------------- | ----------------------------------------------- |
| `-l, --limit`   | Max number of search results (default: 10)      |
| `--tags-limit`  | Max number of tags shown per image (default: 5) |
| `-h, --help`    | Print help                                      |
| `-V, --version` | Print version                                   |

### Examples

```bash
docker-search-rs postgres --limit 3 --tags-limit 2
```

## License

MIT
