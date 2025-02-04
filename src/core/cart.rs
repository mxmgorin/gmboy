use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Cart {
    pub header: CartHeader,
    pub rom_data: Vec<u8>,
    pub checksum_valid: bool,
}

impl Display for Cart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = format!(
            "Cart {{ header: {:?}, checksum_valid: {:?} }}",
            self.header, self.checksum_valid
        );
        write!(f, "{}", str)
    }
}

impl Cart {
    pub fn new(bytes: Vec<u8>) -> Result<Cart, String> {
        let checksum = calc_checksum(&bytes)?;
        let header = CartHeader::new(&bytes)?;

        Ok(Self {
            checksum_valid: checksum == header.header_checksum,
            header,
            rom_data: bytes,
        })
    }

    pub fn read(&self, address: u16) -> u8 {
        // todo: impl ram
        self.rom_data[address as usize]
    }

    pub fn write(&mut self, _address: u16, _value: u8) {
        // self.bytes[address as usize] = value;
        unimplemented!("cart write")
    }
}

fn calc_checksum(bytes: &[u8]) -> Result<u8, String> {
    let end = 0x014C;

    if bytes.len() < end {
        return Err("Can't calc_checksum: bytes are shorter than 0x014E".into());
    }

    let start = 0x0134;
    let mut checksum: u8 = 0;

    for &byte in &bytes[start..=end] {
        checksum = checksum.wrapping_sub(byte).wrapping_sub(1);
    }

    Ok(checksum)
}

#[derive(Debug, Clone)]
pub struct CartHeader {
    /// 0x0100-0x0103: Execution start point
    pub entry_point: [u8; 4],
    /// 0x0104-0x0133: Nintendo logo
    pub nintendo_logo: [u8; 48],
    /// 0x0134-0x0143: Game title
    pub title: String,
    /// 0x013F-0x0142: Manufacturer code (if exists)
    pub manufacturer_code: Option<String>,
    /// 0x0143: Game Boy Color compatibility
    pub cgb_flag: CgbFlag,
    /// 0x0144-0x0145: New licensee code
    pub new_licensee_code: NewLicenseeCode,
    /// 0x0146: This byte specifies whether the game supports SGB functions.
    /// The SGB will ignore any command packets if this byte is set to a value other than $03 (typically $00).
    pub sgb_flag: u8,
    /// 0x0147: Type of cartridge
    pub cart_type: CartridgeType,
    /// 0x0148: ROM size
    pub rom_size: RomSize,
    /// 0x0149: RAM size
    pub ram_size: RamSize,
    /// 0x014A: Destination code (Japan or non-Japan)
    pub destination_code: DestinationCode,
    /// 0x014B: Old licensee code
    pub old_licensee_code: OldLicenseeCode,
    /// 0x014C: This byte specifies the version number of the game. It is usually $00.
    pub mask_rom_version: u8,
    /// 0x014D: Header checksum
    pub header_checksum: u8,
    /// 0x014E-0x014F: Global checksum
    pub global_checksum: u16,
}

impl CartHeader {
    pub fn new(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 0x50 {
            return Err("Insufficient data for cart header".into());
        }

        Ok(Self {
            entry_point: bytes[0x0100..0x0104].try_into().unwrap(),
            nintendo_logo: bytes[0x0104..0x0134].try_into().unwrap(),
            title: parse_title(bytes)?,
            manufacturer_code: if bytes[0x013F..0x0143] != [0x00, 0x00, 0x00, 0x00] {
                Some(String::from_utf8_lossy(&bytes[0x013F..0x0143]).to_string())
            } else {
                None
            },
            cgb_flag: bytes[0x0143].try_into().unwrap_or(CgbFlag::NonCGBMode),
            new_licensee_code: bytes[0x0144..0x0146].into(),
            sgb_flag: bytes[0x0146],
            cart_type: bytes[0x0147].try_into()?,
            rom_size: bytes[0x0148].try_into()?,
            ram_size: bytes[0x0149].try_into()?,
            destination_code: bytes[0x014A].try_into()?,
            old_licensee_code: bytes[0x014B].into(),
            mask_rom_version: bytes[0x014C],
            header_checksum: bytes[0x014D],
            global_checksum: u16::from_be_bytes(bytes[0x014E..0x0150].try_into().unwrap()),
        })
    }
}

