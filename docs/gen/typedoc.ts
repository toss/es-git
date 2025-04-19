import assert from 'node:assert';
import type { DeclarationReflection, ParameterReflection, Reflection, SignatureReflection, SomeType } from 'typedoc';
import type { ParameterDoc, ReferenceDoc, ReturnsDoc } from './doc';

const ReflectionKind = {
  Enum: 8,
  Interface: 256,
  TypeAlias: 2097152,
} as const;

function getSummary(reflection: Reflection): string | undefined {
  return reflection.comment?.summary.map(x => x.text).join('');
}

function getSignature(reflection: SignatureReflection): string {
  const signatureTag = reflection.comment?.blockTags?.find(x => x.tag === '@signature');
  assert(signatureTag != null, '`@signature` tag not exists');

  return signatureTag.content.map(x => x.text).join('\n');
}

function formatType(type?: SomeType): string {
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
      return `${type.name}<${type.typeArguments.map(formatType).join(', ')}>`;
    }
    case 'union':
      return type.types.map(formatType).join(' | ');
    case 'unknown':
      return 'unknown';
    case 'array': {
      const single =
        type.elementType.type === 'intrinsic' ||
        type.elementType.type === 'literal' ||
        type.elementType.type === 'reference' ||
        type.elementType.type === 'unknown';
      return single ? `${formatType(type.elementType)}[]` : `(${formatType(type.elementType)})[]`;
    }
    case 'reflection': {
      if (type.declaration.variant === 'declaration') {
        const sig = type.declaration.signatures?.[0];
        if (sig == null) {
          return '';
        }
        const params = (sig.parameters ?? []).map(x => `${x.name}: ${formatType(x.type)}`).join(', ');
        const returns = formatType(sig.type);
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
  const kinds = [ReflectionKind.Enum, ReflectionKind.Interface, ReflectionKind.TypeAlias];
  if (child != null && kinds.includes(child.kind as any)) {
    return child;
  }
  return undefined;
}

function getParameterDoc(reflection: ParameterReflection | DeclarationReflection): ParameterDoc {
  const child = getChildReflectionOfParameter(reflection.type);
  const children =
    child != null && isDeclarationReflection(child) ? (child.children?.filter(isDeclarationReflection) ?? []) : [];
  const doc: ParameterDoc = {
    name: reflection.name,
    type: formatType(reflection.type),
    required: !reflection.flags.isOptional,
    description: getSummary(reflection)?.replaceAll('\n', ' '),
    children:
      child?.kind === ReflectionKind.TypeAlias
        ? getSummary(child)
        : children.length > 0
          ? children.map(x => getParameterDoc(x))
          : undefined,
  };
  return doc;
}

function getReturnsDoc(reflection: SignatureReflection): ReturnsDoc | undefined {
  const returnsTag = reflection.comment?.blockTags?.find(x => x.tag === '@returns');
  if (returnsTag == null) {
    return undefined;
  }
  const child = getChildReflectionOfParameter(reflection.type);
  const children =
    child != null && isDeclarationReflection(child) ? (child.children?.filter(isDeclarationReflection) ?? []) : [];
  const returnsText = returnsTag.content.map(x => x.text).join(' ');
  const doc: ReturnsDoc = {
    type: formatType(reflection.type),
    description: returnsText,
    children:
      child?.kind === ReflectionKind.TypeAlias
        ? getSummary(child)
        : children.length > 0
          ? children.map(x => getParameterDoc(x))
          : undefined,
  };
  return doc;
}

function getErrors(reflection: SignatureReflection): string[] | undefined {
  return reflection.comment?.blockTags
    ?.filter(tag => tag.tag === '@throws')
    .map(tag => tag.content.map(x => x.text).join(' '));
}

function getExamples(reflection: SignatureReflection): string | undefined {
  const examples = reflection.comment?.blockTags?.filter(x => x.tag === '@example');
  if (examples == null || examples.length === 0) {
    return undefined;
  }
  return examples.map(tag => tag.content.map(x => x.text).join('')).join('\n---\n');
}

export function getReferenceDoc(reflection: DeclarationReflection): ReferenceDoc {
  const sig = reflection.signatures?.[0];
  if (sig == null) {
    throw new Error(`Signature not found: ${reflection.name}`);
  }

  const doc: ReferenceDoc = {
    name: sig.name,
    summary: getSummary(sig),
    signature: getSignature(sig),
    parameters: sig.parameters?.map(x => getParameterDoc(x)),
    returns: getReturnsDoc(sig),
    errors: getErrors(sig),
    examples: getExamples(sig),
  };
  return doc;
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
