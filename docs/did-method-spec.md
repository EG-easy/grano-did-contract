# Grano DID Method Specification

## DID Method Name

The namestring that shall identify this DID method is: `grn`

A DID that uses this method MUST begin with the following prefix: `did:grn`. Per the DID specification, this string
MUST be in lowercase. The remainder of the DID, after the prefix, is specified below.

## Method Specific Identifier

The method specific identifier is represented as the corresponding HEX-encoded grano address, prefixed with `grano1`.

    grn-did = "did:grn:" grano-specific-identifier
    grano-specific-identifier = grano-address
    granoo-address = "grano1" 38*HEXDIG

## CRUD Operation Definitions

### Create (Register)

In order to create a `grn` DID, a grano address, i.e., key pair, needs to be generated. At this point, no
interaction with the grano network is required. The registration is implicit as it is impossible to brute
force an grano address, i.e., guessing the private key for a given public key on the Elliptic Curve
(secp256k1). The holder of the private key is the entity identified by the DID.

The default DID document for an `did:grn<grano address>`, e.g.
`did:grn:grano1fp7rrdjn4rxjqt2x23kpju3t9rd5hdkf2f0yyq` with no transactions to the grano registry looks like this:

```json
{
  "@context": [
    "https://www.w3.org/ns/did/v1",
    "https://w3id.org/security/suites/secp256k1recovery-2020/v2"
  ],
  "id": "did:grn:grano1fp7rrdjn4rxjqt2x23kpju3t9rd5hdkf2f0yyq",
  "verificationMethod": [
    {
      "id": "did:grn:grano1fp7rrdjn4rxjqt2x23kpju3t9rd5hdkf2f0yyq#controller",
      "type": "EcdsaSecp256k1RecoveryMethod2020",
      "controller": "did:grn:grano1fp7rrdjn4rxjqt2x23kpju3t9rd5hdkf2f0yyq"
    }
  ],
  "authentication": [
    "did:grn:grano1fp7rrdjn4rxjqt2x23kpju3t9rd5hdkf2f0yyq#controller"
  ],
  "assertionMethod": [
    "did:grn:grano1fp7rrdjn4rxjqt2x23kpju3t9rd5hdkf2f0yyq#controller"
  ]
}
```

### Read (Resolve)

The DID document is built by using read only functions and contract events on the grano registry.

Any value from the registry that returns a grano address will be added to the `verificationMethod` array of the
DID document with type `EcdsaSecp256k1RecoveryMethod2020`.

Other verification relationships and service entries are added or removed by enumerating contract events (see below).

#### Controller Address

Each identifier always has a controller address. By default, it is the same as the identifier address, but the resolver
MUST check the read only contract message`controller(identifier address)` on the deployed grano contract.

This controller address MUST be represented in the DID document as a `verificationMethod` entry with the `id` set as the
DID being resolved and with the fragment `#controller` appended to it.
A reference to it MUST also be added to the `authentication` and `assertionMethod` arrays of the DID document.

#### Enumerating Contract Events to build the DID Document

The grano contract publishes two types of events for each identifier.

- `DIDControllerChanged` (indicating a change of `controller`)
- `DIDAttributeChanged`

If a change has ever been made for the grano address of an identifier the block number is stored in the
`changed` mapping of the contract.

The latest event can be efficiently looked up by checking for one of the 3 above events at that exact block.

The grano contract event contains a `previousChange` value which contains the block number of the previous change (if any).

To see all changes in history for an address use the following pseudo-code:

1. query `changed(identifier address)` on the grano contract to get the latest block where a change occurred.
2. If result is `null` return.
3. Filter for events for all the above types with the contracts address on the specified block.
4. If event has a previous change then go to 3

After building the history of events for an address, interpret each event to build the DID document like so:

##### Controller changes (`DIDOwnerChanged`)

When the controller address of a `did:grn` is changed, a `DIDControllerChanged` event is emitted.


```js
{
	type: 'wasm',
	attributes: [{
		key: '_contract_address',
		value: 'grano1rrrc5dtyy632tm0az2gqem96943er69vd2twra56xe7gr6y52wfqw7qqm8'
	}, {
		key: 'identifier',
		value: 'grano14fsulwpdj9wmjchsjzuze0k37qvw7n7a7l207u'
	}, {
		key: 'controller',
		value: 'grano1y0k76dnteklegupzjj0yur6pj0wu9e0z35jafv'
	}, {
		key: 'previousChange',
		value: '0'
	}]
}

```


