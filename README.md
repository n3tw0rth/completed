# Completion Notifier

A simple notifier to send notifications on terminal process events.


## Installation

Install directly from source

```bash
  $ git clone https://github.com/n3tw0rth/completion-notifier.git
  $ cd completion-notifier
  $ ./install.sh
  
  $ completion-notifier -h
```

it is better to use a alias for the binary,
```bash
#.bashrc
alias notify='completion-notifier'
```
## Usage

It is easy as passing the command directly, by default a notification will be send once the program completes execution success or failed.

```shell
$ completion-notifier ping google.com 
```
### Profiles
Profiles can used to group notification clients by a specific name.

```toml
[profiles.default]
sendto = ["desktop","email.default"]

[profiles.work]
sendto = ["desktop","gchat.work","email.work"]

```

### Triggers
Triggers can be added to send custom notifications based on the requirement. for example,

```shell
$ completion-notifier -t PING ping google.com 
```
program will start running as usual, and will send additionals notifications before process exit. Based on the string values passed into the `-t` flag. According to this example a notification will be send when program find a specific line contain the word `PING`.

```shell
$ completion-notifier -t approve,'Enter a value' terraform apply
```
flag `-t` will accept comma seperated values, and trigger values containing more words seperated by spaces they can be passed in as shown. In this example user will be notified when there undefined variables and when terraform ask for user confirmation to apply the change.
## Configuration

```toml
[email.default]
from = "acme@dev.com"
to = "me@dev.com"
username = ""
password = ""
port = 465
host = ""

[email.work]
from = "acme@dev.com"
to = "pm@dev.com"
username = ""
password = ""
port = 465
host = ""

[profiles.default]
sendto = ["desktop","email.default"]

[profiles.work]
sendto = ["desktop","gchat.work","email.work"]

[gchat.work]
api_key = ""
webhook = ""

```
