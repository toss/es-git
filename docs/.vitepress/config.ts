import container from 'markdown-it-container';
import { defineConfig } from 'vitepress';
import { renderSandbox } from 'vitepress-plugin-sandpack';
import { en } from './en.js';
import { ko } from './ko';
import { shared } from './shared.js';

export default defineConfig({
  ...shared,
  locales: {
    root: { label: 'English', ...en },
    ko: { label: '한국어', ...ko },
  },
  markdown: {
    config(md) {
      md.use(container, 'sandpack', {
        render(tokens: any[], idx: number) {
          return renderSandbox(tokens, idx, 'sandpack');
        },
      });
    },
  },
});
