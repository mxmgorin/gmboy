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
    pub new_licensee_code: NewLicenseeCode, // 0x0144-0x0145: New licensee code
    pub sgb_flag: u8,            // 0x0146: Super Game Boy compatibility
    pub cartridge_type: CartridgeType, // 0x0147: Type of cartridge
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
            new_licensee_code: NewLicenseeCode::from_bytes(
                data[0x0144..0x0146].try_into().unwrap(),
            ),
            sgb_flag: data[0x0146],
            cartridge_type: data[0x0147].try_into()?,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NewLicenseeCode {
    None,
    NintendoResearchAndDevelopment1,
    Capcom,
    EA,
    HudsonSoft,
    BAI,
    KSS,
    PlanningOfficeWada,
    PCMComplete,
    SanX,
    Kemco,
    SetaCorporation,
    Viacom,
    Nintendo,
    Bandai,
    OceanSoftwareOrAcclaimEntertainment,
    Konami,
    HectorSoft,
    Taito,
    Banpresto,
    UbiSoft,
    Atlus,
    MalibuInteractive,
    Angel,
    BulletProofSoftware,
    Irem,
    Absolute,
    AcclaimEntertainment,
    Activision,
    SammyUsaCorporation,
    HiTechExpressions,
    Ljn,
    Matchbox,
    Mattel,
    MiltonBradleyCompany,
    TitusInteractive,
    VirginGamesLtd,
    LucasfilmGames,
    OceanSoftware,
    Infogrames,
    InterplayEntertainment,
    Broderbund,
    SculpturedSoftware,
    TheSalesCurveLimited,
    THQ,
    Accolade,
    MisawaEntertainment,
    Lozc,
    TokumaShoten,
    TsukudaOriginal,
    ChunsoftCo,
    VideoSystem,
    Varie,
    YonezawaSPal,
    Kaneko,
    PackInVideo,
    BottomUp,
    KonamiYuGiOh,
    MTO,
    Kodansha,
    Unknown(String), // For unrecognized or custom codes
}

impl NewLicenseeCode {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.len() == 2 {
            if let Ok(code) = std::str::from_utf8(bytes) {
                return NewLicenseeCode::from_str(code);
            }
        }

        NewLicenseeCode::Unknown(hex::encode(bytes)) // Encodes invalid bytes to hex for debugging
    }

    pub fn from_str(code: &str) -> Self {
        match code {
            "00" => NewLicenseeCode::None,
            "01" => NewLicenseeCode::NintendoResearchAndDevelopment1,
            "08" => NewLicenseeCode::Capcom,
            "13" => NewLicenseeCode::EA,
            "18" => NewLicenseeCode::HudsonSoft,
            "19" => NewLicenseeCode::BAI,
            "20" => NewLicenseeCode::KSS,
            "22" => NewLicenseeCode::PlanningOfficeWada,
            "24" => NewLicenseeCode::PCMComplete,
            "25" => NewLicenseeCode::SanX,
            "28" => NewLicenseeCode::Kemco,
            "29" => NewLicenseeCode::SetaCorporation,
            "30" => NewLicenseeCode::Viacom,
            "31" => NewLicenseeCode::Nintendo,
            "32" => NewLicenseeCode::Bandai,
            "33" => NewLicenseeCode::OceanSoftwareOrAcclaimEntertainment,
            "34" => NewLicenseeCode::Konami,
            "35" => NewLicenseeCode::HectorSoft,
            "37" => NewLicenseeCode::Taito,
            "38" => NewLicenseeCode::HudsonSoft,
            "39" => NewLicenseeCode::Banpresto,
            "41" => NewLicenseeCode::UbiSoft,
            "42" => NewLicenseeCode::Atlus,
            "44" => NewLicenseeCode::MalibuInteractive,
            "46" => NewLicenseeCode::Angel,
            "47" => NewLicenseeCode::BulletProofSoftware,
            "49" => NewLicenseeCode::Irem,
            "50" => NewLicenseeCode::Absolute,
            "51" => NewLicenseeCode::AcclaimEntertainment,
            "52" => NewLicenseeCode::Activision,
            "53" => NewLicenseeCode::SammyUsaCorporation,
            "54" => NewLicenseeCode::Konami,
            "55" => NewLicenseeCode::HiTechExpressions,
            "56" => NewLicenseeCode::Ljn,
            "57" => NewLicenseeCode::Matchbox,
            "58" => NewLicenseeCode::Mattel,
            "59" => NewLicenseeCode::MiltonBradleyCompany,
            "60" => NewLicenseeCode::TitusInteractive,
            "61" => NewLicenseeCode::VirginGamesLtd,
            "64" => NewLicenseeCode::LucasfilmGames,
            "67" => NewLicenseeCode::OceanSoftware,
            "69" => NewLicenseeCode::EA,
            "70" => NewLicenseeCode::Infogrames,
            "71" => NewLicenseeCode::InterplayEntertainment,
            "72" => NewLicenseeCode::Broderbund,
            "73" => NewLicenseeCode::SculpturedSoftware,
            "75" => NewLicenseeCode::TheSalesCurveLimited,
            "78" => NewLicenseeCode::THQ,
            "79" => NewLicenseeCode::Accolade,
            "80" => NewLicenseeCode::MisawaEntertainment,
            "83" => NewLicenseeCode::Lozc,
            "86" => NewLicenseeCode::TokumaShoten,
            "87" => NewLicenseeCode::TsukudaOriginal,
            "91" => NewLicenseeCode::ChunsoftCo,
            "92" => NewLicenseeCode::VideoSystem,
            "93" => NewLicenseeCode::OceanSoftwareOrAcclaimEntertainment,
            "95" => NewLicenseeCode::Varie,
            "96" => NewLicenseeCode::YonezawaSPal,
            "97" => NewLicenseeCode::Kaneko,
            "99" => NewLicenseeCode::PackInVideo,
            "9H" => NewLicenseeCode::BottomUp,
            "A4" => NewLicenseeCode::KonamiYuGiOh,
            "BL" => NewLicenseeCode::MTO,
            "DK" => NewLicenseeCode::Kodansha,
            _ => NewLicenseeCode::Unknown(code.to_string()),
        }
    }
}

