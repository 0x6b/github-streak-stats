use std::error::Error;

use clap::Parser;
use colorful::{Color, Colorful, RGB};
use github_streak_stats_lib::{github_client::GitHubClient, types::Stats};
use jiff::{civil::DateTime, tz, tz::TimeZone};
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

    let today = jiff::Zoned::now();
    today.offset().checked_sub(offset.parse().unwrap_or_default())?;
    let span = jiff::Span::new();

    let parse_date = |date: &str| -> Result<DateTime, Box<dyn Error>> {
        let d = date.split('-').collect::<Vec<_>>();
        Ok(DateTime::new(
            d[0].parse().unwrap(),
            d[1].parse().unwrap(),
            d[2].parse().unwrap(),
            0,
            0,
            0,
            0,
        )?)
    };

    let start = match from {
        Some(from) => parse_date(&from)?
            .to_zoned(TimeZone::fixed(tz::offset(offset.parse().unwrap_or_default())))?,
        None => {
            // find the first Sunday before start
            let a_year_ago = today.checked_sub(span.weeks(52))?;
            (0..7)
                .flat_map(|i| a_year_ago.checked_sub(span.days(i)))
                .find(|date| date.weekday() == jiff::civil::Weekday::Sunday)
                .unwrap()
        }
    }
    .strftime("%Y-%m-%dT%H:%M:%S.000%z")
    .to_string();

    let end = match to {
        Some(to) => parse_date(&to)?
            .to_zoned(TimeZone::fixed(tz::offset(offset.parse().unwrap_or_default())))?,
        None => {
            // find the first Saturday after start
            (0..7)
                .flat_map(|i| today.checked_add(span.days(i)))
                .find(|date| date.weekday() == jiff::civil::Weekday::Saturday)
                .unwrap()
        }
    }
    .strftime("%Y-%m-%dT%H:%M:%S.000%z")
    .to_string();

    let client = GitHubClient::new(
        "https://api.github.com/graphql",
        "github-streaks-stats-lib/0.0.0",
        &github_token,
    );

    let user = match &login {
        None => client.get_viewer()?,
        Some(login) => client.get_user(login)?,
    };

    let stats = client.get_contributions(&user, &start, &end)?;

    // find max contribution count from the stats
    let max = stats.iter().map(|day| day.contribution_count).max().unwrap();

    // create a matrix of the stats, where each cell is a colored square
    let matrix: Vec<Vec<String>> = stats
        .chunks(7)
        .map(|week| {
            week.iter()
                .map(|day| {
                    if day.contribution_count == 0 {
                        "\u{25A1} ".color(Color::DarkGray)
                    } else {
                        "\u{25A0} ".color(RGB::new(
                            0,
                            255 - (((day.contribution_count as f64 / max as f64) * 256.0) as u8),
                            0,
                        ))
                    }
                })
                .map(|c| c.to_string())
                .collect()
        })
        .collect();

    // transpose the matrix
    let matrix: Vec<Vec<String>> = (0..7)
        .map(|i| matrix.iter().map(|row| row[i].clone()).collect())
        .collect();

    let matrix_string = matrix
        .iter()
        .map(|row| row.join(""))
        .collect::<Vec<String>>()
        .join("\n");

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
        println!("{:#?}", client.get_contributions(&user, &start, &end)?);
    }

    let Stats {
        total_contributions,
        longest_streak,
        current_streak,
    } = client.calc_streak_from_contributions(&stats)?;

    let table = TableBuilder::new()
        .style(TableStyle::rounded())
        .rows(vec![
            Row::new(vec![TableCell::builder(format!(
                "🔥 GitHub contribution stats for https://github.com/{} since {} 🔥",
                if display_public_repositories { user.to_string() } else { user.name },
                start.split('T').next().unwrap(),
            ))
            .alignment(Alignment::Center)
            .col_span(2)
            .build()]),
            Row::new(vec![TableCell::builder(matrix_string)
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
