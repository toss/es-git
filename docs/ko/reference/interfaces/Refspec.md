[es-git](../globals.md) / Refspec

# 인터페이스: Refspec

A structure to represent a git [refspec][1].

Refspecs are currently mainly accessed/created through a `Remote`.

[1]: http://git-scm.com/book/en/Git-Internals-The-Refspec

## 속성

| 속성 | 유형 |
| ------ | ------ |
| <a id="direction"></a> `direction` | [`Direction`](../type-aliases/Direction.md) |
| <a id="src"></a> `src` | `string` |
| <a id="dst"></a> `dst` | `string` |
| <a id="force"></a> `force` | `boolean` |
