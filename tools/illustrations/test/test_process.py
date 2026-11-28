"""process.py end to end on a synthetic tree: singles, idempotence, layer alignment, sheets
and --check."""

import json
import unittest
from pathlib import Path

import helpers

from PIL import Image, ImageDraw

import build_catalog
import catalog
import common
import imaging
import process
import sheets

PACK = "flat"
KEYS = {
    "spot/ideas": (160, 120),
    "spot/calm": (160, 120),
    "spot/memory": (160, 120),
    "spot/reading": (160, 120),
    "palace/hall": (160, 120),
    "portrait/face/round": (256, 256),
    "portrait/face/oval": (256, 256),
}


class Pipeline(unittest.TestCase):
    def setUp(self):
        self.tmp = helpers.scratch("process")
        self.assets = helpers.asset_tree(self.tmp, KEYS)
        templates = {p: build_catalog.extract_templates(self.tmp / "none", p) for p in common.PACK_IDS}
        data = build_catalog.build(self.assets, {"groups": {}, "items": {}}, templates, {})
        self.catalog = self.tmp / "catalog.yaml"
        self.catalog.write_text(catalog.dumps(data), encoding="utf-8")
        self.inbox, self.refs, self.sheets = self.tmp / "inbox", self.tmp / "refs", self.tmp / "sheets"
        self.out, self.work = self.tmp / "packs", self.tmp / "work"
        for key in ("portrait/face/round", "portrait/face/oval"):
            path = self.refs / f"{key}.png"
            path.parent.mkdir(parents=True, exist_ok=True)
            helpers.reference_disc().save(path)

    def tearDown(self):
        helpers.cleanup(self.tmp)

    def run_process(self, *extra):
        argv = [PACK, "--inbox", str(self.inbox), "--out-root", str(self.out), "--work", str(self.work), "--refs", str(self.refs),
                "--sheets", str(self.sheets), "--catalog", str(self.catalog), "--assets", str(self.assets), *extra]
        return process.main(argv)

    def manifest(self):
        return json.loads((self.out / PACK / "pack.json").read_text(encoding="utf-8"))

    def drop(self, key, image):
        path = self.inbox / f"{key}.png"
        path.parent.mkdir(parents=True, exist_ok=True)
        image.save(path)

    def test_a_single_becomes_a_webp_in_pack_json_and_reruns_are_idempotent(self):
        self.drop("spot/ideas", helpers.subject_on())
        self.assertEqual(self.run_process("--key", "spot/ideas"), 0)
        entry = self.manifest()["coverage"]["spot/ideas"]
        webp = self.out / PACK / "spot/ideas.webp"
        self.assertEqual(entry, {"file": "spot/ideas.webp", "sha256": common.sha256_file(webp), "width": 320, "height": 240})
        with Image.open(webp) as image:
            self.assertEqual((image.format, image.size, image.mode), ("WEBP", (320, 240), "RGBA"))
        stamp = webp.stat().st_mtime_ns
        self.assertEqual(self.run_process("--all"), 0)
        self.assertEqual(webp.stat().st_mtime_ns, stamp, "an unchanged input is skipped")
        self.assertEqual(self.manifest()["name"], "Flat")
        self.assertEqual(self.run_process("--key", "spot/ideas", "--force"), 0)
        self.assertEqual(self.manifest()["coverage"]["spot/ideas"]["sha256"], entry["sha256"], "encoding is deterministic")

    def test_flow_jpegs_are_read_and_raw_folders_ignored(self):
        path = self.inbox / "spot/calm.jpg"
        path.parent.mkdir(parents=True, exist_ok=True)
        helpers.as_jpeg(helpers.cream_and_coral(size=(1024, 1024))).save(path, quality=95)
        raw = self.inbox / "_raw" / "flow-download-1.jpg"
        raw.parent.mkdir(parents=True)
        helpers.as_jpeg(helpers.subject_on()).save(raw)
        ctx = process.build_context(process.parse_args([PACK, "--inbox", str(self.inbox), "--out-root", str(self.out), "--work", str(self.work), "--catalog", str(self.catalog), "--assets", str(self.assets)]))
        self.assertEqual(process.inbox_items(ctx), (["spot/calm"], []))
        self.assertEqual(process.find_inbox(self.inbox, "spot/calm"), path)
        self.assertEqual(self.run_process("--all"), 0)
        with Image.open(self.out / PACK / "spot/calm.webp") as image:
            keyed = image.convert("RGBA")
        self.assertEqual(keyed.size, (320, 240))
        self.assertEqual(imaging.magenta_share(keyed), 0.0)

    def test_layers_must_line_up_with_their_reference(self):
        self.drop("portrait/face/round", helpers.disc_layer())
        self.drop("portrait/face/oval", helpers.disc_layer(centre=(0.7, 0.45)))
        self.assertEqual(self.run_process("--all"), 1, "a misaligned layer is a failure")
        coverage = self.manifest()["coverage"]
        self.assertIn("portrait/face/round", coverage)
        self.assertEqual((coverage["portrait/face/round"]["width"], coverage["portrait/face/round"]["height"]), (512, 512))
        self.assertNotIn("portrait/face/oval", coverage, "Classic draws the misaligned layer")
        state = json.loads((self.work / "state" / f"{PACK}.json").read_text(encoding="utf-8"))
        self.assertIn("IoU", state["items"]["portrait/face/oval"]["reasons"][0])

    def test_check_passes_good_files_and_flags_bad_ones(self):
        self.drop("spot/ideas", helpers.subject_on())
        self.drop("palace/hall", helpers.subject_on(shadow=False, box=(0, 0, 799, 599)))
        self.assertEqual(self.run_process("--all", "--check"), 0)
        report = json.loads((self.work / "qa" / f"{PACK}.json").read_text(encoding="utf-8"))
        self.assertEqual({k: v["status"] for k, v in report.items()}, {"spot/ideas": "pass", "palace/hall": "pass"})
        self.assertTrue((self.work / f"contact-{PACK}-spot.png").is_file())
        self.assertTrue((self.work / f"contact-{PACK}-palace.png").is_file())
        pack_dir = self.out / PACK
        bad = Image.new("RGBA", (320, 240), (0, 0, 0, 0))
        draw = ImageDraw.Draw(bad)
        draw.rectangle((0, 0, 40, 40), fill=(20, 20, 20, 255))
        draw.rectangle((100, 60, 220, 180), fill=(255, 0, 255, 255))
        bad.save(pack_dir / "spot/ideas.webp", "WEBP", lossless=True)
        manifest = self.manifest()
        manifest["coverage"]["spot/ideas"]["sha256"] = common.sha256_file(pack_dir / "spot/ideas.webp")
        common.write_json(pack_dir / "pack.json", manifest)
        self.assertEqual(self.run_process("--check"), 1)
        reasons = " ".join(json.loads((self.work / "qa" / f"{PACK}.json").read_text(encoding="utf-8"))["spot/ideas"]["reasons"])
        self.assertIn("corners", reasons)
        self.assertIn("magenta", reasons)

    def test_a_sheet_round_trips_and_bad_cells_are_flagged(self):
        data = catalog.load(self.catalog)
        batches = sheets.sheet_batches(data)
        self.assertEqual(sorted(batches), ["layer-portrait-face-01", "spot-01"])
        self.assertEqual(len(batches["spot-01"]), 4)
        for key in KEYS:
            if key.startswith("spot/"):
                path = self.refs / f"{key}.png"
                path.parent.mkdir(parents=True, exist_ok=True)
                Image.new("RGBA", (320, 240), (10, 120, 200, 255)).save(path)
        layout = sheets.write_sheet(PACK, "spot-01", batches["spot-01"], data["templates"], self.sheets, self.refs)
        self.assertTrue((self.sheets / "spot-01.png").is_file())
        text = (self.sheets / "spot-01.txt").read_text(encoding="utf-8")
        self.assertIn("Row 1, cell 1:", text)
        self.assertIn("Row 2, cell 2:", text)
        generated = self.fake_generation(layout)
        half = generated.resize((generated.width // 2, generated.height // 2), Image.Resampling.LANCZOS)
        self.inbox.mkdir(parents=True, exist_ok=True)
        half.save(self.inbox / "spot-01.png")  # a smaller download than the sheet: boxes are scaled
        self.assertEqual(self.run_process("--sheet", "spot-01"), 1)
        coverage = self.manifest()["coverage"]
        keys = [cell["key"] for cell in layout["cells"]]
        self.assertIn(keys[0], coverage)
        self.assertIn(keys[3], coverage)
        self.assertNotIn(keys[1], coverage, "a part crossing into the gutter")
        self.assertNotIn(keys[2], coverage, "an empty cell")
        state = json.loads((self.work / "state" / f"{PACK}.json").read_text(encoding="utf-8"))["items"]
        self.assertIn("touches the cell edge", state[keys[1]]["reasons"][0])
        self.assertIn("empty", state[keys[2]]["reasons"][0])

    def fake_generation(self, layout):
        """What Flow might return: a subject of two parts in cell 1, one crossing into the gutter in
        cell 2, none in cell 3 and one plain subject in cell 4."""
        image = Image.new("RGB", tuple(layout["size"]), common.MAGENTA)
        draw = ImageDraw.Draw(image)
        for index, cell in enumerate(layout["cells"]):
            x, y, w, h = cell["box"]
            if index == 0:
                draw.ellipse((x + w * 0.1, y + h * 0.2, x + w * 0.7, y + h * 0.8), fill=(30, 140, 220))
                draw.ellipse((x + w * 0.78, y + h * 0.1, x + w * 0.9, y + h * 0.26), fill=(250, 200, 40))
            elif index == 1:
                g = layout["gutter"]
                draw.ellipse((x + w * 0.3, y + h * 0.3, x + w + g, y + h * 0.7), fill=(240, 240, 240))
            elif index != 2:
                draw.ellipse((x + w * 0.2, y + h * 0.15, x + w * 0.8, y + h * 0.85), fill=(30, 140, 220))
        return image


class FlowSheet(unittest.TestCase):
    """A 3x3 sheet composed at 2048 comes back from Flow as a 1024 JPEG and is sliced."""

    def setUp(self):
        self.tmp = helpers.scratch("flowsheet")
        keys = {f"mbti/type{i}": (160, 120) for i in range(9)}
        self.assets = helpers.asset_tree(self.tmp, keys)
        templates = {p: build_catalog.extract_templates(self.tmp / "none", p) for p in common.PACK_IDS}
        self.data = build_catalog.build(self.assets, {"groups": {}, "items": {}}, templates, {})
        self.catalog = self.tmp / "catalog.yaml"
        self.catalog.write_text(catalog.dumps(self.data), encoding="utf-8")
        self.refs = self.tmp / "refs"
        for key in keys:
            path = self.refs / f"{key}.png"
            path.parent.mkdir(parents=True, exist_ok=True)
            Image.new("RGBA", (320, 240), (40, 160, 120, 255)).save(path)

    def tearDown(self):
        helpers.cleanup(self.tmp)

    def test_a_1024_jpeg_of_a_2048_sheet_slices_cleanly(self):
        batches = sheets.sheet_batches(self.data)
        self.assertEqual(list(batches), ["trait-mbti-01"])
        layout = sheets.write_sheet(PACK, "trait-mbti-01", batches["trait-mbti-01"], self.data["templates"], self.tmp / "sheets", self.refs)
        self.assertEqual(max(layout["size"]), 2048)
        self.assertIn(layout["aspect"], {"16:9", "4:3", "1:1", "3:4", "9:16"})
        generated = Image.new("RGB", tuple(layout["size"]), common.MAGENTA)
        draw = ImageDraw.Draw(generated)
        for cell in layout["cells"]:
            x, y, w, h = cell["box"]
            draw.ellipse((x + w * 0.2, y + h * 0.15, x + w * 0.8, y + h * 0.85), fill=helpers.CREAM)
            draw.rectangle((x + w * 0.4, y + h * 0.4, x + w * 0.6, y + h * 0.6), fill=helpers.CORAL)
        scale = 1024 / max(generated.size)
        small = generated.resize((round(generated.width * scale), round(generated.height * scale)), Image.Resampling.LANCZOS)
        inbox = self.tmp / "inbox"
        inbox.mkdir()
        helpers.as_jpeg(small).save(inbox / "trait-mbti-01.jpg", quality=95)
        out = self.tmp / "packs"
        argv = [PACK, "--sheet", "trait-mbti-01", "--inbox", str(inbox), "--out-root", str(out), "--work", str(self.tmp / "work"),
                "--refs", str(self.refs), "--sheets", str(self.tmp / "sheets"), "--catalog", str(self.catalog), "--assets", str(self.assets)]
        self.assertEqual(process.main(argv), 0)
        coverage = json.loads((out / PACK / "pack.json").read_text(encoding="utf-8"))["coverage"]
        self.assertEqual(sorted(coverage), sorted(c["key"] for c in layout["cells"]))
        for key, entry in coverage.items():
            self.assertEqual((entry["width"], entry["height"]), (320, 240), key)
            with Image.open(out / PACK / entry["file"]) as image:
                keyed = image.convert("RGBA")
            self.assertEqual(imaging.magenta_share(keyed), 0.0, key)
            self.assertTrue(imaging.corners_transparent(keyed), key)


class SheetLayout(unittest.TestCase):
    def test_cells_fit_with_gutters_and_a_flow_aspect(self):
        self.assertEqual({s[0] for s in sheets.SIZES}, {"16:9", "4:3", "1:1", "3:4", "9:16"}, "Flow's only aspects")
        for label, width, height in sheets.SIZES:
            a, b = (int(n) for n in label.split(":"))
            self.assertAlmostEqual(width / height, a / b, delta=0.01)
        for count, batch, canvas in [(9, "sheet3x3", (160, 120)), (4, "sheet2x2", (320, 120)), (4, "sheet2x2", (200, 240)), (5, "sheet3x3", (64, 64)), (9, "sheet3x3", (256, 256))]:
            plan = sheets.layout(count, batch, canvas)
            width, height = plan["size"]
            self.assertIn(plan["aspect"], [s[0] for s in sheets.SIZES])
            self.assertLessEqual(max(width, height), 2048)
            for cell in plan["cells"]:
                x, y, w, h = cell["box"]
                self.assertTrue(0 < x and 0 < y and x + w < width and y + h < height, (count, batch, cell))
                self.assertAlmostEqual(w / h, canvas[0] / canvas[1], delta=0.02)
                self.assertGreaterEqual(plan["gutter"], sheets.GUTTER_SHARE * w - 1)
        self.assertEqual(sheets.layout(4, "sheet2x2", (320, 120))["aspect"], "16:9")
        self.assertEqual(sheets.layout(5, "sheet3x3", (64, 64))["grid"], [2, 3])

    def test_missing_refs_are_named(self):
        entries = [{"key": "spot/x", "batch": "sheet2x2", "canvas": [160, 120], "layered": False, "fit": "trim", "subject": "x"},
                   {"key": "spot/y", "batch": "sheet2x2", "canvas": [160, 120], "layered": False, "fit": "trim", "subject": "y"}]
        tmp = helpers.scratch("refs")
        try:
            with self.assertRaises(FileNotFoundError) as caught:
                sheets.write_sheet(PACK, "spot-01", entries, {}, tmp, Path(tmp) / "none")
            self.assertIn("spot/x", str(caught.exception))
            sheets.write_sheet(PACK, "spot-01", entries, {}, tmp, Path(tmp) / "none", placeholder=True)
            self.assertTrue((tmp / "spot-01.png").is_file())
        finally:
            helpers.cleanup(tmp)


if __name__ == "__main__":
    unittest.main()
