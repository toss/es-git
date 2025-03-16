# message

Get the message of a tag.

## 시그니처

```ts
class Tag {
  message(): string | null;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">Returns  <code>null</code>  if there is no message or if it is not valid utf8.</p>
  </li>
</ul>