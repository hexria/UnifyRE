use crate::core::traits::BinaryProvider;
use crate::errors::Result;
use object::{Object, ObjectSection, ObjectSymbol};
use serde::Serialize;

#[derive(Serialize)]
pub struct AnalysisResult {
    pub format: String,
    pub architecture: String,
    pub entry_point: u64,
    pub sections: Vec<SectionInfo>,
    pub symbols: Vec<SymbolInfo>,
    pub findings: Vec<String>,
}

#[derive(Serialize)]
pub struct SectionInfo {
    pub name: String,
    pub address: u64,
    pub size: u64,
    pub entropy: f64,
}

#[derive(Serialize)]
pub struct SymbolInfo {
    pub name: String,
    pub address: u64,
    pub kind: String,
}

pub struct Analyzer<'a> {
    provider: &'a dyn BinaryProvider,
}

impl<'a> Analyzer<'a> {
    pub fn new(provider: &'a dyn BinaryProvider) -> Self {
        Self { provider }
    }

    pub fn analyze(&self) -> Result<AnalysisResult> {
        let file = self.provider.parse()?;

        let mut sections: Vec<SectionInfo> = file
            .sections()
            .map(|s| {
                let data = s.data().unwrap_or_default();
                let entropy = crate::utils::helpers::calculate_entropy(data);
                SectionInfo {
                    name: s.name().unwrap_or_default().to_string(),
                    address: s.address(),
                    size: s.size(),
                    entropy,
                }
            })
            .collect();
        sections.sort_by_key(|s| s.address);

        let mut symbols: Vec<SymbolInfo> = file
            .symbols()
            .map(|s| SymbolInfo {
                name: s.name().unwrap_or_default().to_string(),
                address: s.address(),
                kind: format!("{:?}", s.kind()),
            })
            .collect();
        symbols.sort_by_key(|s| s.address);

        let mut findings: Vec<String> = Vec::new();
        for section in &sections {
            if section.entropy > 7.0 {
                findings.push(format!(
                    "Section {} has high entropy ({:.2}) - potentially packed or encrypted.",
                    section.name, section.entropy
                ));
            }
        }

        let suspicious = crate::utils::helpers::detect_suspicious_sequences(self.provider.data());
        for (offset, desc) in suspicious {
            findings.push(format!("At offset {:#x}: {}", offset, desc));
        }

        Ok(AnalysisResult {
            format: format!("{:?}", file.format()),
            architecture: format!("{:?}", file.architecture()),
            entry_point: file.entry(),
            sections,
            symbols,
            findings,
        })
    }

    pub fn scan_patterns(&self, hex_pattern: &str) -> Result<Vec<u64>> {
        let pattern = hex::decode(hex_pattern).map_err(|e| {
            crate::errors::UnifyError::InvalidArgument(format!("Invalid hex pattern: {}", e))
        })?;

        let mut matches = Vec::new();
        let data = self.provider.data();

        for i in 0..data.len().saturating_sub(pattern.len()) {
            if &data[i..i + pattern.len()] == pattern.as_slice() {
                matches.push(i as u64);
            }
        }

        Ok(matches)
    }
}
