import path from 'node:path';
import { type DefaultTheme, defineConfig } from 'vitepress';
import { getReferenceSidebarItems } from './lib/sidebar';

const docsRoot = path.resolve(import.meta.dirname, '..');

export const en = defineConfig({
  lang: 'en',
  title: 'es-git',
  description: 'Modern git library built for Node.js',
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
    { text: 'Getting Started', link: '/getting-started' },
    { text: 'Usage', link: '/usage/repository' },
    { text: 'Reference', link: '/reference/Repository/openRepository' },
  ];
}

function sidebar(): DefaultTheme.Sidebar {
  return [
    {
      text: 'Guides',
      items: [
        {
          text: 'Getting Started',
          link: '/getting-started',
        },
        {
          text: 'Performance',
          link: '/performance',
        },
        {
          text: 'Usage',
          items: [
            { text: 'Repository', link: '/usage/repository' },
            { text: 'Remotes', link: '/usage/remote' },
            { text: 'Diff', link: '/usage/diff' },
            { text: 'Commit History', link: '/usage/history' },
            { text: 'Commit Changes', link: '/usage/commit' },
            { text: 'Tags', link: '/usage/tag' },
          ],
        },
      ],
    },
    {
      text: 'Reference',
      items: getReferenceSidebarItems(docsRoot),
    },
  ];
}
