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

/// Drive the parse loop shared by every command: the common flags are consumed
/// here, anything else (command flags and positionals) goes to `on_arg`, which
/// pulls a flag's value from the iterator when it takes one. Returns `true`
/// when `-h`/`--help` was seen — the caller should print its usage and exit 0.
pub fn parse_args(
    args: &[String],
    common: &mut CommonOpts,
    mut on_arg: impl FnMut(&str, &mut Iter<String>) -> Result<(), String>,
) -> Result<bool, String> {
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--model" => common.model = parse_model(&next_val(&mut it, "--model")?)?,
            "--timeout" => common.timeout = parse_timeout(&next_val(&mut it, "--timeout")?)?,
            "--protocol" => common.protocol = parse_protocol(&next_val(&mut it, "--protocol")?)?,
            "-h" | "--help" => return Ok(true),
            other => on_arg(other, &mut it)?,
        }
    }

    Ok(false)
}

/// Print the option block shared by every command.
pub fn print_common_usage() {
    eprintln!("COMMON OPTIONS:");
    eprintln!("  --model <dmg|cgb|auto>   force hardware model (default: auto from header)");
    eprintln!("  --timeout <secs>         per-ROM timeout (default: {DEFAULT_TIMEOUT_SECS})");
    eprintln!("  --protocol <p>           auto|mooneye|blargg-serial|blargg-memory|gbmicrotest");
    eprintln!("                           (default: auto)\n");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dump_specs() {
        assert_eq!(parse_dump("C000"), Ok((0xC000, 16)));
        assert_eq!(parse_dump("0xC000:8"), Ok((0xC000, 8)));
        assert_eq!(parse_dump("$ff80:2"), Ok((0xFF80, 2)));
        assert!(parse_dump("C000:0").is_err());
        assert!(parse_dump("xyz").is_err());
    }

    #[test]
    fn vram_specs() {
        assert_eq!(parse_vram("1:9C00:32"), Ok((1, 0x9C00, 32)));
        assert_eq!(parse_vram("0:8000"), Ok((0, 0x8000, 16)));
        assert!(parse_vram("2:8000").is_err());
        assert!(parse_vram("9C00").is_err());
    }

    #[test]
    fn timeouts() {
        assert_eq!(parse_timeout("1.5"), Ok(Duration::from_secs_f64(1.5)));
        assert!(parse_timeout("0").is_err());
        assert!(parse_timeout("-3").is_err());
        assert!(parse_timeout("inf").is_err());
    }
}
