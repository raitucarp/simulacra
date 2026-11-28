"""grid.regions: cells follow the generated sheet's own gaps, not the reference spacing."""

import sys
import unittest
from pathlib import Path

from PIL import Image, ImageDraw

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import grid  # noqa: E402


def layout_3x3_band(count=8):
    """The reference: 4:3 cells in a 3x3 grid centred in a square sheet (a band of it)."""
    cells, size, cw, ch, gutter = [], 900, 260, 195, 30
    oy = (size - (3 * ch + 2 * gutter)) // 2
    ox = (size - (3 * cw + 2 * gutter)) // 2
    for i in range(count):
        row, col = divmod(i, 3)
        cells.append({"key": f"k{i}", "row": row, "col": col, "box": [ox + col * (cw + gutter), oy + row * (ch + gutter), cw, ch]})
    return {"grid": [3, 3], "size": [size, size], "cells": cells}


class Regions(unittest.TestCase):
    def test_a_grid_spread_over_the_canvas_is_found(self):
        layout = layout_3x3_band()
        alpha = Image.new("L", (900, 900), 0)
        draw = ImageDraw.Draw(alpha)
        centres = {}
        for i in range(8):
            row, col = divmod(i, 3)
            cx, cy = 150 + col * 300, 150 + row * 300
            if row == 2:
                cx = 300 + col * 300  # the short last row is centred
            centres[f"k{i}"] = (cx, cy)
            draw.ellipse((cx - 110, cy - 120, cx + 110, cy + 120), fill=255)  # taller than the reference cell
            draw.ellipse((cx + 112, cy - 135, cx + 128, cy - 120), fill=255)  # a sparkle beside it
        found = grid.regions(alpha, layout, (1.0, 1.0))
        for key, (cx, cy) in centres.items():
            x0, y0, x1, y1 = found[key]
            self.assertTrue(x0 < cx - 110 and cx + 128 < x1, f"{key} x {found[key]}")
            self.assertTrue(y0 < cy - 135 and cy + 120 < y1, f"{key} y {found[key]}")

    def test_without_gaps_the_reference_boundaries_stay(self):
        layout = layout_3x3_band(9)
        found = grid.regions(Image.new("L", (900, 900), 255), layout, (1.0, 1.0))
        self.assertEqual(len(found), 9)
        self.assertEqual(found["k0"][0], 0)
        self.assertEqual(found["k8"][2], 900)


if __name__ == "__main__":
    unittest.main()
