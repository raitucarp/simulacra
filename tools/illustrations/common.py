"""Shared paths, constants and small helpers of the illustration pack tooling.

Everything here is stdlib only. The interfaces (key convention, canvases, stored sizes,
pack.json layout, key colours) are shared with the Rust loader in
`src/ui/illustration/pack/` and must stay in step with it.
"""

from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path

TOOLS_DIR = Path(__file__).resolve().parent
REPO = TOOLS_DIR.parents[1]
ASSETS_DIR = REPO / "assets" / "illustrations"
LOCALE_EN = REPO / "locales" / "en.yml"
STYLES_DIR = TOOLS_DIR / "styles"
SUBJECTS_FILE = TOOLS_DIR / "subjects.json"
PACKS_FILE = TOOLS_DIR / "packs.json"
CATALOG_FILE = TOOLS_DIR / "catalog.yaml"
REFS_DIR = TOOLS_DIR / "refs"
INBOX_DIR = TOOLS_DIR / "inbox"
SHEETS_DIR = TOOLS_DIR / "sheets"
WORK_DIR = TOOLS_DIR / "out"
FLAT_OUT = REPO / "assets" / "packs" / "flat"

PACK_IDS = ("flat", "clay3d", "cozy", "isometric")
BUILT_IN_PACK = "flat"
RESERVED_PACK_ID = "classic"
NEW_ART_PREFIXES = ("journey/", "guides/")
MANIFEST = "pack.json"
FILE_EXT = ".webp"

# Layered rasters are drawn in these key colours and recoloured at runtime.
PRIMARY_KEY = "#808080"
SECONDARY_KEY = "#00FFFF"
MAGENTA = (255, 0, 255)

LAYERED_ROOTS = ("portrait", "forms")
CANVAS_FIT_GROUPS = ("palace",)
MAX_STORED_WIDTH = 1024
LAYERED_STORED = (512, 512)
WEBP_QUALITY = 85
WEBP_METHOD = 6

# The five loci of each memory-palace room on its 160x120 canvas
# (copied from src/domain/learning/palace/rooms.rs; keep both in step).
PALACE_CANVAS = (160.0, 120.0)
PALACE_LANDMARKS = {
    "hall": [("door", 24, 62), ("mirror", 54, 44), ("chandelier", 80, 20), ("bench", 108, 85), ("clock", 136, 54)],
    "library": [("ladder", 24, 64), ("shelf", 55, 40), ("globe", 84, 80), ("lamp", 112, 28), ("armchair", 139, 77)],
    "garden": [("gate", 20, 72), ("tree", 50, 34), ("fountain", 82, 82), ("bench", 114, 82), ("birdhouse", 138, 42)],
    "kitchen": [("stove", 25, 72), ("shelf", 54, 32), ("table", 82, 85), ("sink", 114, 76), ("window", 136, 36)],
    "study": [("bookcase", 23, 56), ("painting", 54, 34), ("desk", 84, 78), ("plant", 114, 84), ("window", 136, 40)],
    "stairwell": [("step", 24, 98), ("banister", 54, 66), ("portrait", 86, 31), ("landing", 118, 43), ("window", 140, 20)],
}

_SEGMENT = re.compile(r"^[a-z0-9_-]+$")
_VIEW_BOX = re.compile(r'viewBox="([^"]+)"')
_DRAWABLE = re.compile(r"<(path|rect|circle|ellipse|polygon|polyline|line|use)\b")


def key_of(svg: Path, assets: Path = ASSETS_DIR) -> str:
    """`empty/no_posts` for `<assets>/empty/no_posts.svg`."""
    return svg.relative_to(assets).with_suffix("").as_posix()


def is_valid_key(key: str) -> bool:
    parts = key.split("/")
    return len(parts) >= 2 and all(_SEGMENT.match(part) for part in parts)


def is_layered(key: str) -> bool:
    return key.split("/", 1)[0] in LAYERED_ROOTS


def layer_of(key: str) -> str | None:
    """The layer name (`face`, `body`, ...) of a layered key."""
    parts = key.split("/")
    return parts[-2] if is_layered(key) and len(parts) >= 3 else None


