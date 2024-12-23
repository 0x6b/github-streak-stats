use std::env;

use anyhow::{anyhow, Result};
use graphql_client::{reqwest::post_graphql, GraphQLQuery, Response};
use jiff::{civil::Date, fmt::strtime::parse, Span};
use reqwest::{
    header::{HeaderValue, AUTHORIZATION},
    Client,
};

use crate::types::{
    streak_query, user_query, viewer_query, Contribution, Stats, StreakQuery, User, UserQuery,
    ViewerQuery,
};

/// Simple GitHub client
#[derive(Debug)]
pub struct GitHubClient {
    endpoint: String,
    client: Client,
}

impl Default for GitHubClient {
    fn default() -> Self {
        let token = env::var("GITHUB_TOKEN")
            .expect("Specify GitHub API token with GITHUB_TOKEN environment variable");
        Self::new("https://api.github.com/graphql", "github-streaks-stats-lib/0.0.0", &token)
    }
}

impl GitHubClient {
    /// Create a new instance
    ///
    /// # Arguments
    ///
    /// - `endpoint` - GitHub GraphQL API endpoint
    /// - `user_agent` - User agent string
    /// - `token` - GitHub API token
    pub fn new(endpoint: &str, user_agent: &str, token: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            client: Client::builder()
                .user_agent(user_agent)
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

    /// Calculate streak stats for a given user
    pub async fn calc_streak(&self, login: &User, from: &str, to: &str) -> Result<Stats> {
        let contribution_days = self.get_contributions(login, from, to).await?;
        self.calc_streak_from_contributions(&contribution_days)
    }

    pub fn calc_streak_from_contributions(&self, contributions: &[Contribution]) -> Result<Stats> {
        let mut longest_streak = 0;
        let mut current_streak = 0;
        let mut longest_streak_start = Date::MIN;
        let mut longest_streak_end = Date::MIN;
        let mut current_streak_start = Date::MIN;
        let mut current_streak_end = Date::MIN;
        let mut total_contributions = 0;

        for c in contributions.iter() {
            if c.contribution_count > 0 {
                total_contributions += c.contribution_count;
                current_streak += 1;
                if current_streak >= longest_streak {
                    longest_streak = current_streak;
                    longest_streak_end = c.date;
                    longest_streak_start =
                        c.date.checked_sub(Span::new().days(current_streak - 1))?;
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

    /// Get contributions for a given user
    pub async fn get_contributions(
        &self,
        user: &User,
        from: &str,
        to: &str,
    ) -> Result<Vec<Contribution>> {
        let response = self
            .request::<StreakQuery>(streak_query::Variables {
                login: user.name.to_string(),
                from: Some(from.to_string()),
                to: Some(to.to_string()),
            })
            .await?;

        let contribution_days = response
            .data
            .ok_or(anyhow!("No data"))?
            .user
            .ok_or(anyhow!("No user"))?
            .contributions_collection
            .contribution_calendar
            .weeks
            .into_iter()
            .flat_map(|week| week.contribution_days)
            .map(|day| Contribution {
                date: parse("%Y-%m-%d%z", format!("{}+0000", &day.date))
                    .unwrap()
                    .to_zoned()
                    .unwrap()
                    .date(),
                contribution_count: day.contribution_count,
            })
            .collect::<Vec<_>>();

        Ok(contribution_days)
    }

    pub async fn get_user(&self, login: &str) -> Result<User> {
        let response = self
            .request::<UserQuery>(user_query::Variables { login: login.to_string() })
            .await?
            .data
            .ok_or(anyhow!("No login information. Check your GitHub API token."))?
            .user
            .ok_or(anyhow!("No such user"))?;

        let login = response.login;
        let public_repositories = response.repositories.total_count;
        Ok(User { name: login, public_repositories })
    }

    /// Get login name of the GitHub API token owner
    pub async fn get_viewer(&self) -> Result<User> {
        let response = self
            .request::<ViewerQuery>(viewer_query::Variables {})
            .await?
            .data
            .ok_or(anyhow!("No login information. Check your GitHub API token."))?;
        let login = response.viewer.login;
        let public_repositories = response.viewer.repositories.total_count;
        Ok(User { name: login, public_repositories })
    }

    // Simple helper function to make a request
    async fn request<T: GraphQLQuery>(
        &self,
        variables: T::Variables,
    ) -> Result<Response<T::ResponseData>> {
        Ok(post_graphql::<T, _>(&self.client, &self.endpoint, variables).await?)
    }
}

#[cfg(test)]
mod test {
    use crate::github_client::GitHubClient;

    #[tokio::test]
    async fn get_viewer() {
        let client = GitHubClient::default();
        let user = client.get_viewer().await.unwrap();
        println!("{user}");
    }
}
