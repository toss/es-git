import { createHash, generateKeyPairSync, sign, verify } from 'node:crypto';
import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { initRepository, isValidOid, openRepository } from '../index';
import { useFixture } from './fixtures';
import { makeTmpDir } from './tmp';

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
    const parents = [repo.head().target()!];
    const content = repo.commitCreateBuffer(tree, 'signed commit', {
      author: fixedSignature,
      committer: fixedSignature,
      parents,
    });
    const externalSignature = signCommitContent(content);
    const oid = repo.commit(tree, 'signed commit', {
      updateRef: 'HEAD',
      author: fixedSignature,
      committer: fixedSignature,
      parents,
      signature: externalSignature,
    });
    expect(isValidOid(oid)).toBe(true);
    expect(repo.head().target()).toEqual(oid);
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
