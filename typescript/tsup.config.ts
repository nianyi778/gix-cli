import { defineConfig } from 'tsup';

export default defineConfig({
  entry: ['src/bin/gix.ts'],
  format: ['esm'],
  outDir: 'dist',
  splitting: false,
  sourcemap: true,
  clean: true,
  dts: true,
  target: 'node20',
  banner: {
    js: '#!/usr/bin/env node'
  }
});
