"""Crops a capture to what changed since a reference capture: the menu, popover, dialog, sheet,
tooltip or toast that opened in between.

    python autocrop.py <before.png> <after.png> <out.png> [pad] [min_x min_y max_x max_y]

A modal's scrim dims everything, so the "before" image is first scaled by the dimming factor
(estimated from the median brightness ratio); what still differs is the new surface. Prints the
crop box as "x y w h". With no change found the whole "after" image is kept.
"""
import sys

import numpy as np
from PIL import Image


def main() -> None:
    before_path, after_path, out_path = sys.argv[1:4]
    pad = int(sys.argv[4]) if len(sys.argv) > 4 else 10
    limits = [int(v) for v in sys.argv[5:9]] if len(sys.argv) > 8 else None

    after_img = Image.open(after_path).convert("RGB")
    before = np.asarray(Image.open(before_path).convert("RGB"), dtype=np.float32)
    after = np.asarray(after_img, dtype=np.float32)
    if before.shape != after.shape:
        after_img.save(out_path)
        print(f"0 0 {after.shape[1]} {after.shape[0]}")
        return

    lum_b = before.mean(axis=2)
    lum_a = after.mean(axis=2)
    # Dimming factor of a scrim: the typical ratio over pixels that were not near-black.
    lit = lum_b > 24
    alpha = float(np.median(lum_a[lit] / lum_b[lit])) if lit.any() else 1.0
    alpha = min(1.0, max(0.2, alpha))
    expected = before * alpha
    diff = np.abs(after - expected).max(axis=2)
    changed = diff > 26

    if limits:
        mask = np.zeros_like(changed)
        x0, y0, x1, y1 = limits
        mask[y0:y1, x0:x1] = True
        changed &= mask

    # Ignore thin noise: keep rows and columns with a meaningful run of changed pixels.
    rows = np.where(changed.sum(axis=1) >= 6)[0]
    cols = np.where(changed.sum(axis=0) >= 6)[0]
    height, width = changed.shape
    if rows.size == 0 or cols.size == 0:
        after_img.save(out_path)
        print(f"0 0 {width} {height}")
        return
    x0 = max(0, int(cols[0]) - pad)
    y0 = max(0, int(rows[0]) - pad)
    x1 = min(width, int(cols[-1]) + 1 + pad)
    y1 = min(height, int(rows[-1]) + 1 + pad)
    after_img.crop((x0, y0, x1, y1)).save(out_path)
    print(f"{x0} {y0} {x1 - x0} {y1 - y0}")


if __name__ == "__main__":
    main()
