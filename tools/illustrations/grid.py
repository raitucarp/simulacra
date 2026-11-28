"""Where a generated sprite sheet's cells really are.

The model keeps the reference sheet's rows and columns but not its exact spacing: a 3x3 grid
that sits in a band of the reference is often spread over the whole canvas, or shifted. The
reference layout is only the starting guess. Each boundary between rows (and, within a row,
between cells) moves to the nearest empty magenta band of the generated image, so a cell's
region runs from gap to gap and a subject is never clipped by where the reference put it.
"""

from __future__ import annotations

from PIL import Image

# Mean alpha (0-255) at or below which a row or column of pixels counts as empty.
EMPTY = 3
# How far (as a share of the cell size) a boundary may move to reach a gap.
REACH = 0.45


def profile(alpha: Image.Image, axis: str) -> list[int]:
    """Mean alpha of every column (`axis="x"`) or row (`"y"`)."""
    w, h = alpha.size
    line = alpha.resize((w, 1) if axis == "x" else (1, h), Image.Resampling.BOX)
    return list(line.tobytes())


def gaps(values: list[int]) -> list[tuple[int, int]]:
    """Runs [start, end) of empty positions."""
    out, start = [], None
    for i, v in enumerate(values + [EMPTY + 1]):
        if v <= EMPTY and start is None:
            start = i
        elif v > EMPTY and start is not None:
            out.append((start, i))
            start = None
    return out


def snap(expected: float, values: list[int], reach: float) -> int:
    """The centre of the widest gap near `expected` (closer and wider wins), else `expected`."""
    best, score = None, None
    for start, end in gaps(values):
        if start == 0 or end == len(values):
            continue  # the margins outside the grid are not boundaries
        centre = (start + end) / 2
        distance = abs(centre - expected)
        if distance > reach:
            continue
        s = (end - start) - 0.5 * distance
        if score is None or s > score:
            best, score = centre, s
    return round(best if best is not None else expected)


def widest(values: list[int], count: int) -> list[int]:
    """Centres of the `count` widest inner gaps, left to right."""
    inner = [g for g in gaps(values) if g[0] > 0 and g[1] < len(values)]
    chosen = sorted(inner, key=lambda g: g[1] - g[0], reverse=True)[:count]
    return sorted(round((a + b) / 2) for a, b in chosen)


def mapper(lo: float, hi: float, content: tuple[int, int]):
    """Linear map from the reference grid's extent [lo, hi] to the generated content's extent."""
    c0, c1 = content
    k = (c1 - c0) / max(1.0, hi - lo)
    return lambda v: c0 + (v - lo) * k, k


def regions(alpha: Image.Image, layout: dict, scale: tuple[float, float]) -> dict[str, tuple[int, int, int, int]]:
    """key -> (x0, y0, x1, y1) of every cell in the generated sheet whose alpha is `alpha`."""
    sx, sy = scale
    w, h = alpha.size
    bbox = alpha.point(lambda v: 255 if v > 128 else 0).getbbox() or (0, 0, w, h)
    cells = layout["cells"]
    boxes = {c["key"]: (c["box"][0] * sx, c["box"][1] * sy, c["box"][2] * sx, c["box"][3] * sy) for c in cells}
    fx, kx = mapper(min(b[0] for b in boxes.values()), max(b[0] + b[2] for b in boxes.values()), (bbox[0], bbox[2]))
    fy, ky = mapper(min(b[1] for b in boxes.values()), max(b[1] + b[3] for b in boxes.values()), (bbox[1], bbox[3]))
    rows = sorted({c["row"] for c in cells})
    by_row = {r: sorted((c for c in cells if c["row"] == r), key=lambda c: c["col"]) for r in rows}
    ys = profile(alpha, "y")
    row_edges = [0]
    for upper, lower in zip(rows, rows[1:]):
        a, b = boxes[by_row[upper][0]["key"]], boxes[by_row[lower][0]["key"]]
        expected = fy((a[1] + a[3] + b[1]) / 2)
        row_edges.append(snap(expected, ys, REACH * a[3] * ky))
    row_edges.append(h)
    out = {}
    for index, r in enumerate(rows):
        top, bottom = row_edges[index], row_edges[index + 1]
        xs = profile(alpha.crop((0, top, w, bottom)), "x")
        if len(by_row[r]) < layout["grid"][1]:
            edges = [0] + widest(xs, len(by_row[r]) - 1) + [w]  # a short row may be centred
        else:
            edges = [0]
            for left, right in zip(by_row[r], by_row[r][1:]):
                a, b = boxes[left["key"]], boxes[right["key"]]
                expected = fx((a[0] + a[2] + b[0]) / 2)
                edges.append(snap(expected, xs, REACH * a[2] * kx))
            edges.append(w)
        for i, cell in enumerate(by_row[r]):
            out[cell["key"]] = (edges[i], top, edges[i + 1], bottom)
    return out
