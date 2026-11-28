"""Builds a downloadable `.simpack` from a processed pack folder.

    python tools/illustrations/make_simpack.py <pack> [--src DIR] [--out DIR]

A .simpack is a zip (deflate) holding `pack.json` at its root and every coverage file at its
`file` path, nothing else. Every file is checked against its sha256 in pack.json first (the
app refuses an archive with any mismatch). Prints the archive's sha256 and size and the
entry to paste into assets/packs/downloadable.json once it is published.
"""

from __future__ import annotations

import argparse
import json
import sys
import zipfile
from pathlib import Path

import common

ZIP_EPOCH = (2026, 1, 1, 0, 0, 0)  # fixed timestamps keep archives reproducible
URL_PLACEHOLDER = "<release asset URL, filled when published>"


def verify(src: Path, manifest: dict) -> list[str]:
    problems = []
    for key, entry in sorted(manifest.get("coverage", {}).items()):
        path = src / entry["file"]
        if not path.is_file():
            problems.append(f"{key}: missing {entry['file']}")
        elif common.sha256_file(path) != entry["sha256"]:
            problems.append(f"{key}: sha256 of {entry['file']} does not match pack.json")
    return problems


def _add(archive: zipfile.ZipFile, name: str, data: bytes) -> None:
    info = zipfile.ZipInfo(name, date_time=ZIP_EPOCH)
    info.compress_type = zipfile.ZIP_DEFLATED
    info.external_attr = 0o644 << 16
    archive.writestr(info, data)


def build(src: Path, out_file: Path) -> dict:
    manifest = common.read_json(src / common.MANIFEST)
    if manifest is None:
        raise FileNotFoundError(f"no {common.MANIFEST} in {src}")
    problems = verify(src, manifest)
    if problems:
        raise ValueError("refusing to pack:\n  " + "\n  ".join(problems))
    out_file.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(out_file, "w") as archive:
        _add(archive, common.MANIFEST, (src / common.MANIFEST).read_bytes())
        for entry in sorted(manifest["coverage"].values(), key=lambda e: e["file"]):
            _add(archive, entry["file"], (src / entry["file"]).read_bytes())
    return {
        "id": manifest["id"],
        "name": manifest["name"],
        "description": manifest.get("description", ""),
        "version": manifest["version"],
        "url": URL_PLACEHOLDER,
        "sha256": common.sha256_file(out_file),
        "bytes": out_file.stat().st_size,
    }


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description="Build a .simpack from a pack folder.")
    parser.add_argument("pack")
    parser.add_argument("--src", type=Path, help="default: assets/packs/flat or tools/illustrations/out/packs/<pack>")
    parser.add_argument("--out", type=Path, default=common.WORK_DIR, help="folder of the archive")
    args = parser.parse_args(argv)
    src = args.src or common.pack_out_dir(args.pack)
    manifest = common.read_json(src / common.MANIFEST) or {}
    out_file = args.out / f"{args.pack}-{manifest.get('version', '0')}.simpack"
    try:
        entry = build(src, out_file)
    except (FileNotFoundError, ValueError) as err:
        print(f"make_simpack: {err}", file=sys.stderr)
        return 1
    print(f"{out_file}: {entry['bytes']} bytes, sha256 {entry['sha256']}, {len(manifest.get('coverage', {}))} files")
    print("downloadable.json entry:")
    print(json.dumps(entry, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
