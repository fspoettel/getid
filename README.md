# getid

> A command-line tool to generate random { [cuid](https://github.com/ericelliott/cuid), [nanoid](https://zelark.github.io/nano-id-cc/), [uuidv4](https://en.wikipedia.org/wiki/Universally_unique_identifier#Version_4_(random)) } ids.

## Install

- Install a recent version of Rust using [rustup](https://rustup.rs/) or update
it with:
```
# rustup update
```

- Install `get-id` with cargo:
```
# cargo install getid
```

## Usage

```sh
# getid --help

getid -- Generate a random { cuid, nanoid, uuidv4 }.

Usage:
  getid <command> [<flags>...]
  getid [--help, --version]
  getid cuid
  getid nanoid
  getid uuidv4

Commands:
  cuid           Generate a random cuid.
  nanoid         Generate a random nanoid. [alias: nano]
  uuidv4         Generate a random uuidv4. [alias: uuid]

Options:
  -v, --version  Show program version.
  -h, --help     Show this help again.

Type 'getid <command> --help' for information on a specific command.
```

## Formats

### cuid

```sh
# getid cuid --help

getid cuid -- Generate a random cuid.

Usage:
  getid cuid [--slug]

Options:
  --slug      Generate a smaller id (7-10 characters) intended for short urls.
  -h, --help  Show this help again.
  
For more information on the 'cuid' format, see: https://github.com/ericelliott/cuid.
```

### nanoid

```sh
# getid nanoid --help

getid nanoid -- Generate a random nanoid. Alias: 'nano'.

Usage:
  getid nanoid [--length <len>]

Options:
  --length <len>  Length of generated id. [default: 21]
  -h, --help      Show this help again.
  
For more information on the 'nanoid' format, see: https://zelark.github.io/nano-id-cc/.
```

### uuidv4

```sh
# getid uuidv4 --help

getid uuidv4 -- Generate a random uuidv4. Alias: 'uuid'.

Usage:
  getid uuidv4 [--urn]

Options:
  --urn       Format the generated id as 'urn'.
  -h, --help  Show this help again.

For more information on the 'uuidv4' format, see: https://en.wikipedia.org/wiki/Universally_unique_identifier#Version_4_(random).
```
