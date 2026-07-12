#!/usr/bin/env python3
"""Generate the oxGBC Android launcher icon (media/icon.svg + mipmap PNGs).

Staggered layout, engraved into the cool-blue steel plate:

    o x
      G B C

Lowercase "ox" juts out top-left with "x" above "G"; the "ox" grooves rust (the
name is the pun — ox = oxide), "GBC" keep their Game Boy Color hue. The look
lives in tools/brand.py; this just lays out the letters and wraps the plate.

Pixel-art via SVG <rect>. If `rsvg-convert` is on PATH, this also rasterizes the
five density buckets straight into crates/android/app/src/main/res/mipmap-*/ic_launcher.png.

Usage:
    python3 tools/gen_icon.py
"""
import os
import shutil
import subprocess

from brand import BG_BOTTOM, engraved_body  # tools/brand.py

CELL = 24
MARGIN = 3          # cells of padding inside the square
GAP = 1             # empty rows between the "ox" and "GBC" rows

# 5-wide bitmaps: lowercase o/x (top 2 rows empty -> short) + uppercase G/B/C.
glyphs = {
    'o': ["00000", "00000", "01110", "10001", "10001", "10001", "01110"],
    'x': ["00000", "00000", "10001", "01010", "00100", "01010", "10001"],
    'G': ["01110", "10001", "10000", "10111", "10001", "10001", "01110"],
    'B': ["11110", "10001", "10001", "11110", "10001", "10001", "11110"],
    'C': ["01110", "10001", "10000", "10000", "10000", "10001", "01110"],
}

# (glyph, column-step, row-step). Column step = glyph slot (5 wide + 1 gap).
# "x" and "G" share column 1 -> G sits under x; "o" at column 0 juts out left.
TOP, BOT = 0, 7 + GAP
placements = [
    ('o', 0, TOP),
    ('x', 1, TOP),
    ('G', 1, BOT),
    ('B', 2, BOT),
    ('C', 3, BOT),
]

# Collect filled cells (col, row, glyph) in an un-offset grid.
cells = []
for g, cstep, rbase in placements:
    bc = cstep * 6
    for r, line in enumerate(glyphs[g]):
        for c, bit in enumerate(line):
            if bit == '1':
                cells.append((bc + c, rbase + r, g))

minc = min(c for c, _, _ in cells)
maxc = max(c for c, _, _ in cells)
minr = min(r for _, r, _ in cells)
maxr = max(r for _, r, _ in cells)
content_w = maxc - minc + 1
content_h = maxr - minr + 1
side = max(content_w, content_h) + 2 * MARGIN
SIDE_PX = side * CELL
offx = (side - content_w) // 2 - minc
offy = (side - content_h) // 2 - minr

# Letters engraved into the plate: brand.py bakes the rust/GBC grooves + bevels.
letterset = {(c + offx, r + offy): g for c, r, g in cells}
body = engraved_body(side, side, CELL, letterset)

rx = round(SIDE_PX * 0.18)

svg = f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {SIDE_PX} {SIDE_PX}" width="{SIDE_PX}" height="{SIDE_PX}" shape-rendering="crispEdges" role="img" aria-label="oxGBC">
  <defs>
    <clipPath id="screen"><rect x="0" y="0" width="{SIDE_PX}" height="{SIDE_PX}" rx="{rx}"/></clipPath>
  </defs>
  <rect x="0" y="0" width="{SIDE_PX}" height="{SIDE_PX}" rx="{rx}" fill="{BG_BOTTOM}"/>
  <g clip-path="url(#screen)">
  {body}
  </g>
</svg>
'''

root = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
svg_path = os.path.join(root, "media", "icon.svg")
with open(svg_path, "w") as f:
    f.write(svg)
print(f"wrote {svg_path}  ({SIDE_PX}x{SIDE_PX})")

# Rasterize into the launcher mipmaps if rsvg-convert is available.
buckets = {"mdpi": 48, "hdpi": 72, "xhdpi": 96, "xxhdpi": 144, "xxxhdpi": 192}
rsvg = shutil.which("rsvg-convert")
if not rsvg:
    print("rsvg-convert not found — SVG written; PNGs not regenerated.")
else:
    for name, px in buckets.items():
        out = os.path.join(root, "crates", "android", "app", "src", "main", "res",
                           f"mipmap-{name}", "ic_launcher.png")
        subprocess.run([rsvg, "-w", str(px), "-h", str(px), svg_path, "-o", out], check=True)
        print(f"  {name}: {px}x{px} -> {os.path.relpath(out, root)}")
