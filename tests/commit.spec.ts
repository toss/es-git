import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { isValidOid, openRepository } from '../index';
import { useFixture } from './fixtures';

describe('commit', () => {
  const signature = { name: 'Seokju Na', email: 'seokju.me@gmail.com' };

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
    expect([...revwalk][0]).toEqual(oid);
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
    const oid = repo.commit(tree, 'signed commit', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
      signature:
        '-----BEGIN PGP SIGNATURE-----\\nVersion: GnuPG v1\\n\\niQEcBAABAgAGBQJTest123\\n-----END PGP SIGNATURE-----',
    });
    expect(isValidOid(oid)).toBe(true);
    const signatureInfo = repo.extractSignature(oid);
    expect(signatureInfo).not.toBeNull();

    const { signature: extractedSignature = '', signedData = '' } = signatureInfo || {};

    expect(extractedSignature).toEqual(
      '-----BEGIN PGP SIGNATURE-----\\nVersion: GnuPG v1\\n\\niQEcBAABAgAGBQJTest123\\n-----END PGP SIGNATURE-----'
    );

    expect(signedData).toContain('tree ab9abf28de846b5968a8f12156f1d5ce3f4a198e');
    expect(signedData).toContain('parent a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    expect(signedData).toMatch(/author Seokju Na <seokju\.me@gmail\.com> \d+ \+0000/);
    expect(signedData).toMatch(/committer Seokju Na <seokju\.me@gmail\.com> \d+ \+0000/);
    expect(signedData).toContain('signed commit');
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
