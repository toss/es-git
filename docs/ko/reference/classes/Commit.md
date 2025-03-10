[es-git](../globals.md) / Commit

# 클래스: Commit

A class to represent a git commit.

## 메소드

### id()

> **id**(): `string`

Get the id (SHA1) of a repository commit

#### 반환 형식:

`string`

***

### author()

> **author**(): [`Signature`](../interfaces/Signature.md)

Get the author of this commit.

#### 반환 형식:

[`Signature`](../interfaces/Signature.md)

***

### committer()

> **committer**(): [`Signature`](../interfaces/Signature.md)

Get the committer of this commit.

#### 반환 형식:

[`Signature`](../interfaces/Signature.md)

***

### message()

> **message**(): `string`

Get the full message of a commit.

The returned message will be slightly prettified by removing any
potential leading newlines.

Throws error if the message is not valid utf-8.

#### 반환 형식:

`string`

***

### summary()

> **summary**(): `null` \| `string`

Get the short "summary" of the git commit message.

The returned message is the summary of the commit, comprising the first
paragraph of the message with whitespace trimmed and squashed.

Throws error if the summary is not valid utf-8.

#### 반환 형식:

`null` \| `string`

***

### body()

> **body**(): `null` \| `string`

Get the long "body" of the git commit message.

The returned message is the body of the commit, comprising everything
but the first paragraph of the message. Leading and trailing whitespaces
are trimmed.

Throws error if the summary is not valid utf-8.

#### 반환 형식:

`null` \| `string`

***

### time()

> **time**(): `Date`

Get the commit time (i.e. committer time) of a commit.

#### 반환 형식:

`Date`

***

### tree()

> **tree**(): [`Tree`](Tree.md)

Get the tree pointed to by a commit.

#### 반환 형식:

[`Tree`](Tree.md)

***

### asObject()

> **asObject**(): [`GitObject`](GitObject.md)

Casts this Commit to be usable as an `GitObject`.

#### 반환 형식:

[`GitObject`](GitObject.md)
