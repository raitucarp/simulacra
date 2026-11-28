"""Drive Google Flow (labs.google/fx/tools/flow) in your own Chrome to generate pack art.

Talks to `flowctl.mjs`, which holds one Chrome DevTools Protocol connection over Chrome's
remote debugging (127.0.0.1 only) and drives only the Flow tab it opened or attached to.

    node tools/illustrations/flow/flowctl.mjs            # once, in the background
    python tools/illustrations/flow/flow.py jobs <pack>             # list the batches still to do
    python tools/illustrations/flow/flow.py batch <pack> <id>       # one sheet id or single key
    python tools/illustrations/flow/flow.py run <pack> [--limit N] [--group G]

A job is a sprite sheet (`sheets/<pack>/<id>.{png,txt,json}` from sheets.py) or a single
(`refs-magenta/<key>.png` and the catalog's `prompts.<pack>`). Each job:
  1. uploads its reference to the Flow project (skipped when an asset of that name exists);
  2. sets the job's aspect and the number of outputs, then picks the first Nano Banana model
     whose run costs 0 credits; stops if none does (nothing is generated or spent);
  3. attaches the pack anchor (`anchor-<pack>.jpg`, uploaded once) and the reference;
  4. generates, downloads every variant at 2K into `inbox/<pack>/_variants/`;
  5. scores each variant with process.py in a scratch folder (cells that pass QA) and copies
     the best one to `inbox/<pack>/<id>.jpg`, where `process.py <pack> --all` picks it up.
A job is done once `inbox/<pack>/<id>.*` exists; `run` skips those and logs failures.
"""
import argparse, glob, json, math, os, re, shutil, subprocess, sys, tempfile, time, urllib.error, urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent                                   # tools/illustrations
sys.path.insert(0, str(ROOT))
import catalog as catalog_io                         # noqa: E402
import common                                        # noqa: E402

JS = HERE / 'js'
INFO = Path(os.environ.get('FLOWCTL_INFO') or HERE / 'flowctl.json')
FREE_ORDER = ['Nano Banana 2.1', 'Nano Banana 2 Lite', 'Nano Banana Pro']
FLOW_ASPECTS = {'16:9': 16 / 9, '4:3': 4 / 3, '1:1': 1.0, '3:4': 3 / 4, '9:16': 9 / 16}
REFS_MAGENTA = ROOT / 'refs-magenta'
TILE_JS = ('[...new Set([...document.querySelectorAll("img")]'
           '.filter(i=>i.naturalWidth>=512&&i.src.includes("/asb/")).map(i=>i.src))]')
WRITTEN = re.compile(r': (\d+) written, (\d+) not')


# --- flowctl --------------------------------------------------------------------------------

def fc(obj, timeout=240):
    info = json.loads(INFO.read_text())
    req = urllib.request.Request(
        f"http://127.0.0.1:{info['port']}/", data=json.dumps(obj).encode(),
        headers={'Authorization': f"Bearer {info['token']}", 'content-type': 'application/json'})
    try:
        with urllib.request.urlopen(req, timeout=timeout) as r:
            out = json.loads(r.read() or b'null')
    except urllib.error.HTTPError as e:
        out = json.loads(e.read() or b'null')
    if isinstance(out, dict) and 'error' in out and obj.get('op') != 'eval':
        raise RuntimeError(f"{obj.get('op')}: {out['error']}")
    return out


def ev(expr):
    return fc({'op': 'eval', 'expr': expr})


def js(name, arg=None):
    return ev((JS / name).read_text(encoding='utf-8').replace('__ARG__', json.dumps(arg or '')))


def key(k):
    codes = {'Escape': 27, 'Enter': 13}
    for t in ('keyDown', 'keyUp'):
        fc({'op': 'cdp', 'method': 'Input.dispatchKeyEvent',
            'params': {'type': t, 'key': k, 'code': k, 'windowsVirtualKeyCode': codes[k]}})


def click_xy(x, y):
    for t in ('mouseMoved', 'mousePressed', 'mouseReleased'):
        fc({'op': 'cdp', 'method': 'Input.dispatchMouseEvent',
            'params': {'type': t, 'x': x, 'y': y, 'button': 'left', 'clickCount': 1}})


