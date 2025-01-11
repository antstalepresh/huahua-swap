# Répertoire racine contenant tous les contrats
CONTRACTS_DIR := contracts
ARTIFACTS_DIR := artifacts
# Liste des sous-répertoires dans CONTRACTS_DIR contenant des smart contracts
CONTRACTS := $(shell find $(CONTRACTS_DIR) -mindepth 1 -maxdepth 1 -type d)

# Docker image pour optimiser les smart contracts
OPTIMIZER_IMAGE := cosmwasm/optimizer:0.16.0

$(ARTIFACTS_DIR):
	mkdir -p $(ARTIFACTS_DIR)

# Tâche principale pour tout compiler et optimiser
compile: $(CONTRACTS)

$(CONTRACTS):
	@if [ ! -f "$@/Cargo.toml" ]; then \
		echo "Error: $@ is not a valid Rust project."; \
		exit 1; \
	fi
	docker run --rm \
		-v "$$(pwd)/$@:/code" \
		--mount type=volume,source="$$(basename $@)_cache",target=/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		$(OPTIMIZER_IMAGE)
	@chmod -R u+w $(shell echo "$$(pwd)/$@/artifacts/*.wasm") 
	@mv $(shell echo "$$(pwd)/$@/artifacts/*.wasm") "$(ARTIFACTS_DIR)"


schema:
	# to install ts tooling see here: https://docs.cosmology.zone/ts-codegen
	cd contracts/bonding-curve && cargo run --bin schema
	cd contracts/huahua-factory && cargo run --bin schema

	cosmwasm-ts-codegen generate \
          --schema ./contracts/bonding-curve/schema \
          --out ./ts_types \
          --name BondingCurve \
          --no-bundle \


	cosmwasm-ts-codegen generate \
          --schema ./contracts/huahua-factory/schema \
          --out ./ts_types \
          --name HuahuaFactory \
          --no-bundle \


	
# Nettoyage des caches Docker
clean:
	@echo "Cleaning cache volumes..."
	-docker volume rm -f $(shell docker volume ls -q | grep "_cache") || true


.PHONY: compile clean $(CONTRACTS)
