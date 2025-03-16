import path from 'node:path';
import { type DefaultTheme, defineConfig } from 'vitepress';
import { getReferenceSidebarItems } from './lib/sidebar';

const docsRoot = path.resolve(import.meta.dirname, '..');

export const ko = defineConfig({
  lang: 'ko',
  description: 'Node.js를 위한 현대적인 git 라이브러리',
  themeConfig: {
    nav: nav(),
    sidebar: sidebar(),
    editLink: {
      pattern: 'https://github.com/toss/es-git/edit/main/docs/:path',
      text: 'GitHub에서 수정하기',
    },
    footer: {
      message: 'MIT 라이선스에 따라 배포됩니다.',
      copyright: `Copyright © ${new Date().getFullYear()} Viva Republica, Inc.`,
    },
  },
});

function nav(): DefaultTheme.NavItem[] {
  return [
    { text: '홈', link: '/ko' },
    { text: '시작하기', link: '/ko/getting-started' },
    { text: '사용법', link: '/ko/usage/repository' },
    { text: '레퍼런스', link: '/ko/reference/Repository/openRepository' },
  ];
}

function sidebar(): DefaultTheme.Sidebar {
  return [
    {
      text: '가이드',
      items: [
        {
          text: '시작하기',
          link: '/ko/getting-started',
        },
        {
          text: '사용법',
          items: [
            { text: '리포지토리', link: '/ko/usage/repository' },
            { text: '리모트', link: '/ko/usage/remote' },
            { text: '커밋 히스토리', link: '/ko/usage/history' },
            { text: '커밋하기', link: '/ko/usage/commit' },
            { text: '태그', link: '/ko/usage/tag' },
          ],
        },
      ],
    },
    {
      text: '레퍼런스',
      items: getReferenceSidebarItems(docsRoot, 'ko'),
    },
  ];
}

export const search: DefaultTheme.LocalSearchOptions['locales'] = {
  ko: {
    translations: {
      button: {
        buttonText: '검색',
        buttonAriaLabel: '검색',
      },
      modal: {
        backButtonTitle: '뒤로가기',
        displayDetails: '더보기',
        footer: {
          closeKeyAriaLabel: '닫기',
          closeText: '닫기',
          navigateDownKeyAriaLabel: '아래로',
          navigateText: '이동',
          navigateUpKeyAriaLabel: '위로',
          selectKeyAriaLabel: '선택',
          selectText: '선택',
        },
        noResultsText: '검색 결과를 찾지 못했어요.',
        resetButtonTitle: '모두 지우기',
      },
    },
  },
};
