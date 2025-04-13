# extractSignature

서명된 Git 객체(커밋, 태그 등)에서 서명을 추출해요.

## 시그니처

```ts
class Repository {
  extractSignature(oid: string): ExtractedSignature | null;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oid</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">서명을 추출할 서명된 객체의 ID(SHA1)예요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">ExtractedSignature | null</span>
    <br>
    <p class="param-description">객체가 서명된 경우 서명과 서명된 데이터를 포함하는 ExtractedSignature 객체를 반환하고, 객체가 서명되지 않은 경우 null을 반환해요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">signature</span><span class="param-type">string</span>
        <br>
        <p class="param-description">Git 객체의 GPG 서명이에요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">signedData</span><span class="param-type">string</span>
        <br>
        <p class="param-description">Git 객체의 서명된 데이터예요.</p>
      </li>
    </ul>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const commit = repo.getCommit('a01e9888e46729ef4aa68953ba19b02a7a64eb82');

// 커밋에서 서명 추출하기
const signatureInfo = repo.extractSignature(commit.id());

if (signatureInfo) {
  console.log('서명된 커밋이예요!');
  console.log('서명:', signatureInfo.signature);
  console.log('서명된 데이터:', signatureInfo.signedData);
} else {
  console.log('서명되지 않은 커밋이예요');
}
``` 