The event data MUST be used to update the `#controller` entry in the `verificationMethod` array.
When resolving DIDs with publicKey identifiers, if the controller (`owner`) address is different from the corresponding
address of the publicKey, then the `#controllerKey` entry in the `verificationMethod` array MUST be omitted.

##### Non-Grano Attributes (`DIDAttributeChanged`)

Non-Grano keys, service endpoints etc. can be added using attributes. Attributes exist on the blockchain as
contract events of type `DIDAttributeChanged` and also latest state can be queried from the grano contract too.


```js
{
	type: 'wasm',
	attributes: [{
		key: '_contract_address',
		value: 'grano1napx452awu78vndg7t6nk26zhdct40wz7ha2r5t6a8hlv4a0lcmsnapnqc'
	}, {
		key: 'identifier',
		value: 'grano14fsulwpdj9wmjchsjzuze0k37qvw7n7a7l207u'
	}, {
		key: 'name',
		value: 'service.id'
	}, {
		key: 'value',
		value: '#github'
	}, {
		key: 'validTo',
		value: '1667825742.818354895'
	}, {
		key: 'previousChange',
		value: '0'
	}, {
		key: 'from',
		value: 'grano14fsulwpdj9wmjchsjzuze0k37qvw7n7a7l207u'
	}]
}
```

While any attribute can be stored, for the DID document we support adding to each of these sections of the DID document:

- Public Keys (Verification Methods)
- Service Endpoints

This design decision is meant to discourage the use of custom attributes in DID documents as they would be too easy to
misuse for storing personal user information on-chain.

### Update

The DID Document may be updated by invoking the relevant smart contract functions as defined by the grano standard.
This includes changes to the account owner, adding additional attributes.
description in the [contract documentation](./contract-spec.md).

These functions will trigger the respective CosmWasm events which are used to build the DID Document for a given
account as described in [Enumerating Contract Events to build the DID Document](#Enumerating-Contract-Events-to-build-the-DID-Document).

Some elements of the DID Document will be revoked automatically when their validity period expires. This includes the additional attributes. Please find a detailed description in the [contract documentation](./contract-spec.md). All attribute functions will trigger the respective CosmWasm events which are used to build the DID Document for a given identifier as described in [Enumerating Contract Events to build the DID Document](#Enumerating-Contract-Events-to-build-the-DID-Document).

### Delete (Revoke)

The `owner` property of the identifier MUST be set to `0x0`. Although, `0x0` is a valid grano address, this will
indicate the account has no owner which is a common approach for invalidation, e.g., tokens. To detect if the `owner` is
the `null` address, one MUST get the logs of the last change to the account and inspect if the `owner` was set to the
null address (`grano1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3z33a4`). It is impossible to make any other changes to the DID
document after such a change, therefore all preexisting keys and services MUST be considered revoked.

If the intention is to revoke all the signatures corresponding to the DID, this option MUST be used.

The DID resolution result for a deactivated DID has the following shape:

```json
{
  "didDocumentMetadata": {
    "deactivated": true
  },
  "didResolutionMetadata": {
    "contentType": "application/did+ld+json"
  },
  "didDocument": {
    "@context": "https://www.w3.org/ns/did/v1",
    "id": "<the deactivated DID>",
    "verificationMethod": [],
    "assertionMethod": [],
    "authentication": []
  }
}
```

#### Security considerations

- DID versioning
Applications MUST take precautions when using versioned DID URIs.
If a key is compromised and revoked then it can still be used to issue signatures on behalf of the "older" DID URI.
The use of versioned DID URIs is only recommended in some limited situations where the timestamp of signatures can also
be verified, where malicious signatures can be easily revoked, and where applications can afford to check for these
explicit revocations of either keys or signatures.
Wherever versioned DIDs are in use, it SHOULD be made obvious to users that they are dealing with potentially revoked
data.

## Reference Implementations

The code at [grano-did-resolver](https://github.com/EG-easy/grano-did-resolver) is intended to present a reference implementation of this DID method.

## Author

- [@EG-easy](https://github.com/EG-easy)

## References


**[1]** <https://www.w3.org/TR/did-core>
**[2]** <https://github.com/EG-easy/grano-did-resolver>
**[3]** <https://github.com/EG-easy/grano-did-client>
**[4]** <https://github.com/EG-easy/grano-did-event-exporter>
**[5]** <https://github.com/EG-easy/grano-did-node>
