"""Reading and writing `catalog.yaml` without PyYAML.

The catalog uses a strict subset of YAML that is also trivial to parse:

* block mappings (`name: value`, keys are plain `[A-Za-z0-9_-]+` words) and block sequences
  (`- name: value` items holding mappings, or `- <scalar>`), indented by two spaces;
* every scalar JSON-encoded: double-quoted strings, numbers, `true`/`false`/`null`, and
  flow lists / empty mappings written as JSON (`[240, 160]`, `["a", "b"]`, `{}`);
* whole-line `#` comments (ignored); no inline comments, anchors or multi-line scalars.

JSON strings and arrays are valid YAML flow scalars, so any YAML parser reads the file too.
"""

from __future__ import annotations

import json
import re
from pathlib import Path

INDENT = "  "
_KEY = re.compile(r"^[A-Za-z0-9_-]+$")


def _scalar(value) -> str:
    return json.dumps(value, ensure_ascii=False)


def _is_scalar_list(value) -> bool:
    return isinstance(value, list) and all(not isinstance(v, (dict, list)) for v in value)


def _inline(value) -> str | None:
    """The one-line form of `value`, or None when it needs a block."""
    if isinstance(value, dict):
        return "{}" if not value else None
    if isinstance(value, list):
        return _scalar(value) if (not value or _is_scalar_list(value)) else None
    return _scalar(value)


def _dump_mapping(data: dict, depth: int, out: list[str], first_prefix: str | None = None) -> None:
    pad = INDENT * depth
    for index, (key, value) in enumerate(data.items()):
        if not _KEY.match(str(key)):
            raise ValueError(f"catalog keys must be plain words: {key!r}")
        prefix = first_prefix if (index == 0 and first_prefix is not None) else pad
        inline = _inline(value)
        if inline is not None:
            out.append(f"{prefix}{key}: {inline}")
        else:
            out.append(f"{prefix}{key}:")
            _dump_block(value, depth + 1, out)


def _dump_block(value, depth: int, out: list[str]) -> None:
    if isinstance(value, dict):
        _dump_mapping(value, depth, out)
        return
    pad = INDENT * depth
    for item in value:
        if isinstance(item, dict) and item:
            _dump_mapping(item, depth + 1, out, first_prefix=f"{pad}- ")
        else:
            inline = _inline(item)
            if inline is None:
                raise ValueError("nested sequences are not part of the catalog subset")
            out.append(f"{pad}- {inline}")


def dumps(data: dict, header: str = "", footer: str = "") -> str:
    """`data` as catalog YAML, with optional comment blocks before and after."""
    out: list[str] = []
    out.extend(_comment_lines(header))
    _dump_mapping(data, 0, out)
    out.extend(_comment_lines(footer))
    return "\n".join(out) + "\n"


def _comment_lines(text: str) -> list[str]:
    return [line if line.startswith("#") else f"# {line}".rstrip() for line in text.splitlines()] if text else []


def _lines(text: str) -> list[tuple[int, str]]:
    result = []
    for raw in text.splitlines():
        stripped = raw.strip()
        if not stripped or stripped.startswith("#"):
            continue
        indent = len(raw) - len(raw.lstrip(" "))
        result.append((indent, raw.strip()))
    return result


def _split(content: str) -> tuple[str, str]:
    if content.endswith(":") and _KEY.match(content[:-1]):
        return content[:-1], ""
    key, sep, rest = content.partition(": ")
    if not sep or not _KEY.match(key):
        raise ValueError(f"not a `name: value` line: {content!r}")
    return key, rest


def _parse_block(lines, index: int, indent: int):
    if lines[index][1].startswith("- ") or lines[index][1] == "-":
        return _parse_sequence(lines, index, indent)
    return _parse_mapping(lines, index, indent)


def _parse_mapping(lines, index: int, indent: int, result: dict | None = None):
    result = {} if result is None else result
    while index < len(lines) and lines[index][0] == indent and not lines[index][1].startswith("- "):
        key, rest = _split(lines[index][1])
        index += 1
        if rest:
            result[key] = json.loads(rest)
        elif index < len(lines) and lines[index][0] > indent:
            result[key], index = _parse_block(lines, index, lines[index][0])
        elif index < len(lines) and lines[index][0] == indent and lines[index][1].startswith("- "):
            result[key], index = _parse_sequence(lines, index, indent)
        else:
            result[key] = None
    return result, index


def _parse_sequence(lines, index: int, indent: int):
    items = []
    while index < len(lines) and lines[index][0] == indent and lines[index][1].startswith("- "):
        content = lines[index][1][2:].strip()
        child = indent + 2
        if content.startswith(("{", "[", '"')) or not re.match(r"^[A-Za-z0-9_-]+:( |$)", content):
            items.append(json.loads(content))
            index += 1
            continue
        lines[index] = (child, content)  # the first key sits on the dash line
        item, index = _parse_mapping(lines, index, child)
        items.append(item)
    return items, index


def loads(text: str) -> dict:
    lines = _lines(text)
    if not lines:
        return {}
    data, index = _parse_mapping(lines, 0, lines[0][0])
    if index != len(lines):
        raise ValueError(f"unexpected indentation near: {lines[index][1]!r}")
    return data


def load(path: Path) -> dict:
    return loads(Path(path).read_text(encoding="utf-8"))


def entries_by_key(catalog: dict) -> dict:
    return {entry["key"]: entry for entry in catalog.get("entries", [])}
