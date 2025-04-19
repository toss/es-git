import { Option } from 'clipanion';
import { isEnum } from 'typanion';

/**
 * For contributors translating to new languages,
 * please enter the ISO-639 code of the language you wish to add here.
 */
const LANGUAGES = ['en', 'ko'] as const;

export type Language = (typeof LANGUAGES)[number];

export const LanguageOption = Option.String('-l,--lang', 'en', {
  description: 'Language to generate documentation',
  validator: isEnum(LANGUAGES),
});
