# Tuggo

A lightweight Rust CLI for **reproducing Docker Compose environments locally** from one `tuggo.yml` file.

[![CI](https://github.com/gokulakrisn/tuggo/actions/workflows/ci.yml/badge.svg)](https://github.com/gokulakrisn/tuggo/actions/workflows/ci.yml)
[![Release](https://github.com/gokulakrisn/tuggo/actions/workflows/release.yml/badge.svg)](https://github.com/gokulakrisn/tuggo/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

> **Early preview:** This project is under active development. Try a [pre-release](https://github.com/gokulakrisn/tuggo/releases) and report issues for local development and testing; Tuggo is not a production deployment orchestrator.

## Why Tuggo?

Keep development, staging, and production Compose configurations together, then invoke the Docker Compose commands you already know:

```sh
tuggo compose dev up -d
tuggo compose stage logs -f
tuggo compose prod down
```

Tuggo selects the requested environment, writes a managed Compose configuration, and forwards the remaining arguments to `docker compose`.

## Features

- **Single configuration file:** Define environments as top-level keys in `tuggo.yml`.
- **Native Compose commands:** Forward arguments such as `up -d --build`, `ps`, `logs -f`, and `down`.
- **Isolated project names:** Use a separate Compose project name for each environment.
- **Optional environment variables:** Automatically use `.env.<environment>` when present.
- **Managed output:** Refuse to overwrite an existing generated file without Tuggo's ownership header.
- **Pre-release binaries:** Download builds for Linux, macOS, and Windows from GitHub Releases.

## Requirements

- Docker Engine and **Docker Compose v2** (`docker compose`) installed and available on `PATH`.
- For building from source: a current Rust toolchain with Cargo.

## Installation

Download the archive for your operating system and architecture from the [GitHub Releases page](https://github.com/gokulakrisn/tuggo/releases), extract it, and put the `tuggo` executable (or `tuggo.exe` on Windows) on your `PATH`.

Alternatively, build from source:

```sh
git clone https://github.com/gokulakrisn/tuggo.git
cd tuggo
git switch feature/mvp-compose-cli
cargo build --release
```

The executable will be at `target/release/tuggo` (or `target/release/tuggo.exe` on Windows).

## Quick start

1. Create `tuggo.yml` in your project directory:

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

2. From the directory containing `tuggo.yml`, run:

```sh
tuggo compose dev up -d
tuggo compose dev ps
tuggo compose dev logs -f
tuggo compose dev down
```

You can also copy [the example configuration](examples/tuggo.yml).\n\n## Learn with five progressive examples\n\nWork through the [examples guide](examples/README.md), from a single web server to a multi-service local stack with networks and persistent volumes:\n\n1. [Web server and environment selection](examples/01-web-server/tuggo.yml)\n2. [Per-environment variables](examples/02-environment-variables/tuggo.yml)\n3. [PostgreSQL, Adminer, health checks, and volumes](examples/03-web-and-database/tuggo.yml)\n4. [Optional debug profiles and service dependencies](examples/04-services-profiles/tuggo.yml)\n5. [Four-service stack with network and storage isolation](examples/05-multi-service-isolation/tuggo.yml)\n\nEach example is runnable from its own directory; see the guide for setup, commands, expected behavior, and cleanup.

## Command reference

```text
tuggo compose <environment> <docker-compose-arguments...>
```

| Command | Purpose |
| --- | --- |
| `tuggo compose dev up -d --build` | Build and start development services |
| `tuggo compose stage ps` | Show staging services |
| `tuggo compose prod logs -f` | Follow production logs |
| `tuggo compose prod down` | Stop and remove production services |
| `tuggo --help` | Show CLI help |

The environment must exist as a top-level key in `tuggo.yml` and contain a `services` mapping. Environment names may contain ASCII letters, digits, hyphens, and underscores.

### Environment-specific variables

Create an optional `.env.dev`, `.env.stage`, or `.env.prod` alongside `tuggo.yml`. Tuggo passes the matching file to Docker Compose with `--env-file`.

```dotenv
# .env.dev
APP_PORT=8080
```

Use standard Compose variable interpolation in the selected environment's configuration. Keep secrets out of Git.

### Generated files and project isolation

Tuggo writes the selected Compose YAML to:

```text
.tuggo/compose.generated.yml
```

The file is regenerated on each invocation. Do not edit it manually. If an existing file lacks Tuggo's management header, the CLI refuses to overwrite it.

Compose project names are based on the current directory name plus the environment, helping separate resources for `dev`, `stage`, and `prod`. Tuggo explicitly passes `-f` to Compose rather than relying on automatic override-file discovery.

**Current limitation:** The generated path is shared by all environments, so simultaneous Tuggo commands in the same project directory can interfere with one another. Avoid concurrent invocations until per-environment output and locking are implemented. Relative bind mounts and other path-sensitive Compose settings should be validated with `tuggo compose <environment> config` before deployment.

## Development

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo build --release
```

The [CI workflow](.github/workflows/ci.yml) runs formatting, Clippy, and tests. The [release workflow](.github/workflows/release.yml) builds downloadable archives and checksums for release candidates.

## Contributing

Issues, bug reports, and pull requests are welcome. Please include your operating system, Docker Compose version, reproduction steps, and the relevant (redacted) `tuggo.yml` snippet.

## License

Licensed under the **MIT License**. See [LICENSE](LICENSE). You may use, modify, distribute, sublicense, and sell the software subject to the license's copyright and permission notice requirements.
