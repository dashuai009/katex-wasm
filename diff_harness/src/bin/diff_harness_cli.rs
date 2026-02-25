use std::fs;
use std::env;

use diff_harness::harness::{get_js_parse_tree, get_rust_parse_tree, get_js_html, get_rust_html};

fn print_usage() {
    eprintln!("Usage: diff_harness_cli <formulas.txt> [start_line] [end_line]");
    eprintln!();
    eprintln!("  formulas.txt   Path to a file with one LaTeX formula per line");
    eprintln!("  start_line     Start line number (1-based, inclusive). Default: 1");
    eprintln!("  end_line       End line number (1-based, inclusive). Default: last line");
    eprintln!();
    eprintln!("Example:");
    eprintln!("  cargo run -p diff_harness --bin diff_harness_cli -- tests/fixtures/formulas.txt 1 5");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    let file_path = &args[1];
    let content = fs::read_to_string(file_path)
        .unwrap_or_else(|e| {
            eprintln!("Error reading file '{}': {}", file_path, e);
            std::process::exit(1);
        });

    let all_lines: Vec<&str> = content.lines().collect();
    let total_lines = all_lines.len();

    let start_line: usize = args.get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1)
        .max(1);

    let end_line: usize = args.get(3)
        .and_then(|s| s.parse().ok())
        .unwrap_or(total_lines)
        .min(total_lines);

    if start_line > end_line || start_line > total_lines {
        eprintln!("Invalid line range: {}-{} (file has {} lines)", start_line, end_line, total_lines);
        std::process::exit(1);
    }

    // Determine project root (parent of diff_harness)
    let project_root = env::var("CARGO_MANIFEST_DIR")
        .map(|dir| {
            std::path::Path::new(&dir)
                .parent()
                .unwrap_or(std::path::Path::new("."))
                .to_string_lossy()
                .to_string()
        })
        .unwrap_or_else(|_| ".".to_string());

    eprintln!("Processing lines {}-{} from '{}'", start_line, end_line, file_path);
    eprintln!("Project root: {}", project_root);
    eprintln!();

    for line_num in start_line..=end_line {
        let formula = all_lines[line_num - 1].trim();
        if formula.is_empty() || formula.starts_with('#') {
            continue;
        }

        println!("========== Line {} ==========", line_num);
        println!("Formula: {}", formula);
        println!();

        let js_start = std::time::Instant::now();
        let js_output = get_js_parse_tree(formula, &project_root);
        let js_elapsed = js_start.elapsed();

        let rust_start = std::time::Instant::now();
        let rust_output = get_rust_parse_tree(formula);
        let rust_elapsed = rust_start.elapsed();

        println!("--- JS parseTree ({:.2?}) ---", js_elapsed);
        println!("{}", js_output);
        println!();
        println!("--- Rust parseTree ({:.2?}) ---", rust_elapsed);
        println!("{}", rust_output);
        println!();

        let js_html_start = std::time::Instant::now();
        let js_html = get_js_html(formula, &project_root);
        let js_html_elapsed = js_html_start.elapsed();

        let rust_html_start = std::time::Instant::now();
        let rust_html = get_rust_html(formula);
        let rust_html_elapsed = rust_html_start.elapsed();

        println!("--- JS HTML ({:.2?}) ---", js_html_elapsed);
        println!("{}", js_html);
        println!();
        println!("--- Rust HTML ({:.2?}) ---", rust_html_elapsed);
        println!("{}", rust_html);
        println!();
    }
}
