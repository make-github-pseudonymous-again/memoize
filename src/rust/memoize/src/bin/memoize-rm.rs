use std::env;
use std::fs;
use std::io;

use memoize_lib::cache;

fn memoize_rm (cache_root: String, args: Vec<String>) {

    let cmdline = args[1..].to_vec();

    let key = cache::get_key(&cmdline);

    let cache_path = cache::get_path(&cache_root, &key);

    match fs::remove_dir_all(&cache_path) {
        Ok(_) => {},
        Err(error) => {
            match error.kind() {
                io::ErrorKind::NotFound => {},
                _ => panic!("Could not remove cache directory `{}`: {}", cache_path, error) ,
            }
        },
    };

}

fn main () {

    let cache_root = cache::get_root();
    let args: Vec<String> = env::args().collect();

    memoize_rm(cache_root, args);

}

#[cfg(test)]
mod tests {
    #[test]
    fn remove_non_existing_cache_dir() {
        use crate::memoize_rm;
        use tempfile::tempdir;

        let dir = tempdir()
            .expect("Could not create temporary directory.");

        let cache_root_str = dir.path().to_string_lossy();
        let cache_root = String::from(cache_root_str);

        let cmdline_args = "/path/bin 0 Mary had a little lamb";
        let args_str: Vec<&str> = cmdline_args.split(' ').collect();
        let args: Vec<String> = args_str.iter().map(|&s| String::from(s)).collect();

        memoize_rm(cache_root, args);

        dir.close()
            .expect("Could not delete temporary directory.");
    }
}
