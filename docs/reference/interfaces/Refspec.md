[es-git](../globals.md) / Refspec

# Interface: Refspec

A structure to represent a git [refspec][1].

Refspecs are currently mainly accessed/created through a `Remote`.

[1]: http://git-scm.com/book/en/Git-Internals-The-Refspec

## Properties

| Property | Type |
| ------ | ------ |
| <a id="direction"></a> `direction` | [`Direction`](../type-aliases/Direction.md) |
| <a id="src"></a> `src` | `string` |
| <a id="dst"></a> `dst` | `string` |
| <a id="force"></a> `force` | `boolean` |
