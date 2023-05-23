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
$ github-streak-stats 0x6b
🔥 GitHub contribution stats for 0x6b since 2022-05-22 🔥
- Total contributions: 585
- Longest streak: 30 days (2023-04-23–2023-05-22)
- Current streak: 30 days (2023-04-23–2023-05-22)
```

See help for detail.

```
$ github-streak-stats -h
github-streak-stats 0.1.0
Show GitHub contribution streak

USAGE:
    github-streak-stats [OPTIONS] <login>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --from <from>    Start date
    -t, --to <to>        End date. Please note that the total time spanned by 'from' and 'to' must not exceed 1 year

ARGS:
    <login>    GitHub login name
```

## License

MIT. See [LICENSE](LICENSE) for details.

## Reference

- [Creating a personal access token - GitHub Docs](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token)