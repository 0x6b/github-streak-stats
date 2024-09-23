use std::time::Duration;

use clap::Parser;
// use termbg::Theme;

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

    /// Display contribution matrix
    #[arg(short = 'm', long)]
    pub display_matrix: bool,

    /// Theme for the contribution matrix. Possible values: dark, light, or auto
    #[arg(short = 'e', long, default_value = "auto")]
    pub theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Theme {
    Dark,
    Light,
    #[allow(dead_code)] // For command line parsing only
    Auto,
}

impl From<termbg::Theme> for Theme {
    fn from(theme: termbg::Theme) -> Self {
        match theme {
            termbg::Theme::Dark => Self::Dark,
            termbg::Theme::Light => Self::Light,
        }
    }
}

impl std::str::FromStr for Theme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str().chars().next().unwrap_or_default() {
            'd' => Ok(Theme::Dark),
            'l' => Ok(Theme::Light),
            'a' => Ok(termbg::theme(Duration::from_millis(10))
                .unwrap_or(termbg::Theme::Dark)
                .into()),
            _ => Err("Invalid theme".to_string()),
        }
    }
}
