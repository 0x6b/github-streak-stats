use std::ops::Sub;

use clap::Parser;

use github_streak_stats_lib::{github_client::GitHubClient, types::Stats};

use crate::args::Args;

mod args;

fn main() {
    let Args {
        login,
        from,
        to,
        offset,
        debug,
    } = Args::parse();

    let today = chrono::Local::now();
    let start = format!(
        "{}T00:00:00.000+{}",
        &(match from {
            None => today
                .sub(chrono::Duration::days(365))
                .format("%Y-%m-%d")
                .to_string(),
            Some(date) => date,
        }),
        offset,
    );
    let end = format!(
        "{}T00:00:00.000+{}",
        &(match to {
            None => today.format("%Y-%m-%d").to_string(),
            Some(date) => date,
        }),
        offset,
    );

    let client = GitHubClient::default();

    if debug {
        println!("args: {:#?}", Args::parse());
        println!("start: {}", start);
        println!("end: {}", end);
        println!("{:#?}", client.get_streak(&login, &start, &end).unwrap());
    }

    let Stats {
        total_contributions,
        longest_streak,
        current_streak,
    } = client.calc_streak(&login, &start, &end).unwrap();

    println!(
        r#"🔥 GitHub contribution stats for https://github.com/{} since {} 🔥
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
