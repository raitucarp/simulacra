import { createMDX } from 'fumadocs-mdx/next';

// The site is served at the root of https://simulacra.raitucarp.name, so the base path is empty.
// The deploy workflow passes the one GitHub Pages reports, which is "/simulacra" only when the
// repository has no custom domain and Pages serves it from /<repository>.
const basePath = process.env.DOCS_BASE_PATH ?? '';

/** @type {import('next').NextConfig} */
const config = {
  output: 'export',
  basePath,
  trailingSlash: true,
  reactStrictMode: true,
  images: { unoptimized: true },
  env: { NEXT_PUBLIC_BASE_PATH: basePath },
  turbopack: { root: import.meta.dirname },
  // Next 16 writes an AGENTS.md into the project on `next dev`; this site does not want one.
  agentRules: false,
};

const withMDX = createMDX();

export default withMDX(config);
