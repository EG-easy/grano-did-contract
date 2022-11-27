NETWORK=grano-1
DAEMON=granod
HOME_DIR=~/.granod
CONFIG=~/.granod/config
TOKEN_DENOM=ugrano
MNEMONIC1='estate giraffe icon february goat observe actor left armed zone million note system myth coconut series calm steak dinosaur twin immune mansion morning drastic'
MNEMONIC2='dose amazing web fun patient cousin worth power blast video noodle cherry hope effort pulp buddy ugly tray help riot element figure combine cotton'

rm -rf $HOME_DIR

$DAEMON init $NETWORK --chain-id $NETWORK
echo $MNEMONIC1 | $DAEMON keys add eg1 --keyring-backend=test --recover
echo $MNEMONIC2 | $DAEMON keys add eg2 --keyring-backend=test --recover

$DAEMON add-genesis-account $($DAEMON keys show eg1 -a --keyring-backend=test) 100000000000000$TOKEN_DENOM

sed -i "s/\"stake\"/\"$TOKEN_DENOM\"/g" $HOME_DIR/config/genesis.json

$DAEMON gentx eg1 50000000000000$TOKEN_DENOM --chain-id=$NETWORK --commission-max-change-rate=0.1 --commission-max-rate=1 --commission-rate=0.1 --moniker=eg-validator --keyring-backend=test


$DAEMON collect-gentxs

$DAEMON validate-genesis

$DAEMON start
