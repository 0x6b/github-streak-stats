use chrono::NaiveDate;
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

/// Struct to hold the response from the streak query
#[derive(Debug)]
pub struct Contribution {
    /// Date of the contribution
    pub date: NaiveDate,
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

/// Simple date range
pub struct Streak {
    /// Start date
    pub start: NaiveDate,
    /// End date
    pub end: NaiveDate,
}

/// Converts a tuple of (NaiveDate, NaiveDate) to Streak
impl From<(NaiveDate, NaiveDate)> for Streak {
    fn from(value: (NaiveDate, NaiveDate)) -> Self {
        Self {
            start: value.0,
            end: value.1,
        }
    }
}
