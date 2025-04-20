import { describe, expect, it } from 'vitest';
import { Mailmap, createSignature, openRepository } from '../index';
import { useFixture } from './fixtures';

describe('mailmap', () => {
  it('should create mailmap from buffer', () => {
    const content = '# Comment line (ignored)\nSeokju Me <seokju.me@toss.im> Seokju Na <seokju.me@gmail.com>';
    const mailmap = Mailmap.fromBuffer(content);
    expect(mailmap).not.toBeNull();
  });

  it('should add entries to mailmap', () => {
    const content = '# Comment line (ignored)';
    const mailmap = Mailmap.fromBuffer(content);
    expect(mailmap).not.toBeNull();

    const result = mailmap!.addEntry('Seokju Me', 'seokju.me@toss.im', 'Seokju Na', 'seokju.me@gmail.com');
    expect(result).toBe(true);
  });

  it('should resolve mapped signatures with mailmap', () => {
    const content = 'Seokju Me <seokju.me@toss.im> Seokju Na <seokju.me@gmail.com>';
    const mailmap = Mailmap.fromBuffer(content);
    expect(mailmap).not.toBeNull();

    const originalSig = createSignature('Seokju Na', 'seokju.me@gmail.com');
    const resolvedSig = mailmap!.resolveSignature({
      name: originalSig.name,
      email: originalSig.email,
      timeOptions: { timestamp: originalSig.timestamp },
    });

    expect(resolvedSig).not.toBeNull();
    expect(resolvedSig!.name).toBe('Seokju Me');
    expect(resolvedSig!.email).toBe('seokju.me@toss.im');
  });

  it('should return original signature when not mapped', () => {
    const content = 'Seokju Me <seokju.me@toss.im> Seokju Na <seokju.me@gmail.com>';
    const mailmap = Mailmap.fromBuffer(content);
    expect(mailmap).not.toBeNull();

    const unmappedSig = createSignature('Another Seokju', 'another.seokju.me@gmail.com');
    const unmappedResolvedSig = mailmap!.resolveSignature({
      name: unmappedSig.name,
      email: unmappedSig.email,
      timeOptions: { timestamp: unmappedSig.timestamp },
    });

    expect(unmappedResolvedSig).not.toBeNull();
    expect(unmappedResolvedSig!.name).toBe('Another Seokju');
    expect(unmappedResolvedSig!.email).toBe('another.seokju.me@gmail.com');
  });

  it('should get mailmap from repository', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);

    const mailmap = repo.mailmap();
    expect(mailmap).not.toBeNull();
  });

  it('should get mailmap file from the dedicated mailmap fixture', async () => {
    const p = await useFixture('mailmap');
    const repo = await openRepository(p);

    const mailmap = repo.mailmap();
    expect(mailmap).not.toBeNull();

    const originalSignature = createSignature('Seokju Na', 'seokju.me@gmail.com');
    const mappedSignature = mailmap!.resolveSignature({
      name: originalSignature.name,
      email: originalSignature.email,
      timeOptions: { timestamp: originalSignature.timestamp },
    });

    expect(mappedSignature).not.toBeNull();
    expect(mappedSignature!.name).toBe('Seokju Me');
    expect(mappedSignature!.email).toBe('seokju.me@toss.im');
  });

  it('should apply mailmap to commit authors', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const commitId = 'a01e9888e46729ef4aa68953ba19b02a7a64eb82';
    const commit = repo.getCommit(commitId);

    const originalAuthor = commit.author();
    expect(originalAuthor.name).toBe('Seokju Na');
    expect(originalAuthor.email).toBe('seokju.me@gmail.com');

    const mappedName = 'Seokju Me';
    const mappedEmail = 'seokju.me@toss.im';
    const content = `${mappedName} <${mappedEmail}> ${originalAuthor.name} <${originalAuthor.email}>`;
    const mailmap = Mailmap.fromBuffer(content);

    const mappedAuthor = commit.authorWithMailmap(mailmap!);
    expect(mappedAuthor.name).toBe(mappedName);
    expect(mappedAuthor.email).toBe(mappedEmail);
    expect(mappedAuthor.name).not.toBe(originalAuthor.name);
    expect(mappedAuthor.email).not.toBe(originalAuthor.email);
  });

  it('should apply mailmap to commit committers', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const commitId = 'a01e9888e46729ef4aa68953ba19b02a7a64eb82';
    const commit = repo.getCommit(commitId);

    const originalCommitter = commit.committer();
    expect(originalCommitter.name).toBe('Seokju Na');
    expect(originalCommitter.email).toBe('seokju.me@gmail.com');

    const mappedName = 'Seokju Mapped';
    const mappedEmail = 'seokju.mapped@gmail.com';
    const content = `${mappedName} <${mappedEmail}> ${originalCommitter.name} <${originalCommitter.email}>`;
    const mailmap = Mailmap.fromBuffer(content);

    const mappedCommitter = commit.committerWithMailmap(mailmap!);
    expect(mappedCommitter.name).toBe(mappedName);
    expect(mappedCommitter.email).toBe(mappedEmail);
    expect(mappedCommitter.name).not.toBe(originalCommitter.name);
    expect(mappedCommitter.email).not.toBe(originalCommitter.email);
  });

  it('should preserve timestamps when applying mailmap', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const commitId = 'a01e9888e46729ef4aa68953ba19b02a7a64eb82';
    const commit = repo.getCommit(commitId);

    const originalAuthor = commit.author();
    const originalTimestamp = originalAuthor.timestamp;

    const content = `Seokju Mapped <seokju.mapped@gmail.com> ${originalAuthor.name} <${originalAuthor.email}>`;
    const mailmap = Mailmap.fromBuffer(content);

    const mappedAuthor = commit.authorWithMailmap(mailmap!);
    expect(mappedAuthor.timestamp).toBe(originalTimestamp);
  });
});