def insert_text(text):
    fc({'op': 'cdp', 'method': 'Input.insertText', 'params': {'text': text}})


# --- jobs -----------------------------------------------------------------------------------

def flow_aspect(aspect):
    """The closest of Flow's five aspects to "W:H"."""
    w, h = (float(v) for v in aspect.split(':'))
    return min(FLOW_ASPECTS, key=lambda a: abs(math.log(FLOW_ASPECTS[a] / (w / h))))


def jobs(pack, group=None):
    """Every job of the pack, in catalog order (the most visible groups first)."""
    out, seen = [], set()
    for e in catalog_io.load(common.CATALOG_FILE)['entries']:
        if not e.get('generate', True) or (group and e['group'] != group):
            continue
        if e['batch'] == 'single':
            out.append({'id': e['key'], 'kind': 'key', 'prompt': e['prompts'][pack],
                        'ref': REFS_MAGENTA / f"{e['key']}.png", 'aspect': flow_aspect(e['aspect'])})
        elif e.get('sheet') and e['sheet'] not in seen:
            seen.add(e['sheet'])
            base = common.SHEETS_DIR / pack / e['sheet']
            layout = json.loads(base.with_suffix('.json').read_text(encoding='utf-8'))
            out.append({'id': e['sheet'], 'kind': 'sheet', 'prompt': base.with_suffix('.txt').read_text(encoding='utf-8').strip(),
                        'ref': base.with_suffix('.png'), 'aspect': layout['aspect']})
    return out


def is_done(pack, job_id):
    return any(Path(p).is_file() for p in glob.glob(str(common.INBOX_DIR / pack / f'{job_id}.*')))


def asset_name(pack, job_id):
    return f"ref-{pack}-{job_id.replace('/', '_')}.png"


# --- Flow steps -----------------------------------------------------------------------------

def configure(aspect, n_out):
    """Aspect and outputs, then the first free Nano Banana model; stops if none costs 0."""
    js('open_panel.js'); time.sleep(0.8)
    if js('click_match.js', re.escape(aspect) + '$') == 'missing':
        raise RuntimeError(f'aspect {aspect} not offered')
    time.sleep(0.4)
    js('click_match.js', f'^x{n_out}$'); time.sleep(0.4)
    current = ev('document.querySelector(\'[aria-label="Settings trigger"]\').textContent')
    for m in sorted(FREE_ORDER, key=lambda m: (m not in current, FREE_ORDER.index(m))):
        js('open_panel.js'); time.sleep(0.8)
        if m not in current:
            js('model_dropdown.js'); time.sleep(0.8); js('pick_option.js', m); time.sleep(0.8)
        if (js('credits.js') or '').strip().endswith(' 0 credits'):
            key('Escape'); return m
        current = ''
    key('Escape')
    raise SystemExit('no 0-credit model available: stopping (nothing generated)')


SEARCH = 'input[placeholder="Search assets"]'


def open_picker():
    """Open the "Add ingredients" asset picker (it needs a real click, not element.click())."""
    if not ev("!!document.querySelector(%s)" % json.dumps(SEARCH)):
        r = js('rect_of.js', '^Add ingredients')
        click_xy(r['x'], r['y'])
        for _ in range(20):
            time.sleep(0.4)
            if ev("!!document.querySelector(%s)" % json.dumps(SEARCH)):
                break
        else:
            raise RuntimeError('the asset picker did not open')


def close_picker():
    for _ in range(3):
        if not ev("!!document.querySelector(%s)" % json.dumps(SEARCH)):
            return
        key('Escape'); time.sleep(0.6)


def picker_rows(name):
    """Open the picker and search the project's assets for `name`: the matching rows' text
    (a row reads "Uploading <name>" while its upload runs). The picker is left open."""
    open_picker()
    ev("(()=>{const i=document.querySelector(%s);i.focus();i.select();return 1})()" % json.dumps(SEARCH))
    insert_text(name); time.sleep(2)
    return ev("[...document.querySelectorAll('.cdk-overlay-pane button')].filter(b=>b.innerText.includes(%s))"
              ".map(b=>b.innerText.trim())" % json.dumps(name))


