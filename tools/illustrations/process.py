"""Turns downloaded Flow images into pack files.

    python tools/illustrations/process.py <pack> [--key KEY ...] [--sheet BATCH-ID ...] [--all]
        [--check] [--force] [--inbox DIR] [--out-root DIR] [--work DIR] [--refs DIR]
        [--sheets DIR] [--catalog FILE] [--assets DIR] [--name N] [--pack-version V]
        [--description D] [--sample KEY ...]

Singles come from `inbox/<pack>/<key>.<ext>`, sheets from `inbox/<pack>/<batch-id>.<ext>`
(sliced by `sheets/<pack>/<batch-id>.json`), where <ext> is jpg, jpeg, png or webp (Flow serves
JPEG; folders starting with `_`, such as `_raw/`, are ignored). Each image is keyed (magenta -> alpha, despill, feather),
fitted (trim + pad to the canvas aspect, or the whole canvas for palace rooms and layers),
resized to the stored size and written as WebP with alpha; pack.json is updated. Layers
must match their reference render's alpha (IoU) or they are left out, so Classic draws them.
Runs are idempotent: an unchanged input is skipped unless --force.
"""

from __future__ import annotations

import argparse
import sys
from dataclasses import dataclass, field
from pathlib import Path

from PIL import Image

import catalog as catalog_io
import common
import imaging
import packfile

# Alignment of a layered raster with its reference render (alpha-mask IoU).
IOU_MAIN = 0.80
IOU_DETAIL = 0.60
MAIN_LAYERS = ("back", "face", "hair", "outfit", "body", "aura")
DETAIL_LAYERS = ("eyes", "mouth", "brows", "accessory", "finish", "markings")
SHEET_ASPECT_TOLERANCE = 0.02
INBOX_EXTS = (".jpg", ".jpeg", ".png", ".webp")


def find_inbox(inbox: Path, name: str) -> Path | None:
    """The inbox image of a key or batch id, whatever its extension (the newest if several)."""
    found = [inbox / f"{name}{ext}" for ext in INBOX_EXTS + tuple(e.upper() for e in INBOX_EXTS)]
    found = [path for path in found if path.is_file()]
    return max(found, key=lambda path: path.stat().st_mtime_ns) if found else None


@dataclass
class Context:
    pack: str
    pack_dir: packfile.PackDir
    state: packfile.RunState
    inbox: Path
    refs: Path
    sheets: Path
    work: Path
    assets: Path
    entries: dict
    force: bool = False
    results: list = field(default_factory=list)

    def entry(self, key: str) -> dict:
        if key in self.entries:
            return self.entries[key]
        if (self.assets / f"{key}.svg").is_file():
            return common.entry_info(key, self.assets)
        raise KeyError(f"{key}: not a Classic key (no {key}.svg) and not in the catalog")


def iou_threshold(key: str) -> float:
    return IOU_DETAIL if common.layer_of(key) in DETAIL_LAYERS else IOU_MAIN


def process_keyed(ctx: Context, key: str, keyed: Image.Image, source: str) -> dict:
    """Fits, checks and stores one keyed image; returns `{key, status, reasons}`."""
    info = ctx.entry(key)
    canvas, stored = tuple(info["canvas"]), tuple(info["stored"])
    reasons: list[str] = []
    try:
        shaped = imaging.fit(keyed, info["fit"], canvas, stored)
    except ValueError as err:
        shaped, reasons = None, [str(err)]
    if shaped is not None and info["layered"]:
        reasons += alignment_problems(ctx, key, shaped)
    if reasons:
        ctx.pack_dir.drop(key)
        status = "fail"
    else:
        ctx.pack_dir.put(key, imaging.encode_webp(shaped), stored)
        status = "pass"
    ctx.state.record(key, source, status, reasons)
    result = {"key": key, "status": status, "reasons": reasons}
    ctx.results.append(result)
    return result


def alignment_problems(ctx: Context, key: str, shaped: Image.Image) -> list[str]:
    ref_path = ctx.refs / f"{key}.png"
    if not ref_path.is_file():
        return [f"no reference render {ref_path} to check the layer's alignment"]
    with Image.open(ref_path) as ref:
        iou = imaging.alpha_iou(shaped, ref.convert("RGBA"))
    limit = iou_threshold(key)
    return [] if iou >= limit else [f"layer misaligned: alpha IoU {iou:.2f} < {limit:.2f} against the reference"]