def tint_role(key: str) -> str:
    """Which key colours the runtime recolours in this layer."""
    parts = key.split("/")
    layer = layer_of(key)
    if parts[0] == "portrait":
        return {"face": "skin", "hair": "hair", "brows": "hair"}.get(layer or "", "none")
    if parts[0] == "forms":
        return {"body": "primary+secondary", "markings": "secondary"}.get(layer or "", "none")
    return "none"


def view_box(svg_text: str) -> tuple[int, int]:
    match = _VIEW_BOX.search(svg_text)
    if not match:
        raise ValueError("no viewBox")
    numbers = [float(n) for n in match.group(1).replace(",", " ").split()]
    return round(numbers[2]), round(numbers[3])


def is_blank(svg_text: str) -> bool:
    """True for an intentionally empty template (e.g. `portrait/accessory/none`)."""
    return _DRAWABLE.search(svg_text) is None


def stored_size(canvas: tuple[int, int], layered: bool) -> tuple[int, int]:
    if layered:
        return LAYERED_STORED
    width = min(2 * canvas[0], MAX_STORED_WIDTH)
    return width, round(width * canvas[1] / canvas[0])


def aspect_label(canvas: tuple[int, int]) -> str:
    from math import gcd

    divisor = gcd(canvas[0], canvas[1]) or 1
    return f"{canvas[0] // divisor}:{canvas[1] // divisor}"


def fit_of(key: str) -> str:
    group = key.split("/", 1)[0]
    return "canvas" if is_layered(key) or group in CANVAS_FIT_GROUPS else "trim"


def entry_info(key: str, assets: Path = ASSETS_DIR) -> dict:
    """Canvas, stored size, fit and layering of a Classic key, read from its SVG."""
    svg = assets / f"{key}.svg"
    text = svg.read_text(encoding="utf-8")
    canvas = view_box(text)
    layered = is_layered(key)
    return {
        "key": key,
        "group": key.split("/", 1)[0],
        "canvas": list(canvas),
        "stored": list(stored_size(canvas, layered)),
        "fit": fit_of(key),
        "layered": layered,
        "blank": is_blank(text),
    }


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path) -> str:
    return sha256_bytes(path.read_bytes())


def read_json(path: Path, default=None):
    if not path.is_file():
        return default
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, data) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    text = json.dumps(data, indent=2, sort_keys=True, ensure_ascii=False) + "\n"
    tmp = path.with_suffix(path.suffix + ".partial")
    tmp.write_text(text, encoding="utf-8", newline="\n")
    tmp.replace(path)


def load_packs_meta() -> dict:
    return read_json(PACKS_FILE, {}) or {}


def pack_out_dir(pack: str, out_root: Path | None = None) -> Path:
    """Where a pack's pack.json and files live."""
    if out_root is not None:
        return Path(out_root) / pack
    if pack == BUILT_IN_PACK:
        return FLAT_OUT
    return WORK_DIR / "packs" / pack


def load_subjects(path: Path = SUBJECTS_FILE) -> dict:
    data = read_json(path, {}) or {}
    return {"groups": data.get("groups", {}) or {}, "items": data.get("items", {}) or {}}


def flatten_locale(path: Path = LOCALE_EN) -> dict:
    """Dotted key -> text of a (two-space indented) locale file."""
    out: dict[str, str] = {}
    if not path.is_file():
        return out
    stack: list[str] = []
    for line in path.read_text(encoding="utf-8").splitlines():
        stripped = line.lstrip()
        if not stripped or stripped.startswith("#") or ":" not in stripped:
            continue
        depth = (len(line) - len(stripped)) // 2
        name, value = stripped.split(":", 1)
        del stack[depth:]
        stack.append(name.strip())
        value = value.strip()
        if value:
            if len(value) >= 2 and value[0] == value[-1] == '"':
                value = value[1:-1].replace('\\"', '"')
            out[".".join(stack)] = value
    return out


def humanize(stem: str) -> str:
    return stem.replace("_", " ").replace("-", " ")
