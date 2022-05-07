# dotium

Dot-file management for multiple machines or small teams.

## Prerequisites

* Some shared (private) repository folder like:
  * Private github/gitlab/bitbucket repository (preferred to have better change control)
  * Any sort of file sync: dropbox, owncloud, google-drive, one-drive, ...
* Rust (obviously)

## Installation

```sh
cargo install dotium
```

To create a basic configuration in `~/.config/dotium` with an [age](https://age-encryption.org)-compatible private key to protect sensitive information.
```
dotium init
```

... tbd ...

