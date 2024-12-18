use anyhow::{anyhow, Result};
use clap::Parser;
use colorful::Colorful;
use github_streak_stats_lib::{github_client::GitHubClient, types::Stats};
use jiff::{civil::Weekday, fmt::strtime::parse, Span, Zoned};
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    TableBuilder, TableStyle,
};

use crate::args::Args;

mod args;

#[tokio::main]
async fn main() -> Result<()> {
    let Args {
        login,
        github_token,
        from,
        to,
        offset,
        display_public_repositories,
        no_display_matrix,
        theme,
    } = Args::parse();

    let today = Zoned::now();

    let (start, end) = calc_start_and_end(&today, &from, &to, &offset)?;

    let client = GitHubClient::new(
        "https://api.github.com/graphql",
        "github-streaks-stats-lib/0.0.0",
        &github_token,
    );

    let user = match &login {
        None => client.get_viewer().await?,
        Some(login) => client.get_user(login).await?,
    };

    let contributions = client
        .get_contributions(
            &user,
            &start.strftime("%Y-%m-%dT%H:%M:%S.000%z").to_string(),
            &end.strftime("%Y-%m-%dT%H:%M:%S.000%z").to_string(),
        )
        .await?;

    let Stats {
        total_contributions,
        longest_streak,
        current_streak,
    } = client.calc_streak_from_contributions(&contributions)?;

    let mut rows = vec![Row::new(vec![TableCell::builder(format!(
        "🔥 GitHub contribution stats for https://github.com/{} since {} 🔥",
        if display_public_repositories { user.to_string() } else { user.name },
        start.strftime("%Y-%m-%d"),
    ))
        .alignment(Alignment::Center)
        .col_span(2)
        .build()])];

    if !no_display_matrix {
        // find max contribution count
        let max = contributions.iter().map(|day| day.contribution_count).max().unwrap();

        // map the contribution count to a colored cell
        let colors: Vec<String> = contributions
            .iter()
            .map(|contribution| {
                if contribution.date > today.date() {
                    return "".to_string();
                }

                "\u{25A0}"
                    .color(match contribution.contribution_count as f64 / max as f64 {
                        0.0 => theme.as_ref()[0],
                        0.0..=0.25 => theme.as_ref()[1],
                        0.25..=0.5 => theme.as_ref()[2],
                        0.5..=0.75 => theme.as_ref()[3],
                        _ => theme.as_ref()[4],
                    })
                    .to_string()
            })
            .collect::<Vec<_>>();

        // create a matrix of the stats, where each cell is a colored square
        let matrix: Vec<Vec<String>> = colors.chunks(7).map(|week| week.to_vec()).collect();

        // transpose the matrix as the data is displayed at GitHub
        let matrix: Vec<Vec<String>> = (0..7)
            .map(|i| matrix.iter().map(|row| row[i].clone()).collect())
            .collect();

        // join the matrix into a single string
        let matrix = matrix
            .iter()
            .map(|row| row.join(" "))
            .collect::<Vec<String>>()
            .join("\n");

        rows.push(Row::new(vec![TableCell::builder(matrix)
            .alignment(Alignment::Center)
            .col_span(2)
            .build()]));
    }

    rows.push(Row::new(vec![
        TableCell::new("Total contributions"),
        TableCell::builder(total_contributions)
            .alignment(Alignment::Right)
            .col_span(1)
            .build(),
    ]));

    rows.push(Row::new(vec![
        TableCell::new("Longest and latest streak"),
        TableCell::new(format!(
            "{} days, from {} to {}",
            (longest_streak.end - longest_streak.start).get_days() + 1,
            longest_streak.start,
            longest_streak.end,
        )),
    ]));

    rows.push(Row::new(vec![
        TableCell::new("Current streak"),
        TableCell::new(format!(
            "{} days, from {} to {}",
            (current_streak.end - current_streak.start).get_days() + 1,
            current_streak.start,
            current_streak.end,
        )),
    ]));

    let table = TableBuilder::new().style(TableStyle::rounded()).rows(rows).build();
    println!("{}", table.render());

    Ok(())
}

fn calc_start_and_end(
    today: &Zoned,
    from: &Option<String>,
    to: &Option<String>,
    offset: &str,
) -> Result<(Zoned, Zoned)> {
    let parse_date = |date: &str| -> Result<Zoned> {
        parse("%Y-%m-%d%z", format!("{}{}", date, offset))?
            .to_zoned()
            .map_err(Into::into)
    };

    let find_first_sunday_before = |date: &Zoned| -> Result<Zoned> {
        (0..7)
            .flat_map(|i| date.checked_sub(Span::new().days(i)))
            .find(|date| date.weekday() == Weekday::Sunday)
            .ok_or(anyhow!("No Sunday found")) // really?
    };

    let find_first_saturday_after = |date: &Zoned| -> Result<Zoned> {
        (0..7)
            .flat_map(|i| date.checked_add(Span::new().days(i)))
            .find(|date| date.weekday() == Weekday::Saturday)
            .ok_or(anyhow!("No Sunday found")) // really?
    };

    let (start, end) = match (from, to) {
        (Some(from), Some(to)) => (parse_date(from)?, parse_date(to)?),
        (Some(from), None) => {
            let start = find_first_sunday_before(&parse_date(from)?)?;
            let end = find_first_saturday_after(&start.checked_add(Span::new().weeks(52))?)?;
            (start, end)
        }
        (None, Some(to)) => {
            let end = find_first_saturday_after(&parse_date(to)?)?;
            let start = find_first_sunday_before(&end.checked_sub(Span::new().weeks(52))?)?;
            (start, end)
        }
        (None, None) => (
            find_first_sunday_before(&today.checked_sub(Span::new().weeks(52))?)?,
            find_first_saturday_after(today)?,
        ),
    };
    Ok((start, end))
}
