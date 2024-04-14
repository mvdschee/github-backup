# github-backup
Create a backup of all your GitHub repositories, just in case Microsoft does a Gitlab rm -rf


```bash
Program to back up all repositories from GitHub.

Usage: github-backup [OPTIONS] --token <TOKEN>

Options:
  -t, --token <TOKEN>    Your personal token from GitHub
  -o, --output <OUTPUT>  Where to save the repositories [default: ./backup]
  -h, --help             Print help
  -V, --version          Print version
```

Optional you can copy over the `.env-example` to `.env` and fill in your personal token.

## Installation

1. Get a personal token from GitHub: [Settings > Developer Settings > Personal access tokens](https://github.com/settings/tokens?type=beta)




