"""Builds `catalog.yaml`: one entry per Classic illustration key, with its canvas, batch,
subject, constraints and the fully expanded Flow prompt of every pack.

    python tools/illustrations/build_catalog.py [--assets DIR] [--subjects FILE]
        [--styles DIR] [--locale FILE] [--out FILE]

Inputs: the SVG templates under assets/illustrations/, `subjects.json` (hand-written
subjects; optional), `styles/<pack>.md` (the ```prompt and ```sheet-prompt blocks; a built-in
placeholder template is used when a style is missing) and locales/en.yml (trait labels).
The output is deterministic: rebuilding without input changes rewrites the same bytes.
"""

from __future__ import annotations

import argparse
import re
import sys
from collections import Counter
from pathlib import Path

import catalog as catalog_io
import common

VISIBLE_ORDER = ["splash", "empty", "spot", "guide", "notice", "steps", "classes", "settings", "import", "palace", "garden", "vault"]
LATE_ORDER = ["symbols", "palettes", "palette_steps", "palette_categories"]
LAYERED_ORDER = ["portrait", "forms"]
BODY_ORDER = ["body_kind", "body_form"]
SINGLE_GROUPS = ("splash", "palace")
SHEET_SIZES = {"sheet2x2": 4, "sheet3x3": 9}
NO_TEXT = "no text, letters, numbers, logos or UI chrome anywhere in the image"

DEFAULT_GROUP_SENTENCES = {
    "splash": "the app's brand mark, shown on the splash screen",
    "empty": "an empty-state scene shown when a list or view has nothing in it yet",
    "spot": "a small spot illustration used beside headers and tips",
    "guide": "Mnemo the owl, the app's friendly guide, in one pose",
    "notice": "the art beside a notice or status message",
    "steps": "a wide banner for one step of the agent creation wizard",
    "classes": "full-height character art of an archetype class",
    "settings": "the header art of one settings category",
    "import": "the art of one kind of imported file",
    "palace": "a memory-palace room scene with five landmarks along a walk from left to right",
    "garden": "one growth stage of a digital-garden note, drawn as a round badge",
    "vault": "the wide banner of the knowledge vault's subject index",
    "symbols": "a bullet-journal symbol shown in place of a post's avatar, drawn as a round badge",
    "palettes": "full-height art of a ready-made persona",
    "palette_steps": "an option card of a ready-made persona",
    "palette_categories": "the icon card of a category of ready-made personas",
    "portrait": "one layer of a layered human portrait avatar",
    "forms": "one layer of a layered non-human avatar",
    "body_kind": "an option card of one kind of non-human body",
    "body_form": "an option card of one non-human body form",
}
TRAIT_SENTENCE = "an option card for one trait of a persona"
# Groups whose label names a concrete thing (drawn as such, not as a metaphor).
CONCRETE_GROUPS = ("palace", "import", "garden", "symbols", "palette_categories", "doc_kind", "settings", "vault")

PLACEHOLDER_PROMPT = (
    "Illustration for a desktop app, {pack} style. Subject: {subject}. "
    "Rules: {constraints}. Canvas {canvas} (aspect {aspect}); the subject centred with generous padding. "
    "Solid pure magenta #FF00FF background, flat and even, with no shadow or gradient on it; no magenta in the subject."
)
PLACEHOLDER_SHEET = (
    "A sprite sheet of {rows} rows by {cols} columns of separate illustrations for a desktop app, {pack} style, "
    "each centred in its own cell of the reference sheet and never crossing into a neighbour. Aspect {aspect}.\n{cells}\n"
    "No text, letters, numbers or logos. Solid pure magenta #FF00FF background between and around the cells; no magenta in the subjects."
)

LAYER_CONSTRAINTS = {
    "skin": "paint all skin in neutral mid-grey #808080 (shade it with lighter and darker greys only); it is recoloured at runtime",
    "hair": "paint all hair in neutral mid-grey #808080 (shade it with lighter and darker greys only); it is recoloured at runtime",
    "primary+secondary": "paint the main body colour in neutral mid-grey #808080 and the second colour in pure cyan #00FFFF (shading only as lighter and darker tones of those two); both are recoloured at runtime",
    "secondary": "paint markings that use the second colour in pure cyan #00FFFF; they are recoloured at runtime",
}


