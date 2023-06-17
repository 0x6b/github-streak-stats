use std::{env, error::Error};

use chrono::NaiveDate;
use graphql_client::{
    {GraphQLQuery, Response},
    reqwest::post_graphql_blocking,
};
use reqwest::{
    blocking::Client,
    header::{AUTHORIZATION, HeaderValue},
};

use crate::types::{Contribution, Stats, streak_query, StreakQuery, viewer_query, ViewerQuery};

/// Simple GitHub client
#[derive(Debug)]
pub struct GitHubClient {
    endpoint: String,
    client: Client,
}

// for my own use case, only default implementation is needed at this moment
impl Default for GitHubClient {
    fn default() -> Self {
        let token = env::var("GITHUB_TOKEN")
            .expect("Specify GitHub API token with GITHUB_TOKEN environment variable");
        Self {
            endpoint: "https://api.github.com/graphql".to_string(),
            client: Client::builder()
                .user_agent("github-streaks-stats-lib/0.0.0")
                .default_headers(
                    std::iter::once((
                        AUTHORIZATION,
                        HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
                    ))
                        .collect(),
                )
                .build()
                .unwrap(),
        }
    }
}

impl GitHubClient {
    /// Calculate streak stats for a given user
    pub fn calc_streak(&self, login: &str, from: &str, to: &str) -> Result<Stats, Box<dyn Error>> {
        let contribution_days = self.get_streak(login, from, to).unwrap();
        let mut longest_streak = 0;
        let mut current_streak = 0;
        let mut longest_streak_start = NaiveDate::MIN;
        let mut longest_streak_end = NaiveDate::MIN;
        let mut current_streak_start = NaiveDate::MIN;
        let mut current_streak_end = NaiveDate::MIN;
        let mut total_contributions = 0;

        for c in contribution_days.iter() {
            if c.contribution_count > 0 {
                total_contributions += c.contribution_count;
                current_streak += 1;
                if current_streak >= longest_streak {
                    longest_streak = current_streak;
                    longest_streak_end = c.date;
                    longest_streak_start = c.date - chrono::Duration::days(current_streak - 1);
                }
                if current_streak == 1 {
                    current_streak_start = c.date;
                }
                current_streak_end = c.date;
            } else {
                current_streak = 0;
            }
        }

        Ok(Stats {
            total_contributions,
            longest_streak: (longest_streak_start, longest_streak_end).into(),
            current_streak: (current_streak_start, current_streak_end).into(),
        })
    }

    /// Get streak for a given user
    pub fn get_streak(
        &self,
        login: &str,
        from: &str,
        to: &str,
    ) -> Result<Vec<Contribution>, Box<dyn Error>> {
        let response = self.request::<StreakQuery>(streak_query::Variables {
            login: login.to_string(),
            from: Some(from.to_string()),
            to: Some(to.to_string()),
        })?;

        let contribution_days = response
            .data
            .ok_or("No data")?
            .user
            .ok_or("No user")?
            .contributions_collection
            .contribution_calendar
            .weeks
            .into_iter()
            .flat_map(|week| week.contribution_days)
            .map(|day| Contribution {
                date: NaiveDate::parse_from_str(&day.date, "%Y-%m-%d").unwrap(),
                contribution_count: day.contribution_count,
            })
            .collect::<Vec<_>>();

        Ok(contribution_days)
    }

    /// Get login name of the GitHub API token owner
    pub fn get_viewer(&self) -> Result<String, Box<dyn Error>> {
        let response = self.request::<ViewerQuery>(viewer_query::Variables {})?;
        Ok(response.data.ok_or("No login information. Check your GitHub API token.")?.viewer.login)
    }

    // Simple helper function to make a request
    fn request<T: GraphQLQuery>(
        &self,
        variables: T::Variables,
    ) -> Result<Response<T::ResponseData>, Box<dyn Error>> {
        Ok(post_graphql_blocking::<T, _>(
            &self.client,
            &self.endpoint,
            variables,
        )?)
    }
}

#[cfg(test)]
mod test {
    use crate::github_client::GitHubClient;

    #[test]
    fn get_viewer() {
        let client = GitHubClient::default();
        let viewer = client.get_viewer().unwrap();
        println!("{}", viewer);
    }
}
