# I Forgot
## Recall obscure commands by keyword

_(Not tested on Linux/Windows)_

![Example GIF](https://i.imgur.com/7Y2wEqs.gif)

---

## Installation

```sh
sudo curl -L "https://github.com/akmin04/iforgot/releases/download/v0.1.1/iforgot" -o /usr/local/bin/iforgot && sudo chmod +x /usr/local/bin/iforgot
```

---

## Examples
- `$ iforgot howto update ruby`
- `$ iforgot list`
- `$ iforgot new -c 'git reset --hard head' -k git delete reset`
- `$ iforgot delete --all`

---

    USAGE:
        iforgot [FLAGS] <SUBCOMMAND>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information
        -v, --verbose    Enable verbose logging

    SUBCOMMANDS:
        delete    Delete a specified entry by ID
        help      Prints this message or the help of the given subcommand(s)
        howto     Get a list of commands with matching keywords
        list      List all saved entries
        new       Create a new entry