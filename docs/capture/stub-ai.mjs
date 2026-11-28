// A local, offline stand-in for an OpenAI-compatible AI engine, used only to capture documentation
// screenshots of AI features without real API keys. Answers are canned prose stitched from the
// request's own words; JSON-schema requests get schema-shaped JSON; embeddings are hashed
// bag-of-words vectors (so texts sharing words land near each other).
//
//   node docs/capture/stub-ai.mjs            # listens on http://127.0.0.1:8765/v1
//
// In Simulacra: Settings → AI & Models → Custom Endpoint, URL http://127.0.0.1:8765/v1/chat/completions,
// any model name, any key.
import { createServer } from 'node:http';

const PORT = Number(process.env.STUB_AI_PORT ?? 8765);
const DIMS = 256;

const OPENERS = [
  'Consider the question from the other side:',
  'There is a quieter reading of this:',
  'I would put it differently.',
  'Grant the premise for a moment.',
  'The interesting part is not the claim but its edge:',
  'Here is the tension as I see it:',
];
const BODIES = [
  'what we call %w is less a thing than a practice, kept alive by how often we return to it.',
  'if %w holds, then the burden shifts to explaining why it so rarely feels that way from inside.',
  'the link between %w and %v is doing more work in the argument than either term alone.',
  '%w names a pattern we notice after the fact; the risk is mistaking the map for the walk.',
  'a good test is to ask what would change your mind about %w, and whether you would notice it.',
  'the strongest version of the opposing view treats %w as a consequence rather than a cause.',
];
const CLOSERS = [
  'What would count as evidence against it?',
  'I am not sure the distinction survives contact with practice.',
  'Worth revisiting after a week.',
  'That, at least, is where I would start.',
  'Which of these do you actually believe?',
];

const STOP = new Set(
  'the a an and or but of to in on for with is are was were be been it this that these those as at by from about into your you i we they he she not no yes can could would should will just than then so if what which who how why when where do does did have has had my our their its there here more most very also any some such only own same other'.split(
    ' ',
  ),
);

