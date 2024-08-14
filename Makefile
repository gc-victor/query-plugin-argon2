ARGUMENTS = $(filter-out $@,$(MAKECMDGOALS))
GET_ARGUMENT = $(strip $(call word,$(1),$(ARGUMENTS)))

# Build

build:
	cargo build --target wasm32-wasi --package plugin-argon2

build-test:
	cargo build --tests --target wasm32-wasi --package plugin-test

build-release:
	cargo build --target wasm32-wasi --release --package plugin-argon2

# Changelog

changelog:
	@commits=$$(git log --grep="release: version" --format="%H" -n 2); \
	if [ $$(echo "$$commits" | wc -l) -lt 2 ]; then \
		echo "Error: Less than two 'release' commits found."; \
		exit 1; \
	fi; \
	last=$$(echo "$$commits" | head -n 1); \
	prev=$$(echo "$$commits" | tail -n 1); \
	git cliff $$prev..$$last --prepend CHANGELOG.md; \
	echo "CHANGELOG.md has been updated with changes between commits:"; \
	echo "Previous: $$prev"; \
	echo "Latest: $$last"

# Clean

clean:
	rm -rf Cargo.lock | cargo clean

# Format

fmt:
	cargo fmt -- --check

# Install

install-xtp:
	curl https://static.dylibso.com/cli/install.sh | sudo sh

# Lint

lint:
	cargo clippy --all-targets --all-features

# Test

test:
	cargo build --target wasm32-wasi --package plugin-test
	cargo build --target wasm32-wasi --package plugin-argon2
	xtp plugin test ./target/wasm32-wasi/debug/plugin_argon2.wasm --with ./target/wasm32-wasi/debug/plugin_test.wasm

# Tag

tag:
	perl -pi -e 's/version = "$(call GET_ARGUMENT,1)"/version = "$(call GET_ARGUMENT,2)"/g' ./Cargo.toml
	cargo check --target wasm32-wasi
	git add Cargo.lock
	git add Cargo.toml
	git commit -m "release: version $(call GET_ARGUMENT,2)"
	git commit --amend --no-edit
	git push --force-with-lease
	git tag v$(call GET_ARGUMENT,2)
	git push --tags

tag-rollback:
	@read -p "Are you sure you want to rollback the tag version $(ARGUMENTS)? [Y/n] " REPLY; \
    if [ "$$REPLY" = "Y" ] || [ "$$REPLY" = "y" ] || [ "$$REPLY" = "" ]; then \
        git reset --soft HEAD~1; \
		git reset HEAD Cargo.lock; \
		git reset HEAD Cargo.toml; \
		git checkout -- Cargo.lock; \
		git checkout -- Cargo.toml; \
		git tag -d v$(ARGUMENTS); \
		git push origin --delete v$(ARGUMENTS); \
		git push --force-with-lease; \
	else \
		echo "Aborted."; \
	fi

# catch anything and do nothing
%:
	@:
