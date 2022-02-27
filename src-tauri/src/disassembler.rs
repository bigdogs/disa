/// convert hex to assembly powered by capstone
use anyhow::{format_err, Context, Result};
use capstone::{prelude::*, Capstone};

fn x86_capstone(mode_64: bool) -> Result<Capstone> {
    let mode = if mode_64 {
        arch::x86::ArchMode::Mode64
    } else {
        arch::x86::ArchMode::Mode32
    };
    Capstone::new()
        .x86()
        .mode(mode)
        .syntax(arch::x86::ArchSyntax::Intel)
        .detail(true)
        .build()
        .map_err(|e| format_err!("Capstone create error :{}", e))
}

fn arm_capstone(thumb: bool) -> Result<Capstone> {
    let mode = if thumb {
        arch::arm::ArchMode::Thumb
    } else {
        arch::arm::ArchMode::Arm
    };
    Capstone::new()
        .arm()
        .mode(mode)
        .detail(true)
        .build()
        .map_err(|e| format_err!("Capstone create error :{}", e))
}

fn arm64_capstone() -> Result<Capstone> {
    Capstone::new()
        .arm64()
        .mode(arch::arm64::ArchMode::Arm)
        .detail(true)
        .build()
        .map_err(|e| format_err!("Capstone create error :{}", e))
}

pub(crate) fn hex_to_assembly(msg: &str) -> Result<String> {
    let value = serde_json::from_str::<serde_json::Value>(msg).context("deserilize error")?;
    let arch = value["arch"].as_str().context("no arch")?;

    let data = value["data"].as_str().context("no data")?;
    let data: String = data.chars().filter(|c| !c.is_whitespace()).collect();
    let data = hex::decode(data).context("data hex decode error")?;

    let cs = match arch {
        "armv7" => arm_capstone(false),
        "thumb" => arm_capstone(true),
        "x86" => x86_capstone(false),
        "x86_64" => x86_capstone(true),
        "aarch64" => arm64_capstone(),
        v => return Err(format_err!("unknown arch: {}", v)),
    }?;
    let insns = cs
        .disasm_all(&data, 0)
        .map_err(|e| format_err!("disassembly error: {}", e))?;

    if insns.len() == 0 {
        return Err(format_err!("empyt instruction"));
    }

    Ok(insns
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join("\n"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_disassembler_thumb() {
        let code = super::hex_to_assembly(r#"{"arch": "thumb", "data": "00BF"}"#).unwrap();
        println!("thumb: {}", code);
    }

    #[test]
    fn test_disassembler_armv7() {
        let code = super::hex_to_assembly(r#"{"arch": "armv7", "data": "00F020E3"}"#).unwrap();
        println!("armv7: {}", code);
    }

    #[test]
    fn test_disassembler_aarch64() {
        let code =
            super::hex_to_assembly(r#"{"arch": "aarch64", "data": "1F2003D5 1F2003D5"}"#).unwrap();
        println!("aarch64: {}", code);
    }

    #[test]
    fn test_disassembler_x86() {
        let code = super::hex_to_assembly(r#"{"arch": "x86", "data": "90"}"#).unwrap();
        println!("x86: {}", code);
    }

    #[test]
    fn test_disassembler_x86_64() {
        let code = super::hex_to_assembly(r#"{"arch": "x86_64", "data": "90"}"#).unwrap();
        println!("x86_64: {}", code);
    }
}
