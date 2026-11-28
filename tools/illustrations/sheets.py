"""Composes the reference sheets of sprite-sheet batches and their multi-subject prompts.

    python tools/illustrations/sheets.py <pack> [--group G] [--batch ID ...] [--refs DIR]
        [--out DIR] [--catalog FILE] [--placeholder]

For every `sheet` batch of the catalog (2x2 or 3x3 cells of one group and canvas) this
writes `sheets/<pack>/<id>.png` (the Classic reference renders on solid #FF00FF, magenta
gutters), `<id>.txt` (the pack's sheet prompt, row by row and cell by cell) and `<id>.json`
(the grid, the Flow aspect to pick and each cell's key, `box` and `slot`), which
`process.py --sheet <id>` uses to slice the downloaded result.
"""

from __future__ import annotations

import argparse
import math
import sys
from pathlib import Path

from PIL import Image, ImageDraw

import catalog as catalog_io
import common
import imaging

# Flow-friendly sheet sizes (label, width, height), preferred first on ties.
SIZES = [("1:1", 2048, 2048), ("16:9", 2048, 1152), ("9:16", 1152, 2048), ("4:3", 2048, 1536), ("3:4", 1536, 2048)]
COLUMNS = {"sheet2x2": 2, "sheet3x3": 3}
GUTTER_SHARE = 0.06
SIZE_SLACK = 0.98
PLACEHOLDER_FILL = (128, 128, 128, 255)
LAYER_NOTE = (
    "Every cell is one layer of a stacked avatar: keep every shape exactly where the reference cell has it, "
    "same size and outline; paint the recolourable parts in neutral grey #808080 and second colours in pure cyan #00FFFF."
)


def grid_of(count: int, most: int) -> tuple[int, int]:
    """(rows, cols) of at most `most` x `most` holding `count` cells: fewest empty cells, then
    the squarest grid, then the wider one (4 cells -> 2x2, 5 -> 2x3, 3 -> 1x3)."""
    options = [(math.ceil(count / cols), cols) for cols in range(1, most + 1) if math.ceil(count / cols) <= most]
    return min(options, key=lambda rc: (rc[0] * rc[1] - count, abs(rc[0] - rc[1]), -rc[1]))


def layout(count: int, batch: str, canvas: tuple[int, int]) -> dict:
    """Grid, sheet size and cell boxes for `count` cells of `canvas` aspect."""
    rows, cols = grid_of(count, COLUMNS[batch])
    cw, ch = canvas
    k = max(1.0, ch / cw)
    natural = math.log((cols * cw) / (rows * ch))
    options = []
    for label, width, height in SIZES:
        side = min(width / (cols + GUTTER_SHARE * k * (cols + 1)), height / (rows * ch / cw + GUTTER_SHARE * k * (rows + 1)))
        options.append((math.floor(side), label, width, height))
    largest = max(option[0] for option in options)
    # Among the sizes giving (nearly) the largest cells, the one closest to the grid's own
    # aspect: the least spare magenta for the model to fill.
    near = [o for o in options if o[0] >= SIZE_SLACK * largest]
    cell_w, label, width, height = min(near, key=lambda o: abs(math.log(o[2] / o[3]) - natural))
    cell_h = round(cell_w * ch / cw)
    gutter = math.ceil(GUTTER_SHARE * k * cell_w)
    grid_w, grid_h = cols * cell_w + (cols - 1) * gutter, rows * cell_h + (rows - 1) * gutter
    ox, oy = (width - grid_w) // 2, (height - grid_h) // 2
    boxes = []
    for index in range(count):
        row, col = divmod(index, cols)
        x, y = ox + col * (cell_w + gutter), oy + row * (cell_h + gutter)
        half = gutter // 2
        slot = [max(0, x - half), max(0, y - half), min(width, x + cell_w + half) - max(0, x - half), min(height, y + cell_h + half) - max(0, y - half)]
        boxes.append({"row": row, "col": col, "box": [x, y, cell_w, cell_h], "slot": slot})
    return {"grid": [rows, cols], "size": [width, height], "aspect": label, "gutter": gutter, "cells": boxes}


