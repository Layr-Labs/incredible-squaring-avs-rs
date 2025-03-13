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

tests:
	$(MAKE) start_docker
	$(MAKE) deploy-el-and-avs-contracts
	cargo test --workspace --exclude incredible-bindings

pr:
	$(MAKE) tests
	$(MAKE) clippy
	cargo fmt -- --check

clippy:
	cargo clippy --workspace --lib --examples --tests --benches --all-features --exclude incredible-bindings

integration_tests:
	$(MAKE) start_docker
	$(MAKE) deploy-el-and-avs-contracts
	cargo test  --manifest-path ./integration-tests/Cargo.toml  -- --nocapture

fmt: 
	cargo fmt
	cd contracts && forge fmt
	cd ..


# RUST_BINDINGS_PATH:=cr/rust/crates/utils/src/bindings
RUST_BINDINGS_PATH:=crates/bindings/src

generate-bindings:
	cd contracts && forge build --force --skip test --skip script
	rm -rf ${RUST_BINDINGS_PATH}
	forge bind --alloy --skip-build --overwrite --module \
		--root contracts/  \
		--bindings-path ${RUST_BINDINGS_PATH} \
		--select '^IncredibleSquaringTaskManager$$' \
		--select '^IncredibleSquaringServiceManager$$'
