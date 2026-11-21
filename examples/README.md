# Examples

Each directory contains a standalone `tuggo.yml`.

| Example | Description |
| --- | --- |
| [01 - Web server](01-web-server/) | Environment selection and port mapping |
| [02 - Variables](02-environment-variables/) | Environment-specific configuration |
| [03 - Database](03-web-and-database/) | Services, health checks, and volumes |
| [04 - Profiles](04-services-profiles/) | Optional services and dependencies |
| [05 - Multi-service](05-multi-service-isolation/) | Networks and storage isolation |

Run any example from its directory:

```sh
cd examples/01-web-server
tuggo compose dev up -d
tuggo compose dev down
```

For example 02, copy `.env.dev.example` to `.env.dev` before running.
