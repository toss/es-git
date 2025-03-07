import fs from 'node:fs/promises';
import path from 'node:path';
import { cloneDeep } from 'es-toolkit';
import { execa } from 'execa';

function prefixSidebarLink(items, prefix) {
  const cloned = cloneDeep(items);
  for (const item of cloned) {
    if (item.link != null) {
      item.link = item.link.replace(/^\/typedoc-generated/, '/reference');
      if (prefix != null) {
        item.link = path.join(prefix, item.link);
      }
    }
    if (item.items != null) {
      item.items = prefixSidebarLink(item.items, prefix);
    }
  }
  return cloned;
}

async function genTypedoc(lang) {
  const args = lang != null ? ['exec', 'typedoc', '--lang', lang] : ['exec', 'typedoc'];
  await execa('yarn', args, { cwd: import.meta.dirname, stdio: 'inherit' });

  const src = path.join(import.meta.dirname, 'typedoc-generated/');
  await fs.rm(path.join(src, '_media'), { recursive: true });
  await fs.rm(path.join(src, 'index.md'));

  const sidebarRaw = await fs.readFile(path.join(src, 'typedoc-sidebar.json'), 'utf8');
  const sidebar = JSON.parse(sidebarRaw);

  const dest =
    lang != null ? path.join(import.meta.dirname, lang, 'reference') : path.join(import.meta.dirname, 'reference');
  await fs.rm(dest, { recursive: true, force: true });
  await fs.mkdir(path.dirname(dest), { recursive: true });
  await fs.cp(src, dest, { recursive: true });
  await fs.writeFile(
    path.join(dest, 'typedoc-sidebar.json'),
    JSON.stringify(prefixSidebarLink(sidebar, lang != null ? `/${lang}` : undefined)),
    'utf8'
  );
}

const languages = [undefined, 'ko'];

for (const lang of languages) {
  await genTypedoc(lang);
}
