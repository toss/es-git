import path from 'node:path';
import { type DefaultTheme, defineConfig } from 'vitepress';
import { getAPISidebarItems } from './lib/sidebar';

const docsRoot = path.resolve(import.meta.dirname, '..');

export const en = defineConfig({
  lang: 'en',
  description: 'TODO',
  themeConfig: {
    nav: nav(),
    sidebar: sidebar(),
    editLink: {
      pattern: 'https://github.com/toss/es-git/edit/main/docs/:path',
      text: 'Edit this page on GitHub',
    },
    footer: {
      message: 'Released under the MIT License.',
      copyright: `Copyright © ${new Date().getFullYear()} Viva Republica, Inc.`,
    },
  },
});

function nav(): DefaultTheme.NavItem[] {
  return [
    { text: 'Home', link: '/' },
    { text: 'Introduction', link: '/intro' },
  ];
}

function sidebar(): DefaultTheme.Sidebar {
  return [
    {
      text: 'Guide',
      items: [
        { text: 'Introduction', link: '/intro' },
        { text: 'Installation & Usage', link: '/usage' },
      ],
    },
    {
      text: 'API',
      collapsed: true,
      items: getAPISidebarItems(docsRoot),
    },
  ];
}
