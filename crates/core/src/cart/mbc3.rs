use crate::cart::header::{RamSize, RomSize};
use crate::cart::mbc::{Mbc, MbcData};
use crate::cart::mbc1::BankingMode;
use crate::cart::CartData;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use web_time::SystemTime;

/// Size in bytes of the RTC block appended to the battery save.
///
/// Layout matches the widely used BGB / VBA-M `.sav` RTC footer: ten
/// little-endian `u32` register values (live S/M/H/DL/DH, then the latched
/// copies) followed by a little-endian `u64` Unix timestamp (seconds) marking
/// when the clock was last updated.
const RTC_SAVE_LEN: usize = 10 * 4 + 8; // 48

#[inline]
fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mbc3 {
    data: MbcData,
    rtc: Rtc,
    /// Whether the cartridge actually has the RTC (timer) hardware. Only timer
    /// carts persist the RTC block, so non-timer saves stay byte-for-byte
    /// compatible with plain RAM `.sav` files.
    has_timer: bool,
}

/// MBC3 real-time clock.
///
/// The clock is modelled as a set of counters plus the wall-clock time at which
/// they were last advanced. On each latch (or register write) we add the real
/// seconds elapsed since that point, propagating carries through
/// seconds → minutes → hours → days. This makes writes (setting the clock) and
/// the halt flag behave correctly, and — because the clock keeps ticking off
/// real time even across sessions — matches how the physical, battery-backed
/// RTC behaves while the console is powered off.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct Rtc {
    selected_register: u8,
    latch_state_register: u8,
    /// The running clock. Register writes land here.
    live: RtcRegisters,
    /// The snapshot exposed to reads; refreshed on the latch sequence.
    latched: RtcRegisters,
    /// Unix time (seconds) at which `live` was last advanced.
    last_unix_secs: u64,
}

impl Rtc {
    fn new() -> Self {
        Self {
            last_unix_secs: now_unix_secs(),
            ..Default::default()
        }
    }

    #[inline(always)]
    fn is_selected(&self) -> bool {
        (0x08..=0x0C).contains(&self.selected_register)
    }

    #[inline(always)]
    fn reset_selected(&mut self) {
        self.selected_register = 0;
    }

    /// Advance the live clock by the real time elapsed since the last update.
    fn sync(&mut self) {
        let now = now_unix_secs();
        let elapsed = now.saturating_sub(self.last_unix_secs);
        self.last_unix_secs = now;

        if self.live.halted || elapsed == 0 {
            return;
        }

        self.live.advance(elapsed);
    }

    fn latch(&mut self) {
        self.sync();
        self.latched = self.live.clone();
    }

    fn write(&mut self, value: u8) {
        // Bring the clock current before the game overwrites a register, so no
        // elapsed time is silently dropped.
        self.sync();
        self.live.write(self.selected_register, value);
    }

    fn to_save_bytes(&self) -> [u8; RTC_SAVE_LEN] {
        let mut buf = [0u8; RTC_SAVE_LEN];
        let regs = [
            self.live.seconds as u32,
            self.live.minutes as u32,
            self.live.hours as u32,
            (self.live.days & 0xFF) as u32,
            self.live.control() as u32,
            self.latched.seconds as u32,
            self.latched.minutes as u32,
            self.latched.hours as u32,
            (self.latched.days & 0xFF) as u32,
            self.latched.control() as u32,
        ];
        for (i, reg) in regs.iter().enumerate() {
            buf[i * 4..i * 4 + 4].copy_from_slice(&reg.to_le_bytes());
        }
        buf[40..48].copy_from_slice(&self.last_unix_secs.to_le_bytes());
        buf
    }

    fn load_save_bytes(&mut self, buf: &[u8]) {
        let reg = |i: usize| -> u8 {
            u32::from_le_bytes([buf[i * 4], buf[i * 4 + 1], buf[i * 4 + 2], buf[i * 4 + 3]]) as u8
        };
        self.live = RtcRegisters::from_save(reg(0), reg(1), reg(2), reg(3), reg(4));
        self.latched = RtcRegisters::from_save(reg(5), reg(6), reg(7), reg(8), reg(9));

        let ts = u64::from_le_bytes(buf[40..48].try_into().unwrap());
        // A zero (or future) timestamp means we can't trust the elapsed delta,
        // so anchor the clock to "now" instead of jumping decades.
        self.last_unix_secs = if ts == 0 || ts > now_unix_secs() {
            now_unix_secs()
        } else {
            ts
        };
    }
}