function words(text) {
  return (text.toLowerCase().match(/[\p{L}][\p{L}'-]{3,}/gu) ?? []).filter((w) => !STOP.has(w));
}

function hash(text) {
  let h = 2166136261;
  for (const ch of text) h = Math.imul(h ^ ch.codePointAt(0), 16777619);
  return h >>> 0;
}

function pick(list, seed) {
  return list[seed % list.length];
}

/** Two keywords of the request (frequent, long words of the last user message). */
function keywords(messages) {
  const last = [...messages].reverse().find((m) => m.role === 'user')?.content ?? '';
  const text = typeof last === 'string' ? last : JSON.stringify(last);
  const counts = new Map();
  for (const w of words(text).slice(-400)) counts.set(w, (counts.get(w) ?? 0) + 1);
  const ranked = [...counts.entries()].sort((a, b) => b[1] * b[0].length - a[1] * a[0].length).map(([w]) => w);
  return { text, w: ranked[0] ?? 'attention', v: ranked[1] ?? 'memory', seed: hash(text) };
}

function prose({ w, v, seed }, sentences = 3) {
  const opener = pick(OPENERS, seed);
  let body = pick(BODIES, seed >>> 3).replaceAll('%w', w).replaceAll('%v', v);
  if (opener.endsWith('.')) body = body[0].toUpperCase() + body.slice(1);
  const parts = [opener, body];
  if (sentences > 2) parts.push(pick(CLOSERS, seed >>> 7));
  return parts.join(' ');
}

/** A value shaped like `schema`, filled from the request's keywords. */
function sample(schema, ctx, name = '', depth = 0) {
  if (!schema || depth > 6) return null;
  if (schema.enum) return schema.enum[ctx.seed % schema.enum.length];
  if (schema.const !== undefined) return schema.const;
  const type = Array.isArray(schema.type) ? schema.type.find((t) => t !== 'null') : schema.type;
  if (schema.anyOf || schema.oneOf) return sample((schema.anyOf ?? schema.oneOf)[0], ctx, name, depth + 1);
  switch (type) {
    case 'object': {
      const out = {};
      for (const [key, sub] of Object.entries(schema.properties ?? {})) out[key] = sample(sub, ctx, key, depth + 1);
      return out;
    }
    case 'array': {
      const n = Math.max(schema.minItems ?? 0, Math.min(schema.maxItems ?? 3, 3));
      return Array.from({ length: n }, (_, i) => sample(schema.items, { ...ctx, seed: ctx.seed + i * 7919 }, name, depth + 1));
    }
    case 'integer':
      return Math.min(schema.maximum ?? 4, Math.max(schema.minimum ?? 1, 1 + (ctx.seed % 4)));
    case 'number': {
      const lo = schema.minimum ?? 0;
      const hi = schema.maximum ?? 1;
      return Math.round((lo + ((ctx.seed % 70) / 100 + 0.15) * (hi - lo)) * 100) / 100;
    }
    case 'boolean':
      return ctx.seed % 2 === 0;
    default: {
      const n = name.toLowerCase();
      if (/(title|name|label|term|heading|topic)/.test(n)) return `${ctx.w[0].toUpperCase()}${ctx.w.slice(1)} and ${ctx.v}`;
      if (/(tag|keyword)/.test(n)) return ctx.w;
      if (/(question)/.test(n)) return `What would change your mind about ${ctx.w}?`;
      if (/(answer|definition|summary|reason|note|text|body|content|reply|explanation|claim|statement)/.test(n))
        return prose(ctx, 2);
      return prose(ctx, 2);
    }
  }
}

function embed(text) {
  const vec = new Float64Array(DIMS);
  for (const w of words(text)) {
    const h = hash(w);
    vec[h % DIMS] += 1;
    vec[(h >>> 8) % DIMS] += 0.5;
  }
  const norm = Math.hypot(...vec) || 1;
  return Array.from(vec, (x) => x / norm);
}

function send(res, status, body) {
  res.writeHead(status, { 'content-type': 'application/json' });
  res.end(JSON.stringify(body));
}

createServer((req, res) => {
  let raw = '';
  req.on('data', (chunk) => (raw += chunk));
  req.on('end', () => {
    const body = raw ? JSON.parse(raw) : {};
    const url = req.url ?? '';
    if (req.method === 'GET' && url.endsWith('/models')) {
      return send(res, 200, { object: 'list', data: [{ id: 'stub-thinker', object: 'model' }] });
    }
    if (url.endsWith('/embeddings')) {
      const inputs = Array.isArray(body.input) ? body.input : [body.input ?? ''];
      return send(res, 200, {
        object: 'list',
        model: body.model ?? 'stub-embed',
        data: inputs.map((text, index) => ({ object: 'embedding', index, embedding: embed(String(text)) })),
        usage: { prompt_tokens: 0, total_tokens: 0 },
      });
    }
    if (url.endsWith('/chat/completions')) {
      const ctx = keywords(body.messages ?? []);
      const schema = body.response_format?.json_schema?.schema;
      const content = schema ? JSON.stringify(sample(schema, ctx)) : prose(ctx);
      return send(res, 200, {
        id: `stub-${Date.now()}`,
        object: 'chat.completion',
        model: body.model ?? 'stub-thinker',
        choices: [{ index: 0, finish_reason: 'stop', message: { role: 'assistant', content } }],
        usage: { prompt_tokens: Math.round(raw.length / 4), completion_tokens: Math.round(content.length / 4), total_tokens: 0 },
      });
    }
    send(res, 404, { error: { message: `stub-ai: no route for ${req.method} ${url}` } });
  });
}).listen(PORT, '127.0.0.1', () => console.log(`stub-ai listening on http://127.0.0.1:${PORT}/v1`));
