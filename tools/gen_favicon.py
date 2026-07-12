#!/usr/bin/env python3
"""Generate the web favicon (crates/web/assets/favicon.svg + PNG fallbacks).

The full app icon (tools/gen_icon.py) is a muddy blur at 16px, so the favicon is
a simpler, bolder mark: just "ox" — both letters rusted, engraved into the same
cool-blue steel plate (ox = oxide) — legible in a browser tab. The look lives in
tools/brand.py.

Outputs into crates/web/assets/ (the dir the Pages deploy workflow copies):
    favicon.svg            modern browsers
    favicon-32.png         fallback
    favicon-16.png         fallback
    apple-touch-icon.png   iOS home screen (180x180)

Usage:
    python3 tools/gen_favicon.py
"""
import os
import shutil
import subprocess

from brand import BG_BOTTOM, engraved_body  # tools/brand.py

CELL = 24
MARGIN = 2

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

# Both letters rust (ox = oxide), engraved into the plate.
letterset = {}
col = offx
for g in order:
    for r, line in enumerate(glyphs[g]):
        for c, bit in enumerate(line):
            if bit == '1':
                letterset[(col + c, offy + r)] = g
    col += 6
body = engraved_body(side, side, CELL, letterset, oxide=('o', 'x'))

rx = round(SIDE_PX * 0.18)

# NB: geometricPrecision (not the crispEdges used by the app icon/logo) — a
# favicon renders at 16-24px, where crispEdges snaps each ~1px cell unevenly and
# mangles the mark. Antialiasing keeps the engraved "ox" complete at that size.
svg = f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {SIDE_PX} {SIDE_PX}" width="{SIDE_PX}" height="{SIDE_PX}" shape-rendering="geometricPrecision" role="img" aria-label="oxGBC">
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
assets = os.path.join(root, "crates", "web", "assets")
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
