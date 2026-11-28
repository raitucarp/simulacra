// flowctl: holds one Chrome DevTools Protocol connection to the user's running Chrome
// (remote debugging on 127.0.0.1, endpoint read from DevToolsActivePort) and exposes a
// tiny localhost control API so each automation step is one HTTP call.
// Only tabs this tool creates are driven. No dependencies (Node >= 22).
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { createServer } from 'node:http';
import { randomBytes } from 'node:crypto';

const portFile = join(process.env.LOCALAPPDATA, 'Google', 'Chrome', 'User Data', 'DevToolsActivePort');
const [port, path] = readFileSync(portFile, 'utf8').trim().split(/\r?\n/);
const ws = new WebSocket(`ws://127.0.0.1:${port}${path}`);
let nextId = 1;
const pending = new Map();
const tabs = new Map(); // targetId -> { sessionId, responses: Map }
let current = null;
const downloads = new Map(); // guid -> { name, url, state, bytes, at }
let pendingFiles = null; // files to hand to the next native file chooser
let lastUpload = null;

function send(method, params = {}, sessionId) {
  const id = nextId++;
  const msg = { id, method, params };
  if (sessionId) msg.sessionId = sessionId;
  ws.send(JSON.stringify(msg));
  return new Promise((resolve, reject) => {
    pending.set(id, { resolve, reject });
    setTimeout(() => {
      if (pending.has(id)) { pending.delete(id); reject(new Error(`timeout ${method}`)); }
    }, 120000);
  });
}

ws.onmessage = (ev) => {
  const msg = JSON.parse(ev.data);
  if (msg.id && pending.has(msg.id)) {
    const p = pending.get(msg.id);
    pending.delete(msg.id);
    msg.error ? p.reject(new Error(JSON.stringify(msg.error))) : p.resolve(msg.result);
    return;
  }
  if (msg.method === 'Browser.downloadWillBegin' || msg.method === 'Browser.downloadProgress') {
    const p = msg.params;
    const d = downloads.get(p.guid) || { guid: p.guid };
    Object.assign(d, p.suggestedFilename ? { name: p.suggestedFilename, url: (p.url || '').slice(0, 200) } : {}, p.state ? { state: p.state, bytes: p.receivedBytes } : {}, { at: Date.now() });
    downloads.set(p.guid, d);
    return;
  }
  if (msg.method === 'Page.fileChooserOpened' && msg.sessionId && pendingFiles) {
    const files = pendingFiles; pendingFiles = null;
    send('DOM.setFileInputFiles', { files, backendNodeId: msg.params.backendNodeId }, msg.sessionId)
      .then(() => { lastUpload = { files, at: Date.now() }; })
      .catch((e) => { lastUpload = { error: String(e), at: Date.now() }; });
    return;
  }
  if (msg.method === 'Network.responseReceived' && msg.sessionId) {
    for (const t of tabs.values()) {
      if (t.sessionId !== msg.sessionId) continue;
      const r = msg.params.response;
      if (/^image\//.test(r.mimeType || '')) {
        t.responses.set(msg.params.requestId, { requestId: msg.params.requestId, url: r.url.slice(0, 300), mimeType: r.mimeType, at: Date.now() });
        if (t.responses.size > 400) t.responses.delete(t.responses.keys().next().value);
      }
    }
  }
};

const S = () => {
  if (!current) throw new Error('no tab open');
  return tabs.get(current).sessionId;
};

