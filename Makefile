PHONY: deploy-el-and-avs-contracts

deploy-avs:
	./contracts/anvil/deploy-avs.sh

deploy-eigenlayer:
	./contracts/anvil/deploy-eigenlayer.sh

deploy-el-and-avs-contracts:
	$(MAKE) deploy-eigenlayer
	$(MAKE) deploy-avs


__TESTING__: ##

pr: 
	$(MAKE) deploy-el-and-avs-contracts
	cargo test --workspace --exclude incredible-bindings
	cargo clippy --workspace --lib --examples --tests --benches --all-features --exclude incredible-bindings
	cargo fmt -- --check --exclude incredible-bindings

clippy:
	   cargo clippy --workspace --lib --examples --tests --benches --all-features --exclude incredible-bindings

integration-tests: $(MAKE) deploy-el-and-avs-contracts
				   cargo test --manifest-path ./integration-tests/Cargo.toml

fmt: 
	cargo fmt
	cd contracts && forge fmt
	cd ..