def extract_templates(styles: Path, pack: str) -> dict:
    """The pack's prompt and sheet-prompt templates (placeholders when its style is missing)."""
    path = styles / f"{pack}.md"
    text = path.read_text(encoding="utf-8") if path.is_file() else ""
    prompt = _fenced(text, "prompt")
    sheet = _fenced(text, "sheet-prompt")
    placeholder = prompt is None or sheet is None
    return {
        "prompt": prompt or PLACEHOLDER_PROMPT.replace("{pack}", pack),
        "sheet_prompt": sheet or PLACEHOLDER_SHEET.replace("{pack}", pack),
        "placeholder": placeholder,
    }


def _fenced(text: str, info: str) -> str | None:
    match = re.search(r"^```" + re.escape(info) + r"[ \t]*\r?\n(.*?)\r?\n```", text, re.M | re.S)
    return match.group(1).strip() if match else None


def fill(template: str, values: dict) -> str:
    for name, value in values.items():
        template = template.replace("{" + name + "}", str(value))
    return template


def group_order(groups: set[str]) -> list[str]:
    special = set(VISIBLE_ORDER + LATE_ORDER + LAYERED_ORDER + BODY_ORDER)
    traits = sorted(g for g in groups if g not in special)
    ordered = VISIBLE_ORDER + traits + LATE_ORDER + LAYERED_ORDER + BODY_ORDER
    return [g for g in ordered if g in groups]


def label_and_desc(group: str, stem: str, locale: dict) -> tuple[str | None, str | None]:
    """The English label and description of a key, where the locale has them."""
    get = locale.get
    lookups = {
        "classes": (f"forge.class.{stem}.name", f"forge.class.{stem}.desc"),
        "steps": (f"forge.step.{stem}.label", f"forge.step.{stem}.tagline"),
        "palettes": (f"forge.palette.{stem}.name", f"forge.palette.{stem}.desc"),
        "palette_steps": (f"forge.palette.{stem}.name", f"forge.palette.{stem}.pitch"),
        "palette_categories": (f"forge.palette.category.{stem}", None),
        "settings": (f"prefs.section.{stem}.title", f"prefs.section.{stem}.description"),
        "import": (f"import.family.{stem}", None),
        "palace": (f"learning.palace.room.{stem}", None),
        "garden": (f"posts.garden.stage.{stem}", None),
        "doc_kind": (f"vault.kind.{stem}", None),
    }
    if group == "symbols":
        return _symbol_label(stem, locale), None
    label_key, desc_key = lookups.get(group, (f"traits.{group}.{stem}", f"traits.{group}.{stem}.desc"))
    label = get(label_key)
    # "INTJ · Architect", "Type 4 · Individualist": keep the name, drop the code, so the
    # prompt never hands the model letters or numbers to draw.
    if label and LABEL_SEPARATOR in label:
        label = label.rsplit(LABEL_SEPARATOR, 1)[1].strip() or label
    return label, (get(desc_key) if desc_key else None)


# Separator between a code and a name in trait labels.
LABEL_SEPARATOR = " · "


def _symbol_label(stem: str, locale: dict) -> str | None:
    if f"symbols.signifier.{stem}" in locale:
        return f"the {locale[f'symbols.signifier.{stem}']} signifier"
    entry, _, state = stem.partition("_")
    entry_label = locale.get(f"symbols.entry.{entry}")
    state_label = locale.get(f"symbols.state.{state}") if state else None
    if entry_label and state_label:
        return f"a {entry_label.lower()} that is {state_label.lower()}"
    return f"a {entry_label.lower()}" if entry_label else None


def derive_subject(key: str, group: str, subjects: dict, locale: dict) -> tuple[str, str | None, str]:
    """(subject, context, cell): the full subject of the single prompt, and for sheets the
    sentence shared by the whole group (or None) and the short per-cell description."""
    item = subjects["items"].get(key, {})
    if item.get("subject"):
        subject = item["subject"].strip()
        return subject, None, subject
    parts = key.split("/")
    stem = parts[-1]
    if common.is_layered(key):
        scope = f"{parts[0]}/{common.layer_of(key)}"
        sentence = subjects["groups"].get(scope) or subjects["groups"].get(group) or DEFAULT_GROUP_SENTENCES[group]
        owner = "the shared non-human" if parts[1] == "shared" else (_article(f"{parts[1]} non-human") if group == "forms" else "a human portrait")
        cell = f"the '{common.humanize(stem)}' variant of the {common.layer_of(key)} layer of {owner} avatar"
        return f"{cell} ({sentence.rstrip('.')})", sentence, cell
    sentence = subjects["groups"].get(group) or DEFAULT_GROUP_SENTENCES.get(group, TRAIT_SENTENCE)
    label, desc = label_and_desc(group, stem, locale)
    if not label:
        cell = common.humanize(stem)
        return f"{sentence.rstrip('.')}: {cell}", sentence, cell
    tail = f" ({desc.rstrip('.')})" if desc else ""
    if group in CONCRETE_GROUPS:
        cell = f"{label}{tail}"
        return f"{sentence.rstrip('.')}: {cell}", sentence, cell
    cell = f"a visual metaphor for '{label}'{tail}"
    return f"{sentence.rstrip('.')}; this card is {cell}", sentence, cell


