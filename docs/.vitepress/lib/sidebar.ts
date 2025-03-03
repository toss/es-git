import fs from 'node:fs';
import path from 'node:path';
import type { DefaultTheme } from 'vitepress';

export function getAPISidebarItems(docsRoot: string, lang?: string) {
  const jsonFilepath =
    lang != null
      ? path.join(docsRoot, lang, 'api', 'typedoc-sidebar.json')
      : path.join(docsRoot, 'api', 'typedoc-sidebar.json');
  const json = fs.readFileSync(jsonFilepath, 'utf8');
  return JSON.parse(json) as DefaultTheme.SidebarItem[];
}
