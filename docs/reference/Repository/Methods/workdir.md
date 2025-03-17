# workdir

Get the path of the working directory for this repository.

## Signature

```ts
class Repository {
  workdir(): string | null;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">The path of the working directory for this repository.<br>If this repository is bare, then  <code>null</code>  is returned.<br>```</p>
  </li>
</ul>