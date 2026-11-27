"""Converts raw PNG captures into the WebP files the site serves, and writes the manifest.

    python docs/capture/convert.py

raw/dark/<id>.png  -> public/screenshots/<id>.webp
raw/light/<id>.png -> public/screenshots/light/<id>.webp
components/screenshots.json: {"<id>": {"width": .., "height": .., "light": true?}}
Needs Pillow (pip install pillow).
"""
import json
from pathlib import Path

from PIL import Image

HERE = Path(__file__).resolve().parent
DOCS = HERE.parent
OUT = DOCS / "public" / "screenshots"
MANIFEST = DOCS / "components" / "screenshots.json"
QUALITY = 84


def convert(src: Path, dest: Path) -> tuple[int, int]:
    dest.parent.mkdir(parents=True, exist_ok=True)
    with Image.open(src) as img:
        img = img.convert("RGB")
        if not dest.exists() or dest.stat().st_mtime < src.stat().st_mtime:
            img.save(dest, "WEBP", quality=QUALITY, method=6)
        return img.size


def main() -> None:
    manifest: dict[str, dict] = {}
    for src in sorted((HERE / "raw" / "dark").glob("*.png")):
        width, height = convert(src, OUT / f"{src.stem}.webp")
        manifest[src.stem] = {"width": width, "height": height}
    for src in sorted((HERE / "raw" / "light").glob("*.png")):
        if src.stem in manifest:
            convert(src, OUT / "light" / f"{src.stem}.webp")
            manifest[src.stem]["light"] = True
    # Drop published images whose capture no longer exists.
    for webp in OUT.rglob("*.webp"):
        if webp.stem not in manifest:
            webp.unlink()
    MANIFEST.write_text(json.dumps(manifest, indent=1, sort_keys=True) + "\n", encoding="utf-8")
    total = sum(p.stat().st_size for p in OUT.rglob("*.webp"))
    print(f"{len(manifest)} screenshots, {total / 1e6:.1f} MB")


if __name__ == "__main__":
    main()
