# format

Prints this describe result, returning the result as a string.

## Signature

```ts
class Describe {
  format(options?: DescribeFormatOptions | null | undefined): string;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | DescribeFormatOptions</span>
    <br>
    <p class="param-description">Options for formatting describe.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">abbreviatedSize</span><span class="param-type">number</span>
        <br>
        <p class="param-description">Sets the size of the abbreviated commit id to use.  The value is the lower bound for the length of the abbreviated string, and the default is 7.</p>
      </li>
      <li class="param-li">
        <span class="param-name">alwaysUseLongFormat</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Sets whether or not the long format is used even when a shorter name could be used.</p>
      </li>
      <li class="param-li">
        <span class="param-name">dirtySuffix</span><span class="param-type">string</span>
        <br>
        <p class="param-description">If the workdir is dirty and this is set, this string will be appended to the description string.</p>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">Formatted string for this describe.</p>
  </li>
</ul>