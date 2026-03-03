use std::fs;
use std::path::PathBuf;
use std::process;
use std::thread;
use std::time::Instant;

use clap::{ArgAction, Parser, ValueHint};
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

#[derive(Clone, Debug)]
struct RenderTask {
    line_num: usize,
    formula: String,
}

#[derive(Debug)]
enum RenderOutcome {
    Ok { html: String, elapsed_ms: f64 },
    Error { message: String, elapsed_ms: f64 },
}

#[derive(Debug)]
struct RenderResult {
    line_num: usize,
    formula: String,
    outcome: RenderOutcome,
}

fn render_formula(task: RenderTask) -> RenderResult {
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
        katex_wasm::render_to_string(task.formula.clone(), settings)
    }));
    let elapsed_ms = render_start.elapsed().as_secs_f64() * 1000.0;

    let outcome = match result {
        Ok(html) => RenderOutcome::Ok { html, elapsed_ms },
        Err(panic_info) => {
            let message = if let Some(msg) = panic_info.downcast_ref::<String>() {
                msg.clone()
            } else if let Some(msg) = panic_info.downcast_ref::<&str>() {
                msg.to_string()
            } else {
                "unknown panic".to_string()
            };
            RenderOutcome::Error { message, elapsed_ms }
        }
    };

    RenderResult {
        line_num: task.line_num,
        formula: task.formula,
        outcome,
    }
}

fn render_tasks(tasks: Vec<RenderTask>, multi_threaded: bool) -> Vec<RenderResult> {
    if tasks.is_empty() {
        return Vec::new();
    }

    if !multi_threaded {
        return tasks.into_iter().map(render_formula).collect();
    }

    let worker_count = thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1)
        .min(tasks.len());

    if worker_count <= 1 {
        return tasks.into_iter().map(render_formula).collect();
    }

    let chunk_size = tasks.len().div_ceil(worker_count);
    let mut results = Vec::with_capacity(tasks.len());

    thread::scope(|scope| {
        let mut handles = Vec::new();

        for chunk in tasks.chunks(chunk_size) {
            let chunk_tasks = chunk.to_vec();
            handles.push(scope.spawn(move || {
                chunk_tasks
                    .into_iter()
                    .map(render_formula)
                    .collect::<Vec<RenderResult>>()
            }));
        }

        for handle in handles {
            match handle.join() {
                Ok(mut chunk_results) => results.append(&mut chunk_results),
                Err(_) => {
                    eprintln!("A worker thread panicked while rendering formulas");
                    process::exit(1);
                }
            }
        }
    });

    results.sort_by_key(|result| result.line_num);
    results
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

    /// Only print the final summary
    #[arg(long, visible_alias = "summery-only")]
    summary_only: bool,

    /// Whether to render formulas in parallel (enabled by default)
    #[arg(long, default_value_t = true, action = ArgAction::Set)]
    multi_threaded: bool,
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
    let summary_only = args.summary_only;
    let multi_threaded = args.multi_threaded;

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

    if !summary_only {
        eprintln!(
            "Processing lines {}-{} from '{}'",
            start_line,
            end_line,
            formula_file_path.display()
        );
        eprintln!();
    }

    let mut pass_count: usize = 0;
    let mut error_count: usize = 0;
    let mut tasks = Vec::new();

    for line_num in start_line..=end_line {
        let formula = all_lines[line_num - 1].trim();
        if formula.is_empty() || formula.starts_with('#') {
            continue;
        }

        tasks.push(RenderTask {
            line_num,
            formula: formula.to_string(),
        });
    }

    let results = render_tasks(tasks, multi_threaded);

    for result in results {
        if !summary_only {
            println!("{BOLD}========== Line {} =========={RESET}", result.line_num);
            println!("Formula: {}", result.formula);
            println!();
        }

        match result.outcome {
            RenderOutcome::Ok { html, elapsed_ms } => {
                if !summary_only {
                    println!("{DIM}--- Rust HTML ({elapsed_ms:.2}ms) ---{RESET}");
                    println!("{html}");
                    println!();
                    println!("{GREEN}✓ OK{RESET}");
                }
                pass_count += 1;
            }
            RenderOutcome::Error { message, elapsed_ms } => {
                if !summary_only {
                    println!("{RED}✗ ERROR ({elapsed_ms:.2}ms): {message}{RESET}");
                }
                error_count += 1;
            }
        }

        if !summary_only {
            println!();
        }
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
