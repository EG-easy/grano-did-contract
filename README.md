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

### Query

#### controller

```json
{
	"controller": {
		"identifier": "grano14fsulwpdj9wmjchsjzuze0k37qvw7n7am3reev"
	}
}
```

#### attribute

```json
{
	"attribute": {
		"identifier": "grano14fsulwpdj9wmjchsjzuze0k37qvw7n7am3reev",
		"name": "service"
	}
}
```

#### validTo

```json
{
	"valid_to": {
		"identifier": "grano14fsulwpdj9wmjchsjzuze0k37qvw7n7am3reev",
		"name": "service",
		"value": "github"
	}
}
```

### Execute

#### changeController

```json
{
	"change_controller": {
		"identifier": "grano14fsulwpdj9wmjchsjzuze0k37qvw7n7am3reev",
		"new_controller": "grano1m2pz9nj72lj2yxnpcmxqwfwk50v35gq7wd399m"
	}
}
```

#### setAttribute

```json
{
	"set_attribute": {
		"identifier": "grano14fsulwpdj9wmjchsjzuze0k37qvw7n7am3reev",
		"name": "service",
		"value": "github",
		"validity": 1000
	}
}
```

#### revokeAttribute

```json
{
	"revoke_attribute": {
		"identifier": "grano14fsulwpdj9wmjchsjzuze0k37qvw7n7am3reev",
		"name": "service",
		"value": "github"
	}
}
```
