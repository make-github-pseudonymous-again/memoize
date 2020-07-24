use std::env;
use std::process;

/// ```
/// use memoize_lib::args::_parse;
/// let v1: Vec<&str> = "/path/bin 0 Mary had a little lamb".split(' ').collect();
/// let v2: Vec<String> = v1.iter().map(|&s| String::from(s)).collect();
/// let (validity, cmdline) = _parse(&v2);
/// assert_eq!(validity, 0);
/// assert_eq!(cmdline.join(" "), "Mary had a little lamb");
/// ```
pub fn _parse ( args: &Vec<String> ) -> (i64, Vec<String>) {

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

pub fn parse ( ) -> (i64, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    return _parse(&args);
}

pub fn dump ( args: &Vec<String> ) {

    for arg in args {
        print!("{}\0",arg)
    }

}
