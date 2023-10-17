# PROXY=https://gateway.elrond.com
# CHAIN_ID="1"
PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"

# WALLET="./wallets/mainnet-shard1.pem"
# ADDRESS=$(erdpy data load --key=address-devnet)
WALLET="./wallets/devnet.pem"
ADDRESS=$(erdpy data load --key=address-devnet)

###############################################################################

CALLER_ADDRESS="erd149axj8feledcw7zck5f3ecwrncgd0gemcr9q69yxqlk0zvnl5zvs065jqu"
CALLER_ADDRESS_HEX="0x$(erdpy wallet bech32 --decode ${CALLER_ADDRESS})"
CALLER_ADDRESS_ONLY_HEX="$(erdpy wallet bech32 --decode ${CALLER_ADDRESS})"

TOKEN_ID="ZOG-481946"
TOKEN_ID_HEX="0x$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"
TOKEN_ID_ONLY_HEX="$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"

TOKEN_PRICE=500000000000000          # 0.0005 EGLD
PRESALE_GOAL_AMOUNT=100000000000    # 100000 EVLD

deploy() {
    erdpy --verbose contract deploy \
    --project=${PROJECT} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=100000000 \
    --arguments ${TOKEN_ID_HEX} ${TOKEN_PRICE} ${PRESALE_GOAL_AMOUNT} \
    --send \
    --metadata-payable \
    --outfile="deploy-devnet.interaction.json" \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} || return

    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-devnet --value=${ADDRESS}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

# UPGRADE_ADDRESS="erd1qqqqqqqqqqqqqpgqwrgwyc0ngwt8fqh7tdsx2295exk9fewzudlsa8n0w9"
# UPGRADE_ADDRESS_ONLY_HEX="$(erdpy wallet bech32 --decode ${UPGRADE_ADDRESS})"
UPGRADE_ADDRESS="erd1qqqqqqqqqqqqqpgql4nul2lhsy2z3d5pqgj0wmuva8fjwafh5zvs8qcytr"
UPGRADE_ADDRESS_ONLY_HEX="$(erdpy wallet bech32 --decode ${UPGRADE_ADDRESS})"

upgrade() {
    erdpy --verbose contract upgrade ${UPGRADE_ADDRESS_ONLY_HEX} --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --outfile="upgrade.json" --proxy=${PROXY} --chain=${CHAIN_ID} \
    --metadata-payable \
    --gas-limit=100000000 \
    --arguments ${TOKEN_ID_HEX} ${TOKEN_PRICE} ${PRESALE_GOAL_AMOUNT}
}

SWAP_TOKEN_ID="ZOG-481946"
SWAP_TOKEN_ID_HEX="0x$(echo -n ${SWAP_TOKEN_ID} | xxd -p -u | tr -d '\n')"
SWAP_TOKEN_ID_ONLY_HEX="$(echo -n ${SWAP_TOKEN_ID} | xxd -p -u | tr -d '\n')"

SWAP_NFT_TOKEN_ID="BABYORC-2ce93a"
SWAP_NFT_TOKEN_ID_HEX="0x$(echo -n ${SWAP_NFT_TOKEN_ID} | xxd -p -u | tr -d '\n')"
SWAP_NFT_TOKEN_ID_ONLY_HEX="$(echo -n ${SWAP_NFT_TOKEN_ID} | xxd -p -u | tr -d '\n')"

SWAP_PRICE=100000000          # 100 TRO

addNftPrice() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=10000000 \
    --function="addNftPrice" \
    --arguments ${SWAP_TOKEN_ID_HEX} ${SWAP_NFT_TOKEN_ID_HEX} ${SWAP_PRICE} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

buy() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --value=1000000000000000000 \
    --function="buy" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

withdraw() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="withdraw" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}