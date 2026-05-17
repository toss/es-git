# indexToWorkdir

인덱스의 파일과 작업 디렉터리의 파일 간의 차이점에 대한 자세한 정보에 액세스해요.

## 시그니처

```ts
class StatusEntry {
  indexToWorkdir(): DiffDelta | null;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">DiffDelta | null</span>
    <br>
    <p class="param-description">인덱스의 파일과 작업 디렉터리의 파일 간의 차이점.</p>
  </li>
</ul>