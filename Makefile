# Makefile — platform-holidays (API Octor)
# Desenvolvimento local: Cargo + Vault OPS
# Docs: https://backstage.octor.com.br/secrets-vault.html
#
# make env → ~/.octor/run/$(APP_NAME)/env + symlink ./.env

APP_NAME ?= $(shell basename $(CURDIR))
VAULT_URL ?= http://10.8.0.9:8200
VAULT_PATH ?= apps/$(APP_NAME)/dev
OCTOR_BIN ?= $(HOME)/.octor/bin
OCTOR_CLI := $(OCTOR_BIN)/octor-env
OCTOR_BACKSTAGE_REPO ?= 0ctor/web-backstage
OCTOR_BACKSTAGE_REF ?= dev

.PHONY: help install-cli env env-push env-diff run dev up down test fmt clippy docker agents-sync install-agents-sync

help:
	@echo "Comandos (VPN Octor requerida para vault):"
	@echo "  make env / env-push / env-diff  - secrets no Vault (materialização)"
	@echo "  make run | make dev            - env + agents-sync + cargo run"
	@echo "  make up                        - env + agents-sync + compose/run"
	@echo "  make agents-sync               - AGENTS.md + skills octor-*"
	@echo "  make test | fmt | clippy | docker"
	@echo "Vault path: $(VAULT_PATH)"

install-cli:
	@mkdir -p "$(OCTOR_BIN)"
	@tmp=$$(mktemp); \
	if curl -sfL --max-time 15 "$(VAULT_URL)/cli/octor-env" -o "$$tmp" 2>/dev/null; then \
	  mv "$$tmp" "$(OCTOR_CLI)"; echo "octor-env: vault"; \
	else \
	  rm -f "$$tmp"; \
	  if [ -f "$(CURDIR)/../web-backstage/scripts/octor-env" ]; then \
	    cp "$(CURDIR)/../web-backstage/scripts/octor-env" "$(OCTOR_CLI)"; \
	  elif [ -f "$(CURDIR)/scripts/octor-env" ]; then \
	    cp "$(CURDIR)/scripts/octor-env" "$(OCTOR_CLI)"; \
	  elif command -v gh >/dev/null 2>&1; then \
	    gh api "repos/$(OCTOR_BACKSTAGE_REPO)/contents/scripts/octor-env?ref=$(OCTOR_BACKSTAGE_REF)" \
	      -H "Accept: application/vnd.github.raw" > "$(OCTOR_CLI).tmp" && \
	    mv "$(OCTOR_CLI).tmp" "$(OCTOR_CLI)"; \
	  elif [ -x "$(OCTOR_CLI)" ]; then true; \
	  else echo "octor-env: sem fonte"; exit 1; fi; \
	  echo "octor-env: $(OCTOR_CLI)"; \
	fi
	@chmod +x "$(OCTOR_CLI)"

$(OCTOR_CLI):
	@$(MAKE) install-cli

env: $(OCTOR_CLI)
	@VAULT_URL="$(VAULT_URL)" "$(OCTOR_CLI)" pull "$(VAULT_PATH)"

env-push: $(OCTOR_CLI)
	@VAULT_URL="$(VAULT_URL)" "$(OCTOR_CLI)" push "$(VAULT_PATH)"

env-diff: $(OCTOR_CLI)
	@VAULT_URL="$(VAULT_URL)" "$(OCTOR_CLI)" diff "$(VAULT_PATH)"

run: env agents-sync
	cargo run

dev: run

up: env agents-sync
	@if [ -f docker-compose.yml ]; then docker compose up -d; \
	elif [ -f docker-compose.example.yml ]; then docker compose -f docker-compose.example.yml up -d; \
	else $(MAKE) run; fi

down:
	@if [ -f docker-compose.yml ]; then docker compose down; \
	elif [ -f docker-compose.example.yml ]; then docker compose -f docker-compose.example.yml down; \
	else true; fi

test:
	cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy --all-targets -- -D warnings

docker:
	docker build -t $(APP_NAME):local .

# --- octor-agents-sync (auto) ---
OCTOR_AGENTS_SYNC ?= $(OCTOR_BIN)/octor-agents-sync

agents-sync: install-agents-sync
	@$(OCTOR_AGENTS_SYNC) .

install-agents-sync:
	@mkdir -p "$(OCTOR_BIN)"
	@if [ -f "$(CURDIR)/../web-backstage/scripts/octor-agents-sync" ]; then \
	  cp "$(CURDIR)/../web-backstage/scripts/octor-agents-sync" "$(OCTOR_AGENTS_SYNC)"; \
	elif command -v gh >/dev/null 2>&1; then \
	  gh api "repos/$(OCTOR_BACKSTAGE_REPO)/contents/scripts/octor-agents-sync?ref=$(OCTOR_BACKSTAGE_REF)" \
	    -H "Accept: application/vnd.github.raw" > "$(OCTOR_AGENTS_SYNC).tmp" && \
	  mv "$(OCTOR_AGENTS_SYNC).tmp" "$(OCTOR_AGENTS_SYNC)"; \
	elif [ -x "$(OCTOR_AGENTS_SYNC)" ]; then \
	  true; \
	else \
	  echo "octor-agents-sync: pulando (sem fonte)"; exit 0; \
	fi
	@chmod +x "$(OCTOR_AGENTS_SYNC)"
