PHONY:reset-anvil
.PHONY: integration-tests

deploy-avs:
	./contracts/anvil/deploy-avs.sh

deploy-eigenlayer:
	./contracts/anvil/deploy-eigenlayer.sh

deploy-el-and-avs-contracts:
	$(MAKE) deploy-eigenlayer
	$(MAKE) deploy-avs

start-anvil: reset-anvil ## 
			 $(MAKE) start-anvil-chain-with-el-and-avs-deployed
			 docker start anvil

__TESTING__: ##

reset-anvil:
	-docker stop anvil
	-docker rm anvil

pr: 
	$(MAKE) start-anvil > /dev/null &
	sleep 4 
	cargo test --workspace
	cargo clippy --workspace --lib --examples --tests --benches --all-features
	cargo fmt -- --check
	docker stop anvil

integration-tests: 
				   $(MAKE) start-anvil > /dev/null 
				   cargo test --manifest-path ./integration-tests/Cargo.toml

fmt: 
	cargo fmt
	cd contracts && forge fmt
	cd ..
