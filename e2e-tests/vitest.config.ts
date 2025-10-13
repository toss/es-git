import { loadEnv } from 'vite';
import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    env: loadEnv('', process.cwd(), ''),
    environment: 'node',
    clearMocks: true,
    testTimeout: 60_000,
    globalSetup: ['./vitest.global-setup.ts'],
  },
});
