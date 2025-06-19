import fs from 'node:fs/promises';
import path from 'node:path';
import { Command, Option } from 'clipanion';
import micromatch from 'micromatch';
import OpenAI from 'openai';
import type { DeclarationReflection } from 'typedoc';
import { type ReferenceDoc, renderReferenceDoc } from '../doc';
import { findRootDir } from '../fs';
import { LanguageOption } from '../lang';
import { translate } from '../translate';
import { findCategory, getReferenceDoc, runTypedoc } from '../typedoc';

export class ReferenceCommand extends Command {
  static paths = [['reference']];

  readonly patterns = Option.Array('-p,--pattern', []);
  readonly lang = LanguageOption;
  readonly withOutput = Option.Boolean('--with-output', true);
  readonly translateAiToken = Option.String('--translate-ai-token', {
    description:
      'Translate documentation with OpenAI when the token is given. Only works when language options is not "en".',
    env: 'TRANSLATE_AI_TOKEN',
  });
  readonly translateAiModel = Option.String('--translate-ai-model', {
    description: 'AI model for when translating documentation with OpenAI. Default model is "gpt-4o".',
    env: 'TRANSLATE_AI_MODEL',
  });
  readonly translateAiBaseUrl = Option.String('--translate-ai-base-url', {
    description: 'Base url for Open AI.',
    env: 'TRANSLATE_AI_BASE_URL',
  });

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
      doc: ReferenceDoc;
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
        doc: getReferenceDoc(reflection),
      });
    });

    const ai =
      this.translateAiToken != null && this.lang !== 'en'
        ? new OpenAI({
            apiKey: this.translateAiToken,
            baseURL: this.translateAiBaseUrl,
          })
        : null;

    for (let i = 0; i < references.length; i += 1) {
      const reference = references[i]!;
      const filename =
        this.lang === 'en'
          ? path.join('reference', ...reference.category, `${reference.name}.md`)
          : path.join(this.lang, 'reference', ...reference.category, `${reference.name}.md`);
      const filepath = path.join(rootDir, 'docs', filename);
      await fs.mkdir(path.dirname(filepath), { recursive: true });
      const doc =
        ai != null
          ? await translate(ai, reference.doc, this.lang, { model: this.translateAiModel as OpenAI.ChatModel })
          : reference.doc;
      await fs.writeFile(filepath, renderReferenceDoc(doc, this.lang), 'utf8');
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
}
