# Tuggo

Reproduce local Docker Compose environments from a single configuration file.

[![CI](https://github.com/gokulakrisn/tuggo/actions/workflows/ci.yml/badge.svg)](https://github.com/gokulakrisn/tuggo/actions/workflows/ci.yml)
[![Release](https://github.com/gokulakrisn/tuggo/actions/workflows/release.yml/badge.svg)](https://github.com/gokulakrisn/tuggo/actions/workflows/release.yml)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Tuggo selects an environment from `tuggo.yml` and runs standard `docker compose` commands against it. It is intended for local development and testing, not production orchestration.

## Installation

Download a binary from [Releases](https://github.com/gokulakrisn/tuggo/releases) or build from source (requires Rust):

```sh
git clone https://github.com/gokulakrisn/tuggo.git
cd tuggo
cargo build --release
```

Docker Engine and Docker Compose v2 are required.

## Usage

Create `tuggo.yml`:

```yaml
dev:
  services:
    web:
      image: nginx:alpine
      ports:
        - "8080:80"

test:
  services:
    web:
      image: nginx:stable-alpine
      ports:
        - "8081:80"
```

Run Compose commands from the directory containing the file:

```sh
tuggo compose dev up -d
tuggo compose dev ps
tuggo compose dev logs -f
tuggo compose dev down
```

Syntax: `tuggo compose <environment> <compose-arguments...>`

Each top-level key defines a complete Compose configuration. Tuggo uses a distinct Compose project name per environment and automatically loads `.env.<environment>` if present.

## Examples

| | Example | Topics |
| --- | --- | --- |
| 01 | [Web server](examples/01-web-server/) | Environment selection and ports |
| 02 | [Environment variables](examples/02-environment-variables/) | Per-environment `.env` files |
| 03 | [Web and database](examples/03-web-and-database/) | Dependencies, health checks, volumes |
| 04 | [Service profiles](examples/04-services-profiles/) | Optional debugging services |
| 05 | [Multi-service stack](examples/05-multi-service-isolation/) | Networks, storage, isolation |

See [examples/README.md](examples/README.md) for commands.

## Limitations

Tuggo writes a managed `.tuggo/compose.generated.yml`. This path is shared across environments within the same directory; avoid concurrent commands targeting different environments. For strict reproducibility, pin container image digests and external dependencies.

## Development

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
```

## License

[MIT](LICENSE)
