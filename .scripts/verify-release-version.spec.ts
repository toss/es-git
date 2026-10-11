import { describe, expect, it } from 'vitest';
import { compareSemver, parseSemver } from './verify-release-version.ts';

describe('compareSemver', () => {
  it('orders by major, minor and patch', () => {
    expect(compareSemver('1.0.0', '0.9.9')).toBeGreaterThan(0);
    expect(compareSemver('0.7.0', '0.6.0')).toBeGreaterThan(0);
    expect(compareSemver('0.7.1', '0.7.0')).toBeGreaterThan(0);
    expect(compareSemver('0.7.0', '0.7.0')).toBe(0);
  });

  it('refuses a patch of an older minor as newer', () => {
    // The mistake this guards: releasing 1.0.1 while 1.1.0 is already published.
    expect(compareSemver('1.0.1', '1.1.0')).toBeLessThan(0);
  });

  it('places a prerelease before its release', () => {
    expect(compareSemver('0.7.0-next.1', '0.7.0')).toBeLessThan(0);
    expect(compareSemver('0.7.0', '0.7.0-next.1')).toBeGreaterThan(0);
  });

  it('compares prerelease identifiers field by field', () => {
    expect(compareSemver('0.7.0-next.2', '0.7.0-next.10')).toBeLessThan(0);
    expect(compareSemver('0.7.0-alpha', '0.7.0-beta')).toBeLessThan(0);
    expect(compareSemver('0.7.0-next.1', '0.7.0-next')).toBeGreaterThan(0);
    // A numeric identifier always precedes an alphanumeric one.
    expect(compareSemver('0.7.0-1', '0.7.0-alpha')).toBeLessThan(0);
  });

  it('ignores build metadata', () => {
    expect(compareSemver('0.7.0-next.42+0aabbcc', '0.7.0-next.42')).toBe(0);
    expect(compareSemver('0.7.0+build', '0.7.0')).toBe(0);
  });
});

describe('parseSemver', () => {
  it('splits prerelease identifiers', () => {
    expect(parseSemver('0.7.0-next.42+0aabbcc')).toEqual({
      major: 0,
      minor: 7,
      patch: 0,
      prerelease: ['next', '42'],
    });
  });

  it('rejects a version it cannot compare', () => {
    expect(() => parseSemver('v0.7.0')).toThrow('Not a valid semver version');
    expect(() => parseSemver('0.7')).toThrow('Not a valid semver version');
  });
});
