# openDefaultConfig

전역, XDG 및 시스템 구성 파일 열기

전역, XDG 및 시스템 구성 파일을 찾아 리포지토리 외부에서 기본 구성 데이터에 액세스할 때 사용할 수 있는 단일 우선 순위 구성 개체로 열어주는 유틸리티 래퍼예요.

## 시그니처

```ts
function openDefaultConfig(): Config;
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Config</span>
    <br>
    <p class="param-description">git 구성 키/값 저장소를 나타내는 Config 인스턴스.</p>
  </li>
</ul>