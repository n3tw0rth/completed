<div align="center">

# hark

**Get notified when your long-running commands finish.**

[![CI](https://github.com/n3tw0rth/hark/actions/workflows/ci.yml/badge.svg)](https://github.com/n3tw0rth/hark/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/built_with-Rust-orange.svg)](https://www.rust-lang.org/)

</div>

`hark` wraps any command, streams its output, and sends a notification when it exits or the moment a trigger string appears. Works on the desktop and in headless environments like CI runners and VMs.

```bash
hark cargo build --release
```

## Features

- **Desktop, email, and Google Chat** notifications
- **Triggers**: notify when a string appears in stdout *or* stderr (e.g. a prompt waiting for input)
- **Profiles**: group destinations and fan out to several at once
- **Transparent**: output streams through untouched and the child's exit code is preserved, so it's safe inside scripts and pipelines

## Install

```bash
git clone https://github.com/n3tw0rth/hark.git
cd hark
./install.sh
```

## Usage

```bash
hark [OPTIONS] <COMMAND>...
```

| Option | Description |
| --- | --- |
| `-t, --triggers <A,B>` | Notify when a string appears in the output |
| `-p, --profiles <A,B>` | Destination profiles to notify (default: `default`) |
| `-n, --name <NAME>` | Label prepended to notification titles |

```bash
# notify when done
hark ping -c 5 google.com

# notify when terraform asks for input, quote phrases with spaces
hark -t approve,'Enter a value' terraform apply

# fan out to multiple profiles, labelled per run
hark -p default,work -n api-build make release
```

Handy as an alias:

```bash
alias notify='hark'
alias terraform='notify -t approve,"Enter a value" terraform'
```

## Configuration

A default config is created at `~/.config/hark/config.toml` on first run. Profiles map to one or more destinations: `desktop`, `email.<name>`, or `gchat.<name>`.

```toml
[profiles.default]
sendto = ["desktop"]

[profiles.work]
sendto = ["desktop", "email.work", "gchat.work"]

[email.work]
from     = "ci@dev.com"
to       = "me@dev.com"
username = ""
password = ""
host     = "smtp.dev.com"
port     = 465

[gchat.work]
webhook = "https://chat.googleapis.com/v1/spaces/..."
```

## License

[Apache-2.0](LICENSE)
