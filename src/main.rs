use getid::{
    args::{parse_args, AppArgs},
    ids::{get_cuid, get_hostname, get_nanoid, get_uuid},
};

fn main() {
    match parse_args() {
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }

        Ok(args) => match args {
            AppArgs::Cuid { show_help, slug } => {
                output_or_help(show_help, get_cuid(slug), HELP_CUID);
            }

            AppArgs::Nanoid { show_help, length } => {
                output_or_help(show_help, get_nanoid(length), HELP_NANOID);
            }

            AppArgs::Uuidv4 {
                show_help,
                urn,
                simple,
            } => {
                if urn && simple {
                    eprintln!("warning: '--urn' and '--simple' are mutually exclusive, ignoring '--simple'.");
                }
                output_or_help(show_help, get_uuid(urn, simple), HELP_UUIDV4);
            }

            AppArgs::Hostname {
                show_help,
                token_length,
            } => {
                output_or_help(show_help, get_hostname(token_length), HELP_HOSTNAME);
            }

            AppArgs::Global {
                show_help: _,
                version,
            } => {
                output_or_help(!version, env!("CARGO_PKG_VERSION").to_string(), HELP);
            }
        },
    }
}

pub fn output_or_help(show_help: bool, value: String, help: &str) {
    if show_help {
        println!("{}", help);
    } else {
        println!("{}", value);
    }
}

const HELP: &str = "
getid -- Generate a random { cuid, nanoid, uuidv4 }.

Usage:
  getid [--help, --version]
  getid <command> [<flags>...]

Commands:
  cuid           Generate a random cuid.
  hostname       Generate a random, heroku-like hostname. [alias: heroku]
  nanoid         Generate a random nanoid. [alias: nano]
  uuidv4         Generate a random uuidv4. [alias: uuid]

Options:
  -v, --version  Show program version.
  -h, --help     Show this help again.

Examples:
  getid cuid
  getid hostname
  getid nanoid
  getid uuidv4

Type 'getid <command> --help' for information on a specific command.
";

const HELP_CUID: &str = "
getid cuid -- Generate a random cuid.

Usage:
  getid cuid [--slug]

Options:
  --slug      Generate a smaller id (7-10 characters) intended for short urls.
  -h, --help  Show this help again.

For more information on the 'cuid' format, see: https://github.com/ericelliott/cuid.
";

const HELP_HOSTNAME: &str = "
getid hostname -- Generate a random, heroku-like hostname.

Usage:
  getid hostname [--token_length <len>]

Options:
  --token_length <len>  Length of the appended token. [default: 4]
  -h, --help            Show this help again.

Aliases:
  getid heroku
";

const HELP_NANOID: &str = "
getid nanoid -- Generate a random nanoid.

Usage:
  getid nanoid [--length <len>]

Options:
  --length <len>  Length of generated id. [default: 21]
  -h, --help      Show this help again.

Aliases:
  getid nano

For more information on the 'nanoid' format, see: https://zelark.github.io/nano-id-cc/.
";

const HELP_UUIDV4: &str = "
getid uuidv4 -- Generate a random uuidv4.

Usage:
  getid uuidv4 [--urn]

Options:
  --urn       Format the generated id as 'urn'.
  --simple    Format the generated id without hyphens.
  -h, --help  Show this help again.

Aliases:
  getid uuid

For more information on the 'uuidv4' format, see: https://en.wikipedia.org/wiki/Universally_unique_identifier#Version_4_(random).
";
