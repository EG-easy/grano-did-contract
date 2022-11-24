# Grano Contract Specification

## Execute Message
**ChangeController**
Sets the controller of the given identifier to another grano account.

```js
change_controller: {
  identifier: 'grano14fsulwpdj9wmjchsjzuze0k37qvw7n7a7l207u',
  new_controller: 'grano1y0k76dnteklegupzjj0yur6pj0wu9e0z35jafv',
}
```

**SetAttribute**
Sets an attribute with the given `name` and `value`, valid for `validity` seconds.


```js
set_attribute: {
  identifier: 'grano14fsulwpdj9wmjchsjzuze0k37qvw7n7a7l207u',
  name: 'service.serviceEndpoint',
  value: 'github.com/EG-easy',
  validity: 3600 * 24,
}
```

**RevokeAttribute**
Revokes an attribute.

```js
revoke_attribute: {
  identifier: 'grano14fsulwpdj9wmjchsjzuze0k37qvw7n7a7l207u',
  name: 'service.serviceEndpoint',
  value: 'github.com/EG-easy',
}
```

## Query Message
Returns the controller of the given identifier.

**Controller**
```js
controller: {
  identifier: 'grano14fsulwpdj9wmjchsjzuze0k37qvw7n7a7l207u'
}
```

**Attribute**
Returns the attributes of the given identifier and name.

```js
attribute: {
  identifier: 'grano14fsulwpdj9wmjchsjzuze0k37qvw7n7a7l207u',
  name: 'service',
}
```

**ValidTo**
Returns the validity of the given identifier, name, and value.

```js
valid_to: {
  identifier: 'grano14fsulwpdj9wmjchsjzuze0k37qvw7n7a7l207u',
  name: 'service',
  value: '{"id":"#github","type":"github","serviceEndpoint":"github.com/EG-easy"},',
}
```
