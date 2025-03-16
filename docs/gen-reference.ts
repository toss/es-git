import assert from 'node:assert';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import type {
  CommentTag,
  DeclarationReflection,
  ParameterReflection,
  ProjectReflection,
  Reflection,
  SignatureReflection,
  SomeType,
} from 'typedoc';
import * as typedoc from 'typedoc';

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
    ko: '반환값',
  },
  example: {
    en: 'Examples',
    ko: '예시',
  },
} as const;

function t(name: keyof typeof texts): string {
  return texts[name][lang];
}

function ref(text: string, target: number): string {
  const a = `\x02${text}:${target}\x02`;
  return a;
}

function code(text: string): string {
  return `\`${text}\``;
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
      const reflection = type.reflection;
      if (reflection != null) {
        return ref(type.name, reflection.id);
      }
      if (type.typeArguments == null || type.typeArguments.length === 0) {
        return `${type.name}`;
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

function genParameter(reflection: ParameterReflection): string {
  const name = reflection.name;
  const type = genType(reflection.type);
  return `- ${code(name)} (${code(type)}): ${genSummary(reflection)}`;
}

function genParameters(reflection: SignatureReflection): string | undefined {
  const parameters = reflection.parameters;
  if (parameters == null || parameters.length === 0) {
    return undefined;
  }
  return [`### ${t('parameters')}`, '', ...parameters.map(genParameter)].join('\n');
}

function genReturns(reflection: SignatureReflection): string | undefined {
  const returnsTag = reflection.comment?.blockTags?.find(x => x.tag === '@returns');
  if (returnsTag == null) {
    return undefined;
  }
  const returnsText = returnsTag.content.map(x => x.text).join(' ');
  const type = genType(reflection.type);
  return [`### ${t('returns')}`, '', type === 'void' ? returnsText : `(${code(type)}): ${returnsText}`].join('\n');
}

function genExample(reflection: SignatureReflection): string | undefined {
  const exampleTag = reflection.comment?.blockTags?.find(x => x.tag === '@example');
  if (exampleTag == null) {
    return undefined;
  }
  const exampleText = exampleTag.content.map(x => x.text).join('');
  return [`## ${t('example')}`, '', exampleText].join('\n');
}

function genBorder(): string {
  return '---';
}

function genFunctionDoc(reflection: DeclarationReflection): string {
  assert(reflection.kind === typedoc.ReflectionKind.Function);
  const sig = reflection.signatures?.[0];
  assert(sig != null);

  const summary = genSummary(sig);
  const signature = genSignature(sig);
  const parameters = genParameters(sig);
  const returns = genReturns(sig);
  const example = genExample(sig);

  return [
    `# ${reflection.name}`,
    summary,
    genBorder(),
    signature,
    parameters,
    returns,
    example != null ? [genBorder(), example] : undefined,
  ]
    .flat()
    .filter(x => x != null)
    .join('\n\n');
}

function genInterfaceDoc(reflection: DeclarationReflection): string {
  assert(reflection.kind === typedoc.ReflectionKind.Interface);
  const summary = genSummary(reflection);
}

function resolveRef(_: ProjectReflection, doc: string): string {
  // biome-ignore lint/suspicious/noControlCharactersInRegex: <explanation>
  const regexp = /\x02(.+)\x02/g;
  return doc.replaceAll(regexp, substring => {
    const replaced = substring.replaceAll('\x02', '');
    console.log(replaced);
    return replaced;
  });
}

function findCategory(reflection: DeclarationReflection): string | undefined {
  const categoryTag =
    reflection.comment?.blockTags.find(x => x.tag === '@category') ||
    reflection.signatures?.[0]?.comment?.blockTags?.find(x => x.tag === '@category');
  return categoryTag?.content.map(x => x.text).join('');
}

const dirname = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.join(dirname, '..');

const app = await typedoc.Application.bootstrap({
  entryPoints: [path.join(rootDir, 'index.d.ts')],
  blockTags: ['@signature'],
  tsconfig: path.join(rootDir, 'tsconfig.json'),
});
const project = await app.convert();
if (project == null) {
  throw new Error('cannot get project');
}
await app.generateJson(project, path.join(dirname, 'typedoc-generated.json'));

const types = project.children!.filter(x => x.kind === typedoc.ReflectionKind.Function && x.name === 'cloneRepository');
const doc = genFunctionDoc(types[0]!);
console.log(resolveRef(project, doc));
