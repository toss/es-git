// TODO: Migrate to https://docs.grit.io/

const options = {
  quote: 'single',
  trailingComma: false,
};

/**
 * @param {import('jscodeshift').FileInfo} file
 * @param {import('jscodeshift').API} api
 * @returns {string}
 */
export default function transform(file, { j }) {
  if (!file.path.endsWith('.d.ts')) {
    return file.source;
  }
  let source = file.source;
  source = transformStringEnums(source, j, ['CredentialType']);
  source = transformCredentialUnion(source, j);
  return source;
}

/**
 * Transform string enums to literal type
 * @param {string} source
 * @param {import('jscodeshift').API.j} j
 * @param {string[]} ignore
 * @returns {string}
 */
function transformStringEnums(source, j, ignore) {
  return j(source)
    .find(j.TSEnumDeclaration, node => {
      const ignored = ignore.includes(node.id.name);
      const isStringEnum = node.members.some(member => typeof member.initializer.value === 'string');
      return !ignored && isStringEnum;
    })
    .replaceWith(path => {
      const node = j.tsTypeAliasDeclaration.from({
        comments: path.node.comments ?? null,
        id: j.identifier(path.node.id.name),
        typeAnnotation: j.tsUnionType(
          path.node.members.map(member => {
            const type = j.tsLiteralType.from({
              comments: member.comments ?? null,
              literal: j.stringLiteral(member.initializer.value),
            });
            return type;
          })
        ),
      });
      return node;
    })
    .toSource(options);
}

/**
 * Transform `Credential` as union type
 * @param {string} source
 * @param {import('jscodeshift').API.j} j
 * @returns {string}
 */
function transformCredentialUnion(source, j) {
  let modified = source;
  modified = j(source)
    .find(
      j.ExportNamedDeclaration,
      node => node.declaration.type === 'TSEnumDeclaration' && node.declaration.id.name === 'CredentialType'
    )
    .remove()
    .toSource(options);

  modified = j(modified)
    .find(j.ExportNamedDeclaration, node => {
      return (
        node.declaration.id.name === 'Credential' &&
        (node.declaration.type === 'TSInterfaceDeclaration' || node.declaration.type === 'TSTypeAliasDeclaration')
      );
    })
    .replaceWith(path => {
      const node = j.exportNamedDeclaration.from({
        comments: path.node.comments ?? null,
        declaration: j.tsTypeAliasDeclaration.from({
          id: j.identifier('Credential'),
          typeAnnotation: j.tsUnionType([
            // Default
            j.tsTypeLiteral.from({
              members: [
                j.tsPropertySignature(
                  j.identifier('type'),
                  j.tsTypeAnnotation(j.tsLiteralType(j.stringLiteral('Default')))
                ),
              ],
            }),
            // SSHKeyFromAgent
            j.tsTypeLiteral.from({
              members: [
                j.tsPropertySignature(
                  j.identifier('type'),
                  j.tsTypeAnnotation(j.tsLiteralType(j.stringLiteral('SSHKeyFromAgent')))
                ),
                j.tsPropertySignature(j.identifier('username'), j.tsTypeAnnotation(j.tsStringKeyword()), true),
              ],
            }),
            // SSHKeyFromPath
            j.tsTypeLiteral.from({
              members: [
                j.tsPropertySignature(
                  j.identifier('type'),
                  j.tsTypeAnnotation(j.tsLiteralType(j.stringLiteral('SSHKeyFromPath')))
                ),
                j.tsPropertySignature(j.identifier('username'), j.tsTypeAnnotation(j.tsStringKeyword()), true),
                j.tsPropertySignature(j.identifier('publicKeyPath'), j.tsTypeAnnotation(j.tsStringKeyword()), true),
                j.tsPropertySignature(j.identifier('privateKeyPath'), j.tsTypeAnnotation(j.tsStringKeyword())),
                j.tsPropertySignature(j.identifier('passphrase'), j.tsTypeAnnotation(j.tsStringKeyword()), true),
              ],
            }),
            // SSHKey
            j.tsTypeLiteral.from({
              members: [
                j.tsPropertySignature(
                  j.identifier('type'),
                  j.tsTypeAnnotation(j.tsLiteralType(j.stringLiteral('SSHKey')))
                ),
                j.tsPropertySignature(j.identifier('username'), j.tsTypeAnnotation(j.tsStringKeyword()), true),
                j.tsPropertySignature(j.identifier('publicKey'), j.tsTypeAnnotation(j.tsStringKeyword()), true),
                j.tsPropertySignature(j.identifier('privateKey'), j.tsTypeAnnotation(j.tsStringKeyword())),
                j.tsPropertySignature(j.identifier('passphrase'), j.tsTypeAnnotation(j.tsStringKeyword()), true),
              ],
            }),
            // Plain
            j.tsTypeLiteral.from({
              members: [
                j.tsPropertySignature(
                  j.identifier('type'),
                  j.tsTypeAnnotation(j.tsLiteralType(j.stringLiteral('Plain')))
                ),
                j.tsPropertySignature(j.identifier('username'), j.tsTypeAnnotation(j.tsStringKeyword()), true),
                j.tsPropertySignature(j.identifier('password'), j.tsTypeAnnotation(j.tsStringKeyword())),
              ],
            }),
          ]),
        }),
      });
      return node;
    })
    .toSource(options);

  return modified;
}

export const parser = 'ts';
