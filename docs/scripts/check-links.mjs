// Checks that every internal link (/docs/...) in the MDX pages and meta.json files points at a page.
// Usage: node scripts/check-links.mjs
import { readFileSync, readdirSync, statSync, existsSync } from 'node:fs';
import { join, relative } from 'node:path';

const root = new URL('..', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1');
const content = join(root, 'content/docs');

function walk(path, out) {
  if (statSync(path).isDirectory()) for (const name of readdirSync(path)) walk(join(path, name), out);
  else out.push(path);
  return out;
}

function pageExists(slug) {
  const clean = slug.replace(/^\/docs\/?/, '').replace(/\/$/, '');
  if (clean === '') return existsSync(join(content, 'index.mdx'));
  return existsSync(join(content, `${clean}.mdx`)) || existsSync(join(content, clean, 'index.mdx'));
}

let problems = 0;
for (const file of walk(content, []).filter((f) => f.endsWith('.mdx') || f.endsWith('meta.json'))) {
  const text = readFileSync(file, 'utf8');
  for (const match of text.matchAll(/(?:\]\(|href=")(\/docs[^)"#\s]*)/g)) {
    if (!pageExists(match[1])) {
      console.error(`✗ ${relative(root, file)}: ${match[1]}`);
      problems++;
    }
  }
}
if (problems) {
  console.error(`${problems} broken link(s)`);
  process.exitCode = 1;
} else console.log('Links OK');
