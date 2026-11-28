"""make_simpack.py: the archive holds exactly pack.json and the coverage files, verified."""

import json
import unittest
import zipfile

import helpers

from PIL import Image

import common
import imaging
import make_simpack
import packfile


class Simpack(unittest.TestCase):
    def setUp(self):
        self.tmp = helpers.scratch("simpack")
        self.src = self.tmp / "cozy"
        pack = packfile.PackDir(self.src, "cozy", packfile.pack_meta("cozy"))
        image = Image.new("RGBA", (64, 48), (0, 0, 0, 0))
        image.paste((200, 100, 50, 255), (10, 10, 50, 40))
        pack.put("spot/ideas", imaging.encode_webp(image), (64, 48))
        pack.put("empty/no_posts", imaging.encode_webp(image), (64, 48))
        pack.save()

    def tearDown(self):
        helpers.cleanup(self.tmp)

    def test_archive_contents_and_checksum(self):
        out = self.tmp / "dist" / "cozy.simpack"
        entry = make_simpack.build(self.src, out)
        with zipfile.ZipFile(out) as archive:
            self.assertEqual(sorted(archive.namelist()), ["empty/no_posts.webp", "pack.json", "spot/ideas.webp"])
            self.assertTrue(all(info.compress_type == zipfile.ZIP_DEFLATED for info in archive.infolist()))
            manifest = json.loads(archive.read("pack.json"))
            for key, file in manifest["coverage"].items():
                self.assertEqual(common.sha256_bytes(archive.read(file["file"])), file["sha256"], key)
        self.assertEqual(entry["sha256"], common.sha256_file(out))
        self.assertEqual(entry["bytes"], out.stat().st_size)
        self.assertEqual((entry["id"], entry["version"], entry["name"]), ("cozy", "1.0.0", "Cozy"))
        again = make_simpack.build(self.src, self.tmp / "dist" / "again.simpack")
        self.assertEqual(again["sha256"], entry["sha256"], "archives are reproducible")

    def test_a_tampered_file_is_refused(self):
        (self.src / "spot/ideas.webp").write_bytes(b"not the same")
        with self.assertRaises(ValueError) as caught:
            make_simpack.build(self.src, self.tmp / "x.simpack")
        self.assertIn("spot/ideas", str(caught.exception))


class PackIds(unittest.TestCase):
    def test_classic_is_reserved(self):
        with self.assertRaises(ValueError):
            packfile.pack_meta("classic")
        with self.assertRaises(ValueError):
            packfile.pack_meta("Bad Id")


if __name__ == "__main__":
    unittest.main()
