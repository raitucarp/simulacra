"""Keying the magenta background out, fitting to the canvas and the image checks."""

import unittest

import helpers

from PIL import Image, ImageDraw

import imaging


def magentaness(pixel) -> int:
    return min(pixel[0], pixel[2]) - pixel[1]


class Keying(unittest.TestCase):
    def check_keyed(self, background):
        source = helpers.subject_on(background)
        keyed = imaging.key_background(source)
        width, height = keyed.size
        self.assertEqual(keyed.mode, "RGBA")
        self.assertEqual(keyed.getpixel((3, 3))[3], 0, "background corners become transparent")
        self.assertEqual(keyed.getpixel((width // 2, height // 2)), (140, 60, 190, 255), "an interior purple stays opaque and untouched")
        shadow = keyed.getpixel((width // 2, int(height * 0.76)))
        self.assertTrue(0 < shadow[3] < 200 and max(shadow[:3]) < 40, f"a cast shadow becomes translucent black, not magenta: {shadow}")
        self.assertEqual(imaging.magenta_share(keyed), 0.0)
        alpha = keyed.getchannel("A")
        edge = [keyed.getpixel((x, y)) for y in range(0, height, 7) for x in range(0, width, 3) if 16 < alpha.getpixel((x, y)) < 240]
        self.assertTrue(edge, "anti-aliased edges keep partial alpha")
        fringe = [p for p in edge if magentaness(p) > 40 and not (p[0] < 170 and p[2] > 150)]
        self.assertLess(len(fringe), len(edge) * 0.02, f"magenta fringe on edges: {fringe[:5]}")

    def test_pure_magenta(self):
        self.check_keyed(helpers.MAGENTA)

    def test_drifted_magenta_is_estimated_from_the_border(self):
        self.assertEqual(imaging.estimate_background(helpers.subject_on(helpers.OFF_MAGENTA)), helpers.OFF_MAGENTA)
        self.check_keyed(helpers.OFF_MAGENTA)

    def test_a_flow_jpeg_keys_without_a_pink_fringe(self):
        """Flow serves 4:2:0 JPEG: chroma subsampling smears magenta into the subject's edge."""
        keyed = imaging.key_background(helpers.as_jpeg(helpers.cream_and_coral()))
        alpha = keyed.getchannel("A")
        width, height = keyed.size
        pixels = keyed.load()
        edge = []
        for y in range(0, height, 2):
            for x in range(1, width - 1):
                a = pixels[x, y][3]
                if a > 25 and (a < 255 or pixels[x - 1, y][3] < 255 or pixels[x + 1, y][3] < 255):
                    edge.append(pixels[x, y])
        self.assertGreater(len(edge), 500, "the shapes have anti-aliased edges")
        pink = [p for p in edge if min(p[0], p[2]) - p[1] > imaging.SPILL_ALLOWANCE + 2]
        self.assertEqual(pink, [], "no edge pixel keeps a magenta cast")
        self.assertEqual(imaging.magenta_share(keyed), 0.0)
        self.assertEqual(alpha.getpixel((5, 5)), 0)
        cream_edge = [pixels[x, height * 46 // 100] for x in range(width * 18 // 100, width * 24 // 100) if 25 < pixels[x, height * 46 // 100][3]]
        self.assertTrue(cream_edge)
        for pixel in cream_edge:
            self.assertLess(max(abs(c - k) for c, k in zip(pixel[:3], helpers.CREAM)), 24, f"the cream edge keeps its colour: {pixel}")

    def test_an_image_without_magenta_border_assumes_pure_magenta(self):
        self.assertEqual(imaging.estimate_background(Image.new("RGB", (40, 40), (10, 200, 10))), (255, 0, 255))


class Fitting(unittest.TestCase):
    def keyed_box(self, size=(400, 300), box=(50, 100, 150, 150)):
        image = Image.new("RGBA", size, (0, 0, 0, 0))
        ImageDraw.Draw(image).rectangle(box, fill=(10, 200, 90, 255))
        return image

    def test_trim_pads_to_the_canvas_aspect_with_the_subject_centred(self):
        fitted = imaging.fit(self.keyed_box(), "trim", (160, 120), (320, 240))
        self.assertEqual(fitted.size, (320, 240))
        x0, y0, x1, y1 = imaging.subject_bbox(fitted)
        self.assertAlmostEqual((x1 - x0) / 320, imaging.TRIM_FILL, delta=0.03, msg="a wide subject fills 80 % of the width")
        self.assertAlmostEqual((x0 + x1) / 2, 160, delta=2)
        self.assertAlmostEqual((y0 + y1) / 2, 120, delta=2)
        self.assertTrue(imaging.corners_transparent(fitted))

    def test_tall_subjects_are_limited_by_height(self):
        fitted = imaging.fit(self.keyed_box(box=(10, 10, 40, 290)), "trim", (160, 120), (320, 240))
        x0, y0, x1, y1 = imaging.subject_bbox(fitted)
        self.assertAlmostEqual((y1 - y0) / 240, imaging.TRIM_FILL, delta=0.03)

    def test_canvas_fit_keeps_the_layout(self):
        image = self.keyed_box(size=(1000, 1000), box=(100, 200, 300, 400))
        fitted = imaging.fit(image, "canvas", (256, 256), (512, 512))
        self.assertEqual(fitted.size, (512, 512))
        self.assertEqual(imaging.subject_bbox(fitted)[:2], (51, 102), "nothing is trimmed or moved")
        wide = imaging.fit(self.keyed_box(size=(1100, 1000)), "canvas", (256, 256), (512, 512))
        self.assertEqual(wide.size, (512, 512), "a slightly wider download is centre-cropped")

    def test_an_empty_image_cannot_be_trimmed(self):
        with self.assertRaises(ValueError):
            imaging.fit(Image.new("RGBA", (50, 50), (0, 0, 0, 0)), "trim", (160, 120), (320, 240))


class Checks(unittest.TestCase):
    def test_components_count_subjects_and_ignore_specks(self):
        image = Image.new("RGBA", (300, 200), (0, 0, 0, 0))
        draw = ImageDraw.Draw(image)
        draw.ellipse((20, 20, 120, 120), fill=(255, 255, 255, 255))
        draw.ellipse((170, 40, 270, 140), fill=(255, 255, 255, 255))
        draw.point((290, 190), fill=(255, 255, 255, 255))
        found = imaging.significant(imaging.components(image))
        self.assertEqual(len(found), 2)
        self.assertFalse(found[0]["touches"])

    def test_iou(self):
        reference = helpers.reference_disc()
        self.assertGreater(imaging.alpha_iou(reference, reference), 0.99)
        moved = helpers.reference_disc(centre=(0.62, 0.45))
        self.assertLess(imaging.alpha_iou(moved, reference), 0.7)

    def test_coverage_and_magenta_share(self):
        image = Image.new("RGBA", (10, 10), (255, 0, 255, 255))
        self.assertEqual(imaging.magenta_share(image), 1.0)
        self.assertEqual(imaging.alpha_coverage(image), 1.0)


if __name__ == "__main__":
    unittest.main()
