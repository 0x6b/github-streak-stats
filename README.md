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
╭──────────────────────────────────────────────────────────────────────────────────╮
│   🔥 GitHub contribution stats for https://github.com/0x6b since 2023-08-11 🔥   │
├────────────────────────────────────────┬─────────────────────────────────────────┤
│ Total contributions                    │                                    4192 │
├────────────────────────────────────────┼─────────────────────────────────────────┤
│ Longest and latest streak              │ 252 days, from 2023-12-03 to 2024-08-10 │
├────────────────────────────────────────┼─────────────────────────────────────────┤
│ Current streak                         │ 252 days, from 2023-12-03 to 2024-08-10 │
╰────────────────────────────────────────┴─────────────────────────────────────────╯
```

See help for detail.

```
$ github-streak-stats -h
Show GitHub contribution streak. Export `GITHUB_TOKEN` environment variable with your GitHub personal access token which has `read:user` scope.

Usage: github-streak-stats [OPTIONS] --github-token <GITHUB_TOKEN> [LOGIN]

Arguments:
  [LOGIN]  GitHub login name. Defaults to the login name of the GitHub API token owner

Options:
  -g, --github-token <GITHUB_TOKEN>  GitHub personal access token [env: GITHUB_TOKEN=ghp_...]
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