// TODO:
// When accessing the RTC Registers, it is recommended to wait 4 µs (4 M-cycles in Single Speed Mode) between any separate accesses.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct RtcRegisters {
    seconds: u8, // 0..=59
    minutes: u8, // 0..=59
    hours: u8,   // 0..=23
    days: u16,   // 0..=511 (9-bit day counter)
    halted: bool,
    /// Day-counter overflow (bit 7 of the control register). Sticky until the
    /// game clears it.
    carry: bool,
}

impl RtcRegisters {
    fn from_save(seconds: u8, minutes: u8, hours: u8, days_low: u8, control: u8) -> Self {
        let mut regs = Self {
            seconds: seconds % 60,
            minutes: minutes % 60,
            hours: hours % 24,
            days: days_low as u16,
            halted: false,
            carry: false,
        };
        regs.write_control(control);
        regs
    }

    /// Add `elapsed_secs` to the counters, propagating carries.
    fn advance(&mut self, elapsed_secs: u64) {
        let total_secs = self.seconds as u64 + elapsed_secs;
        self.seconds = (total_secs % 60) as u8;

        let total_mins = self.minutes as u64 + total_secs / 60;
        self.minutes = (total_mins % 60) as u8;

        let total_hours = self.hours as u64 + total_mins / 60;
        self.hours = (total_hours % 24) as u8;

        let total_days = self.days as u64 + total_hours / 24;
        self.days = (total_days % 512) as u16;

        if total_days > 511 {
            self.carry = true; // sticky until cleared by the game
        }
    }

    /// The control register (0x0C): bit 0 = day high, bit 6 = halt, bit 7 = carry.
    #[inline(always)]
    fn control(&self) -> u8 {
        let mut value = ((self.days >> 8) & 0x01) as u8;
        if self.halted {
            value |= 0x40;
        }
        if self.carry {
            value |= 0x80;
        }
        value
    }

    #[inline(always)]
    fn write_control(&mut self, value: u8) {
        self.days = (self.days & 0x00FF) | (((value & 0x01) as u16) << 8);
        self.halted = value & 0x40 != 0;
        self.carry = value & 0x80 != 0;
    }

    #[inline(always)]
    fn read(&self, register: u8) -> u8 {
        match register {
            0x08 => self.seconds,
            0x09 => self.minutes,
            0x0A => self.hours,
            0x0B => (self.days & 0xFF) as u8,
            0x0C => self.control(),
            _ => 0xFF,
        }
    }

    #[inline(always)]
    fn write(&mut self, register: u8, value: u8) {
        match register {
            0x08 => self.seconds = value % 60,
            0x09 => self.minutes = value % 60,
            0x0A => self.hours = value % 24,
            0x0B => self.days = (self.days & 0x100) | value as u16,
            0x0C => self.write_control(value),
            _ => {}
        }
    }
}

impl Mbc3 {
    pub fn new(ram_size: RamSize, rom_size: RomSize, has_timer: bool) -> Self {
        Self {
            data: MbcData::new(vec![0; ram_size.bytes_size()].into_boxed_slice(), rom_size),
            rtc: Rtc::new(),
            has_timer,
        }
    }
}

impl Mbc for Mbc3 {
    #[inline]
    fn read_rom(&self, cart_data: &CartData, address: u16) -> u8 {
        self.data.read_rom(cart_data, address)
    }

