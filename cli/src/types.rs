use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "github-streak-stats",
    about = "Show GitHub contribution streak"
)]
pub struct Args {
    /// GitHub login name
    #[structopt()]
    pub login: String,

    /// Start date
    #[structopt(short, long)]
    pub from: Option<String>,

    /// End date. Please note that the total time spanned by 'from' and 'to' must not exceed 1 year
    #[structopt(short, long)]
    pub to: Option<String>,

    /// Debug mode
    #[structopt(short, long)]
    pub debug: bool,
}
