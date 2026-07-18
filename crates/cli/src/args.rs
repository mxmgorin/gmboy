//! Shared command-line option parsing (a small hand-rolled parser — no clap, to
//! keep the binary thin and fast to build).

use core::emu::config::GbModel;
use core::harness::TestProtocol;
use std::slice::Iter;
use std::time::Duration;

pub const DEFAULT_TIMEOUT_SECS: u64 = 20;

/// Options accepted by every command.
pub struct CommonOpts {
    pub model: Option<GbModel>,
    pub timeout: Duration,
    pub protocol: TestProtocol,
}

impl Default for CommonOpts {
    fn default() -> Self {
        Self {
            model: None,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            protocol: TestProtocol::Auto,
        }
    }
}

/// How one argument matched against the shared options.
pub enum ArgMatch {
    /// Consumed as a common flag — continue the parse loop.
    Common,
    /// `-h`/`--help` seen — the caller should print usage and exit 0.
    Help,
    /// Not a common flag — the caller handles it (command flag or positional).
    Other,
}

impl CommonOpts {
    /// Match `arg` against the shared flags, pulling its value from `it` when the
    /// flag takes one. Lets each command reuse the common parsing and only spell
    /// out its own flags and positionals.
    pub fn match_common(&mut self, arg: &str, it: &mut Iter<String>) -> Result<ArgMatch, String> {
        match arg {
            "--model" => self.model = parse_model(&next_val(it, "--model")?)?,
            "--timeout" => self.timeout = parse_timeout(&next_val(it, "--timeout")?)?,
            "--protocol" => self.protocol = parse_protocol(&next_val(it, "--protocol")?)?,
            "-h" | "--help" => return Ok(ArgMatch::Help),
            _ => return Ok(ArgMatch::Other),
        }

        Ok(ArgMatch::Common)
    }
}

/// Pull the value that follows a value-taking flag, erroring if it is missing.
pub fn next_val(it: &mut Iter<String>, flag: &str) -> Result<String, String> {
    it.next()
        .cloned()
        .ok_or_else(|| format!("{flag} requires a value"))
}

fn parse_model(s: &str) -> Result<Option<GbModel>, String> {
    match s.to_lowercase().as_str() {
        "dmg" => Ok(Some(GbModel::Dmg)),
        "cgb" | "gbc" => Ok(Some(GbModel::Cgb)),
        "auto" => Ok(None),
        other => Err(format!("unknown model '{other}' (use dmg|cgb|auto)")),
    }
}

fn parse_protocol(s: &str) -> Result<TestProtocol, String> {
    match s.to_lowercase().as_str() {
        "auto" => Ok(TestProtocol::Auto),
        "mooneye" => Ok(TestProtocol::Mooneye),
        "blargg-serial" => Ok(TestProtocol::BlarggSerial),
        "blargg-memory" => Ok(TestProtocol::BlarggMemory),
        "gbmicrotest" => Ok(TestProtocol::GbMicrotest),
        other => Err(format!(
            "unknown protocol '{other}' \
             (use auto|mooneye|blargg-serial|blargg-memory|gbmicrotest)"
        )),
    }
}

fn parse_timeout(s: &str) -> Result<Duration, String> {
    let secs: f64 = s
        .parse()
        .map_err(|_| format!("invalid timeout '{s}' (seconds)"))?;
    if secs <= 0.0 || !secs.is_finite() {
        return Err(format!("timeout must be positive: '{s}'"));
    }

    Ok(Duration::from_secs_f64(secs))
}

/// Parse a `--dump` argument `ADDR[:LEN]`: ADDR is hex (optional `0x`/`$`
/// prefix), LEN is decimal (default 16). Returns `(addr, len)`.
pub fn parse_dump(s: &str) -> Result<(u16, u16), String> {
    let (addr_s, len_s) = match s.split_once(':') {
        Some((a, l)) => (a, Some(l)),
        None => (s, None),
    };

    let addr = parse_u16_hex(addr_s).ok_or_else(|| format!("invalid dump address '{addr_s}'"))?;
    let len = match len_s {
        Some(l) => l
            .parse::<u16>()
            .map_err(|_| format!("invalid dump length '{l}'"))?,
        None => 16,
    };

    if len == 0 {
        return Err("dump length must be > 0".to_string());
    }

    Ok((addr, len))
}

/// Parse a `--vram` argument `BANK:ADDR[:LEN]`: BANK is 0/1, ADDR is hex
/// (optional `0x`/`$` prefix), LEN is decimal (default 16). Returns
/// `(bank, addr, len)`.
pub fn parse_vram(s: &str) -> Result<(u8, u16, u16), String> {
    let (bank_s, rest) = s
        .split_once(':')
        .ok_or_else(|| format!("invalid vram spec '{s}' (use BANK:ADDR[:LEN])"))?;

    let bank = match bank_s {
        "0" => 0,
        "1" => 1,
        other => return Err(format!("invalid vram bank '{other}' (use 0 or 1)")),
    };

    let (addr, len) = parse_dump(rest)?;

    Ok((bank, addr, len))
}

fn parse_u16_hex(s: &str) -> Option<u16> {
    let s = s
        .trim_start_matches("0x")
        .trim_start_matches("0X")
        .trim_start_matches('$');

    u16::from_str_radix(s, 16).ok()
}
