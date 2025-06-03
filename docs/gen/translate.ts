import type OpenAI from 'openai';
import type { ReferenceDoc } from './doc';
import type { Language } from './lang';

interface Options {
  /** @defaultValue 'gpt-4o' */
  model?: OpenAI.ChatModel;
  baseUrl?: string;
}

export async function translate(
  ai: OpenAI,
  doc: ReferenceDoc,
  lang: Language,
  options?: Options
): Promise<ReferenceDoc> {
  const prompt = `
Always answer in the JSON format as given in the input, without triple backticks.
Translate the following JSON to ${lang}.

If translating in Korean, write the sentence in 해요 style.
If translating in Korean, translate "Git Object" as "Git 개체".
If translating in Korean, translate "Tree" as "트리".
If translating in Korean, translate "Repository" as "리포지토리".
If translating in Japanese, finish the sentence in ます style.
Finish with a noun if it is a explanation for a parameter or a return value.

===
\`\`\`
${JSON.stringify(doc, null, 2)}
\`\`\`
  `;

  const response = await ai.chat.completions.create({
    model: options?.model ?? 'gpt-4o',
    messages: [{ role: 'user', content: prompt }],
  });

  const translatedItem = response.choices[0]?.message.content;
  if (translatedItem == null) {
    throw new Error(`API Error while translating ${doc.name} to ${lang}.`);
  }
  const translated = JSON.parse(translatedItem) as ReferenceDoc;
  return { ...translated, name: doc.name };
}
