PHONY: deploy-el-and-avs-contracts
PHONY: reset-anvil
PHONY: integration-tests


deploy-avs:
	./contracts/anvil/deploy-avs.sh

deploy-eigenlayer:
	./contracts/anvil/deploy-eigenlayer.sh

deploy-el-and-avs-contracts:
	$(MAKE) deploy-eigenlayer
	$(MAKE) deploy-avs


__TESTING__: ##

reset_anvil:
	-docker stop anvil
	-docker rm anvil 


start_docker:
	$(MAKE) reset_anvil
	docker run -d --name anvil -p 8545:8545 --entrypoint anvil \
		ghcr.io/foundry-rs/foundry:latest --host 0.0.0.0
	sleep 2

pr: 
	$(MAKE) start_docker
	$(MAKE) deploy-el-and-avs-contracts
	cargo test --workspace --exclude incredible-bindings
	cargo clippy --workspace --lib --examples --tests --benches --all-features --exclude incredible-bindings
	cargo fmt -- --check

clippy:
	   cargo clippy --workspace --lib --examples --tests --benches --all-features --exclude incredible-bindings

integration_tests:
				  $(MAKE) start_docker
				  $(MAKE) deploy-el-and-avs-contracts
				  cargo test --manifest-path ./integration-tests/Cargo.toml

fmt: 
	cargo fmt
	cd contracts && forge fmt
	cd ..
