#[derive(Debug, Clone)]
pub struct Cart {
    header: CartHeader,
}

impl Cart {
    pub fn from_bytes(data: &[u8]) -> Result<Cart, String> {
        Ok(Self {
            header: CartHeader::from_bytes(data)?,
        })
    }
}

#[derive(Debug, Clone)]
struct CartHeader {
    pub entry_point: [u8; 4],    // 0x0100-0x0103: Execution start point
    pub nintendo_logo: [u8; 48], // 0x0104-0x0133: Nintendo logo
    pub title: String,           // 0x0134-0x0143: Game title
    pub manufacturer_code: Option<String>, // 0x013F-0x0142: Manufacturer code (if exists)
    pub cgb_flag: u8,            // 0x0143: Game Boy Color compatibility
    pub new_licensee_code: NewLicenseeCode,  // 0x0144-0x0145: New licensee code
    pub sgb_flag: u8,            // 0x0146: Super Game Boy compatibility
    pub cartridge_type: u8,      // 0x0147: Type of cartridge
    pub rom_size: u8,            // 0x0148: ROM size
    pub ram_size: u8,            // 0x0149: RAM size
    pub destination_code: u8,    // 0x014A: Destination code (Japan or non-Japan)
    pub old_licensee_code: u8,   // 0x014B: Old licensee code
    pub mask_rom_version: u8,    // 0x014C: Version number
    pub header_checksum: u8,     // 0x014D: Header checksum
    pub global_checksum: u16,    // 0x014E-0x014F: Global checksum
}

impl CartHeader {
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() < 0x50 {
            return Err("Insufficient data for cartridge header".into());
        }

        Ok(Self {
            entry_point: data[0x0100..0x0104].try_into().unwrap(),
            nintendo_logo: data[0x0104..0x0134].try_into().unwrap(),
            title: String::from_utf8_lossy(&data[0x0134..0x0144])
                .trim_end_matches('\0')
                .to_string(),
            manufacturer_code: if data[0x013F..0x0143] != [0x00, 0x00, 0x00, 0x00] {
                Some(String::from_utf8_lossy(&data[0x013F..0x0143]).to_string())
            } else {
                None
            },
            cgb_flag: data[0x0143],
            new_licensee_code: NewLicenseeCode::try_from_u16(u16::from_be_bytes(data[0x0144..0x0146].try_into().unwrap()))?,
            sgb_flag: data[0x0146],
            cartridge_type: data[0x0147],
            rom_size: data[0x0148],
            ram_size: data[0x0149],
            destination_code: data[0x014A],
            old_licensee_code: data[0x014B],
            mask_rom_version: data[0x014C],
            header_checksum: data[0x014D],
            global_checksum: u16::from_be_bytes(data[0x014E..0x0150].try_into().unwrap()),
        })
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum NewLicenseeCode {
    Nintendo = 0x00,
    Capcom = 0x01,
    Konami = 0x02,
    ElectronicArts = 0x03,
    // todo: add others
}

impl NewLicenseeCode {
    // Convert a u16 value to a NewLicenseeCode enum variant
    pub fn try_from_u16(code: u16) -> Result<Self, String> {
        match code {
            0x00 => Ok(NewLicenseeCode::Nintendo),
            0x01 => Ok(NewLicenseeCode::Capcom),
            0x02 => Ok(NewLicenseeCode::Konami),
            0x03 => Ok(NewLicenseeCode::ElectronicArts),
            _ => Err("Unknown licensee_code".into()),
        }
    }
}
