import { createRequire } from 'node:module';
import path from 'node:path';
import { defineConfig } from 'vitepress';

const require = createRequire(import.meta.url);

export const shared = defineConfig({
  title: 'es-git',
  lastUpdated: true,
  metaChunk: true,
  head: [
    [
      'link',
      {
        rel: 'icon',
        type: 'image/png',
        sizes: '100x100',
        href: '/favicon-100x100.png',
      },
    ],
    [
      'link',
      {
        rel: 'stylesheet',
        href: 'https://static.toss.im/tps/main.css',
      },
    ],
    [
      'link',
      {
        rel: 'stylesheet',
        href: 'https://static.toss.im/tps/others.css',
      },
    ],
    [
      'script',
      {},
      `window.va = window.va || function () { (window.vaq = window.vaq || []).push(arguments); };`,
    ],
    [
      'script',
      {
        src: '/_vercel/insights/script.js',
        defer: 'true',
      },
    ],
    [
      'meta',
      {
        property: 'og:image',
        content: '/og.png',
      },
    ],
  ],
  themeConfig: {
    logo: {
      dark: '/logo_white.png',
      light: '/logo_black.png',
    },

    siteTitle: false,

    search: {
      provider: 'local',
      options: {
        locales: {
          ...koSearch,
          ...zh_hansSearch,
        },
      },
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/toss/es-git' },
      {
        icon: 'npm',
        link: 'https://www.npmjs.com/package/es-git',
        ariaLabel: 'npm',
      },
    ],
  },

  vite: {
    resolve: {
      alias: {
        vue: path.dirname(
          require.resolve('vue/package.json', {
            paths: [require.resolve('vitepress')],
          }),
        ),
        'vue/server-renderer': path.dirname(
          require.resolve('vue/server-renderer', {
            paths: [require.resolve('vitepress')],
          }),
        ),
      },
    },
  },
});
