# submoduleStatus

Get the status for a submodule.

This looks at a submodule and tries to determine the status.  It
will return a combination of the `SubmoduleStatus` values.

## Signature

```ts
class Repository {
  submoduleStatus(name: string, ignore: SubmoduleIgnore): number;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">The name of the submodule.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">ignore</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">SubmoduleIgnore</span>
    <br>
    <p class="param-description">The ignore rules to follow.</p>
    <p class="param-description">Submodule ignore values<br><br>These values represent settings for the <code>submodule.$name.ignore</code><br>configuration value which says how deeply to look at the working<br>directory when getting the submodule status.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">number</span>
    <br>
    <p class="param-description">The combination of the  <code>SubmoduleStatus</code>  values.</p>
  </li>
</ul>

## Examples

```ts
import { openRepository, submoduleStatusContains, SubmoduleStatus } from 'es-git';

const repo = await openRepository('...');
const status = repo.submoduleStatus('mysubmodule', 'None');

console.log(
  submoduleStatusContains(status, SubmoduleStatus.InHead | SubmoduleStatus.InIndex)
); // true
```