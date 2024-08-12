use std::{error::Error, ops::Sub};

use clap::Parser;
use github_streak_stats_lib::{github_client::GitHubClient, types::Stats};
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    TableBuilder, TableStyle,
};

use crate::args::Args;

mod args;

fn main() -> Result<(), Box<dyn Error>> {
    let Args {
        login,
        github_token,
        from,
        to,
        offset,
        display_public_repositories,
        debug,
    } = Args::parse();

    let today = chrono::Local::now();
    let start = format!(
        "{}T00:00:00.000+{}",
        &(match from {
            None => today.sub(chrono::Duration::days(365)).format("%Y-%m-%d").to_string(),
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

    let client = GitHubClient::new(
        "https://api.github.com/graphql",
        "github-streaks-stats-lib/0.0.0",
        &github_token,
    );

    let user = match login {
        None => client.get_viewer()?,
        Some(login) => client.get_user(&login)?,
    };

    if debug {
        println!("args: {:#?}", Args::parse());
        println!("start: {}", start);
        println!("end: {}", end);
        println!("{:#?}", client.get_streak(&user, &start, &end)?);
    }

    let Stats {
        total_contributions,
        longest_streak,
        current_streak,
    } = client.calc_streak(&user, &start, &end)?;

    let table = TableBuilder::new()
        .style(TableStyle::rounded())
        .rows(vec![
            Row::new(vec![TableCell::builder(format!(
                "🔥 GitHub contribution stats for https://github.com/{} since {} 🔥",
                if display_public_repositories { user.to_string() } else { user.name },
                start.split('T').collect::<Vec<&str>>()[0]
            ))
            .alignment(Alignment::Center)
            .col_span(2)
            .build()]),
            Row::new(vec![
                TableCell::new("Total contributions"),
                TableCell::builder(total_contributions)
                    .alignment(Alignment::Right)
                    .col_span(1)
                    .build(),
            ]),
            Row::new(vec![
                TableCell::new("Longest and latest streak"),
                TableCell::new(format!(
                    "{} days, from {} to {}",
                    (longest_streak.end - longest_streak.start).num_days() + 1,
                    longest_streak.start,
                    longest_streak.end,
                )),
            ]),
            Row::new(vec![
                TableCell::new("Current streak"),
                TableCell::new(format!(
                    "{} days, from {} to {}",
                    (current_streak.end - current_streak.start).num_days() + 1,
                    current_streak.start,
                    current_streak.end,
                )),
            ]),
        ])
        .build();
    println!("{}", table.render());

    Ok(())
}
