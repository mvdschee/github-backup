# github-backup

Create a backup of all your GitHub repositories, just in case Microsoft does a Gitlab rm -rf

```bash
Program to back up all repositories from GitHub.

Usage: github-backup [OPTIONS]

Options:
  -t, --token <TOKEN>    Your personal token from GitHub
  -o, --output <OUTPUT>  Where to save the repositories [default: ./backup]
      --org <ORG>        Organisation(s) to back up (repeatable: --org foo --org bar)
      --personal         Also back up personal repositories when --org is specified
  -h, --help             Print help
  -V, --version          Print version
```

Optional you can copy over the `.env-example` to `.env` and fill in your personal token.

By default, personal repositories are backed up. When `--org` is provided, only org repositories are backed up unless `--personal` is also passed.

## Token scopes

To back up private org repositories your token needs:

- `repo` — full repository access
- `read:org` — org membership and repo visibility

If the org enforces SAML SSO, the token also needs to be explicitly authorized for that org on the token settings page.

## Installation

1. Get a personal token from GitHub: [Settings > Developer Settings > Personal access tokens](https://github.com/settings/tokens)
