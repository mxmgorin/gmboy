#!/usr/bin/env python3
"""Generate the oxGBC wordmark logo (media/logo.svg).

Pure-stdlib pixel-art generator: each cell becomes an SVG <rect>, so the result
renders reliably on GitHub (no web fonts, no rasterization) and scales crisply.
The five "oxGBC" letters sit on one baseline, engraved into the cool-blue steel
plate — "ox" rusts (the pun: ox = oxide), "GBC" keep their Game Boy Color hue.
The opaque plate reads on both the light and dark README themes; the look lives
in tools/brand.py.

Usage:
    python3 tools/gen_logo.py         # rewrites media/logo.svg

Tweak CELL (pixel size) or PAD (border, in cells) below; edit the rust, colours,
and engraving in tools/brand.py. Preview on both README themes with e.g.:
    rsvg-convert -w 300 --background-color=white media/logo.svg -o /tmp/w.png
    rsvg-convert -w 300 --background-color='#0d1117' media/logo.svg -o /tmp/d.png
"""
import os

from brand import BG_BOTTOM, engraved_body  # tools/brand.py

CELL = 20          # px per pixel-block
PAD = 2            # border around the wordmark, in cells (keeps the plate tiled)

# 5-wide bitmaps. Lowercase o/x leave the top 2 rows empty so they sit shorter
# on the baseline -> conveys the "ox" (small) + "GBC" (caps) mixed-case brand.
glyphs = {
    'o': ["00000", "00000", "01110", "10001", "10001", "10001", "01110"],
    'x': ["00000", "00000", "10001", "01010", "00100", "01010", "10001"],
    'G': ["01110", "10001", "10000", "10111", "10001", "10001", "01110"],
    'B': ["11110", "10001", "10001", "11110", "10001", "10001", "11110"],
    'C': ["01110", "10001", "10000", "10000", "10000", "10001", "01110"],
}
order = ['o', 'x', 'G', 'B', 'C']

starts = {}
col = 0
for g in order:
    starts[g] = col
    col += 6  # 5 wide + 1 col gap
grid_cols = col - 1
grid_rows = 7

# The plate is the letter grid plus a PAD-cell border, tiled in whole cells.
COLS = grid_cols + 2 * PAD
ROWS = grid_rows + 2 * PAD
W = COLS * CELL
H = ROWS * CELL
rx = round(CELL * 0.9)

# Letters engraved into the plate (brand.py bakes the rust/GBC grooves + bevels).
letterset = {}
for g in order:
    for r, row in enumerate(glyphs[g]):
        for cc, ch in enumerate(row):
            if ch == '1':
                letterset[(PAD + starts[g] + cc, PAD + r)] = g
body = engraved_body(COLS, ROWS, CELL, letterset)

svg = f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" shape-rendering="crispEdges" role="img" aria-label="oxGBC">
  <defs>
    <clipPath id="plate"><rect x="0" y="0" width="{W}" height="{H}" rx="{rx}"/></clipPath>
  </defs>
  <rect x="0" y="0" width="{W}" height="{H}" rx="{rx}" fill="{BG_BOTTOM}"/>
  <g clip-path="url(#plate)">
  {body}
  </g>
</svg>
'''

out = os.path.normpath(os.path.join(
    os.path.dirname(os.path.abspath(__file__)), "..", "media", "logo.svg"))
with open(out, "w") as f:
    f.write(svg)
print(f"wrote {out}  ({W}x{H})")
