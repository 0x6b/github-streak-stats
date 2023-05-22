use std::ops::Sub;

use structopt::StructOpt;

use github_streak_stats_lib::GitHubClient;

#[derive(StructOpt, Debug)]
#[structopt(name = "github-streak-stats", about = "Show GitHub contribution streak")]
struct Args {
    /// GitHub login name
    #[structopt()]
    login: String,

    /// Start date
    #[structopt(short, long)]
    from: Option<String>,

    /// End date. Please note that the total time spanned by 'from' and 'to' must not exceed 1 year
    #[structopt(short, long)]
    to: Option<String>,
}

fn main() {
    let Args { login, from, to } = Args::from_args();

    let start = format!(
        "{}T00:00:00.000+09:00",
        &(match from {
            None => (chrono::Local::now().sub(chrono::Duration::days(365)))
                .format("%Y-%m-%d")
                .to_string(),
            Some(date) => date,
        })
    );
    let end = format!(
        "{}T00:00:00.000+09:00",
        &(match to {
            None => chrono::Local::now().format("%Y-%m-%d").to_string(),
            Some(date) => date,
        })
    );

    let (total, longest, current) = GitHubClient::default()
        .calc_streak(&login, &start, &end)
        .unwrap();

    println!(
        r#"🔥 GitHub contribution stats for {} since {} 🔥
- Total contributions: {}
- Longest streak: {} days ({}–{})
- Current streak: {} days ({}–{})"#,
        login,
        start.split('T').collect::<Vec<&str>>()[0],
        total,
        (longest.1 - longest.0).num_days() + 1,
        longest.0,
        longest.1,
        (current.1 - current.0).num_days() + 1,
        current.0,
        current.1
    );
}
