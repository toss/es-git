import type { Language } from './lang';

const texts = {
  signature: {
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
  examples: {
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

export function t(key: keyof typeof texts, lang: Language): string {
  return texts[key][lang];
}
