# Examples

Five standalone configurations, ordered by complexity. Run commands from the corresponding directory.

| Example | Run | Demonstrates |
| --- | --- | --- |
| [01 — Web server](01-web-server/) | `tuggo compose dev up -d` | Images, ports, environment selection |
| [02 — Variables](02-environment-variables/) | `tuggo compose dev up -d` | `.env.dev` and interpolation |
| [03 — Database](03-web-and-database/) | `tuggo compose dev up -d` | PostgreSQL, Adminer, health checks, volumes |
| [04 — Profiles](04-services-profiles/) | `tuggo compose dev --profile debug up -d` | Optional Redis debugging |
| [05 — Multi-service](05-multi-service-isolation/) | `tuggo compose dev up -d` | Networks, volumes, multiple services |

For example:

```sh
cd examples/03-web-and-database
tuggo compose dev config
tuggo compose dev up -d
tuggo compose dev ps
tuggo compose dev down
```

For example 02, copy `.env.dev.example` to `.env.dev` (and `.env.test.example` to `.env.test` when using the test environment).

All examples require Docker Engine and Docker Compose v2. They use demonstration credentials and are intended for local use. `down -v` removes volumes and their data. Avoid running different environments concurrently in the same directory because Tuggo currently shares one generated Compose file.
