use nanoid::nanoid;
use uuid::Uuid;

pub enum AppArgs {
    Cuid {
        show_help: bool,
        slug: bool,
    },
    Nanoid {
        show_help: bool,
        length: Option<usize>,
    },
    Uuidv4 {
        show_help: bool,
        urn: bool,
    },
    #[allow(dead_code)]
    Global {
        show_help: bool,
        version: bool,
    },
}

pub fn parse_args() -> Result<AppArgs, Box<dyn std::error::Error>> {
    let mut args = pico_args::Arguments::from_env();

    let app_args = match args.subcommand()?.as_deref() {
        Some("cuid") => Ok(AppArgs::Cuid {
            show_help: args.contains(["-h", "--help"]),
            slug: args.contains("--slug"),
        }),
        Some("nanoid") | Some("nano") => Ok(AppArgs::Nanoid {
            show_help: args.contains(["-h", "--help"]),
            length: args.opt_value_from_str("--length")?,
        }),
        Some("uuidv4") | Some("uuid") => Ok(AppArgs::Uuidv4 {
            show_help: args.contains(["-h", "--help"]),
            urn: args.contains("--urn"),
        }),
        Some(s) => Err(format!(
            "unknown subcommand: {}. Type `getid --help` to see available commands.",
            s
        )
        .into()),
        None => Ok(AppArgs::Global {
            show_help: args.contains(["-h", "--help"]),
            version: args.contains(["-v", "--version"]),
        }),
    };

    }

    app_args
}

pub fn get_cuid(as_slug: bool) -> String {
    let id = if as_slug { cuid::slug() } else { cuid::cuid() };
    id.expect("could not acquire cuid:")
}

pub fn get_nanoid(length: Option<usize>) -> String {
    match length {
        Some(len) => nanoid!(len),
        None => nanoid!(),
    }
}

pub fn get_uuid(as_urn: bool) -> String {
    let uuid = Uuid::new_v4();
    if as_urn {
        uuid.to_urn().to_string()
    } else {
        uuid.to_hyphenated().to_string()
    }
}

pub fn output_or_help(show_help: bool, value: String, help: &str) {
    if show_help {
        println!("{}", help);
    } else {
        println!("{}", value);
    }
}
