"""`process.py --check`: automated QA of every file of a pack, a QA report, and one contact
sheet per group for visual review.

Checks per file: it exists and matches its sha256 in pack.json; its size is the stored size
and its aspect within 2 % of the canvas; trimmed art has transparent corners, an alpha
coverage within [0.08, 0.92] and (like everything) almost no magenta left. Layers that failed
alignment while processing are reported too (they are not in pack.json, so Classic draws them).
"""

from __future__ import annotations

from collections import defaultdict
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

import common
import imaging

COVERAGE_RANGE = (0.08, 0.92)
MAGENTA_LIMIT = 0.002
ASPECT_TOLERANCE = 0.02
TILE = 160
ITEMS_PER_ROW = 4
LABEL_HEIGHT = 18
PAD = 8
LIGHT = (244, 241, 234)
DARK = (29, 34, 48)
CHECKER = ((200, 200, 200), (255, 255, 255))
FAIL_COLOUR = (220, 40, 40)
MARKER_COLOUR = (255, 200, 0)


def file_problems(path: Path, entry: dict, info: dict | None) -> list[str]:
    if not path.is_file():
        return [f"missing file {entry['file']}"]
    if common.sha256_file(path) != entry["sha256"]:
        return ["sha256 does not match pack.json"]
    with Image.open(path) as opened:
        image = opened.convert("RGBA")
    problems = []
    if (image.width, image.height) != (entry["width"], entry["height"]):
        problems.append(f"size {image.width}x{image.height} differs from pack.json {entry['width']}x{entry['height']}")
    if info:
        problems += _shape_problems(image, info)
    if imaging.magenta_share(image) > MAGENTA_LIMIT:
        problems.append(f"leftover magenta: {imaging.magenta_share(image):.2%} of opaque pixels")
    return problems


def _shape_problems(image: Image.Image, info: dict) -> list[str]:
    problems = []
    canvas, stored = info["canvas"], info["stored"]
    if [image.width, image.height] != list(stored):
        problems.append(f"size {image.width}x{image.height}, expected {stored[0]}x{stored[1]}")
    want, have = canvas[0] / canvas[1], image.width / image.height
    if abs(have / want - 1.0) > ASPECT_TOLERANCE:
        problems.append(f"aspect {have:.3f} differs from the canvas {want:.3f}")
    if info["fit"] == "trim":
        if not imaging.corners_transparent(image):
            problems.append("corners are not transparent")
        coverage = imaging.alpha_coverage(image)
        if not COVERAGE_RANGE[0] <= coverage <= COVERAGE_RANGE[1]:
            problems.append(f"alpha coverage {coverage:.2f} outside {COVERAGE_RANGE[0]}-{COVERAGE_RANGE[1]}")
    return problems


def check(ctx) -> bool:
    """QA of `ctx`'s pack; writes the report and contact sheets; True when anything failed."""
    report = {}
    for key, entry in sorted(ctx.pack_dir.coverage.items()):
        info = _info(ctx, key)
        problems = file_problems(ctx.pack_dir.root / entry["file"], entry, info)
        report[key] = {"status": "fail" if problems else "pass", "reasons": problems}
    for key, item in ctx.state.items.items():
        if item.get("status") == "fail" and key not in report:
            report[key] = {"status": "fail", "reasons": item.get("reasons", []), "fallback": "classic"}
    common.write_json(ctx.work / "qa" / f"{ctx.pack}.json", report)
    sheets = contact_sheets(ctx, report)
    failed = [k for k, r in report.items() if r["status"] == "fail"]
    print(f"{ctx.pack} QA: {len(report) - len(failed)} pass, {len(failed)} fail; report {ctx.work / 'qa' / (ctx.pack + '.json')}")
    for key in failed:
        print(f"  FAIL {key}: {'; '.join(report[key]['reasons'])}")
    for path in sheets:
        print(f"  contact sheet {path}")
    return bool(failed)


def _info(ctx, key: str) -> dict | None:
    try:
        return ctx.entry(key)
    except KeyError:
        return None


def contact_sheets(ctx, report: dict) -> list[Path]:
    groups = defaultdict(list)
    for key in sorted(ctx.pack_dir.coverage):
        groups[key.split("/", 1)[0]].append(key)
    written = []
    for group, keys in sorted(groups.items()):
        path = ctx.work / f"contact-{ctx.pack}-{group}.png"
        path.parent.mkdir(parents=True, exist_ok=True)
        contact_sheet(ctx, keys, report).save(path)
        written.append(path)
    return written


def _font():
    try:
        return ImageFont.load_default(size=12)
    except TypeError:
        return ImageFont.load_default()


def _backdrop(size, kind: str) -> Image.Image:
    if kind != "checker":
        return Image.new("RGBA", size, (*(LIGHT if kind == "light" else DARK), 255))
    tile = Image.new("RGBA", size, (*CHECKER[1], 255))
    draw = ImageDraw.Draw(tile)
    step = 8
    for y in range(0, size[1], step):
        for x in range(0, size[0], step):
            if (x // step + y // step) % 2 == 0:
                draw.rectangle((x, y, x + step - 1, y + step - 1), fill=(*CHECKER[0], 255))
    return tile


def _panel(image: Image.Image, kind: str, key: str) -> Image.Image:
    panel = _backdrop(image.size, kind)
    panel.alpha_composite(image)
    room = key.split("/")[-1]
    if key.startswith("palace/") and room in common.PALACE_LANDMARKS:
        draw = ImageDraw.Draw(panel)
        for _, x, y in common.PALACE_LANDMARKS[room]:
            cx, cy = x / common.PALACE_CANVAS[0] * panel.width, y / common.PALACE_CANVAS[1] * panel.height
            draw.ellipse((cx - 4, cy - 4, cx + 4, cy + 4), outline=MARKER_COLOUR, width=2)
    return panel


def contact_sheet(ctx, keys: list[str], report: dict) -> Image.Image:
    """Each image on a checkerboard, a light and a dark backdrop, its key underneath;
    failures framed in red and palace rooms marked with their five landmarks."""
    tiles = []
    for key in keys:
        with Image.open(ctx.pack_dir.root / ctx.pack_dir.coverage[key]["file"]) as opened:
            image = opened.convert("RGBA")
        scale = TILE / max(image.size)
        thumb = imaging.resize(image, (max(1, round(image.width * scale)), max(1, round(image.height * scale))))
        tiles.append((key, [_panel(thumb, kind, key) for kind in ("checker", "light", "dark")]))
    cell_w = 3 * TILE + 4 * PAD
    cell_h = TILE + LABEL_HEIGHT + 2 * PAD
    rows = max(1, -(-len(tiles) // ITEMS_PER_ROW))
    sheet = Image.new("RGBA", (ITEMS_PER_ROW * cell_w, rows * cell_h), (255, 255, 255, 255))
    draw, font = ImageDraw.Draw(sheet), _font()
    for index, (key, panels) in enumerate(tiles):
        x0, y0 = (index % ITEMS_PER_ROW) * cell_w, (index // ITEMS_PER_ROW) * cell_h
        for n, panel in enumerate(panels):
            sheet.alpha_composite(panel, (x0 + PAD + n * (TILE + PAD), y0 + PAD))
        if report.get(key, {}).get("status") == "fail":
            draw.rectangle((x0 + 2, y0 + 2, x0 + cell_w - 3, y0 + cell_h - 3), outline=FAIL_COLOUR, width=3)
        draw.text((x0 + PAD, y0 + PAD + TILE + 2), key, fill=(20, 20, 20), font=font)
    return sheet.convert("RGB")
