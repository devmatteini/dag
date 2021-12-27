# DAG - Download Asset from GitHub releases

[![CI](https://github.com/devmatteini/dag/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/devmatteini/dag/actions/workflows/ci.yml)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/devmatteini/dag)

Download an asset from the latest GitHub release.

[Why should I use dag?](#why-should-i-use-dag) •
[Installation](#installation) •
[Usage](#usage) •
[License](#license)

![dag demo](./assets/demo.gif)

## Why should I use dag?
You can do everything `dag` does with the official [GitHub cli](https://cli.github.com/).

`dag` helps you download release assets more easily:
- no authentication for public repository (you cannot use `gh` without authentication)
- [Built-in generation of pattern](#non-interactive) to select an asset to download
  (with `gh` you need to provide [glob pattern](https://cli.github.com/manual/gh_release_download) that you need to create manually).

## Installation

### Recommended

Download from the [latest release](https://github.com/devmatteini/dag/releases/latest).

### From source

```bash
git clone https://github.com/devmatteini/dag && cd dag
make release
./target/release/dag --version
```

### From AUR

Arch Linux users can install [dra](https://aur.archlinux.org/packages/?O=0&SeB=nd&K=download+assets+from+GitHub+release&outdated=&SB=n&SO=a&PP=50&do_Search=Go) from the AUR using an [AUR helper](https://wiki.archlinux.org/index.php/AUR_helpers). For example:

```bash
paru -S dra
```

## Usage

### Interactive

Select and download an asset from a repository

```
$ dag devmatteini/dag-example download
```

Select and download an asset to custom path

```
$ dag devmatteini/dag-example download --output /tmp/dag-example
```

### Non-Interactive

This mode is useful to be used in automated scripts.

First you need to generate an untagged asset name:

```
$ dag devmatteini/dag-example untag
dag-example-{tag}-amd64
```

Copy the output and run:

```
$ dag devmatteini/dag-example download --select "dag-example-{tag}-amd64"
```

This last command can be used in automated scripts without human interaction.

---

For more information on args/flags/options/commands run:

```bash
$ dag --help
$ dag <command> --help
```

## License

`dag` is made available under the terms of the MIT License.

See the [LICENSE](LICENSE) file for license details.