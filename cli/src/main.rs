use std::error::Error;

use clap::Parser;
use colorful::{Color, Colorful, RGB};
use github_streak_stats_lib::{github_client::GitHubClient, types::Stats};
use jiff::{fmt::strtime, Zoned};
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

    let today = Zoned::now();
    today.offset().checked_sub(offset.parse().unwrap_or_default())?;
    let span = jiff::Span::new();

    let parse_date = |date: &str| -> Result<Zoned, Box<dyn Error>> {
        strtime::parse("%Y-%m-%d%z", format!("{}{}", date, offset))?
            .to_zoned()
            .map_err(Into::into)
    };

    let find_first_sunday_before = |date: &Zoned| -> Result<Zoned, Box<dyn Error>> {
        (0..7)
            .flat_map(|i| date.checked_sub(span.days(i)))
            .find(|date| date.weekday() == jiff::civil::Weekday::Sunday)
            .ok_or("No Sunday found".into()) // really?
    };

    let find_first_saturday_after = |date: &Zoned| -> Result<Zoned, Box<dyn Error>> {
        (0..7)
            .flat_map(|i| date.checked_add(span.days(i)))
            .find(|date| date.weekday() == jiff::civil::Weekday::Saturday)
            .ok_or("No Saturday found".into()) // really?
    };

    let (start, end) = match (from.clone(), to.clone()) {
        (Some(from), Some(to)) => (parse_date(&from)?, parse_date(&to)?),
        (Some(from), None) => {
            let start = find_first_sunday_before(&parse_date(&from)?)?;
            let end = find_first_saturday_after(&start.checked_add(span.weeks(52))?)?;
            (start, end)
        }
        (None, Some(to)) => {
            let end = find_first_saturday_after(&parse_date(&to)?)?;
            let start = find_first_sunday_before(&end.checked_sub(span.weeks(52))?)?;
            (start, end)
        }
        (None, None) => (
            find_first_sunday_before(&today.checked_sub(span.weeks(52))?)?,
            find_first_saturday_after(&today)?,
        ),
    };

    let client = GitHubClient::new(
        "https://api.github.com/graphql",
        "github-streaks-stats-lib/0.0.0",
        &github_token,
    );

    let user = match &login {
        None => client.get_viewer()?,
        Some(login) => client.get_user(login)?,
    };

    let stats = client.get_contributions(
        &user,
        &start.strftime("%Y-%m-%dT%H:%M:%S.000%z").to_string(),
        &end.strftime("%Y-%m-%dT%H:%M:%S.000%z").to_string(),
    )?;

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
        println!(
            "{:#?}",
            client.get_contributions(
                &user,
                &start.strftime("%Y-%m-%dT%H:%M:%S.000%z").to_string(),
                &end.strftime("%Y-%m-%dT%H:%M:%S.000%z").to_string()
            )?
        );
    }

    let Stats {
        total_contributions,
        longest_streak,
        current_streak,
    } = client.calc_streak_from_contributions(&stats)?;

    let matrix = if !(from.is_some() && to.is_some()) {
        // find max contribution count from the stats
        let max = stats.iter().map(|day| day.contribution_count).max().unwrap();

        // normalize contribution counts to 0-255
        let stats = stats
            .iter()
            .map(|day| (day.contribution_count as f64 / max as f64 * 255.0) as u8)
            .collect::<Vec<u8>>();

        // create a matrix of the stats, where each cell is a colored square
        let matrix: Vec<Vec<String>> = stats
            .chunks(7)
            .map(|week| {
                week.iter()
                    .map(|contribution| {
                        if contribution == &0 {
                            "\u{25A1} ".color(Color::DarkGray)
                        } else {
                            "\u{25A0} ".color(RGB::new(0, 255 - contribution, 0))
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

        Row::new(vec![TableCell::builder(
            matrix
                .iter()
                .map(|row| row.join(""))
                .collect::<Vec<String>>()
                .join("\n"),
        )
            .alignment(Alignment::Center)
            .col_span(2)
            .build()])
    } else {
        Row::new(vec![TableCell::builder(
            "(No contribution graph available when both 'from' and 'to' are specified)",
        )
            .alignment(Alignment::Center)
            .col_span(2)
            .build()])
    };

    let table = TableBuilder::new()
        .style(TableStyle::rounded())
        .rows(vec![
            Row::new(vec![TableCell::builder(format!(
                "🔥 GitHub contribution stats for https://github.com/{} since {} 🔥",
                if display_public_repositories { user.to_string() } else { user.name },
                start.strftime("%Y-%m-%d"),
            ))
                .alignment(Alignment::Center)
                .col_span(2)
                .build()]),
            matrix,
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
