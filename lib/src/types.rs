use chrono::NaiveDate;
use graphql_client::GraphQLQuery;

// Have to define custom type for DateTime and Date as these are not standard type
type DateTime = String;
type Date = String;

#[derive(GraphQLQuery)]
#[graphql(
schema_path = "graphql/schema.graphql",
query_path = "graphql/streak.graphql",
response_derives = "Debug",
variables_derives = "Debug"
)]
pub struct StreakQuery;

/// Struct to hold the response from the streak query
#[derive(Debug)]
pub struct Contribution {
    pub date: NaiveDate,
    pub contribution_count: i64,
}

/// Stats of the user
pub struct Stats {
    pub total_contributions: i64,
    pub longest_streak: Streak,
    pub current_streak: Streak,
}

/// Simple date range
pub struct Streak {
    pub start: NaiveDate,
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
