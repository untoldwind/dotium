# dotium
![Crates.io](https://img.shields.io/crates/v/dotium)
[![Build](https://github.com/untoldwind/dotium/actions/workflows/build.yml/badge.svg)](https://github.com/untoldwind/dotium/actions/workflows/build.yml)

Dot-file management for multiple machines or small teams.

## Prerequisites

* Some shared (private) repository folder like:
  * Private github/gitlab/bitbucket repository (preferred to have better change control)
  * Any sort of file sync: dropbox, owncloud, google-drive, one-drive, ...
* Rust (optional, but recommended)

## Installation

With `rust` available:
```sh
cargo install dotium
```

Alternatively there is a statically compiled binary on the [Releases page](https://github.com/untoldwind/dotium/releases). Ensure that the provided checksum checks out before using it:
```sh
gunzip dotium.gz
sha256sum -c dotium.sha256sum
chmod 755 dotium
```

### Initialization

To create a basic configuration in `~/.config/dotium` with an [age](https://age-encryption.org)-compatible private key to protect sensitive information.
```
dotium init
```

### Shell completions

Fish:
```sh
dotium completions fish >.config/fish/completions/dotium.fish
```

## Basic concepts

* Any number of dot-files can be created/updated from a dotium-repository
* A dotium-repository is just a folder containing that is supposed to be shared between machines and/or team mates
* The contents in the dotium-repository can be either plain text or age-compatible encrypted
  * Therefore a dotium-repository has the concept of a "recipient" (aka user/machine with the necessary keys to decrypt the content)
  * Initially the only recipient is the person creating the repository.
  * A new user/machine has to create a recipient-request which then has to be approved by anyone who is already a recipient (i.e. has fully access to the repository)

## Usage

### Initialize a brand new repository

```sh
cd <repository folder>
dotium init
```

or

```sh
dotium --repository <repository folder> init-repo
```

... tbd ...

