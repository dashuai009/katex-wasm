use std::fs;
use std::path::PathBuf;
use std::process;
use std::time::Instant;

use clap::{Parser, ValueHint};
use katex_wasm::settings::Settings;

fn parse_line_number(value: &str) -> Result<usize, String> {
    let parsed = value
        .parse::<usize>()
        .map_err(|_| format!("'{value}' is not a valid positive integer"))?;

    if parsed == 0 {
        return Err("line numbers must start at 1".to_string());
    }

    Ok(parsed)
}

#[derive(Debug, Parser)]
#[command(
    name = "katex-rs-cli",
    bin_name = "katex-rs-cli",
    version,
    about = "Render LaTeX formulas from a file with the Rust KaTeX implementation"
)]
struct Cli {
    /// Path to a file with one LaTeX formula per line
    #[arg(value_name = "FORMULAS_TXT", value_hint = ValueHint::FilePath)]
    formula_file_path: PathBuf,

    /// Start line number (1-based, inclusive)
    #[arg(
        value_name = "START_LINE",
        default_value_t = 1,
        value_parser = parse_line_number
    )]
    start_line: usize,

    /// End line number (1-based, inclusive)
    #[arg(value_name = "END_LINE", value_parser = parse_line_number)]
    end_line: Option<usize>,
}

fn main() {
    let args = Cli::parse();
    let formula_file_path = args.formula_file_path;

    let content = match fs::read_to_string(&formula_file_path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!(
                "Error reading file '{}': {}",
                formula_file_path.display(),
                error
            );
            process::exit(1);
        }
    };

    let all_lines: Vec<&str> = content.split('\n').collect();
    let total_lines = all_lines.len();

    let start_line = args.start_line;
    let end_line = args.end_line.unwrap_or(total_lines).min(total_lines);

    if start_line > end_line || start_line > total_lines {
        eprintln!(
            "Invalid line range: {}-{} (file has {} lines)",
            start_line, end_line, total_lines
        );
        process::exit(1);
    }

    const RED: &str = "\x1b[31m";
    const GREEN: &str = "\x1b[32m";
    const RESET: &str = "\x1b[0m";
    const BOLD: &str = "\x1b[1m";
    const DIM: &str = "\x1b[2m";

    eprintln!(
        "Processing lines {}-{} from '{}'",
        start_line,
        end_line,
        formula_file_path.display()
    );
    eprintln!();

    let mut pass_count: usize = 0;
    let mut error_count: usize = 0;

    for line_num in start_line..=end_line {
        let formula = all_lines[line_num - 1].trim();
        if formula.is_empty() || formula.starts_with('#') {
            continue;
        }

        println!("{BOLD}========== Line {line_num} =========={RESET}");
        println!("Formula: {formula}");
        println!();

        let mut settings = Settings::new();
        settings.set_display_mode(true);
        settings.set_output("html".to_string());
        settings.set_throw_on_error(false);
        settings.set_strict("ignore".to_string());
        settings.set_trust(true);
        settings.set_max_size(Some(200000.0));
        settings.set_max_expand(Some(1000));

        let render_start = Instant::now();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            katex_wasm::render_to_string(formula.to_string(), settings)
        }));
        let elapsed_ms = render_start.elapsed().as_secs_f64() * 1000.0;

        match result {
            Ok(html) => {
                println!("{DIM}--- Rust HTML ({elapsed_ms:.2}ms) ---{RESET}");
                println!("{html}");
                println!();
                println!("{GREEN}✓ OK{RESET}");
                pass_count += 1;
            }
            Err(panic_info) => {
                let message = if let Some(msg) = panic_info.downcast_ref::<String>() {
                    msg.clone()
                } else if let Some(msg) = panic_info.downcast_ref::<&str>() {
                    msg.to_string()
                } else {
                    "unknown panic".to_string()
                };
                println!("{RED}✗ ERROR ({elapsed_ms:.2}ms): {message}{RESET}");
                error_count += 1;
            }
        }
        println!();
    }

    println!("{BOLD}══════════ Summary ══════════{RESET}");
    println!("  {GREEN}Pass:{RESET}   {pass_count}");
    if error_count > 0 {
        println!("  {RED}Errors:{RESET} {error_count}");
    }
    println!("  Total:  {}", pass_count + error_count);

    if error_count > 0 {
        process::exit(1);
    }
}
