[es-git](../globals.md) / Reference

# 클래스: Reference

A structure to represent a git [reference][1].

[1]: http://git-scm.com/book/en/Git-Internals-Git-References

## 메소드

### delete()

> **delete**(): `void`

Delete an existing reference.

This method works for both direct and symbolic references. The reference
will be immediately removed on disk.

This function will return an error if the reference has changed from the
time it was looked up.

#### 반환 형식:

`void`

***

### isBranch()

> **isBranch**(): `boolean`

Check if a reference is a local branch.

#### 반환 형식:

`boolean`

***

### isNote()

> **isNote**(): `boolean`

Check if a reference is a note.

#### 반환 형식:

`boolean`

***

### isRemote()

> **isRemote**(): `boolean`

Check if a reference is a remote tracking branch

#### 반환 형식:

`boolean`

***

### isTag()

> **isTag**(): `boolean`

Check if a reference is a tag

#### 반환 형식:

`boolean`

***

### type()

> **type**(): `null` \| [`ReferenceType`](../type-aliases/ReferenceType.md)

Get the reference type of a reference.

If the type is unknown, then `null` is returned.

#### 반환 형식:

`null` \| [`ReferenceType`](../type-aliases/ReferenceType.md)

***

### name()

> **name**(): `string`

Get the full name of a reference.

Throws error if the name is not valid utf-8.

#### 반환 형식:

`string`

***

### shorthand()

> **shorthand**(): `string`

Get the full shorthand of a reference.

This will transform the reference name into a name "human-readable"
version. If no shortname is appropriate, it will return the full name.

Throws error if the shorthand is not valid utf-8.

#### 반환 형식:

`string`

***

### target()

> **target**(): `null` \| `string`

Get the OID pointed to by a direct reference.

Only available if the reference is direct (i.e. an object id reference,
not a symbolic one).

#### 반환 형식:

`null` \| `string`

***

### targetPeel()

> **targetPeel**(): `null` \| `string`

Return the peeled OID target of this reference.

This peeled OID only applies to direct references that point to a hard
Tag object: it is the result of peeling such Tag.

#### 반환 형식:

`null` \| `string`

***

### peelToTree()

> **peelToTree**(): [`Tree`](Tree.md)

Peel a reference to a tree

This method recursively peels the reference until it reaches
a tree.

#### 반환 형식:

[`Tree`](Tree.md)

***

### symbolicTarget()

> **symbolicTarget**(): `null` \| `string`

Get full name to the reference pointed to by a symbolic reference.

Only available if the reference is symbolic.

#### 반환 형식:

`null` \| `string`

***

### resolve()

> **resolve**(): [`Reference`](Reference.md)

Resolve a symbolic reference to a direct reference.

This method iteratively peels a symbolic reference until it resolves to
a direct reference to an OID.

If a direct reference is passed as an argument, a copy of that
reference is returned.

#### 반환 형식:

[`Reference`](Reference.md)

***

### rename()

> **rename**(`newName`, `options`?): [`Reference`](Reference.md)

Rename an existing reference.

This method works for both direct and symbolic references.

If the force flag is not enabled, and there's already a reference with
the given name, the renaming will fail.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `newName` | `string` |
| `options`? | `null` \| [`RenameReferenceOptions`](../interfaces/RenameReferenceOptions.md) |

#### 반환 형식:

[`Reference`](Reference.md)
