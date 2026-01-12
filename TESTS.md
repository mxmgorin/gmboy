## Test Results

- ### SM83:
Passes all of 356 000 tests ✅

- ### Blargg

| CPU Instructions        | Memory Timing          | OAM Bug                 |
| ----------------------- | ---------------------- | ----------------------- |
| 01-special.gb ✅         | 01-read\_timing.gb ✅   | 1-lcd\_sync.gb ✅        |
| 02-interrupts.gb ✅      | 02-write\_timing.gb ✅  | 2-causes.gb ✅           |
| 03-op sp,hl.gb ✅        | 03-modify\_timing.gb ✅ | 3-non\_causes.gb ✅      |
| 04-op r,imm.gb ✅        |                        | 4-scanline\_timing.gb ✅ |
| 05-op rp.gb ✅           |                        | 5-timing\_bug.gb ✅      |
| 06-ld r,r.gb ✅          |                        | 6-timing\_no\_bug.gb ✅  |
| 07-jr,jp,call,ret,rst ✅ |                        | 7-timing\_effect.gb ✅   |
| 08-misc instrs.gb ✅     |                        | 8-instr\_effect.gb ✅    |
| 09-op r,r.gb ✅          |                        |                         |
| 10-bit ops.gb ✅         |                        |                         |
| 11-op a,(hl).gb ✅       |                        |                         |


- ### Mooneye

- acceptance

| General & OAM DMA            | Timing                       | Timer Accuracy                 |
|------------------------------|------------------------------|-------------------------------|
| oam_dma/oam_dma_timing.gb ✅  | call_cc_timing.gb ✅          | div_write.gb ✅             |
| bits/mem_oam.gb ✅            | call_cc_timing2.gb ✅         | rapid_toggle.gb ✅          |
| bits/reg_f.gb ✅              | call_timing.gb ✅             | tim00.gb ✅                 |
| instr/daa.gb ✅               | call_timing2.gb ✅            | tim00_div_trigger.gb ✅     |
| oam_dma/basic.gb ✅           | div_timing.gb ✅              | tim01.gb ✅                 |
| oam_dma/reg_read.gb ✅        | ei_timing.gb ✅               | tim01_div_trigger.gb ✅     |
| oam_dma/oam_dma_restart.gb ✅ | halt_ime0_ei.gb ✅            | tim10.gb ✅                 |
| oam_dma/oam_dma_start.gb ✅   | halt_ime0_nointr_timing.gb ✅ | tim10_div_trigger.gb ✅     |
| sources-GS ✅                 | halt_ime1_timing.gb ✅        | tim11.gb ✅                 |
| unused_hwio-GS.gb ✅          | halt_ime1_timing2-GS.gb ✅    | tim11_div_trigger.gb ✅     |
| ie_push.gb ✅                | jp_cc_timing.gb ✅            | tima_reload.gb ✅           |
|                              | jp_timing.gb ✅               | tima_write_reloading.gb ✅  |
|                              | ld_hl_sp_e_timing.gb ✅       | tma_write_reloading.gb ✅   |
|                              | pop_timing.gb ✅              |                               |
|                              | push_timing.gb ✅             |                               |
|                              | ret_cc_timing.gb ✅           |                               |
|                              | ret_timing.gb ✅              |                               |
|                              | reti_intr_timing.gb ✅        |                               |
|                              | reti_timing.gb ✅             |                               |
|                              | rst_timing.gb ✅              |                               |
|                              |  add_sp_e_timing.gb ✅        |                               |
|                              | di_timing-GS.gb ✅            |                               |
|                              | intr_timing ✅                |                               |

- emulator-only

| mbc1                         | mbc2              | mbc5               |
|------------------------------|-------------------|--------------------|
| bits_bank1.gb ✅              | bits_ramg.gb ✅    | rom_512kb.gb ✅     |
| bits_bank2.gb ✅              | bits_romb.gb ✅    | rom_1Mb.gb ✅       |
| bits_mode.gb ✅               | bits_unused.gb ✅  | rom_2Mb.gb ✅       |
| bits_ramg.gb ✅               | ram.gb ✅          | rom_4Mb.gb ✅       |
| multicart_rom_8Mb.gb ✅      | rom_1Mb.gb ✅      | rom_8Mb.gb ✅       |
| ram_64kb.gb ✅               | rom_2Mb.gb ✅      | rom_16Mb.gb ✅      |
| ram_256kb.gb ✅              | rom_512kb.gb ✅    | rom_32Mb.gb ✅      |
| rom_1Mb.gb ✅                |                   |                    |
| rom_2Mb.gb ✅                |                   |                    |
| rom_4Mb.gb ✅                |                   |                    |
| rom_8Mb.gb ✅                |                   |                    |
| rom_16Mb.gb ✅               |                   |                    |
| rom_512kb.gb ✅              |                   |                    |
