use crate::errors::{Result, UnifyError};
use capstone::prelude::*;
use object::Architecture;

pub struct Disassembler {
    cs: Capstone,
}

impl Disassembler {
    pub fn new(arch: Architecture) -> Result<Self> {
        let cs = match arch {
            Architecture::X86_64 => Capstone::new()
                .x86()
                .mode(arch::x86::ArchMode::Mode64)
                .syntax(arch::x86::ArchSyntax::Intel)
                .build()
                .map_err(|e| UnifyError::DisasmError(e.to_string()))?,
            Architecture::I386 => Capstone::new()
                .x86()
                .mode(arch::x86::ArchMode::Mode32)
                .syntax(arch::x86::ArchSyntax::Intel)
                .build()
                .map_err(|e| UnifyError::DisasmError(e.to_string()))?,
            Architecture::Aarch64 => Capstone::new()
                .arm64()
                .mode(arch::arm64::ArchMode::Arm)
                .build()
                .map_err(|e| UnifyError::DisasmError(e.to_string()))?,
            Architecture::Arm => Capstone::new()
                .arm()
                .mode(arch::arm::ArchMode::Arm)
                .build()
                .map_err(|e| UnifyError::DisasmError(e.to_string()))?,
            _ => return Err(UnifyError::UnsupportedArch(format!("{:?}", arch))),
        };

        Ok(Self { cs })
    }

    pub fn disassemble(&self, code: &[u8], address: u64) -> Result<Vec<InstructionInfo>> {
        let insns = self
            .cs
            .disasm_all(code, address)
            .map_err(|e| UnifyError::DisasmError(e.to_string()))?;

        let result = insns
            .iter()
            .map(|i| InstructionInfo {
                address: i.address(),
                mnemonic: i.mnemonic().unwrap_or_default().to_string(),
                op_str: i.op_str().unwrap_or_default().to_string(),
                bytes: i.bytes().to_vec(),
            })
            .collect();

        Ok(result)
    }
}

pub struct InstructionInfo {
    pub address: u64,
    pub mnemonic: String,
    pub op_str: String,
    pub bytes: Vec<u8>,
}
