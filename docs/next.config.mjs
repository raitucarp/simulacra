import { createMDX } from 'fumadocs-mdx/next';

// GitHub Pages serves the site from /<repository>; the workflow sets DOCS_BASE_PATH=/simulacra.
// Local `npm run dev` leaves it empty so the site lives at the root.
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
