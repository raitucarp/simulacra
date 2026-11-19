"""Draws Simulacra's app icon (the splash brand mark on a dark tile) and writes every size the
installers need: PNGs, a Windows .ico and a macOS .icns. Run from the repository root:

    python packaging/icons/make_icons.py

Colors are the design-system defaults from AGENTS.md (brand violet, dialectic gold, card and
elevated surfaces), so the icon matches the app's default dark theme.
"""
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw

OUT = Path(__file__).resolve().parent
MASTER = 1024
SUPERSAMPLE = 4
CARD = (8, 8, 10, 255)
ELEVATED = (17, 17, 21, 255)
FG = (244, 244, 245, 255)
VIOLET = (139, 92, 246, 255)
GOLD = (234, 179, 8, 255)
PNG_SIZES = [16, 32, 48, 64, 128, 256, 512, 1024]
ICO_SIZES = [16, 24, 32, 48, 64, 128, 256]


def scaled(value):
    """Maps a coordinate of the 160-wide brand viewBox onto the supersampled canvas."""
    size = MASTER * SUPERSAMPLE
    return value * size / 160


def circle_mask(cx, cy, r, size):
    mask = Image.new("L", (size, size), 0)
    ImageDraw.Draw(mask).ellipse([scaled(cx - r), scaled(cy - r), scaled(cx + r), scaled(cy + r)], fill=255)
    return mask


def fill(canvas, mask, color, alpha=1.0):
    layer = Image.new("RGBA", canvas.size, color)
    layer.putalpha(mask.point(lambda v: int(v * alpha)))
    canvas.alpha_composite(layer)


def ring(canvas, cx, cy, r, width):
    ImageDraw.Draw(canvas).ellipse(
        [scaled(cx - r), scaled(cy - r), scaled(cx + r), scaled(cy + r)], outline=FG, width=int(scaled(width))
    )


def draw_master():
    size = MASTER * SUPERSAMPLE
    canvas = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    tile = Image.new("L", (size, size), 0)
    ImageDraw.Draw(tile).rounded_rectangle([0, 0, size - 1, size - 1], radius=int(size * 0.22), fill=255)
    fill(canvas, tile, CARD)
    # The brand mark sits on a 160-wide grid; shift it so the two circles are centred.
    left, right = circle_mask(64, 80, 34, size), circle_mask(96, 80, 34, size)
    fill(canvas, left, VIOLET)
    fill(canvas, right, GOLD, 0.7)
    fill(canvas, ImageChops.multiply(left, right), ELEVATED)
    ring(canvas, 64, 80, 34, 3)
    ring(canvas, 96, 80, 34, 3)
    draw = ImageDraw.Draw(canvas)
    draw.line([scaled(80), scaled(66), scaled(80), scaled(94)], fill=VIOLET, width=int(scaled(2)))
    for cy, color in ((66, VIOLET), (80, GOLD), (94, VIOLET)):
        draw.ellipse([scaled(76), scaled(cy - 4), scaled(84), scaled(cy + 4)], fill=color)
    return canvas.resize((MASTER, MASTER), Image.LANCZOS)


def main():
    master = draw_master()
    for size in PNG_SIZES:
        master.resize((size, size), Image.LANCZOS).save(OUT / f"{size}x{size}.png")
    master.save(OUT / "icon.png")
    master.save(OUT / "icon.ico", sizes=[(s, s) for s in ICO_SIZES])
    master.save(OUT / "icon.icns")


if __name__ == "__main__":
    main()
