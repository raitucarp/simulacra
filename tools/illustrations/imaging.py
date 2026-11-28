"""Pixel work of the pack pipeline (Pillow only, vectorised through ImageMath/ImageChops):
keying the magenta background out to alpha, fitting to the canvas, WebP encoding, connected
subjects and alignment (IoU) checks.
"""

from __future__ import annotations

import io
from collections import deque

from PIL import Image, ImageChops, ImageFilter, ImageMath

import common

# Keying (after the coordinator's Flow prototype): magentaness m = clamp(min(R, B) - G), taken
# relative to the backdrop's own m, so a drifted backdrop keys like pure #FF00FF.
RAMP = (0.35, 0.75)  # relative m at or below the first: opaque; at or above the second: transparent
CORE_ALPHA = 0.6  # ramp alpha from which a pixel may be interior (a soft cast shadow stays below)
EDGE_PX = 2  # the edge band: pixels within this distance of a non-opaque pixel
EXTEND_RADIUS = 3.0  # how far an edge pixel looks inward for the subject's own colour
EXTEND_MIN_ALPHA = 0.0  # edge pixels above this alpha take that colour when tinted (low-alpha unmixing is noisy)
EXTEND_MIN_WEIGHT = 0.12  # share of clean pixels nearby needed to trust the inward colour
TINTED_M = 4.0  # magentaness (0-255) above which an edge pixel counts as tinted by the backdrop
SPILL_ALLOWANCE = 6.0  # magentaness (0-255) any edge pixel keeps at most
FEATHER_PX = 1.0
MIN_BG_MAGENTANESS = 96
# Trim fit: the subject fills this share of the canvas' limiting dimension.
TRIM_FILL = 0.8
ALPHA_SOLID = 8  # alpha above this counts as subject for bounding boxes
SPECK_SHARE = 0.005
# Parts at least this share of a cell's alpha mass are kept (sparkles, dashes); smaller ones are
# JPEG noise or slivers and are dropped.
KEEP_SHARE = 0.0015
COMPONENT_GRID = 160
MAGENTA_DISTANCE = 100


def _eval(fn, **images):
    return ImageMath.lambda_eval(fn, **images)


def _to_l(image_f: Image.Image) -> Image.Image:
    return _eval(lambda x: x["a"] + 0.5, a=image_f).convert("L")


