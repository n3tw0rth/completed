# Completed

A terminal process notification tool that alerts you when long-running commands finish — via desktop notifications, email, or Google Chat.

## Overview

Developers working on multiple tasks in parallel often lose track of long-running processes. `completed` wraps any command and sends a notification when it finishes, regardless of whether it succeeded or failed. It works in both desktop environments and headless environments like CI pipelines and VMs.

## Installation

Clone and install from source:

```bash
git clone https://github.com/n3tw0rth/completed.git
cd completed
./install.sh

completed -h
```

For convenience, add an alias to your shell config:

```bash
# ~/.bashrc or ~/.zshrc
alias notify='completed'
```

## Usage

Wrap any command by passing it directly to `completed`:

```bash
completed ping google.com
```

A notification is sent once the process exits, indicating success or failure.

### Triggers

Use `-t` / `--triggers` to send additional notifications when specific strings appear in stdout:

```bash
completed -t PING ping google.com
```

Multiple trigger strings can be passed as comma-separated values. Phrases with spaces should be quoted:

```bash
completed -t approve,'Enter a value' terraform apply
```

### Profiles

Profiles group notification destinations under a named label. Use `-p` / `--profile` to select one or more profiles:

```bash
completed -p default,work ping google.com
```

Multiple profiles can be specified as comma-separated values. All destinations defined across the selected profiles will receive notifications.

## Configuration

Place your configuration in the appropriate config file. Below is a full example:

```toml
[email.default]
from     = "acme@dev.com"
to       = "me@dev.com"
username = ""
password = ""
port     = 465
host     = ""

[email.work]
from     = "acme@dev.com"
to       = "pm@dev.com"
username = ""
password = ""
port     = 465
host     = ""

[gchat.work]
api_key = ""
webhook = ""

[profiles.default]
sendto = ["desktop", "email.default"]

[profiles.work]
sendto = ["desktop", "gchat.work", "email.work"]
```

## Example Workflow

```bash
alias notify='completed'
alias terraform='notify -t approve,"Enter a value" terraform'
```

With this setup, running `terraform apply` will notify you when Terraform requests input or when the apply completes.

## License

See [LICENSE](LICENSE) for details.
