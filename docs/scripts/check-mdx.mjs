// Compiles MDX pages to catch syntax errors without a full site build, and checks that every
// <Screenshot id="…"> is declared in a shot list.
// Usage: node scripts/check-mdx.mjs [files or folders…]   (default: content/docs)
import { compile } from '@mdx-js/mdx';
import remarkGfm from 'remark-gfm';
import { parse as parseYaml } from 'yaml';
import { readFileSync, readdirSync, statSync, existsSync } from 'node:fs';
import { join, relative } from 'node:path';

const root = new URL('..', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1');
const targets = process.argv.slice(2).length ? process.argv.slice(2) : [join(root, 'content/docs')];

function walk(path, out) {
  if (statSync(path).isDirectory()) for (const name of readdirSync(path)) walk(join(path, name), out);
  else if (path.endsWith('.mdx')) out.push(path);
  return out;
}

const shotIds = new Set();
const shotsDir = join(root, 'capture/shots');
if (existsSync(shotsDir)) {
  for (const name of readdirSync(shotsDir).filter((n) => n.endsWith('.json'))) {
    try {
      for (const shot of JSON.parse(readFileSync(join(shotsDir, name), 'utf8'))) shotIds.add(shot.id);
    } catch (error) {
      console.error(`✗ capture/shots/${name}: ${error.message}`);
      process.exitCode = 1;
    }
  }
}

let failures = 0;
for (const file of targets.flatMap((t) => walk(t, []))) {
  const source = readFileSync(file, 'utf8');
  const body = source.replace(/^---[\s\S]*?---/, '');
  const head = source.match(/^---\r?\n([\s\S]*?)\r?\n---/);
  try {
    const data = head ? parseYaml(head[1]) : null;
    if (!data || typeof data.title !== 'string') throw new Error('missing frontmatter title');
  } catch (error) {
    console.error(`✗ ${relative(root, file)}: frontmatter: ${error.message.split('\n')[0]}`);
    failures++;
  }
  try {
    await compile(body, { remarkPlugins: [remarkGfm] });
  } catch (error) {
    console.error(`✗ ${relative(root, file)}:${error.line ?? '?'}: ${error.reason ?? error.message}`);
    failures++;
  }
  for (const match of body.matchAll(/<Screenshot[^>]*\bid="([^"]+)"/g)) {
    if (!shotIds.has(match[1])) {
      console.error(`✗ ${relative(root, file)}: screenshot "${match[1]}" is not in any capture/shots/*.json`);
      failures++;
    }
  }
}
if (failures) {
  console.error(`${failures} problem(s)`);
  process.exitCode = 1;
} else console.log('MDX OK');
