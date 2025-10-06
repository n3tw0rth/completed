<img width="1001" height="400" alt="Completed" src="https://github.com/user-attachments/assets/0057cc6f-9c18-4305-b21d-a5e9172bdda0" />



A simple notifier to send notifications on terminal process events.

### Why

When working on multiple in parallel developers get distract a lot. It is really annoying when you forget attention seeking processes and that waste a lot of time. But what if we there is a tool for this exact purpose. This project started to show desktop notifications but soon realize that we can use the same tool in environment where there is no desktop environment (eg: pipelines, VMs etc). 

This is easy to use just add it in your scripts or wherever you want

### Installation

Install directly from source

```bash
  $ git clone https://github.com/n3tw0rth/completed.git
  $ cd completed
  $ ./install.sh
  
  $ completed -h
```

it is better to use a alias for the binary,
```bash
#.bashrc
alias notify='completed'
```
### Usage

It is easy as passing the command directly, by default a notification will be send once the program completes execution success or failed.

```shell
$ completed ping google.com 
```

how i use it 🫢

```shell
alias notify='completion-notifier'
alias terraform='notify -t approve,Enter terraform'
```

### Profiles
Profiles can used to group notification clients by a specific name.

```toml
[profiles.default]
sendto = ["desktop","email.default"]

[profiles.work]
sendto = ["desktop","gchat.work","email.work"]

```

Profiles can be passed using the flags `-p` or `--profile`. Similar to triggers you can pass multiple profiles by passing comma seperated profile names. All the destinations defined in the profiles will recieve notifications.

```shell
$ completed -p default,work ping google.com 
```

### Triggers
Triggers can be added to send custom notifications based on the requirement. for example,

```shell
$ completed -t PING ping google.com 
```
Program will start running as usual, and will send additionals notifications before process exit. But based on the string values passed into the `-t` flag `completed` will send additional notifications with the message `Trigger invoked <trigger>`. According to this example a notifications will be send when program find a specific line in stdout contain the word `PING`.

```shell
$ completed -t approve,'Enter a value' terraform apply
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