fn parse_title(bytes: &[u8]) -> Result<String, String> {
    let title_bytes = &bytes[0x0134..0x0144];

    if cfg!(debug_assertions) {
        println!("Cart title bytes: {:?}", title_bytes);
    }

    let title = String::from_utf8_lossy(title_bytes).to_string();
    let trimmed_title = title.trim_end_matches('\0').to_string();

    Ok(trimmed_title)
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum RomSize {
    _32KiB = 0x00,
    _64KiB = 0x01,
    _128KiB = 0x02,
    _256KiB = 0x03,
    _512KiB = 0x04,
    _1MiB = 0x05,
    _2MiB = 0x06,
    _4MiB = 0x07,
    _8MiB = 0x08,
    _1_1MiB = 0x52,
    _1_2MiB = 0x53,
    _1_5MiB = 0x54,
}

impl TryFrom<u8> for RomSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(RomSize::_32KiB),
            0x01 => Ok(RomSize::_64KiB),
            0x02 => Ok(RomSize::_128KiB),
            0x03 => Ok(RomSize::_256KiB),
            0x04 => Ok(RomSize::_512KiB),
            0x05 => Ok(RomSize::_1MiB),
            0x06 => Ok(RomSize::_2MiB),
            0x07 => Ok(RomSize::_4MiB),
            0x08 => Ok(RomSize::_8MiB),
            0x52 => Ok(RomSize::_1_1MiB),
            0x53 => Ok(RomSize::_1_2MiB),
            0x54 => Ok(RomSize::_1_5MiB),
            _ => Err("Invalid ROM size code".into()),
        }
    }
}

impl RomSize {
    pub fn _number_of_banks(&self) -> usize {
        match self {
            RomSize::_32KiB => 2,
            RomSize::_64KiB => 4,
            RomSize::_128KiB => 8,
            RomSize::_256KiB => 16,
            RomSize::_512KiB => 32,
            RomSize::_1MiB => 64,
            RomSize::_2MiB => 128,
            RomSize::_4MiB => 256,
            RomSize::_8MiB => 512,
            RomSize::_1_1MiB => 72,
            RomSize::_1_2MiB => 80,
            RomSize::_1_5MiB => 96,
        }
    }

