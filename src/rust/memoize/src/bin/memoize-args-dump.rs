use std::env;
use std::process;

use memoize_lib::args;

fn main () {

    let cmdline: Vec<String> = env::args().collect();

    args::dump(&cmdline[1..].to_vec());

    process::exit(0);

}
