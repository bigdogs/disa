/// convert assembly to hex powered by keystone
use anyhow::{format_err, Context, Result};
use keystone::{Arch, Keystone, Mode, OptionType, OptionValue};

fn parse_arch_mode(value: &serde_json::Value) -> Result<(Arch, Mode)> {
    let arch = value["arch"].as_str().context("no arch")?;
    Ok(match arch {
        "armv7" => (Arch::ARM, Mode::ARM),
        "thumb" => (Arch::ARM, Mode::THUMB),
        "x86" => (Arch::X86, Mode::MODE_32),
        "x86_64" => (Arch::X86, Mode::MODE_64),
        "aarch64" => (Arch::ARM64, Mode::LITTLE_ENDIAN),
        v => return Err(format_err!("unknown arch: {}", v)),
    })
}

pub(crate) fn assembly_to_hex(msg: &str) -> Result<String> {
    let input = serde_json::from_str::<serde_json::Value>(msg).context("deserilize error")?;
    let data = input["data"].as_str().context("no data")?;

    // parse arch and mode
    let (arch, mode) = parse_arch_mode(&input)?;
    let engine = Keystone::new(arch, mode).map_err(|e| {
        format_err!(
            "Keystone init error, arch={:?}, mode={:?}, error={:?}",
            arch,
            mode,
            e
        )
    })?;

    if matches!(arch, Arch::X86) {
        engine
            .option(OptionType::SYNTAX, OptionValue::SYNTAX_INTEL)
            .map_err(|e| format_err!("Keystone set syntax error: {:?}", e))?;
    }

    let result = engine
        .asm(data.to_string(), 0)
        .map_err(|e| format_err!("assemble error: {:?}", e))?;

    if result.size == 0 {
        return Err(format_err!("empyt result"));
    }

    return Ok(result.to_string());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_assembler_thumb() {
        let code = super::assembly_to_hex(r#"{"arch": "thumb", "data": "nop"}"#).unwrap();
        println!("thumb: {}", code);
    }

    #[test]
    fn test_assembler_armv7() {
        let code = super::assembly_to_hex(r#"{"arch": "armv7", "data": "nop"}"#).unwrap();
        println!("armv7: {}", code);
    }

    #[test]
    fn test_assembler_aarch64() {
        let code = super::assembly_to_hex(r#"{"arch": "aarch64", "data": "nop"}"#).unwrap();
        println!("aarch64: {}", code);
    }

    #[test]
    fn test_assembler_x86() {
        let code = super::assembly_to_hex(r#"{"arch": "x86", "data": "nop"}"#).unwrap();
        println!("x86: {}", code);
    }

    #[test]
    fn test_assembler_x86_64() {
        let code = super::assembly_to_hex(r#"{"arch": "x86_64", "data": "nop"}"#).unwrap();
        println!("x86_64: {}", code);
    }
}