    pub fn _size_in_bytes(&self) -> usize {
        match self {
            RomSize::_32KiB => 32 * 1024,
            RomSize::_64KiB => 64 * 1024,
            RomSize::_128KiB => 128 * 1024,
            RomSize::_256KiB => 256 * 1024,
            RomSize::_512KiB => 512 * 1024,
            RomSize::_1MiB => 1024 * 1024,
            RomSize::_2MiB => 2 * 1024 * 1024,
            RomSize::_4MiB => 4 * 1024 * 1024,
            RomSize::_8MiB => 8 * 1024 * 1024,
            RomSize::_1_1MiB => 1_1 * 1024 * 1024,
            RomSize::_1_2MiB => 1_2 * 1024 * 1024,
            RomSize::_1_5MiB => 1_5 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum RamSize {
    NoRam = 0x00,
    Unused = 0x01,
    _8KiB = 0x02,
    _32KiB = 0x03,
    _128KiB = 0x04,
    _64KiB = 0x05,
}

impl TryFrom<u8> for RamSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(RamSize::NoRam),
            0x01 => Ok(RamSize::Unused),
            0x02 => Ok(RamSize::_8KiB),
            0x03 => Ok(RamSize::_32KiB),
            0x04 => Ok(RamSize::_128KiB),
            0x05 => Ok(RamSize::_64KiB),
            _ => Err("Invalid RAM size code".into()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum DestinationCode {
    Japan = 0x00,
    OverseasOnly = 0x01,
}

impl TryFrom<u8> for DestinationCode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(DestinationCode::Japan),
            0x01 => Ok(DestinationCode::OverseasOnly),
            _ => Err("Invalid DestinationCode".into()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CgbFlag {
    CGBMode,
    NonCGBMode,
}

impl TryFrom<u8> for CgbFlag {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if cfg!(debug_assertions) {
            println!("CGB Flag byte: {value}");
        }

        match value {
            0x80 => Ok(CgbFlag::CGBMode),
            0xC0 => Ok(CgbFlag::NonCGBMode),
            _ => Err("Invalid CGB flag".into()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
    Unknown, // For unrecognized or custom codes
}

impl From<&[u8]> for NewLicenseeCode {
    fn from(value: &[u8]) -> Self {
        if value.len() == 2 {
            if let Ok(code) = std::str::from_utf8(value) {
                return NewLicenseeCode::from_str(code);
            }
        }

        NewLicenseeCode::Unknown
    }
}

impl NewLicenseeCode {
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
            _ => NewLicenseeCode::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OldLicenseeCode {
    None,
    Nintendo,
    Capcom,
    HotB,
    Jaleco,
    CoconutsJapan,
    EliteSystems,
    EA,
    HudsonSoft,
    ITCEntertainment,
    Yanoman,
    JapanClary,
    VirginGamesLtd,
    PCMComplete,
    SanX,
    Kemco,
    SETACorporation,
    Infogrames,
    Bandai,
    UseNewLicenseeCode,
    Konami,
    HectorSoft,
    EntertainmentInteractive,
    Gremlin,
    UbiSoft,
    Atlus,
    MalibuInteractive,
    Angel,
    SpectrumHoloByte,
    Irem,
    Absolute,
    AcclaimEntertainment,
    Activision,
    SammyUSA,
    GameTek,
    ParkPlace,
    LJN,
    Matchbox,
    MiltonBradley,
    Mindscape,
    Romstar,
    NaxatSoft,
    Tradewest,
    TitusInteractive,
    OceanSoftware,
    ElectroBrain,
    InterplayEntertainment,
    Broderbund,
    SculpturedSoftware,
    TheSalesCurve,
    THQ,
    Accolade,
    TriffixEntertainment,
    MicroProse,
    MisawaEntertainment,
    LOZC,
    TokumaShoten,
    BulletProofSoftware,
    VicTokai,
    ApeInc,
    IMax,
    Chunsoft,
    VideoSystem,
    TsubarayaProductions,
    Varie,
    YonezawaSPal,
    Arc,
    NihonBussan,
    Tecmo,
    Imagineer,
    Nova,
    HoriElectric,
    Kawada,
    Takara,
    TechnosJapan,
    ToeiAnimation,
    Toho,
    Namco,
    ASCIIOrNexsoft,
    SquareEnix,
    HALLaboratory,
    SNK,
    PonyCanyon,
    CultureBrain,
    Sunsoft,
    SonyImagesoft,
    SammyCorporation,
    Taito,
    Square,
    TokumaShotenPublishing,
    DataEast,
    TonkinHouse,
    Koei,
    UFL,
    UltraGames,
    VAPInc,
    UseCorporation,
    Meldac,
    Athena,
    AsmikAceEntertainment,
    Natsume,
    KingRecords,
    EpicSonyRecords,
    IGS,
    AWave,
    ExtremeEntertainment,
    NipponComputerSystems,
    HumanEntertainment,
    Altron,
    JalecoEntertainment,
    TowaChiki,
    Yutaka,
    Epoch,
    Banpresto,
    SOFEL,
    Quest,
    SigmaEnterprises,
    ASKKodansha,
    CopyaSystem,
    Tomy,
    Unknown,
}

impl From<u8> for OldLicenseeCode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => OldLicenseeCode::None,
            0x01 => OldLicenseeCode::Nintendo,
            0x08 => OldLicenseeCode::Capcom,
            0x09 => OldLicenseeCode::HotB,
            0x0A => OldLicenseeCode::Jaleco,
            0x0B => OldLicenseeCode::CoconutsJapan,
            0x0C => OldLicenseeCode::EliteSystems,
            0x13 => OldLicenseeCode::EA,
            0x18 => OldLicenseeCode::HudsonSoft,
            0x19 => OldLicenseeCode::ITCEntertainment,
            0x1A => OldLicenseeCode::Yanoman,
            0x1D => OldLicenseeCode::JapanClary,
            0x1F => OldLicenseeCode::VirginGamesLtd,
            0x24 => OldLicenseeCode::PCMComplete,
            0x25 => OldLicenseeCode::SanX,
            0x28 => OldLicenseeCode::Kemco,
            0x29 => OldLicenseeCode::SETACorporation,
            0x30 => OldLicenseeCode::Infogrames,
            0x31 => OldLicenseeCode::Nintendo,
            0x32 => OldLicenseeCode::Bandai,
            0x33 => OldLicenseeCode::UseNewLicenseeCode,
            0x34 => OldLicenseeCode::Konami,
            0x35 => OldLicenseeCode::HectorSoft,
            0x38 => OldLicenseeCode::Capcom,
            0x39 => OldLicenseeCode::Banpresto,
            0x3C => OldLicenseeCode::EntertainmentInteractive,
            0x3E => OldLicenseeCode::Gremlin,
            0x41 => OldLicenseeCode::UbiSoft,
            0x42 => OldLicenseeCode::Atlus,
            0x44 => OldLicenseeCode::MalibuInteractive,
            0x46 => OldLicenseeCode::Angel,
            0x47 => OldLicenseeCode::SpectrumHoloByte,
            0x49 => OldLicenseeCode::Irem,
            0x4A => OldLicenseeCode::VirginGamesLtd,
            0x4D => OldLicenseeCode::MalibuInteractive,
            0x50 => OldLicenseeCode::Absolute,
            0x51 => OldLicenseeCode::AcclaimEntertainment,
            0x52 => OldLicenseeCode::Activision,
            0x53 => OldLicenseeCode::SammyUSA,
            0x54 => OldLicenseeCode::GameTek,
            0x55 => OldLicenseeCode::ParkPlace,
            0x56 => OldLicenseeCode::LJN,
            0x57 => OldLicenseeCode::Matchbox,
            0x59 => OldLicenseeCode::MiltonBradley,
            0x5A => OldLicenseeCode::Mindscape,
            0x5B => OldLicenseeCode::Romstar,
            0x5C => OldLicenseeCode::NaxatSoft,
            0x5D => OldLicenseeCode::Tradewest,
            0x60 => OldLicenseeCode::TitusInteractive,
            0x61 => OldLicenseeCode::VirginGamesLtd,
            0x67 => OldLicenseeCode::OceanSoftware,
            0x69 => OldLicenseeCode::EA,
            0x6E => OldLicenseeCode::EliteSystems,
            0x6F => OldLicenseeCode::ElectroBrain,
            0x70 => OldLicenseeCode::Infogrames,
            0x71 => OldLicenseeCode::InterplayEntertainment,
            0x72 => OldLicenseeCode::Broderbund,
            0x73 => OldLicenseeCode::SculpturedSoftware,
            0x75 => OldLicenseeCode::TheSalesCurve,
            0x78 => OldLicenseeCode::THQ,
            0x79 => OldLicenseeCode::Accolade,
            0x7A => OldLicenseeCode::TriffixEntertainment,
            0x7C => OldLicenseeCode::MicroProse,
            0x7F => OldLicenseeCode::Kemco,
            0x80 => OldLicenseeCode::MisawaEntertainment,
            0x83 => OldLicenseeCode::LOZC,
            0x86 => OldLicenseeCode::TokumaShoten,
            0x8B => OldLicenseeCode::BulletProofSoftware,
            0x8C => OldLicenseeCode::VicTokai,
            0x8E => OldLicenseeCode::ApeInc,
            0x8F => OldLicenseeCode::IMax,
            0x91 => OldLicenseeCode::Chunsoft,
            0x92 => OldLicenseeCode::VideoSystem,
            0x93 => OldLicenseeCode::TsubarayaProductions,
            0x95 => OldLicenseeCode::Varie,
            0x96 => OldLicenseeCode::YonezawaSPal,
            0x97 => OldLicenseeCode::Kemco,
            0x99 => OldLicenseeCode::Arc,
            0x9A => OldLicenseeCode::NihonBussan,
            0x9B => OldLicenseeCode::Tecmo,
            0x9C => OldLicenseeCode::Imagineer,
            0x9D => OldLicenseeCode::Banpresto,
            0x9F => OldLicenseeCode::Nova,
            0xA1 => OldLicenseeCode::HoriElectric,
            0xA2 => OldLicenseeCode::Bandai,
            0xA4 => OldLicenseeCode::Konami,
            0xA6 => OldLicenseeCode::Kawada,
            0xA7 => OldLicenseeCode::Takara,
            0xA9 => OldLicenseeCode::TechnosJapan,
            0xAA => OldLicenseeCode::Broderbund,
            0xAC => OldLicenseeCode::ToeiAnimation,
            0xAD => OldLicenseeCode::Toho,
            0xAF => OldLicenseeCode::Namco,
            0xB0 => OldLicenseeCode::AcclaimEntertainment,
            0xB1 => OldLicenseeCode::ASCIIOrNexsoft,
            0xB2 => OldLicenseeCode::Bandai,
            0xB4 => OldLicenseeCode::SquareEnix,
            0xB6 => OldLicenseeCode::HALLaboratory,
            0xB7 => OldLicenseeCode::SNK,
            0xB9 => OldLicenseeCode::PonyCanyon,
            0xBA => OldLicenseeCode::CultureBrain,
            0xBB => OldLicenseeCode::Sunsoft,
            0xBD => OldLicenseeCode::SonyImagesoft,
            0xBF => OldLicenseeCode::SammyCorporation,
            0xC0 => OldLicenseeCode::Taito,
            0xC2 => OldLicenseeCode::Kemco,
            0xC3 => OldLicenseeCode::Square,
            0xC4 => OldLicenseeCode::TokumaShotenPublishing,
            0xC5 => OldLicenseeCode::DataEast,
            0xC6 => OldLicenseeCode::TonkinHouse,
            0xC8 => OldLicenseeCode::Koei,
            0xC9 => OldLicenseeCode::UFL,
            0xCA => OldLicenseeCode::UltraGames,
            0xCB => OldLicenseeCode::VAPInc,
            0xCC => OldLicenseeCode::UseCorporation,
            0xCD => OldLicenseeCode::Meldac,
            0xCE => OldLicenseeCode::PonyCanyon,
            0xCF => OldLicenseeCode::Angel,
            0xD0 => OldLicenseeCode::Taito,
            0xD1 => OldLicenseeCode::SOFEL,
            0xD2 => OldLicenseeCode::Quest,
            0xD3 => OldLicenseeCode::SigmaEnterprises,
            0xD4 => OldLicenseeCode::ASKKodansha,
            0xD6 => OldLicenseeCode::NaxatSoft,
            0xD7 => OldLicenseeCode::CopyaSystem,
            0xD9 => OldLicenseeCode::Banpresto,
            0xDA => OldLicenseeCode::Tomy,
            0xDB => OldLicenseeCode::LJN,
            0xDD => OldLicenseeCode::NipponComputerSystems,
            0xDE => OldLicenseeCode::HumanEntertainment,
            0xDF => OldLicenseeCode::Altron,
            0xE0 => OldLicenseeCode::JalecoEntertainment,
            0xE1 => OldLicenseeCode::TowaChiki,
            0xE2 => OldLicenseeCode::Yutaka,
            0xE5 => OldLicenseeCode::Epoch,
            0xE7 => OldLicenseeCode::Athena,
            0xE8 => OldLicenseeCode::AsmikAceEntertainment,
            0xE9 => OldLicenseeCode::Natsume,
            0xEA => OldLicenseeCode::KingRecords,
            0xEB => OldLicenseeCode::Atlus,
            0xEC => OldLicenseeCode::EpicSonyRecords,
            0xEE => OldLicenseeCode::IGS,
            0xF0 => OldLicenseeCode::AWave,
            0xF3 => OldLicenseeCode::ExtremeEntertainment,
            0xFF => OldLicenseeCode::LJN,
            _ => OldLicenseeCode::Unknown,
        }
    }
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
