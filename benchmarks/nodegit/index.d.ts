/**
 * Since nodegit does not provide Node.js 22 version prebuilt, and the `node-gyp` fails to build when using python 3.12,
 * We are using `@figma/nodegit` which provides prebuilt so that the installation does not fail when `npm install`.
 */
export * from 'nodegit';