def process_single(ctx: Context, key: str) -> dict | None:
    path = find_inbox(ctx.inbox, key)
    if path is None:
        result = {"key": key, "status": "missing", "reasons": [f"no inbox image {ctx.inbox / key}.(jpg|jpeg|png|webp)"]}
        ctx.results.append(result)
        return result
    source = common.sha256_file(path)
    if not ctx.force and is_current(ctx, key, source):
        return None
    with Image.open(path) as image:
        keyed = imaging.key_background(image)
    return process_keyed(ctx, key, keyed, source)


def is_current(ctx: Context, key: str, source: str) -> bool:
    previous = ctx.state.items.get(key)
    if not previous or previous.get("source") != source:
        return False
    if previous.get("status") == "fail":
        return True
    entry = ctx.pack_dir.coverage.get(key)
    path = ctx.pack_dir.file_path(key)
    return bool(entry) and path.is_file() and common.sha256_file(path) == entry["sha256"]


def scaled(box, sx: float, sy: float) -> tuple[int, int, int, int]:
    x, y, w, h = box
    return round(x * sx), round(y * sy), round((x + w) * sx), round((y + h) * sy)


def cell_problems(keyed: Image.Image, fit_mode: str) -> list[str]:
    """Why a sheet cell cannot be used: empty, or a part reaching the edge of its slot (the cell
    plus half the gutter), where it may cross into a neighbour. A subject may be made of several
    separate parts (sparkles, dashed trails, a key ring beside its key), so parts are not counted;
    the contact sheets catch a cell that holds the wrong subject."""
    parts = imaging.components(keyed)
    if not imaging.significant(parts):
        return ["empty cell: no subject"]
    if fit_mode == "canvas":
        return []  # layers and rooms may hold several shapes and touch their edges
    if any(part["touches"] for part in imaging.kept(parts)):
        return ["a part touches the cell edge (it may cross into a neighbour)"]
    return []


def process_sheet(ctx: Context, sheet_id: str) -> list[dict]:
    layout_path = ctx.sheets / f"{sheet_id}.json"
    image_path = find_inbox(ctx.inbox, sheet_id)
    for path, what in ((layout_path, layout_path), (image_path, f"{ctx.inbox / sheet_id}.(jpg|jpeg|png|webp)")):
        if path is None or not path.is_file():
            result = {"key": sheet_id, "status": "missing", "reasons": [f"no {what}"]}
            ctx.results.append(result)
            return [result]
    layout = common.read_json(layout_path)
    source = common.sha256_file(image_path)
    previous = ctx.state.sheets.get(sheet_id, {})
    if not ctx.force and previous.get("source") == source and all(is_current(ctx, c["key"], f"{source}#{c['key']}") for c in layout["cells"]):
        return []
    with Image.open(image_path) as opened:
        sheet = imaging.flatten(opened)
    sx, sy = sheet.width / layout["size"][0], sheet.height / layout["size"][1]
    warning = abs(sx / sy - 1.0) > SHEET_ASPECT_TOLERANCE
    results = [process_cell(ctx, sheet, layout, cell, (sx, sy), f"{source}#{cell['key']}") for cell in layout["cells"]]
    if warning:
        for result in results:
            result["reasons"].append(f"warning: the download's aspect differs from the sheet's ({sheet.width}x{sheet.height})")
    ctx.state.sheets[sheet_id] = {"source": source, "cells": {r["key"]: r["status"] for r in results}}
    return results


def process_cell(ctx: Context, sheet: Image.Image, layout: dict, cell: dict, scale, source: str) -> dict:
    fit_mode = layout.get("fit") or ctx.entry(cell["key"])["fit"]
    region = cell["box"] if fit_mode == "canvas" else cell["slot"]
    keyed = imaging.key_background(sheet.crop(scaled(region, *scale)))
    problems = cell_problems(keyed, fit_mode)
    label = f"row {cell['row'] + 1}, cell {cell['col'] + 1}"
    if problems:
        ctx.pack_dir.drop(cell["key"])
        ctx.state.record(cell["key"], source, "fail", problems)
        result = {"key": cell["key"], "status": "fail", "reasons": problems, "cell": label}
        ctx.results.append(result)
        return result
    if fit_mode != "canvas":
        keyed = imaging.remove_specks(keyed)
    result = process_keyed(ctx, cell["key"], keyed, source)
    result["cell"] = label
    return result


