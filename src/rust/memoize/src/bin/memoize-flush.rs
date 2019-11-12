use std::fs;

use memoize_lib::cache;

fn main () {

    let cache_root = cache::get_root();

    fs::remove_dir_all(cache_root)
        .expect("Could not remove cache root directory");

}
