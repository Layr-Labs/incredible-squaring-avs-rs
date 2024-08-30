PHONY:reset-anvil

deploy-avs-save-anvil-state:
	./contracts/anvil/deploy-avs-save-anvil-state.sh

deploy-contracts-save-anvil-state:
	./contracts/anvil/deploy-contracts-save-anvil-state.sh

start-anvil-chain-with-el-and-avs-deployed: 
	./contracts/anvil/start-anvil-chain-with-el-and-avs-deployed.sh


start-anvil: reset-anvil ## 
			 $(MAKE) start-anvil-chain-with-el-and-avs-deployed
			 docker start anvil

__TESTING__: ##

reset-anvil:
	-docker stop anvil
	-docker rm anvil

pr: reset-anvil ##
	$(MAKE) start-anvil-chain-with-el-and-avs-deployed
	docker start anvil
	cargo test --workspace
	cargo clippy --workspace --lib --examples --tests --benches --all-features
	cargo +nightly fmt -- --check
	docker stop anvil