def _article(noun: str) -> str:
    return ("an " if noun[:1].lower() in "aeiou" else "a ") + noun


def constraints_of(key: str, info: dict, subjects: dict) -> list[str]:
    own = [c.strip() for c in subjects["items"].get(key, {}).get("constraints", []) if c.strip()]
    out = list(own)
    group = info["group"]
    if group == "palace" and not any("landmark" in c.lower() for c in own):
        out.append(palace_constraint(key.split("/")[-1]))
    if info["layered"]:
        out.append("keep the exact position, size and outline of the reference layer on its square 256x256 layer canvas, because it is stacked with other layers; draw only this layer")
        role = common.tint_role(key)
        if role in LAYER_CONSTRAINTS:
            out.append(LAYER_CONSTRAINTS[role])
    if info["fit"] == "canvas" and not info["layered"]:
        out.append("fill the whole canvas edge to edge like the reference, keeping its layout")
    out.append(NO_TEXT)
    return out


def palace_constraint(room: str) -> str:
    width, height = common.PALACE_CANVAS
    marks = ", ".join(f"{name} at ({x / width:.2f}, {y / height:.2f})" for name, x, y in common.PALACE_LANDMARKS.get(room, []))
    return f"keep the five landmarks exactly where the reference has them, as fractions of width and height from the top left: {marks}"


def batch_of(info: dict, group_size: int) -> str:
    width, height = info["canvas"]
    if info["group"] in SINGLE_GROUPS or group_size <= 1 or info["blank"]:
        return "single"
    if info["layered"] or (width <= 160 and height <= 120):
        return "sheet3x3"
    return "sheet2x2"


def sheet_scope(key: str) -> str:
    return key.rsplit("/", 1)[0] if common.is_layered(key) else key.split("/", 1)[0]


def sheet_prefix(scope: str, group: str, traits: set[str]) -> str:
    if common.is_layered(scope + "/x"):
        return "layer-" + scope.replace("/", "-").replace("_", "-")
    return f"trait-{group}" if group in traits else group


def assign_sheets(entries: list[dict], traits: set[str]) -> None:
    """Fills `sheet` ids: runs of same-scope, same-canvas sheet entries, 4 or 9 per sheet."""
    runs: dict[tuple, list[dict]] = {}
    for entry in entries:
        if entry["batch"] in SHEET_SIZES:
            runs.setdefault((sheet_scope(entry["key"]), tuple(entry["canvas"]), entry["batch"]), []).append(entry)
    for (scope, _, batch), members in runs.items():
        size = SHEET_SIZES[batch]
        prefix = sheet_prefix(scope, members[0]["group"], traits)
        for start in range(0, len(members), size):
            chunk = members[start : start + size]
            if len(chunk) == 1:
                chunk[0]["batch"], chunk[0]["sheet"] = "single", None
                continue
            for entry in chunk:
                entry["sheet"] = f"{prefix}-{start // size + 1:02d}"


def build(assets: Path, subjects: dict, templates: dict, locale: dict) -> dict:
    svgs = sorted(assets.rglob("*.svg"))
    infos = [common.entry_info(common.key_of(svg, assets), assets) for svg in svgs]
    groups = {info["group"] for info in infos}
    order = group_order(groups)
    special = set(VISIBLE_ORDER + LATE_ORDER + LAYERED_ORDER + BODY_ORDER)
    traits = {g for g in groups if g not in special}
    sizes = Counter(info["group"] for info in infos)
    rank = {group: index for index, group in enumerate(order)}
    infos.sort(key=lambda info: (rank[info["group"]], info["key"]))
    entries = [_entry(index, info, assets, subjects, locale, sizes) for index, info in enumerate(infos)]
    assign_sheets(entries, traits)
    for entry in entries:
        _expand_prompts(entry, templates)
    return {
        "version": 1,
        "packs": list(common.PACK_IDS),
        "key_colours": {"primary": common.PRIMARY_KEY, "secondary": common.SECONDARY_KEY},
        "templates": templates,
        "groups": [{"name": g, "count": sizes[g], "trait": g in traits} for g in order],
        "entries": entries,
    }


