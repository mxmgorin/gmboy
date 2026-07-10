#!/usr/bin/env python3
"""Generate the web favicon (web/assets/favicon.svg + PNG fallbacks).

The full app icon (scripts/gen_icon.py) is a muddy blur at 16px, so the favicon
is a simpler, bolder mark: just "ox" (red o + purple x) on the same dark rounded
"screen" background — legible in a browser tab.

Outputs into web/assets/ (the dir the Pages deploy workflow copies):
    favicon.svg            modern browsers
    favicon-32.png         fallback
    favicon-16.png         fallback
    apple-touch-icon.png   iOS home screen (180x180)

Usage:
    python3 scripts/gen_favicon.py
"""
import os
import shutil
import subprocess

CELL = 24
MARGIN = 2

# Match the wordmark/icon palette.
COLORS = {'o': "#ff3b30", 'x': "#a259ff"}  # red, purple

# Bold 7-row glyphs so "ox" fills the tab icon.
glyphs = {
    'o': ["01110", "10001", "10001", "10001", "10001", "10001", "01110"],
    'x': ["10001", "10001", "01010", "00100", "01010", "10001", "10001"],
}
order = ['o', 'x']

content_w = len(order) * 5 + (len(order) - 1)  # 11
content_h = 7
side = max(content_w, content_h) + 2 * MARGIN
SIDE_PX = side * CELL
offx = (side - content_w) // 2
offy = (side - content_h) // 2

rects = []
col = offx
for g in order:
    for r, line in enumerate(glyphs[g]):
        for c, bit in enumerate(line):
            if bit == '1':
                x = (col + c) * CELL
                y = (offy + r) * CELL
                rects.append(
                    f'<rect x="{x}" y="{y}" width="{CELL}" height="{CELL}" fill="{COLORS[g]}"/>')
    col += 6

rx = round(SIDE_PX * 0.18)

# LCD-style pixel grid: thin dark seams on every cell boundary, clipped to the
# rounded screen. Reads as pixel separation over the bright letters; near-
# invisible over the dark background.
grid = []
for k in range(1, side):
    p = k * CELL
    grid.append(f'<line x1="{p}" y1="0" x2="{p}" y2="{SIDE_PX}"/>')
    grid.append(f'<line x1="0" y1="{p}" x2="{SIDE_PX}" y2="{p}"/>')

svg = f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {SIDE_PX} {SIDE_PX}" width="{SIDE_PX}" height="{SIDE_PX}" shape-rendering="crispEdges" role="img" aria-label="oxGBC">
  <defs>
    <linearGradient id="bg" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0" stop-color="#1b2430"/>
      <stop offset="1" stop-color="#0d1117"/>
    </linearGradient>
    <clipPath id="screen"><rect x="0" y="0" width="{SIDE_PX}" height="{SIDE_PX}" rx="{rx}"/></clipPath>
  </defs>
  <rect x="0" y="0" width="{SIDE_PX}" height="{SIDE_PX}" rx="{rx}" fill="url(#bg)"/>
  {chr(10).join("  " + r for r in rects)}
  <g clip-path="url(#screen)" stroke="#000" stroke-opacity="0.3" stroke-width="2">
  {chr(10).join("  " + g for g in grid)}
  </g>
</svg>
'''

root = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
assets = os.path.join(root, "web", "assets")
svg_path = os.path.join(assets, "favicon.svg")
with open(svg_path, "w") as f:
    f.write(svg)
print(f"wrote {os.path.relpath(svg_path, root)}  ({SIDE_PX}x{SIDE_PX})")

pngs = {"favicon-16.png": 16, "favicon-32.png": 32, "apple-touch-icon.png": 180}
rsvg = shutil.which("rsvg-convert")
if not rsvg:
    print("rsvg-convert not found — SVG written; PNG fallbacks not generated.")
else:
    for name, px in pngs.items():
        out = os.path.join(assets, name)
        subprocess.run([rsvg, "-w", str(px), "-h", str(px), svg_path, "-o", out], check=True)
        print(f"  {name}: {px}x{px}")
