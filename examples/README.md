# Progressive Tuggo examples

These five examples progress from a single container to multi-service local environments. Each folder has its own `tuggo.yml`. **Run Tuggo from inside the example folder** because the CLI reads `./tuggo.yml`.

Prerequisites: Tuggo on your `PATH`, Docker Engine running, and Docker Compose v2 available as `docker compose`. Examples use public container images and local-only demonstration credentials; do not expose them to untrusted networks.

| Level | Example | Concepts | Try |
| --- | --- | --- | --- |
| 1 | [Web server](01-web-server/tuggo.yml) | Selecting dev/test, ports, images | `cd examples/01-web-server && tuggo compose dev up -d` |
| 2 | [Environment variables](02-environment-variables/tuggo.yml) | Per-environment .env files, Compose interpolation | `cd examples/02-environment-variables && tuggo compose dev up -d` |
| 3 | [Web + database](03-web-and-database/tuggo.yml) | Multiple services, health checks, persistence | `cd examples/03-web-and-database && tuggo compose dev up -d` |
| 4 | [Profiles](04-services-profiles/tuggo.yml) | Optional debugging service, dependencies | `cd examples/04-services-profiles && tuggo compose dev --profile debug up -d` |
| 5 | [Isolated multi-service stack](05-multi-service-isolation/tuggo.yml) | Networks, volumes, dev/test differences | `cd examples/05-multi-service-isolation && tuggo compose dev up -d` |

## 1. Web server

Start `dev` and visit http://localhost:8080, or start `test` and visit http://localhost:8081. Each environment selects a different image and host port.

```sh
cd examples/01-web-server
tuggo compose dev config
tuggo compose dev up -d
tuggo compose dev ps
tuggo compose dev down
```

## 2. Environment variables

Tuggo automatically passes `.env.<environment>` to Docker Compose when it exists. The example provides templates; copy them first:

```sh
cd examples/02-environment-variables
cp .env.dev.example .env.dev
cp .env.test.example .env.test
tuggo compose dev config
tuggo compose dev up -d
curl http://localhost:8080
tuggo compose dev down
```

On Windows PowerShell, use `Copy-Item .env.dev.example .env.dev` instead of `cp`. Change `GREETING` and `APP_PORT` in the copied file and restart. The `test` environment uses port 8081.

## 3. Web interface and PostgreSQL

`dev` starts PostgreSQL with a named volume and Adminer on http://localhost:8082. `test` uses temporary database storage and Adminer on http://localhost:8083. The database health check gates Adminer startup.

```sh
cd examples/03-web-and-database
tuggo compose dev up -d
tuggo compose dev ps
tuggo compose dev logs db
tuggo compose dev down
```

For Adminer, use server `db`, user `demo`, database `demo`, and the password in the selected YAML. `down -v` **deletes named volumes and their data**; use only when you intentionally want a clean slate.

## 4. Optional debug profile

The `dev` stack has Redis, a one-shot Redis connectivity check (`worker`), and an optional monitoring process (`inspector`) behind the `debug` profile.

```sh
cd examples/04-services-profiles
tuggo compose dev up -d
tuggo compose dev ps -a
tuggo compose dev --profile debug up -d
tuggo compose dev logs inspector
tuggo compose dev --profile debug down
```

The `worker` is a one-shot diagnostic command and is expected to exit. The `inspector` prints Redis commands and is intended only for local debugging.

## 5. Networks, volumes, and environment isolation

The `dev` environment runs an Nginx gateway (http://localhost:8090), PostgreSQL, Adminer (http://localhost:8091), and Redis with named volumes. The `test` environment runs an equivalent stack on ports 8092 and 8093, using disposable database storage.

```sh
cd examples/05-multi-service-isolation
tuggo compose dev config
tuggo compose dev up -d
tuggo compose dev ps
tuggo compose dev down
```

The gateway is deliberately attached only to `frontend`; database, cache and Adminer use `backend`. It serves the stock Nginx welcome page, **not** a configured application reverse proxy. To make it proxy to a backend, add an application service and connect the gateway to the appropriate network.

## Important behavior

- Tuggo expects **top-level environment keys** (`dev`, `test`, etc.), each containing a full Compose configuration. These are not Docker Compose profiles.
- It writes a managed `.tuggo/compose.generated.yml` in the current directory. Do not edit that file.
- The generated file path is shared across environments in the same directory. **Do not run commands for different environments concurrently** until this limitation is addressed.
- Docker Compose project names include the environment, but the shared generated file means you should validate each configuration with `tuggo compose <environment> config`.
- These examples demonstrate local configurations, **not exact bit-for-bit reproducibility**: image tags can move. Pin image digests and control external dependencies when strict repeatability matters.
- Each example can be cleaned up with `tuggo compose <environment> down`; use `down -v` only if you intend to remove volumes.
