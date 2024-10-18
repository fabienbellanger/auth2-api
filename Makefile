.PHONY: help \
	serve \
	watch \
	upgrade \
	upgrade-force \
	lint \
	lint-audit \
	audit-fix \
	test \
	clean \
	prepare \
	build \
	build-no-audit \
	sqlx-prepare \
	doc \
	doc-deps

.DEFAULT_GOAL=help

# Parameters
APP_NAME="Auth2 API"
CURRENT_PATH=$(shell pwd)
DOCKER_COMPOSE=docker-compose
DOCKER=docker
CARGO=cargo
CARGO_BIN_NAME="auth2-api"
USER_LASTNAME="Admin"
USER_FIRSTNAME="Test"
USER_EMAIL="test2@testest.com"
USER_PASSWORD="00000000"

help: Makefile
	@echo
	@echo "Choose a command run in "$(APP_NAME)":"
	@echo
	@sed -n 's/^##//p' $< | column -t -s ':' | sed -e 's/^/ /'
	@echo

## serve: Start web server
serve:
	$(CARGO) run -- serve

## watch: Start web server with hot reload
watch:
	$(CARGO) watch -x "run -- serve"

## upgrade: Upgrade workspace packages and update the dependency versions recorded in the local lock file
upgrade:
	$(CARGO) upgrade
	$(CARGO) update

## upgrade-force: Upgrade workspace packages and update the dependency versions recorded in the local lock file
upgrade-force:
	$(CARGO) upgrade --incompatible
	$(CARGO) update

## lint: Run clippy and rustfmt
lint:
	$(CARGO) fmt
	$(CARGO) clippy -- -Dwarnings

## lint-audit: Run clippy, rustfmt and audit
lint-audit: lint
	$(CARGO) audit

## audit-fix: Fix audit
audit-fix:
	$(CARGO) audit fix

## test: Launch unit tests in a single thread
test:
	$(CARGO) test -- --test-threads=1 --nocapture

## clean: Remove target directory
clean:
	$(CARGO) clean

## prepare: Run lint, test and sqlx-prepare
prepare: lint test sqlx-prepare

## build: Build application in release mode
build: lint-audit test
	$(CARGO) build --release

## build-no-audit: Build application in release mode
build-no-audit: lint test
	$(CARGO) build --release

## sqlx-prepare: Prepare for sqlx offline mode
sqlx-prepare:
	$(CARGO) sqlx prepare -- --bin $(CARGO_BIN_NAME)

## doc: Open Rust documentation without dependencies
doc:
	$(CARGO) doc --open --no-deps --document-private-items

## doc: Open Rust documentation with dependencies
doc-deps:
	$(CARGO) doc --open --document-private-items
