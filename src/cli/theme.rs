use console::Style;

/// RedClaw color theme for terminal output
pub struct Theme;

impl Theme {
    /// Primary brand color — red
    pub fn primary() -> Style {
        Style::new().red().bold()
    }

    /// Success indicator — green
    pub fn success() -> Style {
        Style::new().green().bold()
    }

    /// Warning indicator — yellow
    pub fn warning() -> Style {
        Style::new().yellow()
    }

    /// Error/failure — red (same as primary but used semantically)
    pub fn error() -> Style {
        Style::new().red()
    }

    /// Subtle/secondary text
    pub fn dim() -> Style {
        Style::new().dim()
    }

    /// Emphasis text
    pub fn emphasis() -> Style {
        Style::new().white().bold()
    }

    /// Section headers
    pub fn header() -> Style {
        Style::new().red().bold().underlined()
    }
}
