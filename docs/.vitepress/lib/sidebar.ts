import fs from 'node:fs';
import path from 'node:path';
import type { DefaultTheme } from 'vitepress';

export function getReferenceSidebarItems(docsRoot: string, lang?: string) {
  const jsonFilepath =
    lang != null
      ? path.join(docsRoot, lang, 'reference', 'typedoc-sidebar.json')
      : path.join(docsRoot, 'reference', 'typedoc-sidebar.json');
  const json = fs.readFileSync(jsonFilepath, 'utf8');
  return JSON.parse(json) as DefaultTheme.SidebarItem[];
}
