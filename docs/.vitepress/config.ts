import { defineConfig } from 'vitepress';
import { en } from './en';
import { ko } from './ko';
import { shared } from './shared';

export default defineConfig({
  ...shared,
  locales: {
    root: { label: 'English', ...en },
    ko: { label: '한국어', ...ko },
  },
  ignoreDeadLinks: [/\/index/],
});
