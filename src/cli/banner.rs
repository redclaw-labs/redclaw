use console::Style;

/// Print the RedClaw startup banner to stderr
pub fn print_banner() {
    let red = Style::new().red().bold();
    let dim = Style::new().dim();

    let banner = r#"
    ____           _  ____  _                
   |  _ \ ___  __| |/ ___|| | __ ___      __
   | |_) / _ \/ _` | |    | |/ _` \ \ /\ / /
   |  _ <  __/ (_| | |___ | | (_| |\ V  V / 
   |_| \_\___|\__,_|\____||_|\__,_| \_/\_/  
    "#;

    eprintln!("{}", red.apply_to(banner));
    eprintln!(
        "{}",
        dim.apply_to(format!(
            "  v{} — Battle-hardened AI agent runtime",
            env!("CARGO_PKG_VERSION")
        ))
    );
    eprintln!();
}

/// Print startup message for a specific service
pub fn print_service_start(service: &str) {
    let red = Style::new().red().bold();
    let white = Style::new().white();
    eprintln!(
        "{} {}",
        red.apply_to("▸"),
        white.apply_to(format!("Starting {service}..."))
    );
}

/// Print service ready message
pub fn print_service_ready(service: &str, detail: &str) {
    let green = Style::new().green().bold();
    let white = Style::new().white();
    eprintln!(
        "{} {} {}",
        green.apply_to("✓"),
        white.apply_to(format!("{service} ready")),
        Style::new().dim().apply_to(format!("({detail})"))
    );
}

/// Print shutdown message
pub fn print_shutdown() {
    let dim = Style::new().dim();
    eprintln!("{}", dim.apply_to("  RedClaw shutting down..."));
}

/// Print error banner for fatal errors
pub fn print_fatal(msg: &str) {
    let red = Style::new().red().bold();
    eprintln!("{} {}", red.apply_to("✗ FATAL:"), msg);
}
