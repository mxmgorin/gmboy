#!/usr/bin/env python3
"""Generate the Windows app icon (media/oxgbc.ico) from media/icon.svg.

Windows embeds a multi-resolution .ico into the executable (build.rs +
winresource, see crates/desktop). Each resolution is rendered straight from the
SVG at its native pixel size — rather than downscaling one master — so the
pixel-art stays crisp at every size, then ImageMagick packs them into one .ico
(same SVG-as-source-of-truth approach as tools/gen_icon.py / gen_macos_icon.py).

Usage:
    python3 tools/gen_windows_icon.py   # requires rsvg-convert and ImageMagick
"""
import os
import shutil
import subprocess
import tempfile

SIZES = [16, 32, 48, 64, 128, 256]

root = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
svg_path = os.path.join(root, "media", "icon.svg")
ico_path = os.path.join(root, "media", "oxgbc.ico")

rsvg = shutil.which("rsvg-convert")
if not rsvg:
    raise SystemExit("rsvg-convert not found on PATH — install librsvg (e.g. `apt install librsvg2-bin`).")
magick = shutil.which("magick") or shutil.which("convert")
if not magick:
    raise SystemExit("ImageMagick not found on PATH — install it (e.g. `apt install imagemagick`).")

with tempfile.TemporaryDirectory() as tmp:
    pngs = []
    for px in SIZES:
        out = os.path.join(tmp, f"icon_{px}.png")
        subprocess.run([rsvg, "-w", str(px), "-h", str(px), svg_path, "-o", out], check=True)
        pngs.append(out)
        print(f"  rendered {px}x{px}")
    subprocess.run([magick, *pngs, ico_path], check=True)

print(f"wrote {os.path.relpath(ico_path, root)}  ({len(SIZES)} sizes: {', '.join(map(str, SIZES))})")
