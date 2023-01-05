#!/bin/bash

NETWORK=grano-1
DAEMON=granod
HOME_DIR=~/.granod
CONFIG=~/.granod/config
TOKEN_DENOM=ugrano

# const
TXFLAG="--from=eg1 -y --keyring-backend=test --output=json --node=http://127.0.0.1:26657 --chain-id=$NETWORK --gas-prices 0.001$TOKEN_DENOM --gas auto --gas-adjustment 1.3"

# upload
RES=$($DAEMON tx wasm store target/wasm32-unknown-unknown/release/did_contract.wasm $TXFLAG)
sleep 6

TX=$(echo $RES | jq -r '.txhash')
CODE_ID=$($DAEMON query tx $TX --output=json | jq -r '.logs[0].events[1].attributes[1].value')
echo "-> CODE_ID: $CODE_ID"
$DAEMON query wasm list-contract-by-code $CODE_ID --node=http://127.0.0.1:26657 --output=json

# init
MSG='{}'
$DAEMON tx wasm instantiate $CODE_ID "$MSG" --label="contract1" --admin $($DAEMON keys show eg1 --keyring-backend=test -a) $TXFLAG
sleep 6
CONTRACT=$($DAEMON query wasm list-contract-by-code $CODE_ID --node=http://127.0.0.1:26657 --output=json | jq -r '.contracts[-1]')
echo "-> CONTRACT: $CONTRACT"

# functions
execute_contract () {
  $DAEMON tx wasm execute $CONTRACT "$1" $TXFLAG | jq
  sleep 6
}

query_contract () {
  $DAEMON query wasm contract-state smart $CONTRACT "$1" --output=json | jq
}

### Set Attribute ###
# query
ADDR=$($DAEMON keys show -a eg1 --keyring-backend=test)
MSG=$(echo '{ "attribute": { "identifier": "'$ADDR'", "name": "service" } }' | jq)

echo $MSG
query_contract "$MSG"

# execute
ADDR=$($DAEMON keys show -a eg1 --keyring-backend=test)
MSG=$(echo '{ "set_attribute": { "identifier": "'$ADDR'", "name": "service", "value": "github", "validity": 1000 } }' | jq)
echo $MSG
execute_contract "$MSG"

# query
ADDR=$($DAEMON keys show -a eg1 --keyring-backend=test)
MSG=$(echo '{ "attribute": { "identifier": "'$ADDR'", "name": "service" } }' | jq)
query_contract "$MSG"

# execute
ADDR=$($DAEMON keys show -a eg1 --keyring-backend=test)
MSG=$(echo '{ "set_attribute": { "identifier": "'$ADDR'", "name": "service", "value": "twitter", "validity": 1000 } }' | jq)
execute_contract "$MSG"

# query
ADDR=$($DAEMON keys show -a eg1 --keyring-backend=test)
MSG=$(echo '{ "attribute": { "identifier": "'$ADDR'", "name": "service" } }' | jq)
query_contract "$MSG"

# query
ADDR=$($DAEMON keys show -a eg1 --keyring-backend=test)
MSG=$(echo '{ "valid_to": { "identifier": "'$ADDR'", "name": "service", "value": "github" } }' | jq)
echo $MSG
query_contract "$MSG"


### Revoke Attribute ###
# execute
ADDR=$($DAEMON keys show -a eg1 --keyring-backend=test)
MSG=$(echo '{ "revoke_attribute": { "identifier": "'$ADDR'", "name": "service", "value": "github" } }' | jq)
echo $MSG
execute_contract "$MSG"

# query
ADDR=$($DAEMON keys show -a eg1 --keyring-backend=test)
MSG=$(echo '{ "valid_to": { "identifier": "'$ADDR'", "name": "service", "value": "github" } }' | jq)
query_contract "$MSG"

### Change Controller ###
# query
ADDR=$($DAEMON keys show -a eg1 --keyring-backend=test)
MSG=$(echo '{ "controller": { "identifier": "'$ADDR'" } }' | jq)
echo $MSG
query_contract "$MSG"

# execute
ADDR2=$($DAEMON keys show -a eg2 --keyring-backend=test)
MSG=$(echo '{"change_controller": {"identifier": "'$ADDR'", "new_controller": "'$ADDR2'"}}' | jq )
echo $MSG
execute_contract "$MSG"

# query
ADDR=$($DAEMON keys show -a eg1 --keyring-backend=test)
MSG=$(echo '{ "controller": { "identifier": "'$ADDR'" } }' | jq)
query_contract "$MSG"

# query
ADDR=$($DAEMON keys show -a eg1 --keyring-backend=test)
MSG=$(echo '{ "changed": { "identifier": "'$ADDR'" } }' | jq)
query_contract "$MSG"

# contract version
$DAEMON query wasm contract-state raw $CONTRACT 636F6E74726163745F696E666F --output=json | jq  -r .data | base64 -d | jq
