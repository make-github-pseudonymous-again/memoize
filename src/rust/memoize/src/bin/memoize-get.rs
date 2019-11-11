use std::env;
use std::fs;
use std::process;
use std::time::SystemTime;
use sha1::{Sha1, Digest};

fn main () {

    let args: Vec<String> = env::args().collect();

    let validity = &args[1];
    let validity: i64 = match validity.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("wrong validity {}, expected number (i32)", validity);
            process::exit(1);
        },
    };
    //eprintln!("validity: {}", validity);

    let cmdline = args[2..].to_vec();
    //eprintln!("cmdline: {:?}", cmdline);

    let mut hasher = Sha1::new();

    for arg in &cmdline {
        hasher.input(arg);
        hasher.input("\n");
    }

    let hash = hasher.result();

    let home_dir = dirs::home_dir()
        .expect("Impossible to get your home dir!");

    let cache = format!("{}/.cache/memoize/{:x}", home_dir.display(), hash);

    let out = format!("{}/O", cache);
    let err = format!("{}/E", cache);
    let ret = format!("{}/R", cache);
    let tim = format!("{}/T", cache);

    let output = match fs::read_to_string(out) {
        Ok(string) => string,
        Err(_) => {
            eprintln!("no entry for {:?}", cmdline);
            process::exit(1);
        }
    } ;

    if validity >= 0 {

        let validity = validity as u64;

        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(elapsed) => {

                let now = elapsed.as_secs();

                let previous: u64 = fs::read_to_string(tim)
                    .expect("Could not read tim file.")
                    .trim()
                    .parse()
                    .expect("Could not parse previous.");

                if previous + validity < now {
                    eprintln!("Entry expired for '{:?}'", cmdline);
                    process::exit(2);
                }

            },
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }

    }

    print!("{}",output);

    let error = fs::read_to_string(err)
        .expect("Could not read err file.");
    eprint!("{}",error);

    let rc: i32 = fs::read_to_string(ret)
        .expect("Could not read ret file.")
        .trim()
        .parse()
        .expect("Could not parse rc.");

    process::exit(rc);
}
