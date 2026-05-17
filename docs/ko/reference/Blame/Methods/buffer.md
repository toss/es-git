# buffer

메모리 내 버퍼에서 blame 정보를 생성해요

## 시그니처

```ts
class Blame {
  buffer(buffer: Buffer): Blame;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">buffer</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Buffer</span>
    <br>
    <p class="param-description">blame할 파일 내용을 담고 있는 버퍼</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Blame</span>
    <br>
    <p class="param-description">버퍼 내용에 대한 새로운 Blame 개체</p>
  </li>
</ul>

## 예제

```ts
const buffer = Buffer.from('modified content');
const bufferBlame = blame.buffer(buffer);
```