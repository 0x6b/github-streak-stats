use chrono::NaiveDate;
use graphql_client::GraphQLQuery;

// Have to define custom type for DateTime, GitHubResponse, and URI as these are not standard type
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

#[derive(Debug)]
pub struct Contribution {
    pub date: NaiveDate,
    pub contribution_count: i64,
}

pub struct Stats {
    pub total_contributions: i64,
    pub longest_streak: Streak,
    pub current_streak: Streak,
}

pub struct Streak {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

impl From<(NaiveDate, NaiveDate)> for Streak {
    fn from(value: (NaiveDate, NaiveDate)) -> Self {
        Self {
            start: value.0,
            end: value.1,
        }
    }
}