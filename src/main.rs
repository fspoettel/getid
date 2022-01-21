fn main() {
    match getid::parse_args() {
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }

        Ok(args) => match args {
            getid::AppArgs::Cuid { show_help, slug } => {
                getid::output_or_help(show_help, getid::get_cuid(slug), HELP_CUID);
            }
            getid::AppArgs::Uuidv4 { show_help, urn, simple } => {
                if urn && simple {
                    eprintln!("warning: '--urn' and '--simple' are mutually exclusive, ignoring '--simple'.");
                }
                getid::output_or_help(show_help, getid::get_uuid(urn, simple), HELP_UUIDV4);
            }
            getid::AppArgs::Nanoid { show_help, length } => {
                getid::output_or_help(show_help, getid::get_nanoid(length), HELP_NANOID);
            }
            getid::AppArgs::Global {
                show_help: _,
                version,
            } => {
                // print a default format if not `--help`?
                getid::output_or_help(!version, env!("CARGO_PKG_VERSION").to_string(), HELP);
            }
        },
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
  --simple    Format the generated id without hyphens.
  -h, --help  Show this help again.

For more information on the 'uuidv4' format, see: https://en.wikipedia.org/wiki/Universally_unique_identifier#Version_4_(random).
";