const ops = {
  async open({ url }) {
    const { targetId } = await send('Target.createTarget', { url });
    const { sessionId } = await send('Target.attachToTarget', { targetId, flatten: true });
    for (const d of ['Page', 'Runtime', 'DOM']) await send(`${d}.enable`, {}, sessionId);
    await send('Network.enable', { maxResourceBufferSize: 50e6, maxTotalBufferSize: 400e6 }, sessionId);
    tabs.set(targetId, { sessionId, responses: new Map() });
    current = targetId;
    return { targetId };
  },
  async use({ targetId }) { if (!tabs.has(targetId)) throw new Error('unknown tab'); current = targetId; return { targetId }; },
  async nav({ url }) { return send('Page.navigate', { url }, S()); },
  async eval({ expr }) {
    const r = await send('Runtime.evaluate', { expression: expr, awaitPromise: true, returnByValue: true, userGesture: true }, S());
    if (r.exceptionDetails) throw new Error(JSON.stringify(r.exceptionDetails).slice(0, 2000));
    return r.result.value;
  },
  async shot({ out, scale = 1, clip }) {
    const params = { format: 'png' };
    if (clip) params.clip = { ...clip, scale };
    const { data } = await send('Page.captureScreenshot', params, S());
    mkdirSync(dirname(out), { recursive: true });
    writeFileSync(out, Buffer.from(data, 'base64'));
    return { out };
  },
  async upload({ selector, files }) {
    const { root } = await send('DOM.getDocument', { depth: -1, pierce: true }, S());
    const { nodeId } = await send('DOM.querySelector', { nodeId: root.nodeId, selector }, S());
    if (!nodeId) throw new Error(`no element for ${selector}`);
    await send('DOM.setFileInputFiles', { nodeId, files }, S());
    return { ok: true };
  },
  async responses({ since = 0 }) {
    return [...tabs.get(current).responses.values()].filter((r) => r.at >= since);
  },
  async body({ requestId, out }) {
    const r = await send('Network.getResponseBody', { requestId }, S());
    mkdirSync(dirname(out), { recursive: true });
    writeFileSync(out, r.base64Encoded ? Buffer.from(r.body, 'base64') : Buffer.from(r.body));
    return { out };
  },
  async cdp({ method, params }) { return send(method, params || {}, S()); },
  // Browser-level commands (no session), e.g. Browser.setDownloadBehavior.
  async bcdp({ method, params }) { return send(method, params || {}); },
  // Arm the next file chooser: the page's own "upload" click then receives these files
  // without a native dialog ever showing.
  async armUpload({ files }) {
    pendingFiles = files;
    await send('Page.setInterceptFileChooserDialog', { enabled: true }, S());
    return { armed: files.length };
  },
  async lastUpload() { return lastUpload; },
  async downloads({ since = 0 }) { return [...downloads.values()].filter((d) => d.at >= since); },
  async attach({ targetId }) {
    const { sessionId } = await send('Target.attachToTarget', { targetId, flatten: true });
    for (const d of ['Page', 'Runtime', 'DOM']) await send(`${d}.enable`, {}, sessionId);
    await send('Network.enable', { maxResourceBufferSize: 50e6, maxTotalBufferSize: 400e6 }, sessionId);
    tabs.set(targetId, { sessionId, responses: new Map() });
    current = targetId;
    return { targetId };
  },
  async close() {
    if (!current) return {};
    await send('Target.closeTarget', { targetId: current });
    tabs.delete(current);
    current = tabs.keys().next().value || null;
    return { current };
  },
  async tabs() { return { current, open: [...tabs.keys()] }; },
};

const token = randomBytes(16).toString('hex');
ws.onopen = () => {
  const server = createServer(async (req, res) => {
    if (req.headers.authorization !== `Bearer ${token}`) { res.writeHead(401).end(); return; }
    let body = '';
    for await (const c of req) body += c;
    try {
      const { op, ...args } = JSON.parse(body || '{}');
      const out = await ops[op](args);
      res.writeHead(200, { 'content-type': 'application/json' }).end(JSON.stringify(out ?? null));
    } catch (e) {
      res.writeHead(500, { 'content-type': 'application/json' }).end(JSON.stringify({ error: String(e.message || e) }));
    }
  });
  server.listen(0, '127.0.0.1', () => {
    const info = { port: server.address().port, token };
    writeFileSync(join(import.meta.dirname, 'flowctl.json'), JSON.stringify(info));
    console.log('flowctl ready', info.port);
  });
};
ws.onerror = (e) => { console.error('cdp error', e.message || e); };
ws.onclose = () => { console.log('cdp closed'); process.exit(0); };
