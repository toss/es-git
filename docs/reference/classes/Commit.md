[es-git](../globals.md) / Commit

# Class: Commit

A structure to represent a git commit

## Methods

### id()

> **id**(): `string`

Get the id (SHA1) of a repository commit

#### Returns

`string`

***

### author()

> **author**(): [`Signature`](../interfaces/Signature.md)

Get the author of this commit.

#### Returns

[`Signature`](../interfaces/Signature.md)

***

### committer()

> **committer**(): [`Signature`](../interfaces/Signature.md)

Get the committer of this commit.

#### Returns

[`Signature`](../interfaces/Signature.md)

***

### message()

> **message**(): `string`

Get the full message of a commit.

The returned message will be slightly prettified by removing any
potential leading newlines.

Throws error if the message is not valid utf-8

#### Returns

`string`

***

### summary()

> **summary**(): `null` \| `string`

Get the short "summary" of the git commit message.

The returned message is the summary of the commit, comprising the first
paragraph of the message with whitespace trimmed and squashed.

Throws error if the summary is not valid utf-8.

#### Returns

`null` \| `string`

***

### body()

> **body**(): `null` \| `string`

Get the long "body" of the git commit message.

The returned message is the body of the commit, comprising everything
but the first paragraph of the message. Leading and trailing whitespaces
are trimmed.

Throws error if the summary is not valid utf-8.

#### Returns

`null` \| `string`

***

### time()

> **time**(): `Date`

Get the commit time (i.e. committer time) of a commit.

#### Returns

`Date`

***

### tree()

> **tree**(): [`Tree`](Tree.md)

Get the tree pointed to by a commit.

#### Returns

[`Tree`](Tree.md)

***

### asObject()

> **asObject**(): [`GitObject`](GitObject.md)

Casts this Commit to be usable as an `GitObject`

#### Returns

[`GitObject`](GitObject.md)
