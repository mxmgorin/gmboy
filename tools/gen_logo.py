#!/usr/bin/env python3
"""Generate the oxGBC wordmark logo (media/logo.svg).

Pure-stdlib pixel-art generator: each "on" pixel becomes an SVG <rect>, so the
result renders reliably on GitHub (no web fonts, no rasterization) and scales
crisply. Each letter of "oxGBC" gets one solid Game Boy Color-style colour, and
a thin dark pixel outline keeps every colour (incl. yellow) legible on both the
light and dark README themes over a transparent background.

Usage:
    python3 tools/gen_logo.py         # rewrites media/logo.svg

Tweak CELL (pixel size), PAD (margin), the PALETTE, or OUTLINE/OUT below, then
re-run. Preview on both README themes with e.g.:
    rsvg-convert -w 300 --background-color=white media/logo.svg -o /tmp/w.png
    rsvg-convert -w 300 --background-color='#0d1117' media/logo.svg -o /tmp/d.png
"""
import os

CELL = 16          # px per pixel-block
PAD = 8            # transparent margin around the wordmark
OUT = 3            # dark outline thickness (px)
OUTLINE = "#0d1117"

# Game Boy Color-style palette, one solid colour per letter.
PALETTE = {
    'o': "#ff3b30",  # red
    'x': "#a259ff",  # purple
    'G': "#34c759",  # green
    'B': "#ffd60a",  # yellow
    'C': "#30d5c8",  # turquoise
}

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
W = grid_cols * CELL + 2 * PAD
H = grid_rows * CELL + 2 * PAD


def rect(x, y, w, h, fill):
    return f'<rect x="{x}" y="{y}" width="{w}" height="{h}" fill="{fill}"/>'


outline, fore = [], []
for g in order:
    col0 = starts[g]
    fill = PALETTE[g]
    for r, row in enumerate(glyphs[g]):
        for cc, ch in enumerate(row):
            if ch == '1':
                x = PAD + (col0 + cc) * CELL
                y = PAD + r * CELL
                outline.append(rect(x - OUT, y - OUT, CELL + 2 * OUT, CELL + 2 * OUT, OUTLINE))
                fore.append(rect(x, y, CELL, CELL, fill))

# LCD-style pixel grid: thin dark seams on every cell boundary, clipped to the
# coloured pixels so they show only on the letters (the logo is transparent).
grid = []
gx0, gy0 = PAD, PAD
gx1, gy1 = PAD + grid_cols * CELL, PAD + grid_rows * CELL
for k in range(grid_cols + 1):
    x = PAD + k * CELL
    grid.append(f'<line x1="{x}" y1="{gy0}" x2="{x}" y2="{gy1}"/>')
for k in range(grid_rows + 1):
    y = PAD + k * CELL
    grid.append(f'<line x1="{gx0}" y1="{y}" x2="{gx1}" y2="{y}"/>')

body = "\n".join("  " + r for r in outline + fore)
clip_body = "\n".join("    " + r for r in fore)
grid_body = "\n".join("    " + g for g in grid)
svg = f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" shape-rendering="crispEdges" role="img" aria-label="oxGBC">
  <defs>
    <clipPath id="letters">
{clip_body}
    </clipPath>
  </defs>
{body}
  <g clip-path="url(#letters)" stroke="#000" stroke-opacity="0.3" stroke-width="2">
{grid_body}
  </g>
</svg>
'''

out = os.path.normpath(os.path.join(
    os.path.dirname(os.path.abspath(__file__)), "..", "media", "logo.svg"))
with open(out, "w") as f:
    f.write(svg)
print(f"wrote {out}  ({W}x{H}, {len(fore)} pixels)")
