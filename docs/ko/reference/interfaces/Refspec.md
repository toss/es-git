[es-git](../globals.md) / Refspec

# 인터페이스: Refspec

A data object to represent a git [refspec][1].

Git [refspec][1]을 표현하는 데이터 객체에요.

Refspec은 주로 `Remote`를 통해 접근하거나 생성돼요.

[1]: https://git-scm.com/book/ko/v2/Git%ec%9d%98-%eb%82%b4%eb%b6%80-Refspec

## 속성

| 속성 | 유형 |
| ------ | ------ |
| <a id="direction"></a> `direction` | [`Direction`](../type-aliases/Direction.md) |
| <a id="src"></a> `src` | `string` |
| <a id="dst"></a> `dst` | `string` |
| <a id="force"></a> `force` | `boolean` |
