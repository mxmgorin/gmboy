"""Shared oxGBC brand palette + engraved-mark engine — single source of truth.

Direction: the "oxGBC" word is engraved/stamped INTO a cool-blue brushed-steel
plate, with oxidation pooled in the grooves. Each letter cell is recessed with a
bevel (dark shadow on its top/left lip, light highlight on its bottom/right lip)
over a groove floor: the "ox" grooves are heavy iron-oxide rust — the name is the
pun, ox = oxide — while the "GBC" grooves keep their vivid Game Boy Color hue. A
scorch darkens the plate around the whole word, with extra rust bloom hugging the
"ox". The thin LCD pixel grid seams every cell, tying the marks to the Game Boy
Color heritage.

Every mark generator imports from here, so tweaking the look is an edit here
followed by `make icons`. Colours are baked per-pixel (not SVG gradients) so the
result is identical in every renderer — including GitHub's — and stays crisp
pixel-art. The look is a pure function of cell coordinates via a fixed integer
hash, so re-running a generator is deterministic (byte-identical SVGs, clean
diffs). The scripts run as `python3 tools/gen_*.py`, so `tools/` is on sys.path[0]
and `from brand import ...` resolves to this file.

Public API:
    GBC                              bright per-letter Game Boy Color palette
    BG_BOTTOM                        fill behind the plate (shows at the corners)
    rust_at(t)                       colour along the iron-oxide ramp, t in [0,1]
    mute(hex, amt)                   desaturate a colour toward its own grey
    steel_tiles(cols, rows, cell)    plain cool-blue steel panel (no letters)
    engraved_body(cols, rows, cell, letterset[, oxide])
                                     full inner SVG for a letter mark (tiles +
                                     grid + engraved floors + bevels)
"""

# Canonical bright Game Boy Color per-letter palette (the GBC grooves keep this
# hue; the "ox" grooves ignore it and rust instead).
GBC = {
    'o': "#ff3b30",  # red
    'x': "#a259ff",  # purple
    'G': "#34c759",  # green
    'B': "#ffd60a",  # yellow
    'C': "#30d5c8",  # turquoise
}

# Base fill behind the plate (shows only at the rounded corners).
BG_BOTTOM = "#0e1011"

# The plate: cool-blue brushed steel, greyed a touch so it reads as metal. Cooler
# than neutral so the warm rust "ox" gets complementary contrast against it.
_STEEL_HEX, _STEEL_MUTE = "#3d4756", 0.18

# Iron-oxide ramp the grooves rust along (light bloom -> deep oxide).
RUST_RAMP = [
    (0.00, "#8a6a52"),
    (0.45, "#a5551f"),
    (0.75, "#7a3417"),
    (1.00, "#41210f"),
]

# Engraving recipe --------------------------------------------------------------
# GBC groove floor: the hue kept mostly saturated (small mute) and darkened to
# k+jitter of full brightness, so it reads as vivid colour recessed into metal.
_GBC_GROOVE_MUTE, _GBC_GROOVE_K = 0.12, 0.74
# "ox" groove floor: rust from _OX_LO..(_OX_LO+_OX_SPAN) by per-cell hash; the
# most-corroded cells (hash past _OX_PIT_T) drop to a dark pit. Kept in the
# brighter, oranger part of the ramp with few/light pits so the "ox" reads as
# letters (not a dark smudge) when the icon is shrunk to 32-48px.
_OX_LO, _OX_SPAN = 0.30, 0.35
_OX_PIT_T, _OX_PIT_COL = 0.90, "#3a1c0d"
# Scorch: darken plate cells by Chebyshev distance to the nearest letter cell.
_SCORCH = {1: 0.55, 2: 0.74, 3: 0.88}
# Rust bloom: plate cells within 1 of the "ox" blend toward rust by a hashed amt.
_BLEED_T, _BLEED_K = 0.40, 0.90
# Bevel: strip width as a fraction of the cell; dark shadow on top/left lips,
# light highlight on bottom/right lips (reads as an incised groove).
_BEVEL_FRAC = 0.14
_BEVEL_SHADOW, _BEVEL_SHADOW_OP = "#040302", 0.58
_BEVEL_HILITE, _BEVEL_HILITE_OP = "#c2c8cf", 0.42
# LCD grid seams.
GRID_STROKE, GRID_OPACITY = "#000", 0.22


def _hex_to_rgb(h):
    h = h.lstrip("#")
    return tuple(int(h[i:i + 2], 16) for i in (0, 2, 4))


def _rgb_to_hex(rgb):
    return "#{:02x}{:02x}{:02x}".format(*(max(0, min(255, round(v))) for v in rgb))


def _lerp(a, b, f):
    return tuple(a[k] + (b[k] - a[k]) * f for k in range(3))


def mute(hexcol, amount):
    """Desaturate a colour toward its own grey (luma) by amount in [0,1]."""
    r, g, b = _hex_to_rgb(hexcol)
    luma = 0.299 * r + 0.587 * g + 0.114 * b
    return _rgb_to_hex(tuple(c + (luma - c) * amount for c in (r, g, b)))


def _hash01(*xs):
    """Deterministic [0,1) hash of integer coords (murmur3-style finalizer).

    A fixed integer hash, not an RNG, so every generated SVG is reproducible.
    """
    h = 0
    for i, x in enumerate(xs):
        h ^= (x * (0x9E3779B1 if i % 2 else 0x1F1F1F1F)) & 0xFFFFFFFF
    h &= 0xFFFFFFFF
    h ^= h >> 16
    h = (h * 0x85EBCA6B) & 0xFFFFFFFF
    h ^= h >> 13
    h = (h * 0xC2B2AE35) & 0xFFFFFFFF
    h ^= h >> 16
    return h / 0xFFFFFFFF


