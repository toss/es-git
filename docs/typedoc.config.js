/** @type {import('typedoc').TypeDocOptions & import('typedoc-plugin-markdown').PluginOptions} */
const config = {
  entryPoints: ['../index.d.ts'],
  tsconfig: '../tsconfig.json',
  out: './typedoc-generated',
  plugin: ['typedoc-plugin-markdown', 'typedoc-vitepress-theme'],
  disableGit: true,
  disableSources: true,
  enumMembersFormat: 'table',
  interfacePropertiesFormat: 'table',
  typeDeclarationFormat: 'table',
  parametersFormat: 'table',
  cleanOutputDir: true,
  sort: ['enum-value-ascending'],
};

export default config;
