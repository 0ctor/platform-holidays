# platform-holidays

API Octor de **feriados brasileiros** (nacional, estadual, municipal) com **datas móveis calculadas ano a ano** (Páscoa gregoriana → Carnaval, Sexta-feira Santa, Corpus Christi).

Substitui a dependência de runtime em [rodriguesfas.github.io/holidays](https://rodriguesfas.github.io/holidays/) (JSON estático com Carnaval/Páscoa fixos e endpoints UF/cidade 404).

| Item | Valor |
|------|--------|
| Repo | [`0ctor/platform-holidays`](https://github.com/0ctor/platform-holidays) |
| Template | [`0ctor/template-api-rust`](https://github.com/0ctor/template-api-rust) |
| Host PRD | `https://holidays.octor.com.br` |
| Porta local | `4545` |
| Porta VPN (host) | `15216` |
| Auth nas leituras | **não** (dados públicos de calendário; sem PHI) |

## Rotas

| Método | Path | Descrição |
|--------|------|-----------|
| GET | `/health/live` · `/health/ready` | Health canônico |
| GET | `/v1/holidays?from=&to=&scopes=&uf=&city_ibge=` | Lista unificada |
| GET | `/v1/holidays?year=2026&scopes=national` | Atalho por ano |
| GET | `/v1/holidays/national` | Só nacionais |
| GET | `/v1/holidays/state/{uf}` | Estaduais (UF) |
| GET | `/v1/holidays/city/{city_ibge}` | Municipais (IBGE 7 dígitos) |
| GET | `/v1/health-calendar?year=` | Formato legado (array DD-MM) |
| GET | `/health_calendar.json` | Alias legado (drop-in PHP) |
| POST | `/v1/errors/report` | Erros FE → Loki |

`scopes`: `national`, `state`, `city`, `health` (CSV).

## Local

```bash
cp .env.example .env
# BIND_ADDRESS=0.0.0.0:4545
cargo run
curl -s 'http://127.0.0.1:4545/v1/holidays?year=2026&scopes=national' | jq '.count'
curl -s 'http://127.0.0.1:4545/v1/health-calendar?year=2026' | jq '.[0]'
```

Com Vault OPS: `make run`.

## Consumidores

- **platform-legacy** — `Holidaysapi` → `HOLIDAYS_API_BASE_URL` (padrão `https://holidays.octor.com.br`)
- **web-agenda** — overlay de datas comemorativas / futuros clientes `/v2`

## Deploy

- Push `dev` → CI self-hosted `octor-dev` (GHCR + compose VPN `:15216`)
- Promote `dev` → `main` → `platform-deploy-hook` no sp1 (PRD)

Compose host: `sp1-sd-octor-1` / `sp1-sd-octor-2` → `apps/platform-holidays/`.
