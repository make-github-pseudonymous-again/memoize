use std::env;
use std::process;

use memoize_lib::cache;

fn main () {

    let args: Vec<String> = env::args().collect();

    let key = cache::get_key(&args[1..].to_vec());

    print!("{}",key);

    process::exit(0);

}
