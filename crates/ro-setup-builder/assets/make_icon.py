"""Draws assets/builder.ico (our own art: a disc with a download arrow).
    python3 -I make_icon.py OUT.ico   (needs Pillow; run in the workbench)"""
import math, sys
from PIL import Image, ImageDraw

def draw(n):
    s = 4 * n  # supersample, then shrink
    im = Image.new("RGBA", (s, s), (0, 0, 0, 0))
    d = ImageDraw.Draw(im)
    c, r = s / 2, s * 0.46
    # the disc: silver rings with a rainbow sheen
    for i in range(int(r), 0, -1):
        g = int(170 + 60 * math.sin(i / r * 9.0))
        d.ellipse([c - i, c - i, c + i, c + i], fill=(g, min(255, g + 5), min(255, g + 20), 255))
    d.ellipse([c - r, c - r, c + r, c + r], outline=(90, 95, 110, 255), width=max(1, s // 64))
    for k, col in enumerate([(255, 120, 120, 90), (120, 200, 255, 90), (150, 255, 150, 90)]):
        d.pieslice([c - r * 0.95, c - r * 0.95, c + r * 0.95, c + r * 0.95], 200 + 18 * k, 216 + 18 * k, fill=col)
    h = r * 0.30
    d.ellipse([c - h, c - h, c + h, c + h], fill=(225, 228, 235, 255), outline=(90, 95, 110, 255), width=max(1, s // 64))
    hh = r * 0.11
    d.ellipse([c - hh, c - hh, c + hh, c + hh], fill=(0, 0, 0, 0))
    # the arrow, bottom right
    ax, ay, aw = s * 0.72, s * 0.62, s * 0.15
    green, dark = (46, 160, 67, 255), (20, 90, 35, 255)
    body = [(ax - aw * 0.45, ay - aw * 1.5), (ax + aw * 0.45, ay - aw * 1.5), (ax + aw * 0.45, ay), (ax + aw, ay),
            (ax, ay + aw * 1.05), (ax - aw, ay), (ax - aw * 0.45, ay)]
    d.polygon(body, fill=green, outline=dark, width=max(1, s // 48))
    return im.resize((n, n), Image.LANCZOS)

sizes = [16, 24, 32, 48, 64, 256]
imgs = [draw(n) for n in sizes]
# BMP frames: Windows XP cannot read PNG-compressed icons (Vista can)
imgs[-1].save(sys.argv[1], format="ICO", sizes=[(n, n) for n in sizes], append_images=imgs[:-1], bitmap_format="bmp")

