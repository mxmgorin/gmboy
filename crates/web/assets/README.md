# Bundled ROMs

All ROMs shipped with the web demo are free / open-source homebrew games or
publicly redistributable test ROMs. Each remains © its respective author under
the license listed below — keep this attribution when redistributing. All are
license-compatible with oxGBC's GPL-3.0.

`tobudx.gb` is auto-loaded as the default demo so the emulator shows something
the moment the page opens.

## Games

### Tobu Tobu Girl Deluxe (`tobudx.gb`)
Open-source arcade platformer for GB / GBC / SGB; renders in full CGB color.
- **Author:** Simon Larsen (DX remaster); original *Tobu Tobu Girl* by Tangram Games
- **Source:** https://github.com/SimonLarsen/tobutobugirl-dx
- **License:** code **MIT**; assets (art, text, sound, music) **CC BY 4.0**

### µCity (`ucity.gbc`)
Open-source city-building game for Game Boy Color.
- **Author:** Antonio Niño Díaz (AntonioND)
- **Source:** https://github.com/AntonioND/ucity
- **License:** **GPLv3+**

### Geometrix (`geometrix.gbc`)
Puzzle game for GB / GBC.
- **Author:** Antonio Niño Díaz (AntonioND)
- **Source:** https://github.com/AntonioND/geometrix
- **License:** **GPLv3**

### Libbet and the Magic Floor (`libbet.gb`)
Puzzle game — roll a ball to erase every tile.
- **Author:** Damian Yerrick (pinobatch)
- **Source:** https://github.com/pinobatch/libbet
- **License:** **Zlib**

## Test ROMs

### dmg-acid2 (`dmg-acid2.gb`) · cgb-acid2 (`cgb-acid2.gbc`) · cgb-acid-hell (`cgb-acid-hell.gbc`)
PPU accuracy tests that render a reference "acid2" face. `cgb-acid-hell` is a
harder CGB-only variant that stresses mid-scanline PPU register changes.
- **Author:** Matt Currie (mattcurrie)
- **Source:** https://github.com/mattcurrie/dmg-acid2 ·
  https://github.com/mattcurrie/cgb-acid2 ·
  https://github.com/mattcurrie/cgb-acid-hell
- **License:** **MIT**

### cpu_instrs (`cpu_instrs.gb`)
Blargg's CPU instruction test; prints "Passed" when all sub-tests pass.
- **Author:** Shay Green (blargg)
- **Source:** https://github.com/retrio/gb-test-roms
- **License:** freely distributable (no formal license; long-standing community redistribution)

---

Additional ROMs can be loaded at runtime via the **folder** button — nothing is
uploaded, files are read locally in the browser.
