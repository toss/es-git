import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    environment: 'node',
    clearMocks: true,
    testTimeout: 60_000,
    include: ['./tests/**/*.spec.ts'],
  },
});
