use std::env;
use std::process;

pub fn parse ( ) -> (i64, Vec<String>) {

    let args: Vec<String> = env::args().collect();

    let validity = &args[1];
    let validity: i64 = match validity.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("wrong validity {}, expected number (i64)", validity);
            process::exit(1);
        },
    };

    let cmdline = args[2..].to_vec();

    return (validity, cmdline);
}
