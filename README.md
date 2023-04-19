# mxclear: `$ clear` with fashion

*mxclear* is an alternative to the `clear` command on Linux or the `cls` command
on Windows. Instead of just boringly ereasing the screen - why not wipe the screen
with a matrix flash?

![Example](https://raw.githubusercontent.com/phoenixr-codes/mxclear/master/demo.gif)


## Installation

```console
cargo install --force mxclear
```


## Usage

### Replace `clear` with `mxclear`

If you want to: replace the `clear` command with `mxclear`:

```bash
alias clear=mxclear
```

In order to enable this for each session, add that line to your `~/.bashrc`.
To revert this change, simply remove it from the `~/.bashrc` and use

```bash
unalias clear
```


### The `mxclear` command

```text
`$ clear` with fashion

Usage: mxclear [OPTIONS]

Options:
      --style <style>  Sets the style froma dotted notation [env: MXCLEAR_STYLE=] [default: green]
      --speed <speed>  Sets the intervall between updating a line in milliseconds [env: MXCLEAR_SPEED=] [default: 14]
  -h, --help           Print help
  -V, --version        Print version
```


### Using environment variables

Environment variables can control the style and speed of the "matrix line". Simply set
`MXCLEAR_STYLE` and/or `MXCLEAR_SPEED` with a value that you could as well provide as
a command-line argument.

```bash
export MXCLEAR_STYLE=red.bold  # red and bold text
export MXCLEAR_SPEED=80        # intervall of 80 milliseconds
```


## LICENSE

This project is licensed under either of

* MIT License
  ([LICENSE-MIT](https://github.com/phoenixr-codes/mxclear/blob/master/LICENSE-MIT)
  or https://opensource.org/license/mit/)
* The Unlicense
  ([LICENSE-Unlicense](https://github.com/phoenixr-codes/mxclear/blob/master/LICENSE-Unlicense)
  or https://opensource.org/license/unlicense/)

at your option.



