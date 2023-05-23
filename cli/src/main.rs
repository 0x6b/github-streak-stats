use std::ops::Sub;

use structopt::StructOpt;

use github_streak_stats_lib::{
    github_client::GitHubClient,
    types::Stats,
};

use crate::types::Args;

mod types;

fn main() {
    let Args { login, from, to, debug } = Args::from_args();

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

    let client = GitHubClient::default();

    if debug {
        println!("args: {:#?}", Args::from_args());
        println!("{:#?}", client.get_streak(&login, &start, &end).unwrap());
    }

    let Stats { total_contributions, longest_streak, current_streak } = client
        .calc_streak(&login, &start, &end)
        .unwrap();

    println!(
        r#"🔥 GitHub contribution stats for {} since {} 🔥
Total contributions       | {}
Longest and latest streak | {} days, from {} to {}
Current streak            | {} days, from {} to {}"#,
        login,
        start.split('T').collect::<Vec<&str>>()[0],
        total_contributions,
        (longest_streak.end - longest_streak.start).num_days() + 1,
        longest_streak.start,
        longest_streak.end,
        (current_streak.end - current_streak.start).num_days() + 1,
        current_streak.start,
        current_streak.end,
    );
}
