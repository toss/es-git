import assert from 'node:assert';
import type { DeclarationReflection, ParameterReflection, Reflection, SignatureReflection, SomeType } from 'typedoc';
import { escapeHtml, paramLIHtml, paramULHtml } from './html';
import type { Language } from './lang';
import { t } from './texts';

const ReflectionKind = {
  Enum: 8,
  Interface: 256,
  TypeAlias: 2097152,
} as const;

export function genSummary(reflection: Reflection): string | undefined {
  return reflection.comment?.summary.map(x => x.text).join('');
}

export function genSignature(reflection: SignatureReflection, options: { lang: Language }): string {
  const { lang } = options;
  const signatureTag = reflection.comment?.blockTags?.find(x => x.tag === '@signature');
  assert(signatureTag != null, '`@signature` tag not exists');

  return [`## ${t('singature', lang)}`, '', ...signatureTag.content.map(x => x.text)].join('\n');
}

export function genType(type?: SomeType): string {
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

export function isDeclarationReflection(reflection: Reflection): reflection is DeclarationReflection {
  return reflection.variant === 'declaration';
}

export function getChildReflectionOfParameter(type?: SomeType): Reflection | undefined {
  if (type == null) {
    return undefined;
  }
  const child =
    type.type === 'reference'
      ? type.reflection
      : type.type === 'union'
        ? type.types.find(x => x.type === 'reference')?.reflection
        : undefined;
  const kinds = [ReflectionKind.Enum, ReflectionKind.Interface, ReflectionKind.TypeAlias];
  if (child != null && kinds.includes(child.kind as any)) {
    return child;
  }
  return undefined;
}

export function genParameter(
  reflection: ParameterReflection | DeclarationReflection,
  options: { lang: Language; root?: boolean }
): string {
  const child = getChildReflectionOfParameter(reflection.type);
  const children =
    child != null && isDeclarationReflection(child) ? (child.children?.filter(isDeclarationReflection) ?? []) : [];
  const html = paramLIHtml({
    name: reflection.name,
    type: genType(reflection.type),
    required: !reflection.flags.isOptional,
    description: genSummary(reflection)?.replaceAll('\n', ' '),
    children:
      child != null && child.kind === ReflectionKind.TypeAlias
        ? `<p class="param-description">${escapeHtml(genSummary(child))}</p>`
        : children.length > 0
          ? paramULHtml(children.map(x => genParameter(x, { ...options, root: false })))
          : undefined,
    root: options.root,
    lang: options.lang,
  });
  return html;
}

export function genParameters(reflection: SignatureReflection, options: { lang: Language }): string | undefined {
  const parameters = reflection.parameters;
  if (parameters == null || parameters.length === 0) {
    return undefined;
  }
  return [
    `### ${t('parameters', options.lang)}`,
    '',
    paramULHtml(parameters.map(x => genParameter(x, { ...options, root: true }))),
  ].join('\n');
}

export function genReturns(reflection: SignatureReflection, options: { lang: Language }): string | undefined {
  const returnsTag = reflection.comment?.blockTags?.find(x => x.tag === '@returns');
  if (returnsTag == null) {
    return undefined;
  }
  const child = getChildReflectionOfParameter(reflection.type);
  const children =
    child != null && isDeclarationReflection(child) ? (child.children?.filter(isDeclarationReflection) ?? []) : [];
  const returnsText = returnsTag.content.map(x => x.text).join(' ');
  return [
    `### ${t('returns', options.lang)}`,
    '',
    paramULHtml(
      paramLIHtml({
        required: false,
        type: genType(reflection.type),
        description: returnsText,
        children:
          child != null && child.kind === ReflectionKind.TypeAlias
            ? `<p class="param-description">${escapeHtml(genSummary(child))}</p>`
            : children.length > 0
              ? paramULHtml(children.map(x => genParameter(x, options)))
              : undefined,
        root: true,
        lang: options.lang,
      })
    ),
  ].join('\n');
}

export function genErrors(reflection: SignatureReflection, options: { lang: Language }): string | undefined {
  const throwsTag = reflection.comment?.blockTags?.find(x => x.tag === '@throws');
  if (throwsTag == null) {
    return undefined;
  }
  const errorsText = throwsTag.content.map(x => x.text).join(' ');
  return [
    `### ${t('errors', options.lang)}`,
    '',
    paramULHtml(
      paramLIHtml({
        required: false,
        type: 'Error',
        description: errorsText,
        root: true,
        lang: options.lang,
      })
    ),
  ].join('\n');
}

export function genExample(reflection: SignatureReflection, options: { lang: Language }): string | undefined {
  const exampleTag = reflection.comment?.blockTags?.find(x => x.tag === '@example');
  if (exampleTag == null) {
    return undefined;
  }
  const exampleText = exampleTag.content.map(x => x.text).join('');
  return [`## ${t('example', options.lang)}`, '', exampleText].join('\n');
}

export function findCategory(reflection: DeclarationReflection): string[] | undefined {
  const categoryTag =
    reflection.comment?.blockTags.find(x => x.tag === '@category') ||
    reflection.signatures?.[0]?.comment?.blockTags?.find(x => x.tag === '@category');
  return categoryTag?.content
    .map(x => x.text.trim())
    .join('')
    .split('/');
}

interface RunTypedocOptions {
  entryPoints: string[];
  tsconfig?: string;
}

export async function runTypedoc({ entryPoints, tsconfig }: RunTypedocOptions) {
  const { Application } = await import('typedoc');
  const app = await Application.bootstrap({
    entryPoints,
    blockTags: ['@signature'],
    tsconfig,
    exclude: ['docs/**/*.ts'],
  });
  const project = await app.convert();
  if (project == null) {
    throw new Error('typedoc error: project is not generated');
  }
  return { app, project } as const;
}
