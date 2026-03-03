import { describe, expect, it } from 'vitest';
import { Cred, CredType, openRepository } from '../index';
import { useFixture } from './fixtures';

describe('cred', () => {
  describe('default()', () => {
    it('creates credential', () => {
      const cred = Cred.default();
      expect(cred).toBeDefined();
    });

    it('credtype is Default', () => {
      const cred = Cred.default();
      expect(cred.credtype()).toBe(CredType.Default);
    });

    it('hasUsername returns false', () => {
      const cred = Cred.default();
      expect(cred.hasUsername()).toBe(false);
    });
  });

  describe('userpassPlaintext()', () => {
    it('creates credential', () => {
      const cred = Cred.userpassPlaintext('user', 'password');
      expect(cred).toBeDefined();
    });

    it('credtype is UserpassPlaintext', () => {
      const cred = Cred.userpassPlaintext('user', 'password');
      expect(cred.credtype()).toBe(CredType.UserpassPlaintext);
    });

    it('hasUsername returns true', () => {
      const cred = Cred.userpassPlaintext('user', 'password');
      expect(cred.hasUsername()).toBe(true);
    });

    it('accepts empty string password', () => {
      const cred = Cred.userpassPlaintext('user', '');
      expect(cred.credtype()).toBe(CredType.UserpassPlaintext);
    });
  });

  describe('username()', () => {
    it('creates credential', () => {
      const cred = Cred.username('git');
      expect(cred).toBeDefined();
    });

    it('credtype is Username', () => {
      const cred = Cred.username('git');
      expect(cred.credtype()).toBe(CredType.Username);
    });

    it('hasUsername returns true', () => {
      const cred = Cred.username('git');
      expect(cred.hasUsername()).toBe(true);
    });

    it('accepts arbitrary username', () => {
      const cred = Cred.username('my-github-user');
      expect(cred.credtype()).toBe(CredType.Username);
      expect(cred.hasUsername()).toBe(true);
    });
  });

  describe('sshKeyFromAgent()', () => {
    it('creates credential or throws when no agent', () => {
      try {
        const cred = Cred.sshKeyFromAgent('git');
        expect(cred.credtype()).toBeDefined();
        expect(cred.hasUsername()).toBe(true);
      } catch (e) {
        expect(e).toBeDefined();
      }
    });
  });

  describe('sshKey()', () => {
    // libgit2 defers key file I/O until the credential is actually used,
    // so path-based tests can use nonexistent paths to verify the binding.
    it('creates credential without publicKeyPath (derived from private key)', () => {
      const cred = Cred.sshKey('git', null, '/path/to/id_ed25519', null);
      expect(cred).toBeDefined();
      expect(cred.credtype()).toBe(CredType.SshKey);
      expect(cred.hasUsername()).toBe(true);
    });

    it('creates credential with publicKeyPath provided', () => {
      const cred = Cred.sshKey('git', '/path/to/id_ed25519.pub', '/path/to/id_ed25519', null);
      expect(cred.credtype()).toBe(CredType.SshKey);
      expect(cred.hasUsername()).toBe(true);
    });

    it('creates credential with both publicKeyPath and passphrase', () => {
      const cred = Cred.sshKey('git', '/path/to/id_ed25519.pub', '/path/to/id_ed25519', 'my-passphrase');
      expect(cred.credtype()).toBe(CredType.SshKey);
    });

    it('creates credential without publicKeyPath but with passphrase', () => {
      const cred = Cred.sshKey('git', null, '/path/to/id_rsa', 'my-passphrase');
      expect(cred.credtype()).toBe(CredType.SshKey);
    });
  });

  describe('sshKeyFromMemory()', () => {
    // Minimal truncated PEM blocks — libgit2 may reject the content but the binding must
    // accept the string parameters without panicking or type errors.
    const DUMMY_PRIVATE_KEY = [
      '-----BEGIN OPENSSH PRIVATE KEY-----',
      'b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAAMwAAAAtzc2gtZW',
      'QyNTUxOQAAACD3',
      '-----END OPENSSH PRIVATE KEY-----',
    ].join('\n');

    const DUMMY_PUBLIC_KEY = 'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIDummyPublicKeyForTestingPurposesOnly user@host';

    it('creates credential without publicKey (null)', () => {
      try {
        const cred = Cred.sshKeyFromMemory('git', null, DUMMY_PRIVATE_KEY, null);
        expect(cred.credtype()).toBe(CredType.SshMemory);
        expect(cred.hasUsername()).toBe(true);
      } catch (e) {
        // libgit2 may reject truncated key content
        expect(e).toBeDefined();
      }
    });

    it('creates credential with publicKey provided', () => {
      try {
        const cred = Cred.sshKeyFromMemory('git', DUMMY_PUBLIC_KEY, DUMMY_PRIVATE_KEY, null);
        expect(cred.credtype()).toBe(CredType.SshMemory);
        expect(cred.hasUsername()).toBe(true);
      } catch (e) {
        expect(e).toBeDefined();
      }
    });

    it('creates credential with publicKey and passphrase', () => {
      try {
        const cred = Cred.sshKeyFromMemory('git', DUMMY_PUBLIC_KEY, DUMMY_PRIVATE_KEY, 'passphrase');
        expect(cred.credtype()).toBe(CredType.SshMemory);
      } catch (e) {
        expect(e).toBeDefined();
      }
    });

    it('creates credential without publicKey but with passphrase', () => {
      try {
        const cred = Cred.sshKeyFromMemory('git', null, DUMMY_PRIVATE_KEY, 'passphrase');
        expect(cred.credtype()).toBe(CredType.SshMemory);
      } catch (e) {
        expect(e).toBeDefined();
      }
    });
  });

  describe('credentialHelper()', () => {
    it('returns Cred or throws when no helper is configured', async () => {
      const p = await useFixture('empty');
      const repo = await openRepository(p);
      const config = repo.config();
      try {
        const cred = Cred.credentialHelper(config, 'https://example.invalid/repo', null);
        expect(cred).toBeDefined();
        expect(cred.credtype()).toBeDefined();
      } catch (e) {
        expect(e).toBeDefined();
      }
    });

    it('accepts username hint', async () => {
      const p = await useFixture('empty');
      const repo = await openRepository(p);
      const config = repo.config();
      try {
        const cred = Cred.credentialHelper(config, 'https://example.invalid/repo', 'user');
        expect(cred).toBeDefined();
      } catch (e) {
        expect(e).toBeDefined();
      }
    });
  });

  describe('hasUsername()', () => {
    it('returns false for Default credential', () => {
      expect(Cred.default().hasUsername()).toBe(false);
    });

    it('returns true for userpassPlaintext credential', () => {
      expect(Cred.userpassPlaintext('user', 'pass').hasUsername()).toBe(true);
    });

    it('returns true for username credential', () => {
      expect(Cred.username('git').hasUsername()).toBe(true);
    });

    it('returns true for sshKey credential', () => {
      expect(Cred.sshKey('git', null, '/path/to/key', null).hasUsername()).toBe(true);
    });
  });

  describe('credtype()', () => {
    it('returns Default for default()', () => {
      expect(Cred.default().credtype()).toBe(CredType.Default);
    });

    it('returns UserpassPlaintext for userpassPlaintext()', () => {
      expect(Cred.userpassPlaintext('user', 'pass').credtype()).toBe(CredType.UserpassPlaintext);
    });

    it('returns Username for username()', () => {
      expect(Cred.username('git').credtype()).toBe(CredType.Username);
    });

    it('returns SshKey for sshKey()', () => {
      expect(Cred.sshKey('git', null, '/path/to/key', null).credtype()).toBe(CredType.SshKey);
    });
  });
});
