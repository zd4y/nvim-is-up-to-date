# nvim-is-up-to-date

Checks if neovim is up to date with the latest nightly pre-release in github

## Usage

`nvim-is-up-to-date`

The exit code is 0 if up to date and 1 otherwise.
So you can use it, for example, in `sh` like this:

```sh
if nvim-is-up-to-date
then
  echo Neovim is up to date
else
  echo There is a new version of neovim
fi
```

## Installation

`cargo install --path .`
