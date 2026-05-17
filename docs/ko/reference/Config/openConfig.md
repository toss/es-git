# openConfig

단일 온디스크 파일을 포함하는 새 구성 인스턴스를 만들어요.

## 시그니처

```ts
function openConfig(path: string): Config;
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">구성 파일 경로.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Config</span>
    <br>
    <p class="param-description">git 구성 키/값 저장소를 나타내는 Config 인스턴스.</p>
  </li>
</ul>