use nanoid::nanoid;
use uuid::Uuid;

enum AppArgs {
    Cuid {
        help: bool,
        slug: bool,
    },
    Nanoid {
        help: bool,
        length: Option<usize>,
    },
    Uuidv4 {
        help: bool,
        urn: bool,
    },
    #[allow(dead_code)]
    Global {
        help: bool,
        version: bool,
    },
}

fn main() {
    match parse_args() {
        Err(err) => {
            eprintln!("Error: {}", err);
        }

        Ok(args) => match args {
            AppArgs::Cuid { help, slug } => {
                if help {
                    println!("{}", HELP_CUID);
                } else {
                    let id = if slug { cuid::slug() } else { cuid::cuid() };
                    println!("{}", id.expect("could not acquire cuid:"));
                }
            }

            AppArgs::Nanoid { help, length } => {
                if help {
                    println!("{}", HELP_NANOID);
                } else {
                    let id = match length {
                        Some(len) => nanoid!(len),
                        None => nanoid!(),
                    };
                    println!("{}", id);
                }
            }

            AppArgs::Uuidv4 { help, urn } => {
                if help {
                    println!("{}", HELP_UUIDV4);
                } else {
                    let uuid = Uuid::new_v4();
                    let formatted = if urn {
                        uuid.to_urn().to_string()
                    } else {
                        uuid.to_hyphenated().to_string()
                    };
                    println!("{}", formatted);
                }
            }

            AppArgs::Global { help: _, version } => {
                if version {
                    println!("{}", env!("CARGO_PKG_VERSION"));
                } else {
                    // print a default format if not `--help`?
                    println!("{}", HELP);
                }
            }
        },
    }
}

fn parse_args() -> Result<AppArgs, Box<dyn std::error::Error>> {
    let mut args = pico_args::Arguments::from_env();

    match args.subcommand()?.as_deref() {
        Some("cuid") => Ok(AppArgs::Cuid {
            help: args.contains(["-h", "--help"]),
            slug: args.contains("--slug"),
        }),
        Some("nanoid") | Some("nano") => Ok(AppArgs::Nanoid {
            help: args.contains(["-h", "--help"]),
            length: args.opt_value_from_str("--length")?,
        }),
        Some("uuidv4") | Some("uuid") => Ok(AppArgs::Uuidv4 {
            help: args.contains(["-h", "--help"]),
            urn: args.contains("--urn"),
        }),
        Some(s) => Err(format!(
            "unknown subcommand: {}. Type `getid --help` to see available commands.",
            s
        )
        .into()),
        None => Ok(AppArgs::Global {
            help: args.contains(["-h", "--help"]),
            version: args.contains(["-v", "--version"]),
        }),
    }
}

const HELP: &str = "
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

const HELP_NANOID: &str = "
getid nanoid -- Generate a random nanoid. Alias: 'nano'.

Usage:
  getid nanoid [--length <len>]

Options:
  --length <len>  Length of generated id. [default: 21]
  -h, --help      Show this help again.
  
For more information on the 'nanoid' format, see: https://zelark.github.io/nano-id-cc/.
";

const HELP_UUIDV4: &str = "
getid uuidv4 -- Generate a random uuidv4. Alias: 'uuid'.

Usage:
  getid uuidv4 [--urn]

Options:
  --urn       Format the generated id as 'urn'.
  -h, --help  Show this help again.

For more information on the 'uuidv4' format, see: https://en.wikipedia.org/wiki/Universally_unique_identifier#Version_4_(random).
";