def upload(path):
    fc({'op': 'armUpload', 'files': [str(Path(path).resolve())]})
    open_picker()
    js('click_deep.js', '^Upload media$'); time.sleep(4)
    js('click_deep.js', '^I agree$')                      # shown only on an account's first upload
    close_picker()
    return fc({'op': 'lastUpload'})


def ensure_asset(name, source, wait=180):
    """Upload `source` as `name` unless the project has that asset (or is uploading it), then
    wait until the upload has finished."""
    rows = picker_rows(name); close_picker()
    if not rows:
        staged = common.INBOX_DIR / '_upload' / name
        staged.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, staged)
        upload(staged); time.sleep(2)
    for _ in range(wait // 6):
        rows = picker_rows(name); close_picker()
        if rows and not any(r.startswith('Uploading') for r in rows):
            return
        time.sleep(4)
    raise RuntimeError(f'asset {name!r} did not finish uploading: {rows}')


def attach(names):
    """Add each named project asset to the prompt as an ingredient (exactly one must match)."""
    for name in names:
        rows = picker_rows(name)
        if len(rows) != 1 or rows[0].startswith('Uploading'):
            close_picker(); raise RuntimeError(f'asset {name!r}: {rows} in the picker')
        ev("(()=>{const b=[...document.querySelectorAll('.cdk-overlay-pane button')].find(b=>b.innerText.includes(%s));"
           "b.click();return 1})()" % json.dumps(name))
        time.sleep(1)
        js('click_deep.js', '^Add to prompt$'); time.sleep(1.5)
        close_picker()
    return ev('document.querySelectorAll(\'img[alt="Ingredient image"]\').length')


def generate(prompt, n_out):
    """Type the prompt, press Generate, and wait for n_out new tiles; returns their srcs."""
    before = ev(TILE_JS)
    box = '[contenteditable=true]'
    ev("(()=>{const t=document.querySelector('%s');t.focus();return 1})()" % box)
    insert_text(prompt)
    for _ in range(3):
        time.sleep(2)
        r = js('rect_of.js', '^Start generation$')
        click_xy(r['x'], r['y']); time.sleep(2)
        if ev("document.querySelector('%s').textContent.length" % box) == 0:
            break
    else:
        raise RuntimeError('generation did not start')
    for _ in range(120):
        time.sleep(3)
        new = [s for s in ev(TILE_JS) if s not in before]
        if len(new) >= n_out:
            return new[:n_out]
    raise RuntimeError('timed out waiting for results')


def download(src, dest, size='2K'):
    """Open the tile with this src and download it at `size` to dest (Flow serves JPEG).
    Chrome's download behaviour is reset to its default afterwards."""
    raw = common.INBOX_DIR / '_raw'
    raw.mkdir(parents=True, exist_ok=True)
    fc({'op': 'bcdp', 'method': 'Browser.setDownloadBehavior',
        'params': {'behavior': 'allow', 'downloadPath': str(raw), 'eventsEnabled': True}})
    try:
        pos = ev("(()=>{const e=[...document.querySelectorAll('img')].find(i=>i.src===%s);e.scrollIntoView({block:'center'});"
                 "const b=e.getBoundingClientRect();return [b.x+b.width/2,b.y+b.height/2]})()" % json.dumps(src))
        time.sleep(0.6)
        click_xy(int(pos[0]), int(pos[1]))
        for _ in range(20):
            time.sleep(0.5)
            if '/edit/' in ev('location.href'):
                break
        time.sleep(1.5)
        start = int(time.time() * 1000)
        js('click_match.js', '^Download media$'); time.sleep(1)
        if js('click_match.js', '^' + size) == 'missing':
            raise RuntimeError('download size option missing')
        for _ in range(90):
            time.sleep(2)
            if [d for d in fc({'op': 'downloads', 'since': start}) if d.get('state') == 'completed']:
                break
        else:
            raise RuntimeError('download did not complete')
        time.sleep(0.5)
        newest = max(raw.iterdir(), key=lambda p: p.stat().st_mtime)
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.move(str(newest), dest)
        return dest
    finally:
        fc({'op': 'bcdp', 'method': 'Browser.setDownloadBehavior', 'params': {'behavior': 'default'}})
        js('click_match.js', '^Back button'); time.sleep(2)


# --- choosing a variant ---------------------------------------------------------------------

def score(pack, job, variant):
    """(cells written, -cells failed) when process.py runs on this variant in a scratch folder."""
    with tempfile.TemporaryDirectory() as tmp:
        inbox = Path(tmp) / 'inbox'
        staged = inbox / f"{job['id']}{variant.suffix}"
        staged.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(variant, staged)
        flag = '--sheet' if job['kind'] == 'sheet' else '--key'
        run = subprocess.run([sys.executable, str(ROOT / 'process.py'), pack, flag, job['id'], '--inbox', str(inbox),
                              '--out-root', str(Path(tmp) / 'out'), '--work', str(Path(tmp) / 'work')],
                             capture_output=True, text=True, cwd=str(ROOT))
        m = WRITTEN.search(run.stdout)
        return (int(m[1]), -int(m[2])) if m else (0, -99)


def choose(pack, job, variants):
    scores = [score(pack, job, v) for v in variants]
    best = max(range(len(variants)), key=lambda i: (scores[i], -i))
    dest = common.INBOX_DIR / pack / f"{job['id']}{variants[best].suffix}"
    dest.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(variants[best], dest)
    return dest, scores, best


# --- a job ----------------------------------------------------------------------------------

def run_job(pack, job, n_out=2):
    anchor = common.TOOLS_DIR / 'anchors' / f'{pack}-1.jpg'
    ensure_asset(f'anchor-{pack}.jpg', anchor)
    name = asset_name(pack, job['id'])
    ensure_asset(name, job['ref'])
    model = configure(job['aspect'], n_out)
    js('click_match.js', '^Clear prompt$'); time.sleep(0.5)
    if attach([f'anchor-{pack}.jpg', name]) != 2:
        raise RuntimeError('ingredients not attached')
    srcs = generate(job['prompt'], n_out)
    variants = [download(s, common.INBOX_DIR / pack / '_variants' / f"{job['id']}-v{i + 1}.jpg") for i, s in enumerate(srcs)]
    dest, scores, best = choose(pack, job, variants)
    with open(common.INBOX_DIR / pack / '_variants' / 'log.jsonl', 'a', encoding='utf-8') as f:
        f.write(json.dumps({'job': job['id'], 'model': model, 'aspect': job['aspect'], 'scores': scores,
                            'chosen': best + 1, 'at': time.time()}) + '\n')
    return dest, scores


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest='cmd', required=True)
    j = sub.add_parser('jobs'); j.add_argument('pack'); j.add_argument('--group')
    b = sub.add_parser('batch'); b.add_argument('pack'); b.add_argument('id'); b.add_argument('--outputs', type=int, default=2)
    r = sub.add_parser('run'); r.add_argument('pack'); r.add_argument('--group'); r.add_argument('--limit', type=int, default=0)
    r.add_argument('--outputs', type=int, default=2)
    a = ap.parse_args()
    todo = [x for x in jobs(a.pack, getattr(a, 'group', None)) if not is_done(a.pack, x['id'])]
    if a.cmd == 'jobs':
        for x in todo:
            print(f"{x['kind']:5} {x['aspect']:5} {x['id']}")
        print(f'{len(todo)} to do')
        return
    if a.cmd == 'batch':
        todo = [x for x in jobs(a.pack) if x['id'] == a.id]
        if not todo:
            raise SystemExit(f'no job {a.id!r} in the {a.pack} catalog')
    todo = todo[: a.limit or None] if a.cmd == 'run' else todo
    failed = 0
    for i, x in enumerate(todo):
        print(f"[{i + 1}/{len(todo)}] {x['id']} ({x['kind']}, {x['aspect']})", flush=True)
        try:
            dest, scores = run_job(a.pack, x, a.outputs)
            print(f'  chose {dest.name}; scores {scores}', flush=True)
        except SystemExit:
            raise
        except Exception as e:                       # keep going; failures are re-run later
            failed += 1
            print(f'  FAILED {e}', flush=True)
            key('Escape'); js('click_match.js', '^Clear prompt$')
    print(f'done: {len(todo) - failed} ok, {failed} failed', flush=True)


if __name__ == '__main__':
    main()
