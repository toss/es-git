import path from 'node:path';
import { fileURLToPath } from 'node:url';
import jsdoc from 'jsdoc-api';

const dirname = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.join(dirname, '..', '..');

const result = await jsdoc.explain({
  files: [path.join(rootDir, 'index.d.ts')],
});
console.log(result);