impl NewLicenseeCode {
    // Convert a u16 value to a NewLicenseeCode enum variant
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum CartridgeType {
    RomOnly = 0x00,
    Mbc1 = 0x01,
    Mbc1Ram = 0x02,
    Mbc1RamBattery = 0x03,
    Mbc2 = 0x05,
    Mbc2Battery = 0x06,
    RomRam = 0x08,
    RomRamBattery = 0x09,
    Mmm01 = 0x0B,
    Mmm01Ram = 0x0C,
    Mmm01RamBattery = 0x0D,
    Mbc3TimerBattery = 0x0F,
    Mbc3TimerRamBattery = 0x10,
    Mbc3 = 0x11,
    Mbc3Ram = 0x12,
    Mbc3RamBattery = 0x13,
    Mbc5 = 0x19,
    Mbc5Ram = 0x1A,
    Mbc5RamBattery = 0x1B,
    Mbc5Rumble = 0x1C,
    Mbc5RumbleRam = 0x1D,
    Mbc5RumbleRamBattery = 0x1E,
    PocketCamera = 0xFC,
    BandaiTama5 = 0xFD,
    HuC3 = 0xFE,
    HuC1RamBattery = 0xFF,
}

impl TryFrom<u8> for CartridgeType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(CartridgeType::RomOnly),
            0x01 => Ok(CartridgeType::Mbc1),
            0x02 => Ok(CartridgeType::Mbc1Ram),
            0x03 => Ok(CartridgeType::Mbc1RamBattery),
            0x05 => Ok(CartridgeType::Mbc2),
            0x06 => Ok(CartridgeType::Mbc2Battery),
            0x08 => Ok(CartridgeType::RomRam),
            0x09 => Ok(CartridgeType::RomRamBattery),
            0x0B => Ok(CartridgeType::Mmm01),
            0x0C => Ok(CartridgeType::Mmm01Ram),
            0x0D => Ok(CartridgeType::Mmm01RamBattery),
            0x0F => Ok(CartridgeType::Mbc3TimerBattery),
            0x10 => Ok(CartridgeType::Mbc3TimerRamBattery),
            0x11 => Ok(CartridgeType::Mbc3),
            0x12 => Ok(CartridgeType::Mbc3Ram),
            0x13 => Ok(CartridgeType::Mbc3RamBattery),
            0x19 => Ok(CartridgeType::Mbc5),
            0x1A => Ok(CartridgeType::Mbc5Ram),
            0x1B => Ok(CartridgeType::Mbc5RamBattery),
            0x1C => Ok(CartridgeType::Mbc5Rumble),
            0x1D => Ok(CartridgeType::Mbc5RumbleRam),
            0x1E => Ok(CartridgeType::Mbc5RumbleRamBattery),
            0xFC => Ok(CartridgeType::PocketCamera),
            0xFD => Ok(CartridgeType::BandaiTama5),
            0xFE => Ok(CartridgeType::HuC3),
            0xFF => Ok(CartridgeType::HuC1RamBattery),
            _ => Err(format!("Unknown CartridgeType: {value}")),
        }
    }
}
