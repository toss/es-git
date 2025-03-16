import assert from 'node:assert';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import type { DeclarationReflection, ParameterReflection, Reflection, SignatureReflection, SomeType } from 'typedoc';
import * as typedoc from 'typedoc';
import fs from 'node:fs/promises';
import { isNotNil } from 'es-toolkit';

const dirname = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.join(dirname, '..');

const languages = ['en', 'ko'] as const;
type Language = (typeof languages)[number];

function isLanguage(lang: string): lang is Language {
  return languages.some(x => x === lang);
}

const [, , inputLang = 'en'] = process.argv;
if (!isLanguage(inputLang)) {
  throw new Error(`Unsupported language: ${inputLang}`);
}

const lang = inputLang;
const texts = {
  singature: {
    en: 'Signature',
    ko: '시그니처',
  },
  parameters: {
    en: 'Parameters',
    ko: '파라미터',
  },
  returns: {
    en: 'Returns',
    ko: '반환 값',
  },
  example: {
    en: 'Examples',
    ko: '예제',
  },
  required: {
    en: 'required',
    ko: '필수',
  },
  default: {
    en: 'Default',
    ko: '기본값',
  },
  errors: {
    en: 'Errors',
    ko: '에러',
  },
} as const;

function t(name: keyof typeof texts): string {
  return texts[name][lang];
}

function indent(text: string, count = 1): string {
  const space = '  '.repeat(count);
  return text
    .split('\n')
    .map(line => `${space}${line}`)
    .join('\n');
}

