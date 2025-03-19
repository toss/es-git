export function indent(text: string, size = 2): string {
  const space = ' '.repeat(size);
  return text
    .split('\n')
    .map(line => `${space}${line}`)
    .join('\n');
}