def sheet_batches(catalog: dict, group: str | None = None, ids: list[str] | None = None) -> dict:
    """Sheet id -> its entries, in catalog order."""
    batches: dict[str, list[dict]] = {}
    for entry in catalog["entries"]:
        sheet = entry.get("sheet")
        if not sheet or (group and entry["group"] != group) or (ids and sheet not in ids):
            continue
        batches.setdefault(sheet, []).append(entry)
    return batches


def compose(entries: list[dict], plan: dict, refs: Path, placeholder: bool) -> Image.Image:
    sheet = Image.new("RGBA", tuple(plan["size"]), (*common.MAGENTA, 255))
    missing = [e["key"] for e in entries if not (refs / f"{e['key']}.png").is_file()]
    if missing and not placeholder:
        raise FileNotFoundError("missing reference renders (run the illustration-refs bin): " + ", ".join(missing))
    for entry, cell in zip(entries, plan["cells"]):
        x, y, w, h = cell["box"]
        path = refs / f"{entry['key']}.png"
        if path.is_file():
            with Image.open(path) as opened:
                ref = imaging.resize(opened.convert("RGBA"), (w, h))
            sheet.alpha_composite(ref, (x, y))
        else:
            ImageDraw.Draw(sheet).rounded_rectangle((x, y, x + w - 1, y + h - 1), radius=max(2, w // 12), fill=PLACEHOLDER_FILL)
    return sheet.convert("RGB")


def prompt(template: str, entries: list[dict], plan: dict) -> str:
    contexts = {e.get("context") for e in entries}
    shared = contexts.pop() if len(contexts) == 1 else None
    field = "cell" if shared else "subject"
    lines = [f"All cells: {shared.rstrip('.')}."] if shared else []
    lines += [f"Row {c['row'] + 1}, cell {c['col'] + 1}: {e.get(field) or e['subject']}" for e, c in zip(entries, plan["cells"])]
    if any(e["layered"] for e in entries):
        lines.append(LAYER_NOTE)
    values = {"rows": plan["grid"][0], "cols": plan["grid"][1], "aspect": plan["aspect"], "cells": "\n".join(lines)}
    for name, value in values.items():
        template = template.replace("{" + name + "}", str(value))
    return template


def write_sheet(pack: str, sheet_id: str, entries: list[dict], templates: dict, out: Path, refs: Path, placeholder: bool = False) -> dict:
    first = entries[0]
    plan = layout(len(entries), first["batch"], tuple(first["canvas"]))
    out.mkdir(parents=True, exist_ok=True)
    compose(entries, plan, refs, placeholder).save(out / f"{sheet_id}.png")
    template = templates.get(pack, {}).get("sheet_prompt") or "{cells}"
    (out / f"{sheet_id}.txt").write_text(prompt(template, entries, plan) + "\n", encoding="utf-8", newline="\n")
    cells = [{**cell, "key": entry["key"]} for entry, cell in zip(entries, plan["cells"])]
    data = {
        "id": sheet_id,
        "pack": pack,
        "grid": plan["grid"],
        "size": plan["size"],
        "aspect": plan["aspect"],
        "gutter": plan["gutter"],
        "canvas": first["canvas"],
        "layered": first["layered"],
        "fit": first["fit"],
        "cells": cells,
    }
    common.write_json(out / f"{sheet_id}.json", data)
    return data


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description="Compose reference sheets and their prompts.")
    parser.add_argument("pack", choices=common.PACK_IDS)
    parser.add_argument("--group")
    parser.add_argument("--batch", action="append", default=[], help="a sheet id (repeatable)")
    parser.add_argument("--refs", type=Path, default=common.REFS_DIR)
    parser.add_argument("--out", type=Path, help="default: tools/illustrations/sheets/<pack>")
    parser.add_argument("--catalog", type=Path, default=common.CATALOG_FILE)
    parser.add_argument("--placeholder", action="store_true", help="grey boxes for missing reference renders")
    args = parser.parse_args(argv)
    data = catalog_io.load(args.catalog)
    batches = sheet_batches(data, args.group, args.batch)
    out = args.out or common.SHEETS_DIR / args.pack
    try:
        for sheet_id, entries in batches.items():
            write_sheet(args.pack, sheet_id, entries, data.get("templates", {}), out, args.refs, args.placeholder)
    except FileNotFoundError as err:
        print(f"sheets: {err}", file=sys.stderr)
        return 1
    print(f"sheets: {len(batches)} sheets for {args.pack} in {out}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
