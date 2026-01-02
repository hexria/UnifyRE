use crate::core::analyzer::AnalysisResult;
use serde::Serialize;

#[derive(Serialize)]
pub struct DiffResult {
    pub section_diffs: Vec<String>,
    pub symbol_diffs: Vec<String>,
}

pub struct DiffEngine;

impl DiffEngine {
    pub fn compare(res1: &AnalysisResult, res2: &AnalysisResult) -> DiffResult {
        let mut section_diffs = Vec::new();
        let mut symbol_diffs = Vec::new();

        // Compare sections
        if res1.sections.len() != res2.sections.len() {
            section_diffs.push(format!(
                "Section count mismatch: {} vs {}",
                res1.sections.len(),
                res2.sections.len()
            ));
        }

        for (s1, s2) in res1.sections.iter().zip(res2.sections.iter()) {
            if s1.name != s2.name {
                section_diffs.push(format!("Section name change: {} -> {}", s1.name, s2.name));
            }
            if s1.size != s2.size {
                section_diffs.push(format!(
                    "Section {} size change: {:#x} -> {:#x}",
                    s1.name, s1.size, s2.size
                ));
            }
            if (s1.entropy - s2.entropy).abs() > 0.1 {
                section_diffs.push(format!(
                    "Section {} entropy shift: {:.2} -> {:.2}",
                    s1.name, s1.entropy, s2.entropy
                ));
            }
        }

        // Compare symbols
        let names2: std::collections::HashSet<_> = res2.symbols.iter().map(|s| &s.name).collect();
        for s1 in &res1.symbols {
            if !names2.contains(&s1.name) {
                symbol_diffs.push(format!("Symbol removed: {}", s1.name));
            }
        }

        let names1: std::collections::HashSet<_> = res1.symbols.iter().map(|s| &s.name).collect();
        for s2 in &res2.symbols {
            if !names1.contains(&s2.name) {
                symbol_diffs.push(format!("Symbol added: {}", s2.name));
            }
        }

        DiffResult {
            section_diffs,
            symbol_diffs,
        }
    }
}
