# extractSignature

ID로 식별되는 개체에서 서명을 추출해요.

이 메서드는 커밋이나 태그와 같이 서명될 수 있는 모든 개체에 사용할 수 있어요.

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
    <p class="param-description">서명을 추출할 서명된 개체의 개체 ID (SHA1)</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | ExtractedSignature</span>
    <br>
    <p class="param-description">개체가 서명된 경우 서명과 서명된 데이터를 포함하는 ExtractedSignature 개체,<br>         또는 개체가 서명되지 않은 경우 null</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">signature</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">커밋의 GPG 서명, 또는 커밋이 서명되지 않은 경우 null</p>
      </li>
      <li class="param-li">
        <span class="param-name">signedData</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">커밋의 서명된 데이터</p>
      </li>
    </ul>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const commit = repo.getCommit('a01e9888e46729ef4aa68953ba19b02a7a64eb82');

// 커밋에서 서명 추출
const signatureInfo = repo.extractSignature(commit.id());

if (signatureInfo) {
  console.log('개체가 서명되었습니다!');
  console.log('서명:', signatureInfo.signature);
  console.log('서명된 데이터:', signatureInfo.signedData);
} else {
  console.log('개체가 서명되지 않았습니다');
}
```