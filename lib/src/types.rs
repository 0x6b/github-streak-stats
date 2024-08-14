use std::fmt::Display;

use graphql_client::GraphQLQuery;

// Have to define custom type for DateTime and Date as these are not standard type
type DateTime = String;
type Date = String;

/// Struct to hold the response from the contributionsCollection query
/// https://docs.github.com/en/graphql/reference/objects#contributionscollection
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/streak.graphql",
    response_derives = "Debug",
    variables_derives = "Debug"
)]
pub struct StreakQuery;

/// Struct to hold the response from the viewer query
/// https://docs.github.com/en/graphql/reference/queries#viewer
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/viewer.graphql",
    response_derives = "Debug",
    variables_derives = "Debug"
)]
pub struct ViewerQuery;

/// Struct to hold the response from the user query
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/user.graphql",
    response_derives = "Debug",
    variables_derives = "Debug"
)]
pub struct UserQuery;

/// Struct to hold the response from the streak query
#[derive(Debug)]
pub struct Contribution {
    /// Date of the contribution
    pub date: jiff::civil::Date,
    /// Number of contributions on that date
    pub contribution_count: i64,
}

/// Stats of the user
pub struct Stats {
    /// Total contributions
    pub total_contributions: i64,
    /// Longest streak
    pub longest_streak: Streak,
    /// Current streak
    pub current_streak: Streak,
}

pub struct User {
    /// Login name
    pub name: String,
    /// Total number of public repositories owned
    pub public_repositories: i64,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({} public repos)", self.name, self.public_repositories)
    }
}

/// Simple date range
pub struct Streak {
    /// Start date
    pub start: jiff::civil::Date,
    /// End date
    pub end: jiff::civil::Date,
}

/// Converts a tuple of (Date, Date) to Streak
impl From<(jiff::civil::Date, jiff::civil::Date)> for Streak {
    fn from(value: (jiff::civil::Date, jiff::civil::Date)) -> Self {
        Self { start: value.0, end: value.1 }
    }
}