def estimate_background(image: Image.Image) -> tuple[int, int, int]:
    """The (possibly drifted) magenta of the image border; pure magenta when unclear."""
    rgb = image.convert("RGB")
    width, height = rgb.size
    step = max(1, (width + height) // 400)
    ring = [(x, y) for x in range(0, width, step) for y in (0, 1, height - 2, height - 1)]
    ring += [(x, y) for y in range(0, height, step) for x in (0, 1, width - 2, width - 1)]
    samples = [rgb.getpixel(p) for p in ring]
    magenta = [p for p in samples if min(p[0], p[2]) - p[1] > MIN_BG_MAGENTANESS]
    if len(magenta) < len(samples) // 4:
        return common.MAGENTA
    return tuple(sorted(channel)[len(channel) // 2] for channel in zip(*magenta))


def flatten(image: Image.Image) -> Image.Image:
    """RGB of `image`; transparent parts of an image with alpha are put on magenta first."""
    if image.mode in ("RGBA", "LA", "PA") or (image.mode == "P" and "transparency" in image.info):
        rgba = image.convert("RGBA")
        if rgba.getchannel("A").getextrema()[0] < 255:
            backdrop = Image.new("RGBA", rgba.size, (*common.MAGENTA, 255))
            backdrop.alpha_composite(rgba)
            return backdrop.convert("RGB")
    return image.convert("RGB")


def key_background(image: Image.Image, background: tuple[int, int, int] | None = None) -> Image.Image:
    """`image` with its magenta backdrop removed. Alpha follows a soft ramp of the relative
    magentaness (interior pixels forced opaque); colours are unmixed from the backdrop, edge
    pixels tinted by it (anti-aliasing, JPEG chroma subsampling) take the subject's own colour
    from just inside, any leftover magenta cast is clamped away, and the edge is feathered 1 px."""
    rgb = flatten(image)
    bg = background or estimate_background(rgb)
    bg_m = float(max(1, min(bg[0], bg[2]) - bg[1]))
    red, green, blue = (channel.convert("F") for channel in rgb.split())
    m = _eval(lambda x: x["max"](x["min"](x["r"], x["b"]) - x["g"], 0.0), r=red, g=green, b=blue)
    low, high = RAMP
    ramp = _eval(lambda x: 1.0 - x["max"](x["min"]((x["m"] / bg_m - low) / (high - low), 1.0), 0.0), m=m)
    core = _flag_mask(_eval(lambda x: x["float"](x["a"] >= CORE_ALPHA), a=ramp)).filter(ImageFilter.MinFilter(2 * EDGE_PX + 1)).convert("F")
    alpha = _eval(lambda x: x["max"](x["c"] / 255.0, x["a"]), a=ramp, c=core)
    solid = _flag_mask(_eval(lambda x: x["float"](x["a"] >= 0.999), a=alpha)).filter(ImageFilter.MinFilter(2 * EDGE_PX + 1))
    band = _eval(lambda x: x["float"](x["a"] > 0.0) * (1.0 - x["s"] / 255.0), a=alpha, s=solid.convert("F"))
    unmixed = [_unmix(c, m, float(b), bg_m) for c, b in zip((red, green, blue), bg)]
    channels = _extend_inward(unmixed, rgb, solid, alpha, band, m)
    channels = _suppress_spill(channels, band)
    inside = solid.convert("F")  # the solid interior keeps exactly the colours it was drawn in
    channels = [_eval(lambda x: (x["s"] * x["o"] + (255.0 - x["s"]) * x["c"]) / 255.0, s=inside, o=o, c=c) for o, c in zip((red, green, blue), channels)]
    alpha_l = _to_l(_eval(lambda x: x["a"] * 255.0, a=alpha))
    feathered = ImageChops.darker(alpha_l, alpha_l.filter(ImageFilter.GaussianBlur(FEATHER_PX)))
    return Image.merge("RGBA", [_to_l(c) for c in channels] + [feathered])


def _flag_mask(flags: Image.Image) -> Image.Image:
    """A 0/1 float image as an L mask (0/255)."""
    return _to_l(_eval(lambda x: x["f"] * 255.0, f=flags))


def _unmix(channel: Image.Image, m: Image.Image, bg: float, bg_m: float) -> Image.Image:
    """Observed = c*F + (1-c)*B solved for F, with the coverage c estimated as 1 - m/m_bg."""
    return _eval(
        lambda x: x["max"](x["min"]((x["c"] - (x["m"] / bg_m) * bg) / x["max"](1.0 - x["m"] / bg_m, 0.02), 255.0), 0.0),
        c=channel, m=m,
    )


def _extend_inward(unmixed, rgb: Image.Image, solid: Image.Image, alpha, band, m):
    """Edge pixels tinted by the backdrop take the average colour of the solid subject pixels
    within a few pixels (where there are enough of them); others keep their unmixed colour."""
    source = rgb.convert("RGBA")
    source.putalpha(solid)
    spread = source.convert("RGBa").filter(ImageFilter.GaussianBlur(EXTEND_RADIUS)).convert("RGBA")
    weight = spread.getchannel("A").convert("F")
    use = _eval(
        lambda x: x["b"] * x["float"](x["a"] > EXTEND_MIN_ALPHA) * x["float"](x["m"] > TINTED_M) * x["float"](x["w"] >= EXTEND_MIN_WEIGHT * 255.0),
        b=band, a=alpha, m=m, w=weight,
    )
    inward = [c.convert("F") for c in spread.split()[:3]]
    return [_eval(lambda x: x["u"] * x["i"] + (1.0 - x["u"]) * x["c"], u=use, i=i, c=c) for i, c in zip(inward, unmixed)]


def _suppress_spill(channels, band):
    """Clamps a leftover magenta cast on edge pixels: R and B come down to at most G plus a
    small allowance wherever both exceed it."""
    red, green, blue = channels
    spill = _eval(lambda x: x["max"](x["min"](x["r"], x["b"]) - x["g"] - SPILL_ALLOWANCE, 0.0) * x["w"], r=red, g=green, b=blue, w=band)
    red = _eval(lambda x: x["max"](x["r"] - x["s"], 0.0), r=red, s=spill)
    blue = _eval(lambda x: x["max"](x["b"] - x["s"], 0.0), b=blue, s=spill)
    return [red, green, blue]


def subject_bbox(image: Image.Image):
    return image.getchannel("A").point(lambda v: 255 if v > ALPHA_SOLID else 0).getbbox()


def fit_trim(image: Image.Image, canvas: tuple[int, int], fill: float = TRIM_FILL) -> Image.Image:
    """Crops to the subject and pads it, centred, to the canvas aspect."""
    box = subject_bbox(image)
    if box is None:
        raise ValueError("no subject left after keying")
    subject = image.crop(box)
    sw, sh = subject.size
    aspect = canvas[0] / canvas[1]
    if sw / sh > aspect:
        out_w = sw / fill
        out_h = out_w / aspect
    else:
        out_h = sh / fill
        out_w = out_h * aspect
    out = Image.new("RGBA", (max(1, round(out_w)), max(1, round(out_h))), (0, 0, 0, 0))
    out.paste(subject, ((out.width - sw) // 2, (out.height - sh) // 2))
    return out


def fit_canvas(image: Image.Image, canvas: tuple[int, int]) -> Image.Image:
    """Centre-crops the whole image to the canvas aspect (never trims to the subject)."""
    aspect = canvas[0] / canvas[1]
    width, height = image.size
    if abs(width / height - aspect) < 1e-3:
        return image
    if width / height > aspect:
        new_w = round(height * aspect)
        left = (width - new_w) // 2
        return image.crop((left, 0, left + new_w, height))
    new_h = round(width / aspect)
    top = (height - new_h) // 2
    return image.crop((0, top, width, top + new_h))


def resize(image: Image.Image, size: tuple[int, int]) -> Image.Image:
    """Premultiplied Lanczos resize (no dark fringes from transparent pixels)."""
    if image.size == tuple(size):
        return image
    return image.convert("RGBa").resize(tuple(size), Image.Resampling.LANCZOS).convert("RGBA")


def fit(image: Image.Image, mode: str, canvas: tuple[int, int], stored: tuple[int, int]) -> Image.Image:
    shaped = fit_trim(image, canvas) if mode == "trim" else fit_canvas(image, canvas)
    return resize(shaped, stored)


def encode_webp(image: Image.Image) -> bytes:
    buffer = io.BytesIO()
    image.save(buffer, "WEBP", quality=common.WEBP_QUALITY, method=common.WEBP_METHOD)
    return buffer.getvalue()


def components(image: Image.Image, grid: int = COMPONENT_GRID) -> list[dict]:
    """Connected subjects (8-connected, on a downscaled alpha mask), largest first:
    `mass` share of the alpha mass, `bbox` in grid cells, `touches` the image edge."""
    alpha = image.getchannel("A")
    scale = min(1.0, grid / max(alpha.size))
    small = alpha.resize((max(1, round(alpha.width * scale)), max(1, round(alpha.height * scale))), Image.Resampling.BOX)
    width, height = small.size
    values = list(small.tobytes())
    solid = [v > 64 for v in values]
    total = sum(v for v, s in zip(values, solid) if s) or 1
    seen = [False] * len(values)
    found = []
    for start in range(len(values)):
        if solid[start] and not seen[start]:
            found.append(_flood(start, solid, seen, values, width, height, total))
    return sorted(found, key=lambda c: -c["mass"])


def _flood(start, solid, seen, values, width, height, total):
    queue, mass, touches = deque([start]), 0, False
    seen[start] = True
    xs, ys = [], []
    while queue:
        index = queue.popleft()
        y, x = divmod(index, width)
        mass += values[index]
        xs.append(x)
        ys.append(y)
        touches = touches or x in (0, width - 1) or y in (0, height - 1)
        for dy in (-1, 0, 1):
            for dx in (-1, 0, 1):
                nx, ny = x + dx, y + dy
                if 0 <= nx < width and 0 <= ny < height:
                    neighbour = ny * width + nx
                    if solid[neighbour] and not seen[neighbour]:
                        seen[neighbour] = True
                        queue.append(neighbour)
    return {"mass": mass / total, "bbox": (min(xs), min(ys), max(xs) + 1, max(ys) + 1), "touches": touches}


def significant(found: list[dict]) -> list[dict]:
    return [c for c in found if c["mass"] >= SPECK_SHARE]


def _count(mask_l: Image.Image) -> int:
    return mask_l.histogram()[255]


def _mask(image: Image.Image, size) -> Image.Image:
    alpha = image.getchannel("A") if image.mode in ("RGBA", "LA") else image.convert("L")
    if alpha.size != tuple(size):
        alpha = alpha.resize(tuple(size), Image.Resampling.BILINEAR)
    return alpha.point(lambda v: 255 if v >= 128 else 0)


def alpha_iou(image: Image.Image, reference: Image.Image) -> float:
    """Intersection over union of the two alpha masks (at `image`'s size)."""
    a, b = _mask(image, image.size), _mask(reference, image.size)
    union = _count(ImageChops.lighter(a, b))
    return 1.0 if union == 0 else _count(ImageChops.darker(a, b)) / union


def alpha_coverage(image: Image.Image) -> float:
    histogram = image.getchannel("A").histogram()
    return sum(value * count for value, count in enumerate(histogram)) / (255.0 * image.width * image.height)


def magenta_share(image: Image.Image) -> float:
    """Share of opaque pixels that are still close to magenta."""
    red, green, blue, alpha = (c.convert("F") for c in image.convert("RGBA").split())
    near = _eval(
        lambda x: x["float"](((x["r"] - 255.0) * (x["r"] - 255.0) + x["g"] * x["g"] + (x["b"] - 255.0) * (x["b"] - 255.0)) < MAGENTA_DISTANCE**2)
        * x["float"](x["a"] > 128.0) * 255.0,
        r=red, g=green, b=blue, a=alpha,
    )
    opaque = _count(image.getchannel("A").point(lambda v: 255 if v > 128 else 0)) or 1
    return _count(_to_l(near)) / opaque


def corners_transparent(image: Image.Image, size: int = 4, limit: int = ALPHA_SOLID) -> bool:
    alpha = image.getchannel("A")
    w, h = alpha.size
    boxes = [(0, 0, size, size), (w - size, 0, w, size), (0, h - size, size, h), (w - size, h - size, w, h)]
    return all(alpha.crop(box).getextrema()[1] <= limit for box in boxes)


def kept(found: list[dict]) -> list[dict]:
    """The parts worth keeping: everything but noise-sized specks."""
    return [c for c in found if c["mass"] >= KEEP_SHARE]


def remove_specks(image: Image.Image) -> Image.Image:
    """Drops stray specks, keeping every part of the subject (a subject may be several separate
    shapes: sparkles, dashed trails, a key ring beside its key)."""
    found = components(image)
    keep_parts = kept(found)
    if len(keep_parts) == len(found):
        return image
    scale_x = image.width / max(1, round(image.width * min(1.0, COMPONENT_GRID / max(image.size))))
    scale_y = image.height / max(1, round(image.height * min(1.0, COMPONENT_GRID / max(image.size))))
    mask = Image.new("L", image.size, 0)
    for part in keep_parts:
        x0, y0, x1, y1 = part["bbox"]
        box = (int((x0 - 1) * scale_x), int((y0 - 1) * scale_y), int((x1 + 1) * scale_x), int((y1 + 1) * scale_y))
        mask.paste(255, (max(0, box[0]), max(0, box[1]), min(image.width, box[2]), min(image.height, box[3])))
    alpha = ImageChops.darker(image.getchannel("A"), mask)
    out = image.copy()
    out.putalpha(alpha)
    return out
