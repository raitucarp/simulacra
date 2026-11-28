"""Shared fixtures of the tooling tests: synthetic Flow-like images and tiny asset trees.

Run the suite from the repository root:
    python -m unittest discover -s tools/illustrations/test -p "test_*.py"
"""

from __future__ import annotations

import shutil
import sys
import tempfile
from pathlib import Path

TOOLS = Path(__file__).resolve().parents[1]
if str(TOOLS) not in sys.path:
    sys.path.insert(0, str(TOOLS))

from PIL import Image, ImageDraw, ImageFilter  # noqa: E402

MAGENTA = (255, 0, 255)
OFF_MAGENTA = (236, 18, 229)
SUPERSAMPLE = 4


def scratch(name: str) -> Path:
    """A fresh temp folder."""
    path = Path(tempfile.mkdtemp(prefix=f"simulacra-illus-{name}-"))
    return path


def cleanup(path: Path) -> None:
    shutil.rmtree(path, ignore_errors=True)


def subject_on(background=MAGENTA, size=(800, 600), shadow=True, box=None) -> Image.Image:
    """A white disc with a blue panel and a purple dot (anti-aliased), and optionally a soft
    drop shadow cast onto the background, like a careless generation."""
    width, height = size
    box = box or (int(width * 0.3), int(height * 0.18), int(width * 0.7), int(height * 0.72))
    image = Image.new("RGB", size, background)
    if shadow:
        mask = Image.new("L", size, 0)
        x0, y0, x1, y1 = box
        ImageDraw.Draw(mask).ellipse((x0 + 10, y1 - 10, x1 - 10, y1 + int(height * 0.1)), fill=110)
        image = Image.composite(Image.new("RGB", size, (0, 0, 0)), image, mask.filter(ImageFilter.GaussianBlur(12)))
    big = Image.new("RGBA", (width * SUPERSAMPLE, height * SUPERSAMPLE), (0, 0, 0, 0))
    draw = ImageDraw.Draw(big)
    x0, y0, x1, y1 = (v * SUPERSAMPLE for v in box)
    draw.ellipse((x0, y0, x1, y1), fill=(250, 250, 250, 255))
    cx, cy, rw, rh = (x0 + x1) // 2, (y0 + y1) // 2, (x1 - x0) // 4, (y1 - y0) // 4
    draw.rectangle((cx - rw, cy - rh, cx + rw, cy + rh), fill=(60, 120, 220, 255))
    draw.ellipse((cx - rw // 2, cy - rh // 2, cx + rw // 2, cy + rh // 2), fill=(140, 60, 190, 255))
    small = big.resize(size, Image.Resampling.LANCZOS)
    image.paste(small, (0, 0), small)
    return image


CREAM = (250, 240, 215)
CORAL = (255, 127, 80)


def cream_and_coral(size=(1024, 1024), background=MAGENTA) -> Image.Image:
    """A cream disc, an overlapping coral card and a thin cream line, anti-aliased, on magenta:
    the colours that showed a pink fringe in the first Flow tests."""
    width, height = size
    big = Image.new("RGB", (width * SUPERSAMPLE, height * SUPERSAMPLE), background)
    draw = ImageDraw.Draw(big)
    s = SUPERSAMPLE
    draw.ellipse((width * 0.2 * s, height * 0.2 * s, width * 0.65 * s, height * 0.72 * s), fill=CREAM)
    draw.rounded_rectangle((width * 0.5 * s, height * 0.4 * s, width * 0.82 * s, height * 0.8 * s), radius=width * 0.08 * s, fill=CORAL)
    draw.line((width * 0.15 * s, height * 0.85 * s, width * 0.85 * s, height * 0.87 * s), fill=CREAM, width=int(width * 0.012 * s))
    return big.resize(size, Image.Resampling.LANCZOS)


def as_jpeg(image: Image.Image, quality: int = 85) -> Image.Image:
    """`image` after a round trip through a 4:2:0 JPEG (what Flow serves)."""
    import io

    buffer = io.BytesIO()
    image.convert("RGB").save(buffer, "JPEG", quality=quality, subsampling="4:2:0")
    return Image.open(io.BytesIO(buffer.getvalue())).convert("RGB")


def disc_layer(size=(1024, 1024), centre=(0.5, 0.45), radius=0.28, background=MAGENTA, grey=True) -> Image.Image:
    """A shaded grey (#808080 key colour) disc on magenta, like a generated face layer."""
    width, height = size
    image = Image.new("RGB", size, background)
    big = Image.new("RGBA", (width * 2, height * 2), (0, 0, 0, 0))
    draw = ImageDraw.Draw(big)
    cx, cy, r = centre[0] * width * 2, centre[1] * height * 2, radius * min(size) * 2
    colour = (128, 128, 128, 255) if grey else (200, 150, 110, 255)
    draw.ellipse((cx - r, cy - r, cx + r, cy + r), fill=colour)
    draw.ellipse((cx - r * 0.7, cy - r * 0.8, cx + r * 0.1, cy - r * 0.1), fill=(160, 160, 160, 255))
    draw.ellipse((cx - r * 0.2, cy + r * 0.3, cx + r * 0.8, cy + r * 0.85), fill=(100, 100, 100, 255))
    small = big.resize(size, Image.Resampling.LANCZOS)
    image.paste(small, (0, 0), small)
    return image


def reference_disc(size=(512, 512), centre=(0.5, 0.45), radius=0.28) -> Image.Image:
    """A transparent reference render whose alpha is the disc of `disc_layer`."""
    image = Image.new("RGBA", size, (0, 0, 0, 0))
    cx, cy, r = centre[0] * size[0], centre[1] * size[1], radius * min(size)
    ImageDraw.Draw(image).ellipse((cx - r, cy - r, cx + r, cy + r), fill=(128, 128, 128, 255))
    return image


SVG = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" fill="none"><rect x="4" y="4" width="10" height="10" fill="{{{{accent}}}}"/></svg>'
BLANK_SVG = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" fill="none"></svg>'


def asset_tree(root: Path, keys: dict) -> Path:
    """Writes `<root>/illustrations/<key>.svg` for `{key: (w, h)}` (a `None` size = blank layer)."""
    assets = root / "illustrations"
    for key, size in keys.items():
        path = assets / f"{key}.svg"
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(BLANK_SVG if size is None else SVG.format(w=size[0], h=size[1]), encoding="utf-8")
    return assets
