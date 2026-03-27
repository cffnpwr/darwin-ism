use clap::{CommandFactory as _, Parser, Subcommand};

pub const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "\ncommit: ",
    env!("DARWIN_ISM_GIT_HASH"),
    "\nbuilt-at: ",
    env!("DARWIN_ISM_BUILT_AT"),
);

#[derive(Parser)]
#[command(about, disable_version_flag = true)]
pub struct Cli {
    /// Print version
    #[arg(short, long)]
    pub version: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all input sources
    List(ListArgs),
    /// Enable an input source
    Enable(EnableArgs),
    /// Disable an input source
    Disable(DisableArgs),
}

#[derive(Parser)]
pub struct ListArgs {
    /// Show enabled input sources only
    #[arg(short, long)]
    pub enabled: bool,

    /// Filter by bundle ID
    #[arg(short, long = "bundle-id")]
    pub bundle_id: Option<String>,
}

#[derive(Parser)]
pub struct EnableArgs {
    /// Input source ID to enable
    pub id: String,
}

#[derive(Parser)]
pub struct DisableArgs {
    /// Input source ID to disable
    pub id: String,
}

pub fn run(cli: Cli) -> anyhow::Result<()> {
    if cli.version {
        println!("{VERSION}");
        return Ok(());
    }
    match cli.command {
        Some(Commands::List(args)) => run_list(&args),
        Some(Commands::Enable(args)) => run_enable(&args),
        Some(Commands::Disable(args)) => run_disable(&args),
        None => Cli::command()
            .print_help()
            .map_err(|error| anyhow::anyhow!("failed to print help: {error}")),
    }
}

struct Row {
    id: String,
    enabled: String,
    type_str: String,
    name: String,
}

fn run_list(args: &ListArgs) -> anyhow::Result<()> {
    let sources = if args.enabled {
        darwin_ism::list_enabled()?
    } else if let Some(ref bundle_id) = args.bundle_id {
        darwin_ism::list_with_bundle_id(bundle_id, true)?
    } else {
        darwin_ism::list(true)?
    };

    if sources.is_empty() {
        println!("No input sources found.");
        return Ok(());
    }

    let rows: Vec<Row> = sources
        .iter()
        .map(|source| {
            let id = source
                .id()
                .ok()
                .flatten()
                .unwrap_or_else(|| "Unknown".into());
            let enabled = source
                .is_enabled()
                .map_or("Unknown", |e| if e { "true" } else { "false" })
                .to_string();
            let type_str = source
                .input_source_type()
                .ok()
                .flatten()
                .unwrap_or_else(|| "Unknown".into())
                .replace("TISType", "");
            let name = source
                .localized_name()
                .ok()
                .flatten()
                .unwrap_or_else(|| "Unknown".into());
            Row {
                id,
                enabled,
                type_str,
                name,
            }
        })
        .collect();

    let col1_w = rows.iter().map(|r| display_width(&r.id)).max().unwrap_or(0);
    let col1_w = display_width("ID").max(col1_w) + 2;
    let col2_w = rows
        .iter()
        .map(|r| display_width(&r.enabled))
        .max()
        .unwrap_or(0);
    let col2_w = display_width("Enabled").max(col2_w) + 2;
    let col3_w = rows
        .iter()
        .map(|r| display_width(&r.type_str))
        .max()
        .unwrap_or(0);
    let col3_w = display_width("Type").max(col3_w) + 2;

    let total_width = col1_w + col2_w + col3_w + 20;
    println!(
        "{}",
        format_row("ID", "Enabled", "Type", "Name", col1_w, col2_w, col3_w)
    );
    println!("{}", "-".repeat(total_width));
    for row in &rows {
        println!(
            "{}",
            format_row(
                &row.id,
                &row.enabled,
                &row.type_str,
                &row.name,
                col1_w,
                col2_w,
                col3_w
            )
        );
    }
    println!("\nTotal: {} input source(s)", sources.len());

    Ok(())
}

fn run_enable(args: &EnableArgs) -> anyhow::Result<()> {
    if darwin_ism::enable(&args.id)? {
        println!("Enabled: {}", args.id);
    } else {
        println!("Already enabled: {}", args.id);
    }
    Ok(())
}

fn run_disable(args: &DisableArgs) -> anyhow::Result<()> {
    if darwin_ism::disable(&args.id)? {
        println!("Disabled: {}", args.id);
    } else {
        println!("Already disabled: {}", args.id);
    }
    Ok(())
}

/// Check if a Unicode scalar value is a wide character (CJK, fullwidth, etc.)
fn is_wide_char(value: u32) -> bool {
    matches!(value,
        0x1100..=0x115F  // Hangul Jamo
        | 0x2E80..=0x9FFF  // CJK
        | 0xAC00..=0xD7A3  // Hangul Syllables
        | 0xF900..=0xFAFF  // CJK Compatibility
        | 0xFE10..=0xFE1F  // Vertical forms
        | 0xFF00..=0xFF60  // Fullwidth
        | 0xFFE0..=0xFFE6  // Fullwidth symbols
    )
}

/// Calculate display width of a string (wide characters count as 2)
fn display_width(s: &str) -> usize {
    s.chars()
        .map(|c| if is_wide_char(c as u32) { 2 } else { 1 })
        .sum()
}

/// Pad a string to target display width
fn pad_to_width(s: &str, target: usize) -> String {
    let current = display_width(s);
    if current >= target {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(target - current))
    }
}

fn format_row(
    col1: &str,
    col2: &str,
    col3: &str,
    col4: &str,
    w1: usize,
    w2: usize,
    w3: usize,
) -> String {
    format!(
        "{}{}{}{}",
        pad_to_width(col1, w1),
        pad_to_width(col2, w2),
        pad_to_width(col3, w3),
        col4
    )
}
