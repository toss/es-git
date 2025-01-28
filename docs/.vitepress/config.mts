export default defineConfig({
  ...shared,
  locales: {
    root: { label: 'English', ...en },
    ko: { label: '한국어', ...ko },
    zh_hans: { label: '简体中文', ...zh_hans },
    ja: { label: '日本語', ...ja },
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