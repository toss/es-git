# buffer

Generates blame information from an in-memory buffer

## Signature

```ts
class Blame {
  buffer(buffer: Buffer): Blame;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">buffer</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Buffer</span>
    <br>
    <p class="param-description">Buffer containing file content to blame</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Blame</span>
    <br>
    <p class="param-description">A new Blame object for the buffer content</p>
  </li>
</ul>

## Examples

```ts
const buffer = Buffer.from('modified content');
const bufferBlame = blame.buffer(buffer);
```