#!/bin/bash
set -eux



CONFIG_FILE="$1"

CHIHUAHUA_CHAIN_ID=$(jq -r '.chain_id' $CONFIG_FILE)
CHIHUAHUA_NODE=$(jq -r '.rpc_node' $CONFIG_FILE)
TX_SENDER_WALLET=$(jq -r '.tx_sender_wallet' $CONFIG_FILE)
TX_SENDER_ADDRESS=$(chihuahuad keys show $TX_SENDER_WALLET --keyring-backend test | grep "address:" | sed 's/.*address: //')
HUB_CONNECTION_ID=$(jq -r '.hub_connection_id' $CONFIG_FILE)

CHIHUAHUA_BINARY="chihuahuad"
CHIHUAHUA_CHAIN_ID_FLAG="--chain-id $CHIHUAHUA_CHAIN_ID"
KEYRING_TEST_FLAG="--keyring-backend test"
TX_FLAG="--gas auto --gas-adjustment 1.3"
CHIHUAHUA_NODE_FLAG="--node $CHIHUAHUA_NODE"
CHIHUAHUA_TX_FLAGS="$TX_FLAG --gas-prices 3uhuahua --chain-id $CHIHUAHUA_CHAIN_ID $CHIHUAHUA_NODE_FLAG $KEYRING_TEST_FLAG -y"


HUAHUA_FACTORY_WASM_PATH="./artifacts/huahua_factory.wasm"
BONDING_CURVE_WASM_PATH="./artifacts/bonding_curve.wasm"

HUAHUA_FACTORY_CODE_ID=""
BONDING_CURVE_CODE_ID=""

HUAHUA_FACTORY_SC_LABEL="HuahuaFactory"
BONDING_CURVE_SC_LABEL="BondingCurve"

store_bonding_curve() {
    error_handler() {
        echo "Content of store_bonding_curve_res.json:"
        cat ./store_bonding_curve_res.json
    }
    trap error_handler ERR

    echo 'Storing Bonding Curve wasm...'
    echo "PATH=$PATH"
    which $CHIHUAHUA_BINARY
    chihuahuad tx wasm store $BONDING_CURVE_WASM_PATH --from $TX_SENDER_WALLET $CHIHUAHUA_TX_FLAGS --output json &> ./store_bonding_curve_res.json
    sleep 10

    STORE_BONDING_CURVE_TX_HASH=$(grep -o '{.*}' ./store_bonding_curve_res.json | jq -r '.txhash')
    $CHIHUAHUA_BINARY q tx $STORE_BONDING_CURVE_TX_HASH $CHIHUAHUA_NODE_FLAG --output json &> ./store_bonding_curve_tx.json
    BONDING_CURVE_CODE_ID=$(jq -r '.events[] | select(.type == "store_code") | .attributes[] | select(.key == "code_id") | .value' ./store_bonding_curve_tx.json)
}

store_huahua_factory() {
    error_handler() {
        echo "Content of store_huahua_factory_res.json:"
        cat ./store_huahua_factory_res.json 
    }
    trap error_handler ERR

    echo 'Storing HuahuaFactory wasm...'

    $CHIHUAHUA_BINARY tx wasm store $HUAHUA_FACTORY_WASM_PATH --from $TX_SENDER_WALLET $CHIHUAHUA_TX_FLAGS --output json &> ./store_huahua_factory_res.json
    sleep 10

    STORE_HUAHUA_FACTORY_TX_HASH=$(grep -o '{.*}' ./store_huahua_factory_res.json | jq -r '.txhash')
    $CHIHUAHUA_BINARY q tx $STORE_HUAHUA_FACTORY_TX_HASH $CHIHUAHUA_NODE_FLAG --output json &> ./store_huahua_factory_tx.json
    HUAHUA_FACTORY_CODE_ID=$(jq -r '.events[] | select(.type == "store_code") | .attributes[] | select(.key == "code_id") | .value' ./store_huahua_factory_tx.json)
}

instantiate_huahua_factory() {
    error_handler() {
        echo "Content of instantiate_huahua_factory_res.json:"
        cat ./instantiate_huahua_factory_res.json
    }
    trap error_handler ERR

    echo 'Instantiating Huahua factory contract...'

    INIT_HUAHUA_FACTORY='{"bonding_curve_code_id":'$BONDING_CURVE_CODE_ID',"fee_swap_collector_address":"chihuahua1vv7gk7zccrq5zkezv6dsawjlrfykglvm5hc7hy", "reserve_collector_address":"chihuahua17xpfvakm2amg962yls6f84z3kell8c5lnvww2l"}'

    $CHIHUAHUA_BINARY tx wasm instantiate $HUAHUA_FACTORY_CODE_ID "$INIT_HUAHUA_FACTORY" --admin $TX_SENDER_ADDRESS --label $HUAHUA_FACTORY_SC_LABEL --from $TX_SENDER_WALLET $CHIHUAHUA_TX_FLAGS --output json &> ./instantiate_huahua_factory_res.json
    sleep 10

    INSTANTIATE_HUAHUA_FACTORY_TX_HASH=$(grep -o '{.*}' ./instantiate_huahua_factory_res.json | jq -r '.txhash')
    $CHIHUAHUA_BINARY q tx $INSTANTIATE_HUAHUA_FACTORY_TX_HASH $CHIHUAHUA_NODE_FLAG --output json &> ./instantiate_huahua_factory_tx.json
    export HUAHUA_FACTORY_CONTRACT_ADDRESS=$(jq -r '.events[] | select(.type == "instantiate") | .attributes[] | select(.key == "_contract_address") | .value' ./instantiate_huahua_factory_tx.json)


}


store_bonding_curve
store_huahua_factory

echo 'BondingCurve code ID:' $BONDING_CURVE_CODE_ID
echo 'HuahuaFactory code ID:' $HUAHUA_FACTORY_CODE_ID

instantiate_huahua_factory


echo 'HuahuaFactory contract address:' $HUAHUA_FACTORY_CONTRACT_ADDRESS


echo 'HuahuaFactory instantiated successfully!'