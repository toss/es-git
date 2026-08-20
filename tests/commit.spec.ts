import { createHash, generateKeyPairSync, sign, verify } from 'node:crypto';
import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { isValidOid, openRepository } from '../index';
import { useFixture } from './fixtures';

describe('commit', () => {
  const signature = { name: 'Seokju Na', email: 'seokju.me@gmail.com' };
  const fixedSignature = {
    ...signature,
    timeOptions: { timestamp: 1_700_000_000, offset: 0 },
  };
  const { privateKey, publicKey } = generateKeyPairSync('rsa', { modulusLength: 2048 });

  function oidForCommitContent(content: string) {
    return createHash('sha1')
      .update(`commit ${Buffer.byteLength(content)}\0${content}`)
      .digest('hex');
  }

  function signCommitContent(content: string) {
    const encoded = sign('sha256', Buffer.from(content, 'utf8'), privateKey).toString('base64');
    const body = encoded.match(/.{1,64}/g)?.join('\n') ?? encoded;
    return `-----BEGIN TEST SIGNATURE-----\n${body}\n-----END TEST SIGNATURE-----`;
  }

  function verifyCommitSignature(content: string, signature: string) {
    const encoded = signature.split('\n').slice(1, -1).join('');
    return verify('sha256', Buffer.from(content, 'utf8'), publicKey, Buffer.from(encoded, 'base64'));
  }

  it('get commit', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const commit = repo.getCommit('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    expect(commit.author().name).toEqual(signature.name);
    expect(commit.author().email).toEqual(signature.email);
    expect(commit.author().timestamp).toEqual(1732957216);
    expect(commit.message()).toEqual('second\n');
    expect(commit.summary()).toEqual('second');
    expect(commit.body()).toEqual(null);
    expect(commit.time().toISOString()).toEqual('2024-11-30T09:00:16.000Z');
  });

  it('returns null if oid of commit does not exists', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    const commit = repo.findCommit('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    expect(commit).toBeNull();
  });

  it('commit', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    await fs.writeFile(path.join(p, 'third'), 'third');
    const index = repo.index();
    index.addPath('third');
    index.write();
    const tree = repo.head().peelToTree();
    const oid = repo.commit(tree, 'test commit', {
      author: signature,
      committer: signature,
    });
    expect(isValidOid(oid)).toBe(true);
  });

  it('commit on head tree', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    await fs.writeFile(path.join(p, 'third'), 'third');
    const index = repo.index();
    index.addPath('third');
    const treeSha = index.writeTree();
    const tree = repo.getTree(treeSha);
    const oid = repo.commit(tree, 'test commit', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });
    expect(isValidOid(oid)).toBe(true);
    const revwalk = repo.revwalk();
    revwalk.pushHead();
    expect(revwalk.next()).toEqual(oid);
    const commit = repo.getCommit(oid);
    expect(commit.summary()).toEqual('test commit');
  });

  it('create signed commit', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    await fs.writeFile(path.join(p, 'signed'), 'signed');
    const index = repo.index();
    index.addPath('signed');
    const treeSha = index.writeTree();
    const tree = repo.getTree(treeSha);
    const parents = [repo.head().target()!];
    const content = repo.commitCreateBuffer(tree, 'signed commit', {
      author: fixedSignature,
      committer: fixedSignature,
      parents,
    });
    const externalSignature = signCommitContent(content);
    const oid = repo.commit(tree, 'signed commit', {
      author: fixedSignature,
      committer: fixedSignature,
      parents,
      signature: externalSignature,
    });
    expect(isValidOid(oid)).toBe(true);
    const signatureInfo = repo.extractSignature(oid);
    expect(signatureInfo).not.toBeNull();

    const { signature: extractedSignature = '', signedData = '' } = signatureInfo || {};

    expect(extractedSignature).toEqual(externalSignature);
    expect(signedData).toEqual(content);
    expect(verifyCommitSignature(signedData, extractedSignature)).toBe(true);
  });

  it('creates commit content for external signing without writing an object', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    await fs.writeFile(path.join(p, 'buffered'), 'buffered');
    const index = repo.index();
    index.addPath('buffered');
    const tree = repo.getTree(index.writeTree());

    const content = repo.commitCreateBuffer(tree, 'externally signed commit', {
      author: fixedSignature,
      committer: fixedSignature,
      parents: [repo.head().target()!],
    });

    expect(content).toContain('parent a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    expect(content).toContain('author Seokju Na <seokju.me@gmail.com> 1700000000 +0000');
    expect(content).toContain('committer Seokju Na <seokju.me@gmail.com> 1700000000 +0000');
    expect(content).toContain('externally signed commit');
    expect(repo.findCommit(oidForCommitContent(content))).toBeNull();
  });

  it('rejects an invalid explicit signature instead of using the repository default', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const tree = repo.head().peelToTree();

    expect(() =>
      repo.commitCreateBuffer(tree, 'invalid signature', {
        author: { name: 'invalid\0name', email: signature.email },
        committer: fixedSignature,
      })
    ).toThrow();
  });

  it('creates a signed commit from externally signed content', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    await fs.writeFile(path.join(p, 'externally-signed'), 'externally-signed');
    const index = repo.index();
    index.addPath('externally-signed');
    const tree = repo.getTree(index.writeTree());
    const content = repo.commitCreateBuffer(tree, 'externally signed commit', {
      author: fixedSignature,
      committer: fixedSignature,
      parents: [repo.head().target()!],
    });
    const externalSignature = signCommitContent(content);

    const oid = repo.commitSigned(content, externalSignature);

    expect(isValidOid(oid)).toBe(true);
    expect(oid).not.toEqual(oidForCommitContent(content));
    const signatureInfo = repo.extractSignature(oid);
    expect(signatureInfo).not.toBeNull();
    expect(signatureInfo?.signature).toEqual(externalSignature);
    expect(signatureInfo?.signedData).toEqual(content);
    expect(verifyCommitSignature(signatureInfo?.signedData ?? '', signatureInfo?.signature ?? '')).toBe(true);
  });

  it('extract signature from unsigned commit', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    await fs.writeFile(path.join(p, 'unsigned'), 'unsigned');
    const index = repo.index();
    index.addPath('unsigned');
    const treeSha = index.writeTree();
    const tree = repo.getTree(treeSha);
    const oid = repo.commit(tree, 'unsigned commit', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });
    expect(isValidOid(oid)).toBe(true);

    const signatureInfo = repo.extractSignature(oid);
    expect(signatureInfo).toBeNull();
  });
});
