# peel

Recursively peel a tag until a non tag Git object is found.

## 시그니처

```ts
class Tag {
  peel(): GitObject;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">GitObject</span>
    <br>
    <p class="param-description">Git object for this tag.</p>
  </li>
</ul>