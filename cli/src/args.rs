use std::{ops::Deref, time::Duration};

use clap::Parser;
use colorful::RGB;

#[derive(Parser, Debug)]
#[clap(about, version)]
pub struct Args {
    /// GitHub login name. Defaults to the login name of the GitHub API token owner.
    #[arg()]
    pub login: Option<String>,

    /// GitHub personal access token
    #[arg(short, long, env = "GITHUB_TOKEN")]
    pub github_token: String,

    /// Start date, in YYYY-MM-DD format. Default value will be the first Sunday before 52 weeks
    /// ago. If specified, it will be the first Sunday before the specified date.
    #[arg(short, long)]
    pub from: Option<String>,

    /// End date, in YYYY-MM-DD format. Please note that the total time spanned by 'from' and 'to'
    /// must not exceed 1 year. Default value will be the first Saturday after today. If specified,
    /// it will be the first Saturday after the specified date.
    #[arg(short, long)]
    pub to: Option<String>,

    /// Offset from UTC, in (+|-)HHMM format
    #[arg(short, long, default_value = "+0900")]
    pub offset: String,

    /// Display number of public repositories owned
    #[arg(short = 'r', long)]
    pub display_public_repositories: bool,

    /// Do not display contribution matrix
    #[arg(short = 'm', long)]
    pub no_display_matrix: bool,

    /// Theme for the contribution matrix. Possible values: dark, light, or auto
    #[arg(short = 'e', long, default_value = "auto")]
    pub theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Theme {
    Dark([RGB; 5]),
    Light([RGB; 5]),
    #[allow(dead_code)] // For command line parsing only
    Auto,
}

impl From<termbg::Theme> for Theme {
    fn from(theme: termbg::Theme) -> Self {
        match theme {
            termbg::Theme::Dark => Self::dark(),
            termbg::Theme::Light => Self::light(),
        }
    }
}

impl Deref for Theme {
    type Target = [RGB; 5];

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Dark(palette) | Self::Light(palette) => palette,
            Self::Auto => unreachable!("Invalid theme"),
        }
    }
}

impl Theme {
    fn dark() -> Self {
        Self::Dark([
            RGB::new(22, 27, 34),
            RGB::new(14, 68, 41),
            RGB::new(0, 109, 50),
            RGB::new(38, 166, 65),
            RGB::new(57, 211, 83),
        ])
    }

    fn light() -> Self {
        Self::Light([
            RGB::new(235, 237, 240),
            RGB::new(155, 233, 168),
            RGB::new(64, 196, 99),
            RGB::new(48, 161, 78),
            RGB::new(33, 110, 57),
        ])
    }
}

impl std::str::FromStr for Theme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str().chars().next().unwrap_or_default() {
            'd' => Ok(Self::dark()),
            'l' => Ok(Self::light()),
            'a' => Ok(termbg::theme(Duration::from_millis(10))
                .unwrap_or(termbg::Theme::Dark)
                .into()),
            _ => Err("Invalid theme".to_string()),
        }
    }
}
