use std::process;

use memoize_lib::args;
use memoize_lib::cache;
use memoize_lib::time;

fn main () {

    let (validity, cmdline) = args::parse();

    let key = cache::get_key(&cmdline);

    let cache_path = cache::get_path(&key);

    if ! cache::exists(&cache_path) {
        eprintln!("no entry for {:?}", cmdline);
        process::exit(1);
    }

    if validity >= 0 {

        let validity = validity as u64;

        let now = time::now();

        let previous = cache::get_timestamp(&cache_path);

        if previous + validity < now {
            eprintln!("Entry expired for '{:?}'", cmdline);
            process::exit(2);
        }

    }

    let output = cache::get_output(&cache_path);
    print!("{}",output);

    let error = cache::get_error(&cache_path);
    eprint!("{}",error);

    let rc = cache::get_returncode(&cache_path);
    process::exit(rc);

}
