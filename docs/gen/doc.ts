import { isNotNil } from 'es-toolkit';
import { escapeHtml, paramLIHtml, paramULHtml } from './html';
import type { Language } from './lang';
import { t } from './texts';

export interface ParameterDoc {
  name: string;
  type: string;
  required?: boolean;
  description?: string;
  children?: string | ParameterDoc[];
}

export interface ReturnsDoc {
  type: string;
  description?: string;
  children?: string | ParameterDoc[];
}

export interface ReferenceDoc {
  name: string;
  summary?: string;
  signature?: string;
  parameters?: ParameterDoc[];
  returns?: ReturnsDoc;
  errors?: string[];
  examples?: string;
}

function renderParameterDoc(doc: ParameterDoc, lang: Language, root = true): string {
  const { name, type, required, description, children } = doc;
  return paramLIHtml({
    name,
    type,
    required,
    description,
    children:
      typeof children === 'string'
        ? `<p class="param-description">${escapeHtml(children)}</p>`
        : Array.isArray(children) && children.length > 0
          ? paramULHtml(children.map(x => renderParameterDoc(x, lang, false)))
          : undefined,
    root,
    lang,
  });
}

export function renderReferenceDoc(doc: ReferenceDoc, lang: Language): string {
  const { name, summary, signature, parameters, returns, errors, examples } = doc;

  const signatureDoc = signature != null ? [`## ${t('signature', lang)}`, '', signature].join('\n') : null;

  const parametersDocs =
    parameters != null && parameters.length > 0
      ? [
          `### ${t('parameters', lang)}`,
          '',
          paramULHtml(parameters.map(parameter => renderParameterDoc(parameter, lang))),
        ].join('\n')
      : null;

  const returnsDoc =
    returns != null
      ? [
          `### ${t('returns', lang)}`,
          '',
          paramULHtml(
            paramLIHtml({
              required: false,
              type: returns.type,
              description: returns.description,
              children:
                typeof returns.children === 'string'
                  ? `<p class="param-description">${escapeHtml(returns.children)}</p>`
                  : Array.isArray(returns.children) && returns.children.length > 0
                    ? paramULHtml(returns.children.map(x => renderParameterDoc(x, lang, false)))
                    : undefined,
              root: true,
              lang,
            })
          ),
        ].join('\n')
      : null;
  const errorsDoc =
    errors != null && errors.length > 0
      ? [
          `### ${t('errors', lang)}`,
          '',
          paramULHtml(
            errors.map(error =>
              paramLIHtml({
                required: false,
                type: 'Error',
                description: error,
                root: true,
                lang,
              })
            )
          ),
        ].join('\n')
      : null;
  const examplesDoc = examples != null ? [`## ${t('examples', lang)}`, '', examples].join('\n') : null;
  return [`# ${name}`, summary, signatureDoc, parametersDocs, returnsDoc, errorsDoc, examplesDoc]
    .filter(isNotNil)
    .join('\n\n');
}
