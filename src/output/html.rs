use crate::core::analyzer::AnalysisResult;
use crate::errors::Result;
use std::fs;

pub fn generate_html_report(report: &AnalysisResult, out_path: &str) -> Result<()> {
    let mut html = String::new();

    // Header & CSS
    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str("<title>UnifyRE Analysis Report</title>\n");
    html.push_str("<style>\n");
    html.push_str("body { font-family: 'Inter', system-ui, -apple-system, sans-serif; background-color: #0f172a; color: #e2e8f0; margin: 0; padding: 2rem; }\n");
    html.push_str(".container { max-width: 1200px; margin: 0 auto; }\n");
    html.push_str(
        "h1 { color: #38bdf8; border-bottom: 2px solid #334155; padding-bottom: 0.5rem; }\n",
    );
    html.push_str("h2 { color: #7dd3fc; margin-top: 2rem; }\n");
    html.push_str(".card { background: #1e293b; border-radius: 0.5rem; padding: 1.5rem; margin-bottom: 1rem; border: 1px solid #334155; }\n");
    html.push_str("table { width: 100%; border-collapse: collapse; margin-top: 1rem; }\n");
    html.push_str(
        "th, td { padding: 0.75rem; text-align: left; border-bottom: 1px solid #334155; }\n",
    );
    html.push_str("th { color: #94a3b8; font-weight: 600; }\n");
    html.push_str(".finding { color: #f87171; background: #450a0a; padding: 0.5rem; border-radius: 0.25rem; margin-bottom: 0.5rem; border-left: 4px solid #ef4444; }\n");
    html.push_str(".high-entropy { color: #fbbf24; }\n");
    html.push_str("</style>\n</head>\n<body>\n<div class=\"container\">\n");

    html.push_str("<h1>UnifyRE Analysis Report</h1>\n");

    // Overview
    html.push_str("<div class=\"card\">\n<h2>Overview</h2>\n<table>\n");
    html.push_str(&format!(
        "<tr><th>Format</th><td>{}</td></tr>\n",
        report.format
    ));
    html.push_str(&format!(
        "<tr><th>Architecture</th><td>{}</td></tr>\n",
        report.architecture
    ));
    html.push_str(&format!(
        "<tr><th>Entry Point</th><td>0x{:x}</td></tr>\n",
        report.entry_point
    ));
    html.push_str("</table>\n</div>\n");

    // Findings
    if !report.findings.is_empty() {
        html.push_str("<h2>Suspicious Findings</h2>\n");
        for finding in &report.findings {
            html.push_str(&format!(
                "<div class=\"finding\"><strong>(!)</strong> {}</div>\n",
                finding
            ));
        }
    }

    // Sections
    html.push_str("<h2>Sections</h2>\n<div class=\"card\">\n<table>\n");
    html.push_str("<tr><th>Name</th><th>Address</th><th>Size</th><th>Entropy</th></tr>\n");
    for section in &report.sections {
        let entropy_class = if section.entropy > 7.0 {
            "high-entropy"
        } else {
            ""
        };
        html.push_str(&format!(
            "<tr><td>{}</td><td>0x{:x}</td><td>0x{:x}</td><td class=\"{}\">{:.2}</td></tr>\n",
            section.name, section.address, section.size, entropy_class, section.entropy
        ));
    }
    html.push_str("</table>\n</div>\n");

    html.push_str("</div>\n</body>\n</html>");

    fs::write(out_path, html).map_err(|e| crate::errors::UnifyError::Io(e))?;
    Ok(())
}
