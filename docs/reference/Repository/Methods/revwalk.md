# revwalk

Create a revwalk that can be used to traverse the commit graph.

## Signature

```ts
class Repository {
  revwalk(): Revwalk;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Revwalk</span>
    <br>
    <p class="param-description">Revwalk to traverse the commit graph in this repository.</p>
  </li>
</ul>