# `gitmoji` in Rust

![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)
![Continuous integration](https://github.com/actions-rs/toolchain/workflows/Continuous%20integration/badge.svg)

This is just an opinionated version of [`gitmoji-cli`](https://github.com/carloscuesta/gitmoji-cli) written in Rust .

> A [gitmoji](https://gitmoji.dev/) interactive client for using gitmojis on commit messages.

## Install

Pick up the [latest binary](./releases)

Or If you have the [Rust toolchain](https://rustup.rs/) installed you can install with:

## Check you have the dependencies

```shell
# macOS (Homebrew)
$ brew install openssl@3

# macOS (MacPorts)
$ sudo port install openssl

# macOS (pkgsrc)
$ sudo pkgin install openssl

# Arch Linux
$ sudo pacman -S pkg-config openssl

# Debian and Ubuntu
$ sudo apt-get install pkg-config libssl-dev

# Fedora
$ sudo dnf install pkg-config perl-FindBin openssl-devel

# Alpine Linux
$ apk add pkgconfig openssl-dev

# openSUSE
$ sudo zypper in libopenssl-devel
```

[source](https://docs.rs/openssl/latest/openssl/#automatic)

and then `cargo install`it

```shell
cargo install gitmoji-rs
```

## Usage

### `gitmoji help`, `gitmoji --help`

Provide the CLI help.

```shell
❯ gitmoji help
gitmoji-rs 0.1.0
Igor Laborie <ilaborie@gmail.com>
A gitmoji client for using emojis on commit messages

USAGE:
    gitmoji [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -v, --verbose    Verbose mode
    -V, --version    Print version information

SUBCOMMANDS:
    commit    Interactively commit using the prompts
    help      Print this message or the help of the given subcommand(s)
    init      Setup gitmoji preferences
    list      List all available gitmojis
    search    Search gitmojis
    update    Sync emoji list with the repository
```

Note that you can also ask help on a specific sub-command, try `gitmoji help init`

### `gitmoji init`

Interactively initialize the configuration

```shell
❯ gitmoji init
✔ Enable automatic "git add ." · no
✔ Select how emojis should be used in commits · 😄
✔ Enable signed commits · no
✔ Enable scope prompt · no
✔ Set gitmojis api url · https://gitmoji.dev/api/gitmojis
```

Note that you can generate a default configuration without interaction you can use `gitmoji init --default`.

### `gitmoji commit`

Interactively create a git commit

```shell
❯ gitmoji commit
✔ Pick your flavor · 🚀 :rocket: rocket - Deploy stuff.
✔ Enter the commit title · Initial version
✔ Enter the commit message: · Adding require feature for a basic usage (init, update, list, search, and commit)
[main f4525b2] :rocket: Initial version
 12 files changed, 213 insertions(+), 78 deletions(-)
 create mode 100644 .github/workflows/bloat.yaml
 create mode 100644 .github/workflows/lint.yaml
 rename .github/workflows/{ci.yaml => tests.yaml} (51%)
 create mode 100644 CHANGELOG.md
 create mode 100644 README.md
```

Note that it's internally use the `git` command.

You can forward extra flags to `git commit` by appending `--` followed by the flags:

```shell
❯ gitmoji commit -- --no-verify
❯ gitmoji commit -- --no-verify --signoff
```

### `gitmoji update`

Update the gitmojis list based on the provided api url.

```shell
❯ gitmoji update
🎨    :art:   Improve structure / format of the code.
⚡️    :zap:    Improve performance.
🔥    :fire:    Remove code or files.
🐛    :bug:    Fix a bug.
🚑️    :ambulance:    Critical hotfix.
✨    :sparkles:    Introduce new features.
📝    :memo:    Add or update documentation.
🚀    :rocket:    Deploy stuff.
💄    :lipstick:    Add or update the UI and style files.
🎉    :tada:    Begin a project.
...
```

### `gitmoji list`

List available gitmojis.

```shell
❯ gitmoji list
🎨    :art:    Improve structure / format of the code.
⚡️    :zap:    Improve performance.
🔥    :fire:    Remove code or files.
🐛    :bug:    Fix a bug.
🚑️    :ambulance:    Critical hotfix.
✨    :sparkles:    Introduce new features.
📝    :memo:    Add or update documentation.
🚀    :rocket:    Deploy stuff.
💄    :lipstick:    Add or update the UI and style files.
🎉    :tada:    Begin a project.
...
```

### `gitmoji search`

Search a gitmoji

```shell
❯ gitmoji search bug
🐛    :bug:    Fix a bug.
🏗️    :building_construction:    Make architectural changes.
👔    :necktie:    Add or update business logic
```

## Missing features

Due to a bug, we cannot use as a git hook yet.

See [Issue #1](./issues/1), help welcome

## License

This Action is distributed under the terms of the MIT license, see [LICENSE](./LICENSE-MIT) for details.

## Contribute and support

Any contributions are welcomed!
