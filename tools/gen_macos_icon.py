#!/usr/bin/env python3
"""Generate the macOS app iconset (media/oxgbc.iconset/) from media/icon.svg.

macOS packages an app icon as an `.icns` file assembled by `iconutil` from a
directory of specifically-named PNGs (an ".iconset"). Only `iconutil` (macOS)
can write the final `.icns`, so CI runs that step on the runner; here we just
rasterize the SVG into the ten required PNG buckets so the source of truth stays
the SVG (same approach as tools/gen_icon.py for the Android launcher mipmaps).

Each bucket is rendered straight from the SVG at its native pixel size (rather
than downscaling one master) so the pixel-art stays crisp at every resolution.

Usage:
    python3 tools/gen_macos_icon.py     # requires rsvg-convert on PATH
"""
import os
import shutil
import subprocess

# The exact filenames iconutil expects, mapped to their pixel sizes.
BUCKETS = {
    "icon_16x16.png": 16,
    "icon_16x16@2x.png": 32,
    "icon_32x32.png": 32,
    "icon_32x32@2x.png": 64,
    "icon_128x128.png": 128,
    "icon_128x128@2x.png": 256,
    "icon_256x256.png": 256,
    "icon_256x256@2x.png": 512,
    "icon_512x512.png": 512,
    "icon_512x512@2x.png": 1024,
}

root = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
svg_path = os.path.join(root, "media", "icon.svg")
iconset = os.path.join(root, "media", "oxgbc.iconset")

rsvg = shutil.which("rsvg-convert")
if not rsvg:
    raise SystemExit("rsvg-convert not found on PATH — install librsvg (e.g. `apt install librsvg2-bin`).")

os.makedirs(iconset, exist_ok=True)
for name, px in BUCKETS.items():
    out = os.path.join(iconset, name)
    subprocess.run([rsvg, "-w", str(px), "-h", str(px), svg_path, "-o", out], check=True)
    print(f"  {name}: {px}x{px} -> {os.path.relpath(out, root)}")

print(f"wrote {os.path.relpath(iconset, root)}  ({len(BUCKETS)} buckets)")
print("On macOS, assemble the .icns with:  iconutil -c icns media/oxgbc.iconset")