def inbox_items(ctx: Context) -> tuple[list[str], list[str]]:
    """Every single key and sheet id waiting in the pack's inbox."""
    keys, sheets = [], []
    for path in sorted(ctx.inbox.rglob("*")):
        relative = path.relative_to(ctx.inbox)
        if path.suffix.lower() not in INBOX_EXTS or any(part.startswith(("_", ".")) for part in relative.parts):
            continue
        name = relative.with_suffix("").as_posix()
        bucket = keys if "/" in name else sheets
        if name not in bucket:
            bucket.append(name)
    return keys, sheets


def build_context(args) -> Context:
    overrides = {"name": args.name, "version": args.pack_version, "description": args.description, "samples": args.sample or None}
    meta = packfile.pack_meta(args.pack, overrides)
    out = common.pack_out_dir(args.pack, args.out_root)
    entries = catalog_io.entries_by_key(catalog_io.load(args.catalog)) if args.catalog.is_file() else {}
    return Context(
        pack=args.pack,
        pack_dir=packfile.PackDir(out, args.pack, meta),
        state=packfile.RunState(args.work, args.pack),
        inbox=(args.inbox or common.INBOX_DIR / args.pack),
        refs=args.refs,
        sheets=(args.sheets or common.SHEETS_DIR / args.pack),
        work=args.work,
        assets=args.assets,
        entries=entries,
        force=args.force,
    )


def parse_args(argv):
    parser = argparse.ArgumentParser(description="Turn downloaded Flow images into pack files.")
    parser.add_argument("pack")
    parser.add_argument("--key", action="append", default=[], help="a single (inbox/<pack>/<key>.jpg|png|webp)")
    parser.add_argument("--sheet", action="append", default=[], help="a sheet batch id (inbox/<pack>/<id>.jpg|png|webp)")
    parser.add_argument("--all", action="store_true", help="everything in the pack's inbox")
    parser.add_argument("--check", action="store_true", help="QA the pack and write contact sheets")
    parser.add_argument("--force", action="store_true", help="reprocess unchanged inputs")
    parser.add_argument("--inbox", type=Path, help="default: tools/illustrations/inbox/<pack>")
    parser.add_argument("--out-root", type=Path, help="pack folders go to <dir>/<pack> (default: assets/packs/flat or out/packs/<pack>)")
    parser.add_argument("--work", type=Path, default=common.WORK_DIR, help="state, QA reports and contact sheets")
    parser.add_argument("--refs", type=Path, default=common.REFS_DIR)
    parser.add_argument("--sheets", type=Path, help="default: tools/illustrations/sheets/<pack>")
    parser.add_argument("--catalog", type=Path, default=common.CATALOG_FILE)
    parser.add_argument("--assets", type=Path, default=common.ASSETS_DIR)
    parser.add_argument("--name")
    parser.add_argument("--pack-version")
    parser.add_argument("--description")
    parser.add_argument("--sample", action="append", default=[], help="a sample key shown in Settings (repeatable)")
    return parser.parse_args(argv)


def main(argv=None) -> int:
    args = parse_args(argv)
    ctx = build_context(args)
    keys, sheet_ids = list(args.key), list(args.sheet)
    if args.all:
        found_keys, found_sheets = inbox_items(ctx)
        keys += [k for k in found_keys if k not in keys]
        sheet_ids += [s for s in found_sheets if s not in sheet_ids]
    for key in keys:
        process_single(ctx, key)
    for sheet_id in sheet_ids:
        process_sheet(ctx, sheet_id)
    ctx.pack_dir.save()
    ctx.state.save()
    failed = report(ctx)
    if args.check:
        import qa

        failed = qa.check(ctx) or failed
    return 1 if failed else 0


def report(ctx: Context) -> bool:
    passed = [r for r in ctx.results if r["status"] == "pass"]
    bad = [r for r in ctx.results if r["status"] != "pass"]
    print(f"{ctx.pack}: {len(passed)} written, {len(bad)} not, {len(ctx.pack_dir.coverage)} in pack.json ({ctx.pack_dir.root})")
    for result in bad:
        where = f" [{result['cell']}]" if "cell" in result else ""
        print(f"  REGENERATE {result['key']}{where}: {'; '.join(result['reasons'])}")
    return bool(bad)


if __name__ == "__main__":
    sys.exit(main())
