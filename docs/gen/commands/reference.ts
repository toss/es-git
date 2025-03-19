import fs from 'node:fs/promises';
import path from 'node:path';
import { Command, Option } from 'clipanion';
import { isNotNil } from 'es-toolkit';
import micromatch from 'micromatch';
import type { DeclarationReflection } from 'typedoc';
import { findRootDir } from '../fs';
import { LanguageOption } from '../lang';
import {
  findCategory,
  genErrors,
  genExample,
  genParameters,
  genReturns,
  genSignature,
  genSummary,
  runTypedoc,
} from '../typedoc';

export class ReferenceCommand extends Command {
  static paths = [['reference']];

  readonly patterns = Option.Array('-p,--pattern', []);
  readonly lang = LanguageOption;
  readonly withOutput = Option.Boolean('--with-output', true);

  async execute() {
    const rootDir = await findRootDir();
    const { app, project } = await runTypedoc({
      entryPoints: [path.join(rootDir, 'index.d.ts')],
      tsconfig: path.join(rootDir, 'tsconfig.json'),
    });
    if (this.withOutput) {
      await app.generateJson(project, path.join(rootDir, 'docs', 'typedoc-generated.json'));
    }
    const references: Array<{
      name: string;
      category: string[];
      doc: string;
    }> = [];
    this.traverseReflections(project.children!, reflection => {
      const category = findCategory(reflection);
      if (category == null) {
        return;
      }
      const fullpath = [...category, reflection.name].join('/');
      const matches =
        this.patterns.length > 0 ? this.patterns.some(pattern => micromatch.isMatch(fullpath, pattern)) : true;
      if (!matches) {
        return;
      }

      references.push({
        name: reflection.name,
        category,
        doc: this.genReferenceDoc(reflection),
      });
    });
    for (let i = 0; i < references.length; i += 1) {
      const reference = references[i]!;
      const filename =
        this.lang === 'en'
          ? path.join('reference', ...reference.category, `${reference.name}.md`)
          : path.join(this.lang, 'reference', ...reference.category, `${reference.name}.md`);
      const filepath = path.join(rootDir, 'docs', filename);
      await fs.mkdir(path.dirname(filepath), { recursive: true });
      await fs.writeFile(filepath, reference.doc, 'utf8');
      console.log(`[${i + 1}/${references.length}] ${filename} generated`);
    }
  }

  private traverseReflections(
    children: DeclarationReflection[],
    callback: (reflection: DeclarationReflection) => void
  ) {
    for (const child of children) {
      if (child.variant !== 'declaration') {
        continue;
      }
      callback(child);
      if (child.children != null && child.children.length > 0) {
        this.traverseReflections(child.children, callback);
      }
    }
  }

  private genReferenceDoc(reflection: DeclarationReflection) {
    const sig = reflection.signatures?.[0];
    if (sig == null) {
      throw new Error(`Signature not found: ${reflection.name}`);
    }

    const summary = genSummary(sig);
    const signature = genSignature(sig, { lang: this.lang });
    const parameters = genParameters(sig, { lang: this.lang });
    const returns = genReturns(sig, { lang: this.lang });
    const errors = genErrors(sig, { lang: this.lang });
    const example = genExample(sig, { lang: this.lang });

    return [`# ${reflection.name}`, summary, signature, parameters, returns, errors, example]
      .flat()
      .filter(isNotNil)
      .join('\n\n');
  }
}
