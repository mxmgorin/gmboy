#!/usr/bin/env python3
"""Generate the oxGBC Android launcher icon (assets/icon.svg + mipmap PNGs).

Staggered layout on a dark rounded "screen" background:

    o x
      G B C

Lowercase "ox" with the "o" jutting out top-left; "x" sits directly above "G".
Each letter uses one solid Game Boy Color-style colour (matching the wordmark).

Pixel-art via SVG <rect>. If `rsvg-convert` is on PATH, this also rasterizes the
five density buckets straight into android/app/src/main/res/mipmap-*/ic_launcher.png.

Usage:
    python3 scripts/gen_icon.py
"""
import os
import shutil
import subprocess

CELL = 24
MARGIN = 3          # cells of padding inside the square
GAP = 1             # empty rows between the "ox" and "GBC" rows

# Game Boy Color-style palette, one solid colour per letter (matches the wordmark).
PALETTE = {
    'o': "#ff3b30",  # red
    'x': "#a259ff",  # purple
    'G': "#34c759",  # green
    'B': "#ffd60a",  # yellow
    'C': "#30d5c8",  # turquoise
}

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

# Collect filled cells (col, row, colour) in an un-offset grid.
cells = []
for g, cstep, rbase in placements:
    bc = cstep * 6
    for r, line in enumerate(glyphs[g]):
        for c, bit in enumerate(line):
            if bit == '1':
                cells.append((bc + c, rbase + r, PALETTE[g]))

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

rects = [
    f'<rect x="{(c + offx) * CELL}" y="{(r + offy) * CELL}" width="{CELL}" height="{CELL}" fill="{col}"/>'
    for c, r, col in cells
]

rx = round(SIDE_PX * 0.18)
svg = f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {SIDE_PX} {SIDE_PX}" width="{SIDE_PX}" height="{SIDE_PX}" role="img" aria-label="oxGBC">
  <defs>
    <linearGradient id="bg" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0" stop-color="#1b2430"/>
      <stop offset="1" stop-color="#0d1117"/>
    </linearGradient>
  </defs>
  <rect x="0" y="0" width="{SIDE_PX}" height="{SIDE_PX}" rx="{rx}" fill="url(#bg)"/>
  {chr(10).join("  " + r for r in rects)}
</svg>
'''

root = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
svg_path = os.path.join(root, "assets", "icon.svg")
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
        out = os.path.join(root, "android", "app", "src", "main", "res",
                           f"mipmap-{name}", "ic_launcher.png")
        subprocess.run([rsvg, "-w", str(px), "-h", str(px), svg_path, "-o", out], check=True)
        print(f"  {name}: {px}x{px} -> {os.path.relpath(out, root)}")
