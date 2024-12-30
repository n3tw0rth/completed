cli tool which accepts a list of string arguments and execute them

```bash
$ mybin terraform plan

```

this will run the executable _terraform_ and pass the other params into that executable. while streaming the stdout to the terminal.

once the programs completes (failed/success) based on the exit code send a notification to desktop,email.


users can do some configuration according to their preferences

1. user can config smtp and other clients
2. user can define a template for rich text supported clients




at last planning to add,
1. metrics (time elapsed,cpu/memory usage)



!!! Important
read the stdout and see if the program expects user inputs

user can predefine them by passing a flag
