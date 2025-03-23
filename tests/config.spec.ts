import { describe, expect, it } from 'vitest';
import { openDefaultConfig, openRepository, parseConfigBool, parseConfigI32, parseConfigI64 } from '../index';
import { useFixture } from './fixtures';

describe('config', () => {
  it('open default config', () => {
    openDefaultConfig();
  });

  it('set config boolean value', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    const config = repo.config();
    config.setBool('core.autocrlf', true);
    expect(config.getBool('core.autocrlf')).toBe(true);
  });

  it('iterate config entries', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    const config = repo.config();
    expect([...config.entries()]).instanceof(Array);
  });

  it('throws error if config entry is not exists', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const config = repo.config();
    const error = /libgit2 error: invalid config item name 'unknown_config_variable';/;
    expect(() => config.remove('unknown_config_variable')).toThrowError(error);
    expect(() => config.getString('unknown_config_variable')).toThrowError(error);
  });

  it('parse boolean config value', () => {
    expect(parseConfigBool('true')).toBe(true);
    expect(parseConfigBool('yes')).toBe(true);
    expect(parseConfigBool('on')).toBe(true);
    expect(parseConfigBool('1')).toBe(true);
    expect(parseConfigBool('false')).toBe(false);
    expect(parseConfigBool('no')).toBe(false);
    expect(parseConfigBool('off')).toBe(false);
    expect(parseConfigBool('0')).toBe(false);
    expect(() => parseConfigBool('not_a_bool')).toThrowError(
      /libgit2 error: failed to parse 'not_a_bool' as a boolean value;/
    );
  });

  it('parse boolean config value', () => {
    expect(parseConfigBool('true')).toBe(true);
    expect(parseConfigBool('yes')).toBe(true);
    expect(parseConfigBool('on')).toBe(true);
    expect(parseConfigBool('1')).toBe(true);
    expect(parseConfigBool('false')).toBe(false);
    expect(parseConfigBool('no')).toBe(false);
    expect(parseConfigBool('off')).toBe(false);
    expect(parseConfigBool('0')).toBe(false);
    expect(() => parseConfigBool('not_a_bool')).toThrowError(
      /libgit2 error: failed to parse 'not_a_bool' as a boolean value;/
    );
  });

  it('parse i32 config value', () => {
    expect(parseConfigI32('1')).toBe(1);
    expect(parseConfigI32('0')).toBe(0);
    expect(parseConfigI32('1024M')).toBe(1073741824);
    expect(() => parseConfigI32('not_a_i32')).toThrowError(
      /libgit2 error: failed to parse 'not_a_i32' as a 32-bit integer;/
    );
  });

  it('parse i64 config value', () => {
    expect(parseConfigI64('1')).toBe(1);
    expect(parseConfigI64('0')).toBe(0);
    expect(parseConfigI64('1024M')).toBe(1073741824);
    expect(() => parseConfigI64('not_a_i64')).toThrowError(/libgit2 error: failed to parse 'not_a_i64' as an integer;/);
  });
});
