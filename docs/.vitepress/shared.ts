import { createRequire } from 'node:module';
import path from 'node:path';
import { defineConfig } from 'vitepress';
import { search as koSearch } from './ko';

const require = createRequire(import.meta.url);

export const shared = defineConfig({
  title: 'es-git',
  lastUpdated: true,
  metaChunk: true,
  head: [
    [
      'link',
      {
        rel: 'preconnect',
        href: 'https://static.toss.im',
        crossorigin: 'anonymous',
      },
    ],
    [
      'link',
      {
        rel: 'icon',
        type: 'image/png',
        sizes: '96x96',
        href: '/favicon-96x96.png',
      },
    ],
    [
      'link',
      {
        rel: 'stylesheet',
        fetchpriority: 'low',
        href: 'https://static.toss.im/tps/main.css',
        media: 'none',
        onload: "this.onload=null; this.media='all'",
        crossorigin: 'anonymous',
      },
    ],
    [
      'link',
      {
        rel: 'stylesheet',
        fetchpriority: 'low',
        href: 'https://static.toss.im/tps/others.css',
        media: 'none',
        onload: "this.onload=null; this.media='all'",
        crossorigin: 'anonymous',
      },
    ],
    ['script', {}, 'window.va = window.va || function () { (window.vaq = window.vaq || []).push(arguments); };'],
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
    [
      'meta',
      {
        name: 'twitter:image',
        content: '/og.png',
      },
    ],
    [
      'meta',
      {
        name: 'twitter:card',
        content: 'summary',
      },
    ],
  ],
  themeConfig: {
    logo: {
      dark: '/logo.png',
      light: '/logo.png',
    },
    search: {
      provider: 'local',
      options: {
        locales: {
          ...koSearch,
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
          })
        ),
        'vue/server-renderer': path.dirname(
          require.resolve('vue/server-renderer', {
            paths: [require.resolve('vitepress')],
          })
        ),
      },
    },
  },
  srcExclude: ['**/README.md']
});
