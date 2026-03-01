# submoduleStatus

서브모듈의 상태를 가져와요.

이 함수는 서브모듈을 확인하고 상태를 판단하려고 해요. `SubmoduleStatus` 값들의 조합을 반환해요.

## 시그니처

```ts
class Repository {
  submoduleStatus(name: string, ignore: SubmoduleIgnore): number;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">서브모듈의 이름</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">ignore</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">SubmoduleIgnore</span>
    <br>
    <p class="param-description">따를 무시 규칙</p>
    <p class="param-description">서브모듈 ignore 값<br><br>이 값들은 <code>submodule.$name.ignore</code> 설정 값에 대한 설정을 나타내며, 서브모듈 상태를 가져올 때 작업 디렉터리를 얼마나 깊게 확인할지 말해요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">number</span>
    <br>
    <p class="param-description"><code>SubmoduleStatus</code> 값들의 조합</p>
  </li>
</ul>

## 예제

```ts
import { openRepository, submoduleStatusContains, SubmoduleStatus } from 'es-git';

const repo = await openRepository('...');
const status = repo.submoduleStatus('mysubmodule', 'None');

console.log(
  submoduleStatusContains(status, SubmoduleStatus.InHead | SubmoduleStatus.InIndex)
); // true
```