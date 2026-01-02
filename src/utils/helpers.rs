/// Calculate the Shannon entropy of a byte slice
pub fn calculate_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut frequencies = [0usize; 256];
    for &byte in data {
        frequencies[byte as usize] += 1;
    }

    let len = data.len() as f64;
    let mut entropy = 0.0;

    for &count in &frequencies {
        if count > 0 {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }
    }

    entropy
}

/// Detect suspicious byte patterns (e.g., NOP sleds)
pub fn detect_suspicious_sequences(data: &[u8]) -> Vec<(usize, String)> {
    let mut findings = Vec::new();

    // Detect NOP sleds (e.g., 20+ NOPs)
    let mut nop_count = 0;
    let mut nop_start = 0;
    for (i, &byte) in data.iter().enumerate() {
        if byte == 0x90 {
            if nop_count == 0 {
                nop_start = i;
            }
            nop_count += 1;
        } else {
            if nop_count >= 20 {
                findings.push((
                    nop_start,
                    format!("NOP sled detected (length: {})", nop_count),
                ));
            }
            nop_count = 0;
        }
    }

    findings
}
