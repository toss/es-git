# getMergeBaseMany

커밋 목록이 주어졌을 때 병합 베이스를 찾아요

이것은 [`git merge-base`](https://git-scm.com/docs/git-merge-base#_discussion)와 비슷하게 동작해요.
세 개의 커밋 `a`, `b`, `c`가 주어졌을 때, `getMergeBaseMany([a, b, c])`는
`b`와 `c` 사이의 병합인 가상의 커밋 `m`을 계산해요.

예를 들어, 다음과 같은 토폴로지에서:
```text
       o---o---o---o---C
      /
     /   o---o---o---B
    /   /
---2---1---o---o---o---A
```

`getMergeBaseMany([a, b, c])`의 결과는 1이에요. 이는 `b`와 `c` 사이의
병합 커밋 `m`이 있는 동등한 토폴로지가 다음과 같기 때문이에요:
```text
       o---o---o---o---o
      /                 \
     /   o---o---o---o---M
    /   /
---2---1---o---o---o---A
```

그리고 `getMergeBaseMany([a, m])`의 결과는 1이에요.

---

주어진 모든 커밋 사이의 공통 병합 베이스를 받으려면,
`getMergeBaseOctopus`를 사용하세요.

## 시그니처

```ts
class Repository {
  getMergeBaseMany(oids: string[]): string;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oids</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string[]</span>
    <br>
    <p class="param-description">커밋의 OID들</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">모든 커밋을 고려한 병합 베이스의 OID</p>
  </li>
</ul>