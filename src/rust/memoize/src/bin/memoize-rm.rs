use std::env;
use std::fs;

use memoize_lib::cache;

fn main () {

    let args: Vec<String> = env::args().collect();

    let cmdline = args[1..].to_vec();

    let key = cache::get_key(&cmdline);

    let cache_path = cache::get_path(&key);

    fs::remove_dir_all(cache_path)
        .expect("Could not remove cache directory");

}
