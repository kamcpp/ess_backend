.PHONY: install-deps
install-deps:
	sudo apt update
	sudo apt install -y build-essential
	sudo apt install -y libpq-dev

.PHONY: install-diesel-cli
install-diesel-cli:
	cargo install diesel_cli --no-default-features --features postgres

.PHONY: generate
generate:
	mkdir -p migrations
	diesel migration generate employee
