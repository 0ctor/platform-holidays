# template-api-rust — Template API Rust/Actix Octor

Template canônico de **API Rust** para apps `0ctor/platform-*` (e backends de `web-*`).

Use via **GitHub Template** → novo repo → ajuste nome/porta/DB → registre no `platform-deploy-hook` ([docs/NOVO_APP.md](./docs/NOVO_APP.md)).

## O que já vem pronto

| Peça | Detalhe |
|------|---------|
| **Health** | `GET /health/live` + `/health/ready` (JSON canônico) |
| **Auth** | Bearer → `AUTH_INTROSPECT_URL` (web-auth) |
| **Loki** | push backend + `POST /v1/errors/report` (FE → API) |
| **Soft delete** | entidade sample `items` com `deleted_at` / `deleted_by` |
| **CRUD sample** | `GET/POST /v1/items`, `GET/DELETE /v1/items/{uuid}` |
| **Docker** | multi-stage + HEALTHCHECK live |
| **CI** | `cargo fmt` · `clippy` · `test` · `build` |

## Setup local

**Com VPN + Vault:**

```bash
make env    # apps/<repo>/dev
make run    # = make env + cargo run
```

| Make | Função |
|------|--------|
| `make env` / `env-push` / `env-diff` | Sync Vault OPS |
| `make run` / `make dev` | Secrets + `cargo run` |

Docs: [desenvolvimento-local](https://backstage.octor.com.br/desenvolvimento-local).

**Sem vault:**

```bash
cp .env.example .env
# Ajuste DATABASE_URL (user de app, nunca root)
cargo run
```

Health: `curl -s localhost:8080/health/live`

## Rotas

| Método | Path | Auth |
|--------|------|------|
| GET | `/health/live` | não |
| GET | `/health/ready` | não |
| POST | `/v1/errors/report` | não (rate limit) |
| GET/POST | `/v1/items` | Bearer |
| GET/DELETE | `/v1/items/{uuid}` | Bearer |

`DELETE` = soft delete (`UPDATE … SET deleted_at`).

## Deploy

1. Template → `0ctor/<nome>` (`dev` / `main`)
2. Host: compose + `.env` + `apps.json` (ver `docs/`)
3. Merge `dev` → `main` → hook deploya

Frontend complementar: [`0ctor/template-frontend`](https://github.com/0ctor/template-frontend).
