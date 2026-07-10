#!/usr/bin/env python3
"""Generate an alternative web favicon: a pixel-art Game Boy Color handheld.

A tiny GBC — lit screen, power LED, D-pad, A/B buttons, Start/Select — drawn as
one <rect> per pixel on the same dark rounded "screen" background as the app
icon, with a subtle LCD pixel grid to match the rest of the brand marks.

This is a sibling to gen_favicon.py (the "ox" wordmark favicon); both write the
same web/assets/ outputs, so run whichever mark you want to ship:
    favicon.svg            modern browsers
    favicon-32.png         fallback
    favicon-16.png         fallback
    apple-touch-icon.png   iOS home screen (180x180)

Usage:
    python3 scripts/gen_favicon_gbc.py

Tweak CELL, the DEVICE bitmap, or COLORS below, then re-run.
"""
import os
import shutil
import subprocess

CELL = 20
MARGIN = 0  # no padding: the device fills the canvas top-to-bottom

# Palette. '.' pixels are transparent and show the dark background through.
COLORS = {
    '#': "#b7b1d8",   # body shell (atomic purple)
    'k': "#161b22",   # screen bezel (dark)
    's': "#9aa4a0",   # screen (greyish LCD)
    'r': "#ff3b30",   # power LED (red)
    'b': "#161b22",   # buttons: D-pad, A/B, Start/Select (dark)
}

# 13-wide device bitmap. Rounded top/bottom corners are left transparent.
DEVICE = [
    "..#########..",
    ".###########.",
    ".#kkkkkkkkk#.",
    ".#kkkkkkkkk#.",
    ".#kkssssskk#.",
    ".#kkssssskk#.",
    ".#kkssssskk#.",
    ".#kkkkkkkkk#.",
    ".#kkkkkkkkk#.",
    ".###########.",
    ".##b######b#.",
    ".#bbb###b###.",
    ".##b########.",
    ".####b#b####.",
    ".###########.",
    "..#########..",
]

dev_h = len(DEVICE)
dev_w = max(len(row) for row in DEVICE)
side = max(dev_w, dev_h) + 2 * MARGIN
SIDE_PX = side * CELL
offx = (side - dev_w) // 2
offy = (side - dev_h) // 2

rects = []
clip = []
for r, row in enumerate(DEVICE):
    for c, ch in enumerate(row):
        if ch == '.':
            continue
        x = (offx + c) * CELL
        y = (offy + r) * CELL
        rects.append(
            f'<rect x="{x}" y="{y}" width="{CELL}" height="{CELL}" fill="{COLORS[ch]}"/>')
        clip.append(f'<rect x="{x}" y="{y}" width="{CELL}" height="{CELL}"/>')

# LCD-style pixel grid: thin dark seams on every cell boundary, clipped to the
# device pixels so seams show only on the handheld (background is transparent).
grid = []
for k in range(1, side):
    p = k * CELL
    grid.append(f'<line x1="{p}" y1="0" x2="{p}" y2="{SIDE_PX}"/>')
    grid.append(f'<line x1="0" y1="{p}" x2="{SIDE_PX}" y2="{p}"/>')

svg = f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {SIDE_PX} {SIDE_PX}" width="{SIDE_PX}" height="{SIDE_PX}" shape-rendering="crispEdges" role="img" aria-label="oxGBC">
  <defs>
    <clipPath id="device">
  {chr(10).join("  " + c for c in clip)}
    </clipPath>
  </defs>
  {chr(10).join("  " + r for r in rects)}
  <g clip-path="url(#device)" stroke="#000" stroke-opacity="0.22" stroke-width="2">
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
