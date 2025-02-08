import fs from 'node:fs';
import path from 'node:path';
import glob from 'fast-glob';
import type { DefaultTheme } from 'vitepress';

export function getSidebarItems(docsRoot: string, ...parts: string[]): DefaultTheme.SidebarItem[] {
  const files = glob.sync(path.join(docsRoot, ...parts, '*'));
  const paths = files.map(x => `/${path.relative(docsRoot, x)}`);

  return paths.map(p => {
    const filename = path.basename(p).replace(/\.md$/g, '');

    return {
      text: filename,
      link: p.replace(/\.md$/g, ''),
    };
  });
}

export function getAPISidebarItems(docsRoot: string, lang?: string) {
  const jsonFilepath =
    lang != null
      ? path.join(docsRoot, lang, 'api', 'typedoc-sidebar.json')
      : path.join(docsRoot, 'api', 'typedoc-sidebar.json');
  const json = fs.readFileSync(jsonFilepath, 'utf8');
  return JSON.parse(json) as DefaultTheme.SidebarItem[];
}