function htmlEscape(text = ''): string {
  return text
    .replaceAll(/&/g, '&amp;')
    .replaceAll(/</g, '&lt;')
    .replaceAll(/>/g, '&gt;')
    .replaceAll(/"/g, '&quot;')
    .replaceAll(/'/g, '&#39;')
    .replaceAll(/\n/g, '<br>')
    .replaceAll(/`([^`]+)`/g, '<code>$1</code>');
}

function paramULHtml(children: string | string[]) {
  const childrenHtml = (Array.isArray(children) ? children : [children]).join('\n');
  return ['<ul class="param-ul">', indent(childrenHtml), '</ul>'].join('\n');
}

interface ParamLIHtmlOptions {
  name?: string;
  type: string;
  /** @defaultValue false */
  required?: boolean;
  defaultValue?: string;
  description?: string;
  children?: string;
  /** @defaultValue false */
  root?: boolean;
}

function paramLIHtml({
  name,
  type,
  required = false,
  defaultValue,
  description,
  children,
  root = false,
}: ParamLIHtmlOptions) {
  const separator = '&nbsp;·&nbsp;';
  const labels = [
    required ? `<span class="param-required">${t('required')}</span>` : null,
    `<span class="param-type">${htmlEscape(type)}</span>`,
    defaultValue != null ? `<span class="param-default">${htmlEscape(defaultValue)}</span>` : null,
  ]
    .filter(isNotNil)
    .join(separator);
  const title = [name != null ? `<span class="param-name">${htmlEscape(name)}</span>` : null, labels]
    .filter(isNotNil)
    .join('');

  return [
    `<li class="param-li${root ? ' param-li-root' : ''}">`,
    indent(title),
    indent('<br>'),
    description != null ? indent(`<p class="param-description">${htmlEscape(description)}</p>`) : null,
    children != null ? indent(children) : null,
    '</li>',
  ]
    .filter(isNotNil)
    .join('\n');
}

function genSummary(reflection: Reflection): string | undefined {
  return reflection.comment?.summary.map(x => x.text).join('');
}

function genSignature(reflection: SignatureReflection): string {
  const signatureTag = reflection.comment?.blockTags?.find(x => x.tag === '@signature');
  assert(signatureTag != null, '`@signature` tag not exists');

  return [`## ${t('singature')}`, '', ...signatureTag.content.map(x => x.text)].join('\n');
}

function genType(type?: SomeType): string {
  if (type == null) {
    return 'any';
  }
  switch (type.type) {
    case 'intrinsic':
      return type.name;
    case 'literal':
      return String(type.value);
    case 'reference': {
      if (type.reflection != null || type.typeArguments == null || type.typeArguments.length === 0) {
        return type.name;
      }
      return `${type.name}<${type.typeArguments.map(genType).join(', ')}>`;
    }
    case 'union':
      return type.types.map(genType).join(' | ');
    case 'unknown':
      return 'unknown';
    case 'array': {
      const single =
        type.elementType.type === 'intrinsic' ||
        type.elementType.type === 'literal' ||
        type.elementType.type === 'reference' ||
        type.elementType.type === 'unknown';
      return single ? `${genType(type.elementType)}[]` : `(${genType(type.elementType)})[]`;
    }
    case 'reflection': {
      if (type.declaration.variant === 'declaration') {
        const sig = type.declaration.signatures?.[0];
        if (sig == null) {
          return '';
        }
        const params = (sig.parameters ?? []).map(x => `${x.name}: ${genType(x.type)}`).join(', ');
        const returns = genType(sig.type);
        return `(${params}) => ${returns}`;
      }
      throw new Error('supports only declaration variant');
    }
    default:
      throw new Error(`unsupported type: ${type.type}`);
  }
}

function isDeclarationReflection(reflection: Reflection): reflection is DeclarationReflection {
  return reflection.variant === 'declaration';
}

function getChildReflectionOfParameter(type?: SomeType): Reflection | undefined {
  if (type == null) {
    return undefined;
  }
  const child =
    type.type === 'reference'
      ? type.reflection
      : type.type === 'union'
        ? type.types.find(x => x.type === 'reference')?.reflection
        : undefined;
  const kinds = [typedoc.ReflectionKind.Enum, typedoc.ReflectionKind.Interface, typedoc.ReflectionKind.TypeAlias];
  if (child != null && kinds.includes(child.kind)) {
    return child;
  }
  return undefined;
}

function genParameter(reflection: ParameterReflection | DeclarationReflection, root = false): string {
  const child = getChildReflectionOfParameter(reflection.type);
  const children =
    child != null && isDeclarationReflection(child) ? (child.children?.filter(isDeclarationReflection) ?? []) : [];
  const html = paramLIHtml({
    name: reflection.name,
    type: genType(reflection.type),
    required: !reflection.flags.isOptional,
    description: genSummary(reflection)?.replaceAll('\n', ' '),
    children:
      child != null && child.kind === typedoc.ReflectionKind.TypeAlias
        ? `<p class="param-description">${htmlEscape(genSummary(child))}</p>`
        : children.length > 0
          ? paramULHtml(children.map(x => genParameter(x)))
          : undefined,
    root,
  });
  return html;
}

function genParameters(reflection: SignatureReflection): string | undefined {
  const parameters = reflection.parameters;
  if (parameters == null || parameters.length === 0) {
    return undefined;
  }
  return [`### ${t('parameters')}`, '', paramULHtml(parameters.map(x => genParameter(x, true)))].join('\n');
}

function genReturns(reflection: SignatureReflection): string | undefined {
  const returnsTag = reflection.comment?.blockTags?.find(x => x.tag === '@returns');
  if (returnsTag == null) {
    return undefined;
  }
  const child = getChildReflectionOfParameter(reflection.type);
  const children =
    child != null && isDeclarationReflection(child) ? (child.children?.filter(isDeclarationReflection) ?? []) : [];
  const returnsText = returnsTag.content.map(x => x.text).join(' ');
  return [
    `### ${t('returns')}`,
    '',
    paramULHtml(
      paramLIHtml({
        required: false,
        type: genType(reflection.type),
        description: returnsText,
        children:
          child != null && child.kind === typedoc.ReflectionKind.TypeAlias
            ? `<p class="param-description">${htmlEscape(genSummary(child))}</p>`
            : children.length > 0
              ? paramULHtml(children.map(x => genParameter(x)))
              : undefined,
        root: true,
      })
    ),
  ].join('\n');
}

function genErrors(reflection: SignatureReflection): string | undefined {
  const throwsTag = reflection.comment?.blockTags?.find(x => x.tag === '@throws');
  if (throwsTag == null) {
    return undefined;
  }
  const errorsText = throwsTag.content.map(x => x.text).join(' ');
  return [
    `### ${t('errors')}`,
    '',
    paramULHtml(
      paramLIHtml({
        required: false,
        type: 'Error',
        description: errorsText,
        root: true,
      })
    ),
  ].join('\n');
}

function genExample(reflection: SignatureReflection): string | undefined {
  const exampleTag = reflection.comment?.blockTags?.find(x => x.tag === '@example');
  if (exampleTag == null) {
    return undefined;
  }
  const exampleText = exampleTag.content.map(x => x.text).join('');
  return [`## ${t('example')}`, '', exampleText].join('\n');
}

function genDoc(reflection: DeclarationReflection): string {
  const sig = reflection.signatures?.[0];
  assert(sig != null);

  const summary = genSummary(sig);
  const signature = genSignature(sig);
  const parameters = genParameters(sig);
  const returns = genReturns(sig);
  const errors = genErrors(sig);
  const example = genExample(sig);

  return [`# ${reflection.name}`, summary, signature, parameters, returns, errors, example]
    .flat()
    .filter(isNotNil)
    .join('\n\n');
}

function findCategory(reflection: DeclarationReflection): string[] | undefined {
  const categoryTag =
    reflection.comment?.blockTags.find(x => x.tag === '@category') ||
    reflection.signatures?.[0]?.comment?.blockTags?.find(x => x.tag === '@category');
  return categoryTag?.content
    .map(x => x.text.trim())
    .join('')
    .split('/');
}

function traverseReflections(children: DeclarationReflection[], callback: (reflection: DeclarationReflection) => void) {
  for (const child of children) {
    if (child.variant !== 'declaration') {
      continue;
    }
    callback(child);
    if (child.children != null && child.children.length > 0) {
      traverseReflections(child.children, callback);
    }
  }
}

async function run() {
  const app = await typedoc.Application.bootstrap({
    entryPoints: [path.join(rootDir, 'index.d.ts')],
    blockTags: ['@signature'],
    tsconfig: path.join(rootDir, 'tsconfig.json'),
    exclude: ['docs/**/*.ts'],
  });
  const project = await app.convert();
  if (project == null) {
    throw new Error('cannot get project');
  }
  await app.generateJson(project, path.join(dirname, 'typedoc-generated.json'));

  const references: Array<{
    name: string;
    category: string[];
    doc: string;
  }> = [];

  traverseReflections(project.children!, reflection => {
    const category = findCategory(reflection);
    if (category == null) {
      return;
    }
    references.push({
      name: reflection.name,
      category,
      doc: genDoc(reflection),
    });
  });

  for (let i = 0; i < references.length; i += 1) {
    const reference = references[i]!;
    const filename =
      inputLang === 'en'
        ? path.join('reference', ...reference.category, `${reference.name}.md`)
        : path.join(inputLang, 'reference', ...reference.category, `${reference.name}.md`);
    const filepath = path.join(dirname, filename);
    await fs.mkdir(path.dirname(filepath), { recursive: true });
    await fs.writeFile(filepath, reference.doc, 'utf8');
    console.log(`[${i + 1}/${references.length}] ${filename} generated`);
  }
}

await run();
