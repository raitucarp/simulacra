"""A pack folder on disk: its `pack.json` manifest, its files, and the tooling's own run state.

pack.json (read by the Rust loader, `src/domain/packs/`):
    {"id", "name", "version", "description", "style", "samples": [keys],
     "coverage": {key: {"file": "<key>.webp", "sha256": hex, "width": W, "height": H}}}
"""

from __future__ import annotations

import re
from pathlib import Path

import common

_ID = re.compile(r"^[a-z0-9_-]+$")
DEFAULT_SAMPLES = ["guide/cheer", "spot/ideas", "empty/no_posts"]


def pack_meta(pack: str, overrides: dict | None = None) -> dict:
    """Name, version, description, style and samples of `pack` (packs.json, then overrides)."""
    if not _ID.match(pack) or pack == common.RESERVED_PACK_ID:
        raise ValueError(f"invalid pack id {pack!r}: use [a-z0-9_-]+ and never {common.RESERVED_PACK_ID!r}")
    known = common.load_packs_meta().get(pack, {})
    meta = {
        "name": known.get("name", pack.replace("_", " ").title()),
        "version": known.get("version", "1.0.0"),
        "description": known.get("description", ""),
        "style": known.get("style", pack),
        "samples": list(known.get("samples", DEFAULT_SAMPLES)),
    }
    meta.update({k: v for k, v in (overrides or {}).items() if v is not None})
    return meta


class PackDir:
    """One pack's output folder (pack.json + `<key>.webp` files)."""

    def __init__(self, root: Path, pack: str, meta: dict):
        self.root = Path(root)
        self.pack = pack
        self.manifest = common.read_json(self.root / common.MANIFEST) or {}
        self.manifest.setdefault("coverage", {})
        self.manifest.update({"id": pack, **{k: meta[k] for k in ("name", "version", "description", "style", "samples")}})

    @property
    def coverage(self) -> dict:
        return self.manifest["coverage"]

    def file_path(self, key: str) -> Path:
        return self.root / f"{key}{common.FILE_EXT}"

    def put(self, key: str, data: bytes, size: tuple[int, int]) -> dict:
        path = self.file_path(key)
        path.parent.mkdir(parents=True, exist_ok=True)
        if not path.is_file() or path.read_bytes() != data:
            path.write_bytes(data)
        entry = {"file": f"{key}{common.FILE_EXT}", "sha256": common.sha256_bytes(data), "width": size[0], "height": size[1]}
        self.coverage[key] = entry
        return entry

    def drop(self, key: str) -> bool:
        """Removes `key` (its file and coverage entry); True when it was there."""
        had = self.coverage.pop(key, None) is not None
        path = self.file_path(key)
        if path.is_file():
            path.unlink()
            had = True
        return had

    def save(self) -> None:
        common.write_json(self.root / common.MANIFEST, self.manifest)


class RunState:
    """What earlier runs did, so runs are idempotent and resumable:
    `{"items": {key: {"source": sha256, "status", "reasons"}}, "sheets": {id: {"source", "cells"}}}`."""

    def __init__(self, work: Path, pack: str):
        self.path = Path(work) / "state" / f"{pack}.json"
        data = common.read_json(self.path, {}) or {}
        self.items = data.get("items", {})
        self.sheets = data.get("sheets", {})

    def record(self, key: str, source: str, status: str, reasons: list[str]) -> None:
        self.items[key] = {"source": source, "status": status, "reasons": reasons}

    def save(self) -> None:
        common.write_json(self.path, {"items": self.items, "sheets": self.sheets})
