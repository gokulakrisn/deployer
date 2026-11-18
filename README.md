# Deployer

A lightweight Rust CLI for running **Docker Compose across multiple environments** from one `deployer.yml` file.

[![CI](https://github.com/gokulakrisn/deployer/actions/workflows/ci.yml/badge.svg)](https://github.com/gokulakrisn/deployer/actions/workflows/ci.yml)
[![Release](https://github.com/gokulakrisn/deployer/actions/workflows/release.yml/badge.svg)](https://github.com/gokulakrisn/deployer/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

> **Early preview:** This project is under active development. Try a [pre-release](https://github.com/gokulakrisn/deployer/releases) and report issues before using it for production deployments.

## Why Deployer?

Keep development, staging, and production Compose configurations together, then invoke the Docker Compose commands you already know:

```sh
deployer compose dev up -d
deployer compose stage logs -f
deployer compose prod down
```

Deployer selects the requested environment, writes a managed Compose configuration, and forwards the remaining arguments to `docker compose`.

## Features

- **Single configuration file:** Define environments as top-level keys in `deployer.yml`.
- **Native Compose commands:** Forward arguments such as `up -d --build`, `ps`, `logs -f`, and `down`.
- **Isolated project names:** Use a separate Compose project name for each environment.
- **Optional environment variables:** Automatically use `.env.<environment>` when present.
- **Managed output:** Refuse to overwrite an existing generated file without Deployer's ownership header.
- **Pre-release binaries:** Download builds for Linux, macOS, and Windows from GitHub Releases.

## Requirements

- Docker Engine and **Docker Compose v2** (`docker compose`) installed and available on `PATH`.
- For building from source: a current Rust toolchain with Cargo.

## Installation

Download the archive for your operating system and architecture from the [GitHub Releases page](https://github.com/gokulakrisn/deployer/releases), extract it, and put the `deployer` executable (or `deployer.exe` on Windows) on your `PATH`.

Alternatively, build from source:

```sh
git clone https://github.com/gokulakrisn/deployer.git
cd deployer
git switch feature/mvp-compose-cli
cargo build --release
```

The executable will be at `target/release/deployer` (or `target/release/deployer.exe` on Windows).

## Quick start

1. Create `deployer.yml` in your project directory:

```yaml
dev:
  services:
    web:
      image: nginx:alpine
      ports:
        - "8080:80"

stage:
  services:
    web:
      image: nginx:stable
      ports:
        - "8081:80"

prod:
  services:
    web:
      image: nginx:stable
      ports:
        - "8082:80"
```

2. From the directory containing `deployer.yml`, run:

```sh
deployer compose dev up -d
deployer compose dev ps
deployer compose dev logs -f
deployer compose dev down
```

You can also copy [the example configuration](examples/deployer.yml).

## Command reference

```text
deployer compose <environment> <docker-compose-arguments...>
```

| Command | Purpose |
| --- | --- |
| `deployer compose dev up -d --build` | Build and start development services |
| `deployer compose stage ps` | Show staging services |
| `deployer compose prod logs -f` | Follow production logs |
| `deployer compose prod down` | Stop and remove production services |
| `deployer --help` | Show CLI help |

The environment must exist as a top-level key in `deployer.yml` and contain a `services` mapping. Environment names may contain ASCII letters, digits, hyphens, and underscores.

### Environment-specific variables

Create an optional `.env.dev`, `.env.stage`, or `.env.prod` alongside `deployer.yml`. Deployer passes the matching file to Docker Compose with `--env-file`.

```dotenv
# .env.dev
APP_PORT=8080
```

Use standard Compose variable interpolation in the selected environment's configuration. Keep secrets out of Git.

### Generated files and project isolation

Deployer writes the selected Compose YAML to:

```text
.deployer/compose.generated.yml
```

The file is regenerated on each invocation. Do not edit it manually. If an existing file lacks Deployer's management header, the CLI refuses to overwrite it.

Compose project names are based on the current directory name plus the environment, helping separate resources for `dev`, `stage`, and `prod`. Deployer explicitly passes `-f` to Compose rather than relying on automatic override-file discovery.

**Current limitation:** The generated path is shared by all environments, so simultaneous Deployer commands in the same project directory can interfere with one another. Avoid concurrent invocations until per-environment output and locking are implemented. Relative bind mounts and other path-sensitive Compose settings should be validated with `deployer compose <environment> config` before deployment.

## Development

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo build --release
```

The [CI workflow](.github/workflows/ci.yml) runs formatting, Clippy, and tests. The [release workflow](.github/workflows/release.yml) builds downloadable archives and checksums for release candidates.

## Contributing

Issues, bug reports, and pull requests are welcome. Please include your operating system, Docker Compose version, reproduction steps, and the relevant (redacted) `deployer.yml` snippet.

## License

Licensed under the **MIT License**. See [LICENSE](LICENSE). You may use, modify, distribute, sublicense, and sell the software subject to the license's copyright and permission notice requirements.
