import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { initRepository, isValidOid, openRepository } from '../index';
import { useFixture } from './fixtures';
import { makeTmpDir } from './tmp';

describe('commit', () => {
  const signature = { name: 'Seokju Na', email: 'seokju.me@gmail.com' };
  const gpgSignature =
    '-----BEGIN PGP SIGNATURE-----\\nVersion: GnuPG v1\\n\\niQEcBAABAgAGBQJTest123\\n-----END PGP SIGNATURE-----';

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
    const oid = repo.commit(tree, 'signed commit', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
      signature: gpgSignature,
    });
    expect(isValidOid(oid)).toBe(true);
    expect(repo.head().target()).toEqual(oid);
    const signatureInfo = repo.extractSignature(oid);
    expect(signatureInfo).not.toBeNull();

    const { signature: extractedSignature = '', signedData = '' } = signatureInfo || {};

    expect(extractedSignature).toEqual(gpgSignature);

    expect(signedData).toContain('tree ab9abf28de846b5968a8f12156f1d5ce3f4a198e');
    expect(signedData).toContain('parent a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    expect(signedData).toMatch(/author Seokju Na <seokju\.me@gmail\.com> \d+ \+0000/);
    expect(signedData).toMatch(/committer Seokju Na <seokju\.me@gmail\.com> \d+ \+0000/);
    expect(signedData).toContain('signed commit');
  });

  it('signed commit records reflog entry', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    await fs.writeFile(path.join(p, 'signed'), 'signed');
    const index = repo.index();
    index.addPath('signed');
    const tree = repo.getTree(index.writeTree());
    const oid = repo.commit(tree, 'signed commit\n\nbody the reflog entry must not contain', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
      signature: gpgSignature,
    });
    const entry = repo.reflog('HEAD').get(0);
    expect(entry?.idNew()).toEqual(oid);
    expect(entry?.message()).toEqual('commit: signed commit');
  });

  it('signed commit updates an existing branch ref', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const tip = repo.head().target()!;
    await fs.writeFile(path.join(p, 'signed'), 'signed');
    const index = repo.index();
    index.addPath('signed');
    const tree = repo.getTree(index.writeTree());
    const oid = repo.commit(tree, 'signed commit on main', {
      updateRef: 'refs/heads/main',
      author: signature,
      committer: signature,
      parents: [tip],
      signature: gpgSignature,
    });
    expect(repo.findReference('refs/heads/main')?.target()).toEqual(oid);
    expect(repo.reflog('refs/heads/main').get(0)?.message()).toEqual('commit: signed commit on main');
  });

  it('create signed commit on unborn HEAD', async () => {
    const p = await makeTmpDir('signed-unborn');
    const repo = await initRepository(p, { initialHead: 'main' });
    await fs.writeFile(path.join(p, 'first'), 'first');
    const index = repo.index();
    index.addPath('first');
    const tree = repo.getTree(index.writeTree());
    const oid = repo.commit(tree, 'initial signed commit', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      signature: gpgSignature,
    });
    expect(repo.head().name()).toEqual('refs/heads/main');
    expect(repo.head().target()).toEqual(oid);
    expect(repo.reflog('HEAD').get(0)?.message()).toEqual('commit (initial): initial signed commit');
  });

  it('create signed commit updating a new ref', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const headBefore = repo.head().target()!;
    await fs.writeFile(path.join(p, 'signed'), 'signed');
    const index = repo.index();
    index.addPath('signed');
    const tree = repo.getTree(index.writeTree());
    const oid = repo.commit(tree, 'signed commit on new branch', {
      updateRef: 'refs/heads/sign-target',
      author: signature,
      committer: signature,
      parents: [headBefore],
      signature: gpgSignature,
    });
    expect(repo.findReference('refs/heads/sign-target')?.target()).toEqual(oid);
    expect(repo.head().target()).toEqual(headBefore);
  });

  it('signed commit rejects updateRef when first parent is not the current tip', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const headBefore = repo.head().target();
    await fs.writeFile(path.join(p, 'signed'), 'signed');
    const index = repo.index();
    index.addPath('signed');
    const tree = repo.getTree(index.writeTree());
    expect(() =>
      repo.commit(tree, 'orphaned signed commit', {
        updateRef: 'HEAD',
        author: signature,
        committer: signature,
        signature: gpgSignature,
      })
    ).toThrowError(/current tip is not the first parent/);
    const revwalk = repo.revwalk();
    revwalk.pushHead();
    revwalk.next();
    const older = revwalk.next()!;
    expect(() =>
      repo.commit(tree, 'orphaned signed commit', {
        updateRef: 'HEAD',
        author: signature,
        committer: signature,
        parents: [older],
        signature: gpgSignature,
      })
    ).toThrowError(/current tip is not the first parent/);
    expect(repo.head().target()).toEqual(headBefore);
  });

  it('signed merge commit records merge reflog message', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const revwalk = repo.revwalk();
    revwalk.pushHead();
    const tip = revwalk.next()!;
    const older = revwalk.next()!;
    await fs.writeFile(path.join(p, 'signed'), 'signed');
    const index = repo.index();
    index.addPath('signed');
    const tree = repo.getTree(index.writeTree());
    const oid = repo.commit(tree, 'signed merge commit', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [tip, older],
      signature: gpgSignature,
    });
    expect(repo.head().target()).toEqual(oid);
    expect(repo.reflog('HEAD').get(0)?.message()).toEqual('commit (merge): signed merge commit');
  });

  it('signed commit skips first-parent validation for a nonexistent ref', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const headBefore = repo.head().target();
    await fs.writeFile(path.join(p, 'signed'), 'signed');
    const index = repo.index();
    index.addPath('signed');
    const tree = repo.getTree(index.writeTree());
    const oid = repo.commit(tree, 'rootless signed commit', {
      updateRef: 'refs/heads/no-validate',
      author: signature,
      committer: signature,
      signature: gpgSignature,
    });
    expect(repo.findReference('refs/heads/no-validate')?.target()).toEqual(oid);
    expect(repo.head().target()).toEqual(headBefore);
  });

  it('signed commit rejects an invalid updateRef name like unsigned commits', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    await fs.writeFile(path.join(p, 'signed'), 'signed');
    const index = repo.index();
    index.addPath('signed');
    const tree = repo.getTree(index.writeTree());
    expect(() =>
      repo.commit(tree, 'signed commit', {
        updateRef: 'not a valid ref name',
        author: signature,
        committer: signature,
        parents: [repo.head().target()!],
        signature: gpgSignature,
      })
    ).toThrowError(/not valid/);
  });

  it('signed commit updates detached HEAD', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const tip = repo.head().target()!;
    repo.setHeadDetached(repo.getCommit(tip));
    await fs.writeFile(path.join(p, 'signed'), 'signed');
    const index = repo.index();
    index.addPath('signed');
    const tree = repo.getTree(index.writeTree());
    const oid = repo.commit(tree, 'detached signed commit', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [tip],
      signature: gpgSignature,
    });
    expect(repo.head().target()).toEqual(oid);
    expect(repo.findReference('refs/heads/main')?.target()).toEqual(tip);
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
