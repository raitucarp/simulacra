"""Tiles captures into labelled contact sheets for reviewing layout and composition at a glance.

    python docs/capture/contact_sheet.py <out_dir> <png or folder>... [--cols 2] [--per 4] [--width 1440]
"""
import argparse
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("out")
    parser.add_argument("inputs", nargs="+")
    parser.add_argument("--cols", type=int, default=2)
    parser.add_argument("--per", type=int, default=4)
    parser.add_argument("--width", type=int, default=1440)
    args = parser.parse_args()

    files: list[Path] = []
    for item in args.inputs:
        path = Path(item)
        files.extend(sorted(path.glob("*.png")) if path.is_dir() else [path])
    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    try:
        font = ImageFont.truetype("arial.ttf", 15)
    except OSError:
        font = ImageFont.load_default()

    cell_w = args.width // args.cols
    label_h = 22
    for sheet_no, start in enumerate(range(0, len(files), args.per)):
        batch = files[start : start + args.per]
        thumbs = []
        for file in batch:
            with Image.open(file) as img:
                img = img.convert("RGB")
                scale = cell_w / img.width
                thumbs.append((file.stem, img.resize((cell_w, max(1, int(img.height * scale))), Image.LANCZOS)))
        rows = (len(thumbs) + args.cols - 1) // args.cols
        row_h = [max(t.height for _, t in thumbs[r * args.cols : (r + 1) * args.cols]) + label_h for r in range(rows)]
        sheet = Image.new("RGB", (args.width, sum(row_h)), (128, 128, 128))
        draw = ImageDraw.Draw(sheet)
        y = 0
        for r in range(rows):
            for c, (name, thumb) in enumerate(thumbs[r * args.cols : (r + 1) * args.cols]):
                x = c * cell_w
                draw.rectangle([x, y, x + cell_w, y + label_h], fill=(250, 250, 250))
                draw.text((x + 6, y + 3), name, fill=(0, 0, 0), font=font)
                sheet.paste(thumb, (x, y + label_h))
            y += row_h[r]
        target = out / f"sheet-{sheet_no + 1:02d}.png"
        sheet.save(target)
        print(target)


if __name__ == "__main__":
    main()
