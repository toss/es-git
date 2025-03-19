import { isNotNil } from 'es-toolkit';
import type { Language } from './lang';
import { t } from './texts';
import { indent } from './util';

export function escapeHtml(text = ''): string {
  return text
    .replaceAll(/&/g, '&amp;')
    .replaceAll(/</g, '&lt;')
    .replaceAll(/>/g, '&gt;')
    .replaceAll(/"/g, '&quot;')
    .replaceAll(/'/g, '&#39;')
    .replaceAll(/\n/g, '<br>')
    .replaceAll(/`([^`]+)`/g, '<code>$1</code>');
}

export function paramULHtml(children: string | string[]): string {
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
  lang: Language;
}

export function paramLIHtml({
  name,
  type,
  required = false,
  defaultValue,
  description,
  children,
  root = false,
  lang,
}: ParamLIHtmlOptions) {
  const separator = '&nbsp;·&nbsp;';
  const labels = [
    required ? `<span class="param-required">${t('required', lang)}</span>` : null,
    `<span class="param-type">${escapeHtml(type)}</span>`,
    defaultValue != null ? `<span class="param-default">${escapeHtml(defaultValue)}</span>` : null,
  ]
    .filter(isNotNil)
    .join(separator);
  const title = [name != null ? `<span class="param-name">${escapeHtml(name)}</span>` : null, labels]
    .filter(isNotNil)
    .join('');

  return [
    `<li class="param-li${root ? ' param-li-root' : ''}">`,
    indent(title),
    indent('<br>'),
    description != null ? indent(`<p class="param-description">${escapeHtml(description)}</p>`) : null,
    children != null ? indent(children) : null,
    '</li>',
  ]
    .filter(isNotNil)
    .join('\n');
}
