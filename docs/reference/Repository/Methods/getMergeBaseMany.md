# getMergeBaseMany

Find a merge base given a list of commits

This behaves similar to [`git merge-base`](https://git-scm.com/docs/git-merge-base#_discussion).
Given three commits `a`, `b`, and `c`, `getMergeBaseMany([a, b, c])`
will compute a hypothetical commit `m`, which is a merge between `b`
and `c`.

For example, with the following topology:
```text
       o---o---o---o---C
      /
     /   o---o---o---B
    /   /
---2---1---o---o---o---A
```

the result of `getMergeBaseMany([a, b, c])` is 1. This is because the
equivalent topology with a merge commit `m` between `b` and `c` would
is:
```text
       o---o---o---o---o
      /                 \
     /   o---o---o---o---M
    /   /
---2---1---o---o---o---A
```

and the result of `getMergeBaseMany([a, m])` is 1.

---

If you're looking to recieve the common merge base between all the
given commits, use `getMergeBaseOctopus`.

## Signature

```ts
class Repository {
  getMergeBaseMany(oids: string[]): string;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oids</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string[]</span>
    <br>
    <p class="param-description">Oids of the commits.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">The OID of a merge base considering all the commits.</p>
  </li>
</ul>