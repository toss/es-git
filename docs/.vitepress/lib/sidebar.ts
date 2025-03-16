import path from 'node:path';
import type { DefaultTheme } from 'vitepress';
import glob from 'fast-glob';

export function getReferenceSidebarItems(docsRoot: string, lang?: string) {
  const files = glob.sync('**/*.md', {
    cwd: lang != null ? path.join(docsRoot, lang, 'reference') : path.join(docsRoot, 'reference'),
    onlyFiles: true,
  });
  files.sort((a, b) => {
    const aName = path.basename(a, '.md');
    const bName = path.basename(b, '.md');
    if (aName < bName) {
      return -1;
    }
    if (aName > bName) {
      return 1;
    }
    return 0;
  });

  const items: DefaultTheme.SidebarItem[] = [];
  for (const file of files) {
    const parts = file.split('/');
    const link = lang != null ? `/${lang}/reference/${file}` : `/reference/${file}`;
    let current = items;
    if (parts.length > 1) {
      for (const part of parts.slice(0, parts.length - 1)) {
        let node = current.find(x => x.text === part);
        if (node == null) {
          node = { text: part, items: [], collapsed: true };
          current.push(node);
        }
        current = node.items!;
      }
    }
    current.push({
      text: path.basename(file, '.md'),
      link: link.replace(path.extname(link), ''),
    });
  }
  sortItems(items);
  return items;
}

function sortItems(items: DefaultTheme.SidebarItem[]) {
  items.sort((a, b) => {
    const textDiff = a.text! < b.text! ? -1 : a.text! < b.text! ? 1 : 0;
    if (a.link != null && b.link == null) {
      return 1;
    }
    if (a.link == null && b.link != null) {
      return -1;
    }
    return textDiff;
  });
  for (const item of items) {
    if (item.items != null) {
      sortItems(item.items);
    }
  }
}
