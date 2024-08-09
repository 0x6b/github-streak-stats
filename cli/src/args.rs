use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version)]
pub struct Args {
    /// GitHub login name. Defaults to the login name of the GitHub API token owner.
    #[arg()]
    pub login: Option<String>,

    /// GitHub personal access token
    #[arg(short, long, env = "GITHUB_TOKEN")]
    pub github_token: String,

    /// Start date, in YYYY-MM-DD format. Defaults is 1 year ago from today.
    #[arg(short, long)]
    pub from: Option<String>,

    /// End date, in YYYY-MM-DD format. Please note that the total time spanned by 'from' and 'to'
    /// must not exceed 1 year. Defaults is today.
    #[arg(short, long)]
    pub to: Option<String>,

    /// Offset from UTC, in HH:MM format
    #[arg(short, long, default_value = "09:00")]
    pub offset: String,

    /// Display number of public repositories owned
    #[arg(short = 'r', long)]
    pub display_public_repositories: bool,

    /// Debug mode
    #[arg(short, long, hide = true)]
    pub debug: bool,
}
