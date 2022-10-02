# did-contract

did-contract is cosmwasm based did (decentralized identifier) implementation inspired by [ERC1056 DIDRegistry implementation](https://github.com/uport-project/ethr-did-registry/blob/develop/contracts/EthereumDIDRegistry.sol).

**WARNING**: did-contract is under VERY ACTIVE DEVELOPMENT and should be treated as pre-alpha software. This means it is not meant to be run in production, its APIs are subject to change without warning and should not be relied upon, and it should not be used to hold any value.

## Test Contract
```
sh scripts/check.sh
```

## Build Contract
```
sh scripts/build.sh
```

## Start local chain
```
sh scripts/start.sh
```

## Deploy & Interact with Contract
```
sh scripts/wasm.sh
```


## Msg Type

### Controller
```
{
	"controller": {
		"identifier": "wasm1fp7rrdjn4rxjqt2x23kpju3t9rd5hdkf2f0yyq"
	}
}
```

### Change Controller
```
{
	"change_controller": {
		"identifier": "wasm1fp7rrdjn4rxjqt2x23kpju3t9rd5hdkf2f0yyq",
		"new_controller": "wasm1rhhjhf4wmsu7nl07ftxk9h6ks3ffled9y2kn7y"
	}
}
```
