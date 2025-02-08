import fs from 'node:fs/promises';
import path from 'node:path';
import { cloneDeep } from 'es-toolkit';
import { execa } from 'execa';

await execa('yarn', ['exec', 'typedoc'], {
  cwd: import.meta.dirname,
  stdio: 'inherit',
});

const src = path.join(import.meta.dirname, 'api');
await fs.rm(path.join(src, '_media'), { recursive: true });
await fs.rm(path.join(src, 'index.md'));

const sidebarRaw = await fs.readFile(path.join(src, 'typedoc-sidebar.json'), 'utf8');
const sidebar = JSON.parse(sidebarRaw);

function prefixSidebarLink(items, prefix) {
  const cloned = cloneDeep(items);
  for (const item of cloned) {
    if (item.link != null) {
      item.link = path.join(prefix, item.link);
    }
    if (item.items != null) {
      item.items = prefixSidebarLink(item.items, prefix);
    }
  }
  return cloned;
}

const otherLanguages = ['ko'];

for (const lang of otherLanguages) {
  console.log(`copy "/api" -> "/${lang}/api"`);
  const dest = path.join(import.meta.dirname, lang, 'api');
  await fs.rm(dest, { recursive: true, force: true });
  await fs.mkdir(path.dirname(dest), { recursive: true });
  await fs.cp(src, dest, { recursive: true });
  await fs.writeFile(
    path.join(dest, 'typedoc-sidebar.json'),
    JSON.stringify(prefixSidebarLink(sidebar, `/${lang}`)),
    'utf8'
  );
}
