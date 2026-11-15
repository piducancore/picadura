import { defineConfig } from 'astro/config';

export default defineConfig({
  site: 'https://piducan.dev',
  base: '/picadura',
  output: 'static',
  trailingSlash: 'always',
});
