# github-streak-stats

Simple CLI to show GitHub contribution streak for specified user.

## Installation

```
$ cargo install --git https://github.com/0x6b/github-streak-stats github-streak-stats-cli
```

## Setup

Export `GITHUB_TOKEN` environment variable with your GitHub personal access token which has `read:user` scope.

## Usage

```console
$ github-streak-stats
╭─────────────────────────────────────────────────────────────────────────────────────────────────╮
│  🔥 GitHub contribution stats for https://github.com/0x6b (49 public repos) since 2022-08-23 🔥 │
├────────────────────────────────────────────────┬────────────────────────────────────────────────┤
│ Total contributions                            │                                           1507 │
├────────────────────────────────────────────────┼────────────────────────────────────────────────┤
│ Longest and latest streak                      │ 123 days, from 2023-04-23 to 2023-08-23        │
├────────────────────────────────────────────────┼────────────────────────────────────────────────┤
│ Current streak                                 │ 123 days, from 2023-04-23 to 2023-08-23        │
╰────────────────────────────────────────────────┴────────────────────────────────────────────────╯
```

See help for detail.

```
$ github-streak-stats -h
Show GitHub contribution streak. Export `GITHUB_TOKEN` environment variable with your GitHub personal access token which has `read:user` scope.

Usage: github-streak-stats [OPTIONS] [LOGIN]

Arguments:
  [LOGIN]  GitHub login name. Defaults to the login name of the GitHub API token owner

Options:
  -f, --from <FROM>                  Start date, in YYYY-MM-DD format. Defaults is 1 year ago from today
  -t, --to <TO>                      End date, in YYYY-MM-DD format. Please note that the total time spanned by 'from' and 'to' must not exceed 1 year. Defaults is today
  -o, --offset <OFFSET>              Offset from UTC, in HH:MM format [default: 09:00]
  -r, --display-public-repositories  Display number of public repositories owned
  -h, --help                         Print help
  -V, --version                      Print version
```

## License

MIT. See [LICENSE](LICENSE) for details.

## Reference

- [Creating a personal access token - GitHub Docs](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token)
