"""catalog.yaml: the YAML-subset round trip and build_catalog's entries, batches and sheets."""

import unittest
from collections import Counter

import helpers  # noqa: F401  (puts the tools on sys.path)

import build_catalog
import catalog
import common


class CatalogFormat(unittest.TestCase):
    def test_round_trip(self):
        data = {
            "version": 1,
            "packs": ["flat", "cozy"],
            "empty_map": {},
            "templates": {"flat": {"prompt": 'say "hi"\nnext line', "placeholder": True}},
            "entries": [
                {"key": "empty/no_posts", "canvas": [240, 160], "sheet": None, "prompts": {"flat": "a × b"}, "constraints": []},
                {"key": "spot/ideas", "order": 2, "ratio": 0.5},
            ],
        }
        text = catalog.dumps(data, header="a header", footer="journey:\n  later")
        self.assertTrue(text.startswith("# a header"))
        self.assertIn("# journey:", text)
        self.assertEqual(catalog.loads(text), data)

    def test_any_yaml_parser_reads_it(self):
        try:
            import yaml
        except ImportError:
            self.skipTest("PyYAML not installed")
        data = {"entries": [{"key": "a/b", "prompts": {"flat": "x: y # not a comment"}, "canvas": [1, 2]}]}
        self.assertEqual(yaml.safe_load(catalog.dumps(data)), data)

    def test_rejects_keys_outside_the_subset(self):
        with self.assertRaises(ValueError):
            catalog.dumps({"not a word": 1})


class BuildCatalog(unittest.TestCase):
    def setUp(self):
        self.tmp = helpers.scratch("catalog")

    def tearDown(self):
        helpers.cleanup(self.tmp)

    def build(self, keys, subjects=None):
        assets = helpers.asset_tree(self.tmp, keys)
        templates = {p: build_catalog.extract_templates(self.tmp / "no-styles", p) for p in common.PACK_IDS}
        subjects = subjects or {"groups": {}, "items": {}}
        return build_catalog.build(assets, subjects, templates, {"traits.mbti.intj": "INTJ", "traits.mbti.intj.desc": "Plans."})

    def test_batches_sheets_and_order(self):
        keys = {f"mbti/t{i:02d}": (160, 120) for i in range(10)}
        keys.update({"mbti/intj": (160, 120), "splash/brand": (160, 120), "empty/a": (240, 160), "empty/b": (240, 160)})
        keys.update({"vault/subject_index": (320, 120), "palace/hall": (160, 120), "palace/study": (160, 120)})
        keys.update({"portrait/face/round": (256, 256), "portrait/face/oval": (256, 256), "portrait/accessory/none": None})
        data = self.build(keys)
        entries = catalog.entries_by_key(data)
        self.assertEqual(len(entries), len(keys))
        self.assertEqual([g["name"] for g in data["groups"]], ["splash", "empty", "palace", "vault", "mbti", "portrait"])
        self.assertEqual(entries["splash/brand"]["batch"], "single")
        self.assertEqual(entries["vault/subject_index"]["batch"], "single", "a group of one is a single")
        self.assertEqual(entries["palace/hall"]["batch"], "single")
        self.assertEqual(entries["palace/hall"]["fit"], "canvas")
        self.assertIn("door at (0.15, 0.52)", " ".join(entries["palace/hall"]["constraints"]))
        self.assertEqual((entries["empty/a"]["batch"], entries["empty/a"]["sheet"]), ("sheet2x2", "empty-01"))
        mbti = [e for e in data["entries"] if e["group"] == "mbti"]
        sheets = Counter(e["sheet"] for e in mbti)
        self.assertEqual(sheets["trait-mbti-01"], 9)
        self.assertEqual(sheets["trait-mbti-02"], 2)
        face = entries["portrait/face/round"]
        self.assertEqual((face["batch"], face["layered"], face["stored"], face["role"]), ("sheet3x3", True, [512, 512], "skin"))
        self.assertEqual(face["sheet"], "layer-portrait-face-01")
        self.assertIn("#808080", " ".join(face["constraints"]))
        blank = entries["portrait/accessory/none"]
        self.assertEqual((blank["generate"], blank["batch"], blank["sheet"]), (False, "single", None))
        self.assertEqual([e["order"] for e in data["entries"]], list(range(len(keys))))

    def test_subjects_and_prompts(self):
        subjects = {"groups": {}, "items": {"empty/a": {"subject": "a quiet desk", "constraints": ["one lamp"]}}}
        data = self.build({"empty/a": (240, 160), "mbti/intj": (160, 120)}, subjects)
        entries = catalog.entries_by_key(data)
        self.assertEqual(entries["empty/a"]["subject"], "a quiet desk")
        self.assertEqual(entries["empty/a"]["constraints"][0], "one lamp")
        self.assertIn("INTJ", entries["mbti/intj"]["subject"])
        for pack in common.PACK_IDS:
            prompt = entries["empty/a"]["prompts"][pack]
            self.assertIn("a quiet desk", prompt)
            self.assertIn("240x160", prompt)
            self.assertNotIn("{subject}", prompt)
        self.assertTrue(all(t["placeholder"] for t in data["templates"].values()))

    def test_fenced_templates_are_extracted(self):
        styles = self.tmp / "styles"
        styles.mkdir()
        (styles / "flat.md").write_text("# Flat\n\n```prompt\nFLAT {subject} {canvas}\n```\n\n```sheet-prompt\nSHEET {rows}x{cols}\n{cells}\n```\n", encoding="utf-8")
        templates = build_catalog.extract_templates(styles, "flat")
        self.assertEqual(templates["prompt"], "FLAT {subject} {canvas}")
        self.assertFalse(templates["placeholder"])

    def test_the_real_tree_has_one_entry_per_svg(self):
        svgs = list(common.ASSETS_DIR.rglob("*.svg"))
        templates = {p: build_catalog.extract_templates(common.STYLES_DIR, p) for p in common.PACK_IDS}
        data = build_catalog.build(common.ASSETS_DIR, common.load_subjects(), templates, common.flatten_locale())
        self.assertEqual(len(data["entries"]), len(svgs))
        self.assertEqual(sum(g["count"] for g in data["groups"]), len(svgs))
        self.assertEqual(data["groups"][0]["name"], "splash")
        for entry in data["entries"]:
            width, height = entry["canvas"]
            if entry["batch"] == "sheet3x3":
                self.assertTrue(entry["layered"] or (width <= 160 and height <= 120), entry["key"])
            if entry["group"] in ("splash", "palace"):
                self.assertEqual(entry["batch"], "single")
            self.assertTrue(common.is_valid_key(entry["key"]), entry["key"])


if __name__ == "__main__":
    unittest.main()