def _entry(index: int, info: dict, assets: Path, subjects: dict, locale: dict, sizes: Counter) -> dict:
    key, group = info["key"], info["group"]
    canvas = tuple(info["canvas"])
    entry = {
        "key": key,
        "group": group,
        "order": index,
        "source": (assets / f"{key}.svg").relative_to(common.REPO).as_posix() if assets.is_relative_to(common.REPO) else f"{key}.svg",
        "canvas": list(canvas),
        "aspect": common.aspect_label(canvas),
        "stored": info["stored"],
        "fit": info["fit"],
        "layered": info["layered"],
        "role": common.tint_role(key),
        "generate": not info["blank"],
        "batch": batch_of(info, sizes[group]),
        "sheet": None,
        "subject": "",
        "context": None,
        "cell": "",
        "constraints": constraints_of(key, info, subjects),
    }
    if info["blank"]:
        entry["subject"] = entry["cell"] = "intentionally empty layer: nothing to generate"
    else:
        entry["subject"], entry["context"], entry["cell"] = derive_subject(key, group, subjects, locale)
    return entry


def join_constraints(constraints: list[str]) -> str:
    """Constraints as sentences: capitalised, each ending in a full stop."""
    sentences = []
    for text in constraints:
        text = text.strip()
        if text:
            text = text[0].upper() + text[1:]
            sentences.append(text if text[-1] in ".!?" else text + ".")
    return " ".join(sentences)


def _expand_prompts(entry: dict, templates: dict) -> None:
    canvas = f"{entry['canvas'][0]}x{entry['canvas'][1]}"
    entry["prompts"] = {}
    for pack in common.PACK_IDS:
        template = templates[pack]["prompt"]
        # The no-text rule is left out where the pack's template already states it.
        own = [c for c in entry["constraints"] if not (c == NO_TEXT and "no text" in template.lower())]
        values = {"subject": entry["subject"].rstrip(". "), "constraints": join_constraints(own), "canvas": canvas, "aspect": entry["aspect"]}
        entry["prompts"][pack] = fill(template, values).strip()


HEADER = """Simulacra illustration catalog: every Classic illustration key and how to make it in each pack.
Generated by tools/illustrations/build_catalog.py from assets/illustrations/, subjects.json,
styles/<pack>.md and locales/en.yml. Do not edit by hand; edit those inputs and rebuild.
Format: a strict YAML subset (every scalar JSON-encoded), read by tools/illustrations/catalog.py."""

FOOTER = """
journey:
  Placeholder, filled later: the new Journey art (badge emblems, trophies, class emblems, quest
  scenes, forest growth stages by season, garden flowers, skyline buildings, Pomodoro scenes,
  celebrations and hero) gets entries here under keys starting with "journey/".
guides:
  Placeholder, filled later: the visual guides ("How this works" cards, the onboarding tour and
  coach marks) get entries here under keys starting with "guides/"."""


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--assets", type=Path, default=common.ASSETS_DIR)
    parser.add_argument("--subjects", type=Path, default=common.SUBJECTS_FILE)
    parser.add_argument("--styles", type=Path, default=common.STYLES_DIR)
    parser.add_argument("--locale", type=Path, default=common.LOCALE_EN)
    parser.add_argument("--out", type=Path, default=common.CATALOG_FILE)
    args = parser.parse_args(argv)
    templates = {pack: extract_templates(args.styles, pack) for pack in common.PACK_IDS}
    data = build(args.assets.resolve(), common.load_subjects(args.subjects), templates, common.flatten_locale(args.locale))
    missing = [pack for pack, t in templates.items() if t["placeholder"]]
    header = HEADER + (f"\nPLACEHOLDER prompt templates in use for: {', '.join(missing)} (styles/<pack>.md missing or incomplete)." if missing else "")
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(catalog_io.dumps(data, header=header, footer=FOOTER), encoding="utf-8", newline="\n")
    report(data, args.out)
    return 0


def report(data: dict, out: Path) -> None:
    entries = data["entries"]
    print(f"{out}: {len(entries)} entries")
    for group in data["groups"]:
        print(f"  {group['name']:<22} {group['count']}")
    batches = Counter(e["batch"] for e in entries)
    sheets = len({e["sheet"] for e in entries if e["sheet"]})
    print("  batches: " + ", ".join(f"{name} {count}" for name, count in sorted(batches.items())) + f"; {sheets} sheets")


if __name__ == "__main__":
    sys.exit(main())
