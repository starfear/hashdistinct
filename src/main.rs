extern crate clap;

use {
    colored::{Colorize},

    std::{
        collections::{HashMap},
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
                    .version("0.1.0")
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

    // calculate hashes
    let mut map: HashMap<Vec<u8>, Vec<String>> = HashMap::new();

    status(silent, "Calculating hashes . . .");

    let targets = matches.values_of("targets").unwrap();
    let tlen    = targets.len();

    let mut to_remove = Vec::new();

    for (index, target) in targets.enumerate() {
        info(silent, &format!("{:50.50} : {:?}%", target, (index*100)/tlen));

        // open file
        let file   = match File::open(&target) {
            Ok(v)  => v,
            Err(e) => {
                eprintln!("{} Failed to read file {:?}: '{}'", "ERROR".red(), target, e);
                
                continue;
            }
        };

        let reader = BufReader::new(&file);
        let sum    = match sum(reader, &algorithm) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{} Failed to read file {:?}: '{}'", "ERROR".red(), target, e);
                
                continue;
            }
        };
        let sum    = sum.as_ref();

        match map.get_mut(sum) {
            Some(_) => {
                to_remove.push(target);
            },

            None => {
                map.insert(sum.to_vec(), vec![target.into()]);
            }
        }
    }

    status(silent, "Calculating hashes done!");
    info(silent, &format!("Total files: {}, files to delete: {}", tlen.to_string().blue(), to_remove.len().to_string().red()));

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