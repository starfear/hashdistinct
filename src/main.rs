extern crate clap;

use {
    colored::{Colorize},

    std::{
        collections::{HashMap, BTreeMap},
        fs::{File},
        io::{Result, BufReader, Read}
    },

    clap::{Arg, App},

    ring::{digest::{Context, Digest, SHA256, SHA384, SHA512, SHA512_256}},
};

fn status(silent: bool, msg: &str) {
    if silent == false {
        println!("{} {}", "STATUS".green(), msg);
    }
}

fn info(silent: bool, msg: &str) {
    if silent == false {
        println!("{} {}", "INFO".blue(), msg);
    }
}

fn error(silent: bool, msg: &str) {
    if silent == false {
        println!("{} {}", "ERROR".red(), msg);
    }
}

fn main() -> Result<()> {
    let matches = App::new("Distinct Hash")
                    .version("0.3.3")
                    .author("Starfear https://github.com/starfear")
                    .about("Utility for deletion duplications with same hash.")
                    .arg(Arg::with_name("silent")
                        .help("silent mode")
                        .long("silent")
                        .short("s"))
                    .arg(Arg::with_name("targets")
                        .help("targets")
                        .required(true)
                        .index(1)
                        .multiple(true)
                    )
                    .arg(Arg::with_name("algorithm")
                        .help("hash algorithm. Supported algorithms: [SHA256, SHA384, SHA512, SHA512_256]")
                        .short("a")
                        .long("algorithm")
                        .takes_value(true))
                    .get_matches();

    let silent = matches.is_present("silent");
    let alg = matches.value_of("algorithm").unwrap_or("SHA256").to_uppercase();
    let algorithm = match alg.as_str() {
        "SHA256"     => &SHA256,
        "SHA384"     => &SHA384,
        "SHA512"     => &SHA512,
        "SHA512_256" => &SHA512_256,

        _ => {
            eprintln!("{} Hash algorithm '{}' is not supported.\nSupported algorithms: [SHA256, SHA384, SHA512, SHA512_256]", "ERROR".red(), alg.red());
            std::process::exit(1);
        }
    };

    // collect meta data
    let targets = matches.values_of("targets").unwrap();
    status(silent, "Collecting metadata (size) . . .");

    let mut sizes: BTreeMap<u64, Vec<&str>> = BTreeMap::new();
    for target in targets {
        let meta = std::fs::metadata(target)?;

        match sizes.get_mut(&meta.len()) {
            Some(v) => v.push(target),
            None    => { sizes.insert(meta.len(), vec![target]); }
        };
    }

    // calculate hashes
    status(silent, "Calculating hashes . . .");

    let mut to_remove = Vec::new();
    
    for (size, mut targets) in sizes {
        if targets.len() == 1 { continue; }

        info(silent, &format!("Size: {}, els: {}", size, targets.len()));
        let hash = sum(BufReader::new(File::open(targets.remove(0))?), algorithm)?.as_ref().to_vec();
    
        for target in targets {
            if hash == sum(BufReader::new(File::open(target)?), algorithm)?.as_ref() {
                info(silent, &format!("Found duplicate: {}", target));
                to_remove.push(target);
            }
        }
    }

    info(silent, &format!("Files to delete: {}", to_remove.len().to_string().red()));

    for target in to_remove {
        info(silent, &format!("Delete file {:?}", target));

        match std::fs::remove_file(target) {
            Ok(_)  => (),
            Err(e) => error(silent, &format!("Failed to delete file {:?}: '{}'", target, e))
        };
    }

    status(silent, "All duplications were deleted");
    Ok(())
}

fn sum<R: Read>(mut reader: R, algorithm: &'static ring::digest::Algorithm) -> Result<Digest> {
    let mut context = Context::new(algorithm);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}