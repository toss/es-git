import { defineConfig } from 'vitepress';
import { en } from './en.js';
import { ko } from './ko';
import { shared } from './shared.js';

export default defineConfig({
  ...shared,
  locales: {
    root: { label: 'English', ...en },
    ko: { label: '한국어', ...ko },
  },
});
