/** The path GitHub Pages serves the site under (`/simulacra`), or empty in local development. */
export const basePath = process.env.NEXT_PUBLIC_BASE_PATH ?? '';

/** `path` (starting with `/`) prefixed with the base path, for files in `public/`. */
export function withBasePath(path: string): string {
  return `${basePath}${path}`;
}