def rust_at(t):
    """Colour at position t in [0,1] along the iron-oxide ramp."""
    t = 0.0 if t < 0 else 1.0 if t > 1 else t
    for (t0, c0), (t1, c1) in zip(RUST_RAMP, RUST_RAMP[1:]):
        if t <= t1:
            f = 0.0 if t1 == t0 else (t - t0) / (t1 - t0)
            return _lerp(_hex_to_rgb(c0), _hex_to_rgb(c1), f)
    return _hex_to_rgb(RUST_RAMP[-1][1])


# Cool-blue steel, greyed, resolved once.
STEEL = mute(_STEEL_HEX, _STEEL_MUTE)


def _cheb(cell, pts):
    c, r = cell
    return min(max(abs(c - pc), abs(r - pr)) for pc, pr in pts)


def _rect(c, r, cell, fill, x0=0, y0=0):
    return (f'<rect x="{x0 + c * cell}" y="{y0 + r * cell}" '
            f'width="{cell}" height="{cell}" fill="{fill}"/>')


def _plate_fill(c, r):
    """Base brushed-steel colour of a plate cell: STEEL with a soft 2x2 mottle."""
    v = (_hash01(c // 2, r // 2) * 2 - 1) * 0.08
    return [x * (1 + v) for x in _hex_to_rgb(STEEL)]


def steel_tiles(cols, rows, cell, x0=0, y0=0):
    """<rect>s tiling a cols x rows brushed cool-blue steel panel (no letters).

    Used for marks that sit an object on the plate (e.g. the handheld favicon)
    rather than engraving letters into it. Returns a list of rect strings.
    """
    return [_rect(c, r, cell, _rgb_to_hex(_plate_fill(c, r)), x0, y0)
            for r in range(rows) for c in range(cols)]


def _bevel(c, r, cell, bw, letterset):
    """Incised bevel on a letter cell's edges that border the plate."""
    x0, y0 = c * cell, r * cell
    out = []

    def strip(x, y, w, h, fill, op):
        out.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" '
                   f'fill="{fill}" fill-opacity="{op}"/>')

    if (c, r - 1) not in letterset:                       # top lip -> shadow
        strip(x0, y0, cell, bw, _BEVEL_SHADOW, _BEVEL_SHADOW_OP)
    if (c - 1, r) not in letterset:                       # left lip -> shadow
        strip(x0, y0, bw, cell, _BEVEL_SHADOW, _BEVEL_SHADOW_OP)
    if (c, r + 1) not in letterset:                       # bottom lip -> highlight
        strip(x0, y0 + cell - bw, cell, bw, _BEVEL_HILITE, _BEVEL_HILITE_OP)
    if (c + 1, r) not in letterset:                       # right lip -> highlight
        strip(x0 + cell - bw, y0, bw, cell, _BEVEL_HILITE, _BEVEL_HILITE_OP)
    return out


def engraved_body(cols, rows, cell, letterset, oxide=("o", "x")):
    """Inner SVG for a letter mark engraved into the steel plate.

    letterset maps {(col, row): glyph} for every filled letter cell in a
    cols x rows grid; glyphs in `oxide` rust, the rest keep their GBC hue. Draw
    the returned body inside a <g> clipped to the rounded plate; wrap with the
    BG_BOTTOM backing rect. Returns the markup for: plate tiles (scorched near
    the word, rust-bloomed by the "ox") + LCD grid + engraved groove floors +
    bevels, in paint order.
    """
    oxide = set(oxide)
    letter_cells = list(letterset.keys())
    ox_cells = [(c, r) for (c, r), g in letterset.items() if g in oxide]
    bw = max(2, round(cell * _BEVEL_FRAC))

    tiles = []
    for r in range(rows):
        for c in range(cols):
            fill = _plate_fill(c, r)
            if (c, r) not in letterset:
                d = _cheb((c, r), letter_cells)
                if d in _SCORCH:
                    fill = [x * _SCORCH[d] for x in fill]
                if ox_cells and _cheb((c, r), ox_cells) <= 1:
                    b = _hash01(c, r, 9)
                    if b > _BLEED_T:
                        fill = _lerp(fill, rust_at(0.55 + 0.4 * _hash01(c, r)),
                                     (b - _BLEED_T) * _BLEED_K)
            tiles.append(_rect(c, r, cell, _rgb_to_hex(fill)))

    floors, bevels = [], []
    for (c, r), g in letterset.items():
        p = _hash01(c, r, 3)
        if g in oxide:
            floor = (_hex_to_rgb(_OX_PIT_COL) if p > _OX_PIT_T
                     else rust_at(_OX_LO + _OX_SPAN * p))
        else:
            hue = _hex_to_rgb(mute(GBC[g], _GBC_GROOVE_MUTE))
            floor = [x * (_GBC_GROOVE_K + 0.12 * p) for x in hue]
        floors.append(_rect(c, r, cell, _rgb_to_hex(floor)))
        bevels += _bevel(c, r, cell, bw, letterset)

    w_px, h_px = cols * cell, rows * cell
    lines = []
    for k in range(1, cols):
        x = k * cell
        lines.append(f'<line x1="{x}" y1="0" x2="{x}" y2="{h_px}"/>')
    for k in range(1, rows):
        y = k * cell
        lines.append(f'<line x1="0" y1="{y}" x2="{w_px}" y2="{y}"/>')
    grid = (f'<g stroke="{GRID_STROKE}" stroke-opacity="{GRID_OPACITY}" '
            f'stroke-width="2" shape-rendering="geometricPrecision">'
            f'{"".join(lines)}</g>')

    return "\n".join(tiles + [grid] + floors + bevels)
