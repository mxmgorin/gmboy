// Object attributes reside in the object attribute memory (OAM) at $FE00-FE9F.
// Has 40 movable objects.

pub const OAM_ITEMS_COUNT: usize = 40;
pub const OAM_ADDR_START: usize = 0xFE00;

#[derive(Debug, Clone)]
pub struct OamRam {
    pub items: [OamItem; OAM_ITEMS_COUNT],
}

impl Default for OamRam {
    fn default() -> Self {
        Self {
            items: [OamItem::default(); OAM_ITEMS_COUNT],
        }
    }
}

impl OamRam {
    pub fn read(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        let addr = if addr >= OAM_ADDR_START {
            addr - OAM_ADDR_START
        } else {
            addr
        };

        let (item_index, byte_offset) = self.get_index_and_offset(addr as u16);

        match byte_offset {
            0 => self.items[item_index].y,
            1 => self.items[item_index].x,
            2 => self.items[item_index].tile_index,
            3 => self.items[item_index].flags,
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        let addr = addr as usize;
        let addr = if addr >= OAM_ADDR_START {
            addr - OAM_ADDR_START
        } else {
            addr
        };

        let (item_index, byte_offset) = self.get_index_and_offset(addr as u16);

        match byte_offset {
            0 => self.items[item_index].y = value,
            1 => self.items[item_index].x = value,
            2 => self.items[item_index].tile_index = value,
            3 => self.items[item_index].flags = value,
            _ => unreachable!(),
        };
    }

    /// Determine the index in the oam_ram array and the specific byte to update
    fn get_index_and_offset(&self, addr: u16) -> (usize, usize) {
        let item_index = (addr / 4) as usize; // Each `OamItem` is 4 bytes
        let byte_offset = (addr % 4) as usize;

        (item_index, byte_offset)
    }
}

//  Bit7   BG and Window over OBJ (0=No, 1=BG and Window colors 1-3 over the OBJ)
//  Bit6   Y flip          (0=Normal, 1=Vertically mirrored)
//  Bit5   X flip          (0=Normal, 1=Horizontally mirrored)
//  Bit4   Palette number  **Non CGB Mode Only** (0=OBP0, 1=OBP1)
//  Bit3   Tile VRAM-Bank  **CGB Mode Only**     (0=Bank 0, 1=Bank 1)
//  Bit2-0 Palette number  **CGB Mode Only**     (OBP0-7)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct OamItem {
    pub y: u8,
    pub x: u8,
    pub tile_index: u8,
    pub flags: u8,
}

impl OamItem {
    pub fn f_cgb_pn(&self) -> u8 {
        self.flags & 0b0000_0111 // Extract bits 0-2
    }

    pub fn f_cgb_vram_bank(&self) -> bool {
        (self.flags & 0b0000_1000) != 0 // Bit 3
    }

    pub fn f_pn(&self) -> bool {
        (self.flags & 0b0001_0000) != 0 // Bit 4
    }

    pub fn f_x_flip(&self) -> bool {
        (self.flags & 0b0010_0000) != 0 // Bit 5
    }

    pub fn f_y_flip(&self) -> bool {
        (self.flags & 0b0100_0000) != 0 // Bit 6
    }

    pub fn f_bgp(&self) -> bool {
        (self.flags & 0b1000_0000) != 0 // Bit 7
    }
}