    #[inline]
    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.data.write_ram_enabled(value),
            0x2000..=0x3FFF => {
                let bank_number = if value == 0 { 1 } else { value };
                self.data.rom_bank_number = bank_number as u16;
                self.data.clamp_rom_bank_number();
            }
            0x4000..=0x5FFF => {
                match value {
                    0x00..=0x03 => {
                        self.data.ram_bank_number = value & 0x03;
                        self.rtc.reset_selected();
                    }
                    0x08..=0x0C => self.rtc.selected_register = value,
                    _ => {}
                };
            }
            0x6000..=0x7FFF => {
                // Latch sequence: 0 -> 1 triggers latch
                if self.rtc.latch_state_register == 0 && value == 1 {
                    self.rtc.latch();
                }

                self.rtc.latch_state_register = value;
            }
            _ => {}
        }
    }

    #[inline]
    fn read_ram(&self, address: u16) -> u8 {
        if self.rtc.is_selected() {
            return self.rtc.latched.read(self.rtc.selected_register);
        }

        self.data.read_ram(address, BankingMode::RamBanking)
    }

    #[inline]
    fn write_ram(&mut self, address: u16, value: u8) {
        if self.rtc.is_selected() {
            self.rtc.write(value);
            return;
        }

        self.data.write_ram(address, value, BankingMode::RamBanking);
    }

    fn load_ram(&mut self, bytes: Box<[u8]>) {
        if !self.has_timer {
            self.data.load_ram(bytes);
            return;
        }

        let ram_len = self.data.ram_len();
        if bytes.len() >= ram_len + RTC_SAVE_LEN {
            let (ram, footer) = bytes.split_at(ram_len);
            self.data.load_ram(ram.to_vec().into_boxed_slice());
            self.rtc.load_save_bytes(&footer[..RTC_SAVE_LEN]);
        } else {
            // Older RAM-only save with no RTC footer: keep the RAM, leave the
            // clock anchored to the current time.
            self.data.load_ram(bytes);
        }
    }

    fn dump_ram(&self) -> Option<Box<[u8]>> {
        if !self.has_timer {
            return self.data.dump_ram();
        }

        let mut out = self
            .data
            .dump_ram()
            .map(|ram| ram.into_vec())
            .unwrap_or_default();
        out.extend_from_slice(&self.rtc.to_save_bytes());
        Some(out.into_boxed_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn regs(seconds: u8, minutes: u8, hours: u8, days: u16) -> RtcRegisters {
        RtcRegisters {
            seconds,
            minutes,
            hours,
            days,
            halted: false,
            carry: false,
        }
    }

    #[test]
    fn advance_propagates_carries() {
        let mut r = regs(59, 59, 23, 0);
        r.advance(1);
        assert_eq!((r.seconds, r.minutes, r.hours, r.days), (0, 0, 0, 1));
    }

    #[test]
    fn advance_full_day_in_seconds() {
        let mut r = regs(0, 0, 0, 5);
        r.advance(86_400); // exactly one day
        assert_eq!((r.seconds, r.minutes, r.hours, r.days), (0, 0, 0, 6));
    }

    #[test]
    fn day_counter_overflow_sets_carry() {
        let mut r = regs(0, 0, 0, 511);
        r.advance(86_400); // rolls 511 -> 512, wraps to 0 with carry
        assert_eq!(r.days, 0);
        assert!(r.carry);
    }

    #[test]
    fn control_register_round_trip() {
        let mut r = regs(0, 0, 0, 256); // day bit 8 set
        r.halted = true;
        r.carry = true;
        let c = r.control();
        assert_eq!(c & 0x01, 0x01); // day high
        assert_eq!(c & 0x40, 0x40); // halt
        assert_eq!(c & 0x80, 0x80); // carry

        let mut r2 = regs(0, 0, 0, 0);
        r2.write_control(c);
        assert_eq!(r2.days & 0x100, 0x100);
        assert!(r2.halted);
        assert!(r2.carry);
    }

    #[test]
    fn halted_clock_does_not_advance() {
        let mut rtc = Rtc::new();
        rtc.live.halted = true;
        rtc.last_unix_secs = rtc.last_unix_secs.saturating_sub(120);
        rtc.sync();
        assert_eq!(rtc.live.seconds, 0);
        assert_eq!(rtc.live.minutes, 0);
    }

    #[test]
    fn save_round_trip_preserves_registers() {
        let mut rtc = Rtc::new();
        rtc.live = regs(12, 34, 5, 300);
        rtc.live.carry = true;
        rtc.latched = regs(7, 8, 9, 100);
        rtc.last_unix_secs = 1_600_000_000;

        let bytes = rtc.to_save_bytes();
        let mut restored = Rtc::new();
        restored.load_save_bytes(&bytes);

        assert_eq!(restored.live.seconds, 12);
        assert_eq!(restored.live.minutes, 34);
        assert_eq!(restored.live.hours, 5);
        assert_eq!(restored.live.days, 300);
        assert!(restored.live.carry);
        assert_eq!(restored.latched.days, 100);
        assert_eq!(restored.last_unix_secs, 1_600_000_000);
    }
}
